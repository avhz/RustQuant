// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;
use nalgebra::{DMatrix, DVector, Dim, Dyn, RowDVector};
use ndarray::{concatenate, prelude::*};
use ndrustfft::{ndfft, FftHandler};
use num_complex::Complex;
use rand::Rng;
use rand_distr::StandardNormal;
use rayon::prelude::*;

/// Method used to generate the Fractional Brownian Motion.
#[derive(Debug)]
pub enum FractionalProcessGeneratorMethod {
    /// Chooses the Cholesky decomposition method.
    CHOLESKY,
    /// Chooses the Davies-Harte method.
    FFT,
}

/// Struct containing the Fractional Brownian Motion parameters.
#[derive(Debug)]
pub struct FractionalBrownianMotion {
    /// Hurst parameter of the process.
    pub hurst: f64,
    /// Method used to generate the process.
    pub method: FractionalProcessGeneratorMethod,
}

impl Default for FractionalBrownianMotion {
    fn default() -> Self {
        Self::new(0.5, FractionalProcessGeneratorMethod::FFT)
    }
}

impl FractionalBrownianMotion {
    /// Create a new Fractional Brownian Motion process.
    pub fn new(hurst: f64, method: FractionalProcessGeneratorMethod) -> Self {
        assert!((0.0..=1.0).contains(&hurst));

        Self { hurst, method }
    }

    /// Autocovariance function (ACF).
    fn acf_vector(&self, n: usize) -> RowDVector<f64> {
        let h = self.hurst;

        let mut v = RowDVector::<f64>::zeros(n);
        v[0] = 1.0;

        for i in 1..n {
            let idx = i as f64;

            v[i] = 0.5
                * ((idx + 1.0).powf(2.0 * h) - 2.0 * idx.powf(2.0 * h) + (idx - 1.0).powf(2.0 * h))
        }

        v
    }

    /// Autocovariance matrix.
    fn acf_matrix_sqrt(&self, n: usize) -> DMatrix<f64> {
        let acf_vector = self.acf_vector(n);

        let mut m = DMatrix::<f64>::from_diagonal_element_generic(
            Dyn::from_usize(n),
            Dyn::from_usize(n),
            acf_vector[0],
        );

        for i in 1..n {
            for j in 0..i {
                m[(i, j)] = acf_vector[i - j];
            }
        }

        m.cholesky().unwrap().l()
    }

    /// Fractional Gaussian noise.
    pub fn fgn_cholesky(&self, n: usize, t_n: f64) -> Vec<f64> {
        let acf_sqrt = self.acf_matrix_sqrt(n);
        let noise = rand::thread_rng()
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);
        let noise = (acf_sqrt * noise).transpose() * (1.0 * t_n / n as f64).powf(self.hurst);

        noise.data.as_vec().clone()
    }

    /// Fractional Gaussian noise via FFT.
    pub fn fgn_fft(&self, n: usize, t_n: f64) -> Vec<f64> {
        if !(0.0..=1.0).contains(&self.hurst) {
            panic!("Hurst parameter must be between 0 and 1");
        }

        let r = Array::from_shape_fn((n + 1,), |i| {
            if i == 0 {
                1.0
            } else {
                0.5 * ((i as f64 + 1.0).powf(2.0 * self.hurst)
                    - 2.0 * (i as f64).powf(2.0 * self.hurst)
                    + (i as f64 - 1.0).powf(2.0 * self.hurst))
            }
        });

        let r = concatenate(
            Axis(0),
            &[r.view(), r.slice(s![..;-1]).slice(s![1..-1]).view()],
        )
        .unwrap();

        let mut data = Array1::<Complex<f64>>::zeros(r.len());
        for (i, v) in r.iter().enumerate() {
            data[i] = Complex::new(*v, 0.0);
        }
        let mut r_fft = FftHandler::new(r.len());
        let mut lambda = Array1::<Complex<f64>>::zeros(r.len());
        ndfft(&data, &mut lambda, &mut r_fft, 0);
        let lambda = lambda.mapv(|x| x.re / (2.0 * n as f64)).mapv(f64::sqrt);

        let mut rng = rand::thread_rng();
        let mut rnd = Array1::<Complex<f64>>::zeros(2 * n);
        for (_, v) in rnd.iter_mut().enumerate() {
            let real: f64 = rng.sample(StandardNormal);
            let imag: f64 = rng.sample(StandardNormal);
            *v = Complex::new(real, imag);
        }

        let mut fgn = Array1::<Complex<f64>>::zeros(2 * n);
        for (i, v) in rnd.iter().enumerate() {
            fgn[i] = lambda[i] * v;
        }
        let mut fgn_fft_handler = FftHandler::new(2 * n);
        let mut fgn_fft = Array1::<Complex<f64>>::zeros(2 * n);
        ndfft(&fgn, &mut fgn_fft, &mut fgn_fft_handler, 0);

        let fgn = fgn_fft.slice(s![1..n + 1]).mapv(|x| x.re);
        fgn.mapv(|x| (x * (n as f64).powf(-self.hurst)) * t_n.powf(self.hurst))
            .to_vec()
    }
}

impl StochasticProcess for FractionalBrownianMotion {
    fn drift(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        1.0
    }

