// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use nalgebra::{DMatrix, DVector, Dim, Dyn, RowDVector};
use rand::Rng;
use rand_distr::StandardNormal;
use rayon::prelude::*;
use std::cmp::Ordering::{Equal, Greater, Less};

#[cfg(feature = "seedable")]
use rand::{rngs::StdRng, SeedableRng};

use crate::stochastics::*;

/// Struct containin the Geometric Brownian Motion parameters.
#[derive(Debug)]
pub struct FractionalBrownianMotion {
    /// Hurst parameter of the process.
    pub hurst: f64,
}

impl Default for FractionalBrownianMotion {
    fn default() -> Self {
        Self::new(0.7)
    }
}

impl FractionalBrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new(hurst: f64) -> Self {
        assert!(hurst >= 0.0 && hurst <= 1.0);
        Self { hurst }
    }

    /// Autocovariance function.
    fn afc_vector(&self, n: usize, hurst: f64) -> RowDVector<f64> {
        let mut v = RowDVector::<f64>::zeros(n);
        v[0] = 1.0;

        for i in 1..n {
            let idx = i as f64;

            v[i] = 0.5
                * ((idx + 1.0).powf(2.0 * hurst) - 2.0 * idx.powf(2.0 * hurst)
                    + (idx - 1.0).powf(2.0 * hurst))
        }

        v
    }

    /// Autocovariance matrix.
    fn afc_matrix_sqrt(&self, n: usize, hurst: f64) -> DMatrix<f64> {
        let acf_v = self.afc_vector(n, hurst);
        let mut m = DMatrix::<f64>::zeros_generic(Dyn::from_usize(n), Dyn::from_usize(n));

        for i in 0..n {
            for j in 0..n {
                match i.cmp(&j) {
                    Equal => m[(i, j)] = acf_v[0],
                    Greater => m[(i, j)] = acf_v[i - j],
                    Less => continue,
                }
            }
        }

        m.cholesky().unwrap().l()
    }

    /// Fractional Gaussian noise.
    fn fgn_cholesky(&self, n: usize, hurst: f64) -> RowDVector<f64> {
        let acf_sqrt = self.afc_matrix_sqrt(n, hurst);
        let noise = rand::thread_rng()
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        (acf_sqrt * noise).transpose() * (n as f64).powf(-hurst)
    }

    #[cfg(feature = "seedable")]
    fn seedable_fgn_cholesky(&self, n: usize, hurst: f64, seed: u64) -> RowDVector<f64> {
        let acf_sqrt = self.afc_matrix_sqrt(n, hurst);
        let mut rng = StdRng::seed_from_u64(seed);
        let noise = rng
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        (acf_sqrt * noise).transpose() * (n as f64).powf(-hurst)
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
            let fgn = self.fgn_cholesky(n_steps, self.hurst);

            for t in 0..n_steps {
                path[t + 1] = path[t]
                    + self.drift(path[t], times[t]) * dt
                    + self.diffusion(path[t], times[t]) * fgn[t] * t_n.powf(self.hurst);
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
            let fgn = self.seedable_fgn_cholesky(n_steps, self.hurst, seed);

            for t in 0..n_steps {
                path[t + 1] = path[t]
                    + self.drift(path[t], times[t]) * dt
                    + self.diffusion(path[t], times[t]) * fgn[t] * t_n.powf(self.hurst);
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
mod sde_tests {
    // use std::time::Instant;

    use super::*;
    use crate::{assert_approx_equal, utilities::*};

    #[test]
    fn test_brownian_motion() -> Result<(), Box<dyn std::error::Error>> {
        let fbm = FractionalBrownianMotion::new(0.7);

        // AT LEAST 100 PATHS BEFORE PARALLEL IS WORTH IT.
        // for _steps in [1, 10, 100, 1000] {
        //     for paths in [1, 10, 100, 1000] {
        //         let start_serial = Instant::now();
        //         (&bm).euler_maruyama(10.0, 0.0, 0.5, 1000, paths, false);
        //         let duration_serial = start_serial.elapsed();

        //         let start_parallel = Instant::now();
        //         (&bm).euler_maruyama(10.0, 0.0, 0.5, 1000, paths, true);
        //         let duration_parallel = start_parallel.elapsed();

        //         println!(
        //             "{},{},{:?},{:?}",
        //             1000,
        //             paths,
        //             duration_serial.as_micros(),
        //             duration_parallel.as_micros()
        //         );
        //     }
        // }
        // assert!(1 == 2);

        let output_serial = fbm.euler_maruyama(0.0, 0.0, 0.5, 100, 1000, false);
        // let output_parallel = (&bm).euler_maruyama(10.0, 0.0, 0.5, 100, 10, true);

        // let file1 = "./images/BM1.png";
        // plot_vector((&output_serial.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./images/BM2.png";
        // plot_vector((&output_serial.trajectories[1]).clone(), file2).unwrap();
        // let file2 = "./images/BM3_parallel.png";
        // plot_vector((&output_parallel.trajectories[0]).clone(), file2)

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output_serial
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        let V_XT = variance(&X_T, VarianceType::Sample);
        // E[X_T] = 0
        assert_approx_equal!(E_XT, 0.0, 0.5);
        // V[X_T] = T
        assert_approx_equal!(V_XT, 0.5, 0.5);

        std::result::Result::Ok(())
    }
}
