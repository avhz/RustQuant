// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::{StochasticProcess, Trajectories};
use nalgebra::{DMatrix, DVector, Dim, Dyn, RowDVector};
use rand::Rng;
#[cfg(feature = "seedable")]
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::StandardNormal;
use rayon::prelude::*;

/// Struct containing the Fractional Brownian Motion parameters.
#[derive(Debug)]
pub struct FractionalBrownianMotion {
    /// Hurst parameter of the process.
    pub hurst: f64,
}

impl Default for FractionalBrownianMotion {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl FractionalBrownianMotion {
    /// Create a new Fractional Brownian Motion process.
    ///
    /// # Panics
    ///
    /// Will panic if Hurst parameter is not in [0, 1].
    #[must_use]
    pub fn new(hurst: f64) -> Self {
        assert!((0.0..=1.0).contains(&hurst));

        Self { hurst }
    }

    /// Autocovariance function (ACF).
    fn acf_vector(&self, n: usize) -> RowDVector<f64> {
        let h = self.hurst;

        let mut v = RowDVector::<f64>::zeros(n);
        v[0] = 1.0;

        for i in 1..n {
            let idx = i as f64;

            v[i] = 0.5
                * ((idx + 1.0).powf(2.0 * h) - 2.0 * idx.powf(2.0 * h) + (idx - 1.0).powf(2.0 * h));
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
    #[must_use]
    pub fn fgn_cholesky(&self, n: usize, t_n: f64) -> RowDVector<f64> {
        let acf_sqrt = self.acf_matrix_sqrt(n);
        let noise = rand::thread_rng()
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        (acf_sqrt * noise).transpose() * (1.0 * t_n / n as f64).powf(self.hurst)
    }

    #[cfg(feature = "seedable")]
    fn seedable_fgn_cholesky(&self, n: usize, t_n: f64, seed: u64) -> RowDVector<f64> {
        let acf_sqrt = self.acf_matrix_sqrt(n);
        let rng = StdRng::seed_from_u64(seed);
        let noise = rng
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        (acf_sqrt * noise).transpose() * (1.0 * t_n / n as f64).powf(self.hurst)
    }
}

impl StochasticProcess for FractionalBrownianMotion {
    fn drift(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        1.0
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
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
            let fgn = self.fgn_cholesky(n_steps, t_n);

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
    use crate::{assert_approx_equal, statistics::*};

    #[test]
    fn test_chol() {
        let fbm = FractionalBrownianMotion::new(0.7);
        let n = 3;
        let hurst = 0.7;

        let acf_vector = fbm.acf_vector(n);
        let acf_matrix = fbm.acf_matrix_sqrt(n);

        println!("ACF vector = {:?}", acf_vector);
        println!("ACF matrix = {:?}", acf_matrix);

        let noise = rand::thread_rng()
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        let fgn = (acf_matrix * noise).transpose() * (n as f64).powf(-hurst);

        println!("{:?}", fgn);
    }

    #[test]
    fn test_brownian_motion() {
        let fbm = FractionalBrownianMotion::new(0.7);
        let output_serial = fbm.euler_maruyama(0.0, 0.0, 0.5, 100, 1000, false);
        // let output_parallel = (&bm).euler_maruyama(10.0, 0.0, 0.5, 100, 10, true);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output_serial
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        // E[X_T] = 0
        assert_approx_equal!(X_T.mean(), 0.0, 0.5);
        // V[X_T] = T
        assert_approx_equal!(X_T.variance(), 0.5, 0.5);
    }
}