    fn jump(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }

    fn euler_maruyama(
        &self,
        x_0: f64,
        t_0: f64,
        t_n: f64,
        n_steps: usize,
        m_paths: usize,
        parallel: bool,
    ) -> Trajectories {
        assert!(t_0 < t_n);

        let dt: f64 = (t_n - t_0) / (n_steps as f64);

        // Initialise empty paths and fill in the time points.
        let mut paths = vec![vec![x_0; n_steps + 1]; m_paths];
        let times: Vec<f64> = (0..=n_steps).map(|t| t_0 + dt * (t as f64)).collect();

        let path_generator = |path: &mut Vec<f64>| {
            let fgn = match self.method {
                FractionalProcessGeneratorMethod::FFT => self.fgn_fft(n_steps, t_n),
                FractionalProcessGeneratorMethod::CHOLESKY => self.fgn_cholesky(n_steps, t_n),
            };

            for t in 0..n_steps {
                path[t + 1] = path[t]
                    + self.drift(path[t], times[t]) * dt
                    + self.diffusion(path[t], times[t]) * fgn[t];
            }
        };

        if parallel {
            paths.par_iter_mut().for_each(path_generator);
        } else {
            paths.iter_mut().for_each(path_generator);
        }

        Trajectories { times, paths }
    }

    #[cfg(feature = "seedable")]
    fn seedable_euler_maruyama(
        &self,
        x_0: f64,
        t_0: f64,
        t_n: f64,
        n_steps: usize,
        m_paths: usize,
        parallel: bool,
        seed: u64,
    ) -> Trajectories {
        assert!(t_0 < t_n);

        let dt: f64 = (t_n - t_0) / (n_steps as f64);

        // Initialise empty paths and fill in the time points.
        let mut paths = vec![vec![x_0; n_steps + 1]; m_paths];
        let times: Vec<f64> = (0..=n_steps).map(|t| t_0 + dt * (t as f64)).collect();

        let path_generator = |path: &mut Vec<f64>| {
            let fgn = self.seedable_fgn_cholesky(n_steps, t_n, seed);

            for t in 0..n_steps {
                path[t + 1] = path[t]
                    + self.drift(path[t], times[t]) * dt
                    + self.diffusion(path[t], times[t]) * fgn[t];
            }
        };

        if parallel {
            paths.par_iter_mut().for_each(path_generator);
        } else {
            paths.iter_mut().for_each(path_generator);
        }

        Trajectories { times, paths }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_fractional_brownian_motion {
    // use std::time::Instant;

    use super::*;
    use crate::ml::{Decomposition, LinearRegressionInput};

    fn higuchi_fd(x: &Vec<f64>, kmax: usize) -> f64 {
        let n_times = x.len();

        let mut lk = vec![0.0; kmax];
        let mut x_reg = vec![0.0; kmax];
        let mut y_reg = vec![0.0; kmax];

        for k in 1..=kmax {
            let mut lm = vec![0.0; k];

            for m in 0..k {
                let mut ll = 0.0;
                let n_max = ((n_times - m - 1) as f64 / k as f64).floor() as usize;

                for j in 1..n_max {
                    ll += (x[m + j * k] - x[m + (j - 1) * k]).abs();
                }

                ll /= k as f64;
                ll *= (n_times - 1) as f64 / (k * n_max) as f64;
                lm[m] = ll;
            }

            lk[k - 1] = lm.iter().sum::<f64>() / k as f64;
            x_reg[k - 1] = (1.0 / k as f64).ln();
            y_reg[k - 1] = lk[k - 1].ln();
        }

        let x_reg = DMatrix::from_vec(kmax, 1, x_reg);
        let y_reg = DVector::from_vec(y_reg);
        let linear_regression = LinearRegressionInput::new(x_reg, y_reg);
        let regression_result = linear_regression.fit(Decomposition::None).unwrap();

        regression_result.coefficients[0]
    }

    #[test]
    fn test_chol() {
        let fbm = FractionalBrownianMotion::new(0.7, FractionalProcessGeneratorMethod::FFT);
        let hursts = vec![0.1, 0.3, 0.5, 0.7, 0.9];

        for hurst in hursts {
            let fbm = FractionalBrownianMotion::fgn_cholesky(&fbm, 2000, 1.0);
            let higuchi_fd = higuchi_fd(&fbm.to_vec(), 10);
            let est_hurst = 2.0 - higuchi_fd;
            print!("hurst: {}, higuchi_fd: {}\n", hurst, est_hurst);
            assert!(est_hurst - hurst < 0.05);
        }
    }

    #[test]
    fn test_fft() {
        let fbm = FractionalBrownianMotion::new(0.7, FractionalProcessGeneratorMethod::FFT);
        let hursts = vec![0.1, 0.3, 0.5, 0.7, 0.9];

        for hurst in hursts {
            let fbm = FractionalBrownianMotion::fgn_fft(&fbm, 2000, 1.0);
            let higuchi_fd = higuchi_fd(&fbm.to_vec(), 10);
            let est_hurst = 2.0 - higuchi_fd;
            print!("hurst: {}, higuchi_fd: {}\n", hurst, est_hurst);
            assert!(est_hurst - hurst < 0.05);
        }
    }
}
