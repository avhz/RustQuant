// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! `Trajectories` is the return type of all the stochastic processes.
//! `StochasticProcess` is the base trait for all the stochastic processes.
//!
//! Currently only autonomous stochastic processes are implemented.
//! Autonomous refers to processes where the drift and diffusion
//! do not explicitly depend on the time `t`.

use nalgebra::{DMatrix, DVector, Dim, Dyn, RowDVector};
use rand::Rng;
use rand_distr::StandardNormal;
use rayon::prelude::*;
use std::cmp::Ordering::{Equal, Greater, Less};

#[cfg(feature = "seedable")]
use rand::{rngs::StdRng, SeedableRng};

/// Struct to contain the time points and path values of the process.
pub struct Trajectories {
    /// Vector of time points.
    pub times: Vec<f64>,
    /// Vector of process trajectories.
    pub paths: Vec<Vec<f64>>,
}

/// Trait to implement stochastic processes.
pub trait FractionalStochasticProcess: Sync {
    /// Base method for the process' drift.
    fn drift(&self, x: f64, t: f64) -> f64;

    /// Base method for the process' diffusion.
    fn diffusion(&self, x: f64, t: f64) -> f64;

    /// Base method for the process' jump term (if applicable).
    fn jump(&self, x: f64, t: f64) -> f64;

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
    fn fgn(&self, n: usize, hurst: f64) -> RowDVector<f64> {
        let acf_sqrt = self.afc_matrix_sqrt(n, hurst);
        let noise = rand::thread_rng()
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        (acf_sqrt * noise).transpose() * (n as f64).powf(-hurst)
    }

    /// Fractional Gaussian noise.
    #[cfg(feature = "seedable")]
    fn seedable_fgn(&self, n: usize, hurst: f64, seed: u64) -> RowDVector<f64> {
        use rand::rngs::StdRng;

        let acf_sqrt = self.afc_matrix_sqrt(n, hurst);
        let noise = StdRng::seed_from_u64(seed)
            .sample_iter::<f64, StandardNormal>(StandardNormal)
            .take(n)
            .collect();
        let noise = DVector::<f64>::from_vec(noise);

        (acf_sqrt * noise).transpose() * (n as f64).powf(-hurst)
    }

    /// Euler-Maruyama discretisation scheme.
    ///
    /// # Arguments:
    /// * `x_0` - The process' initial value at `t_0`.
    /// * `t_0` - The initial time point.
    /// * `t_n` - The terminal time point.
    /// * `n_steps` - The number of time steps between `t_0` and `t_n`.
    /// * `m_paths` - How many process trajectories to simulate.
    /// * `parallel` - Run in parallel or not (recommended for > 1000 paths).
    /// * `hurst` - The Hurst parameter of the fractional Brownian motion.
    fn euler_maruyama(
        &self,
        x_0: f64,
        t_0: f64,
        t_n: f64,
        n_steps: usize,
        m_paths: usize,
        parallel: bool,
        hurst: f64,
    ) -> Trajectories {
        assert!(t_0 < t_n);

        let dt: f64 = (t_n - t_0) / (n_steps as f64);

        // Initialise empty paths and fill in the time points.
        let mut paths = vec![vec![x_0; n_steps + 1]; m_paths];
        let times: Vec<f64> = (0..=n_steps).map(|t| t_0 + dt * (t as f64)).collect();

        let path_generator = |path: &mut Vec<f64>| {
            let fgn = self.fgn(n_steps, hurst);
            path[0] = 0.0;

            for t in 1..n_steps {
                path[t] = path[t - 1]
                    + self.drift(path[t - 1], times[t - 1]) * dt
                    + self.diffusion(path[t - 1], times[t]) * fgn[t - 1] * t_n.powf(hurst);
            }
        };

        if parallel {
            paths.par_iter_mut().for_each(path_generator);
        } else {
            paths.iter_mut().for_each(path_generator);
        }

        Trajectories { times, paths }
    }

    /// Euler-Maruyama discretisation scheme with a choice of random seed.
    ///
    /// # Arguments:
    /// * `x_0` - The process' initial value at `t_0`.
    /// * `t_0` - The initial time point.
    /// * `t_n` - The terminal time point.
    /// * `n_steps` - The number of time steps between `t_0` and `t_n`.
    /// * `m_paths` - How many process trajectories to simulate.
    /// * `parallel` - Run in parallel or not (recommended for > 1000 paths).
    /// * `seed` - The seed for the random number generator.
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
        hurst: f64,
    ) -> Trajectories {
        assert!(t_0 < t_n);

        let dt: f64 = (t_n - t_0) / (n_steps as f64);

        // Initialise empty paths and fill in the time points.
        let mut paths = vec![vec![x_0; n_steps + 1]; m_paths];
        let times: Vec<f64> = (0..=n_steps).map(|t| t_0 + dt * (t as f64)).collect();

        let path_generator = |path: &mut Vec<f64>| {
            let fgn = self.seedable_fgn(n_steps, hurst, seed);
            path[0] = 0.0;

            for t in 1..n_steps {
                path[t] = path[t - 1]
                    + self.drift(path[t - 1], times[t - 1]) * dt
                    + self.diffusion(path[t - 1], times[t]) * fgn[t - 1] * t_n.powf(hurst);
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

#[cfg(test)]
mod test_process {
    use super::*;
    use crate::stochastics::FractionalBrownianMotion;
    use std::time::Instant;

    #[test]
    fn test_euler_maruyama() {
        let fbm = FractionalBrownianMotion::new(0.7);

        let start = Instant::now();
        fbm.euler_maruyama(10.0, 0.0, 1.0, 2000, 1, false, fbm.hurst);
        let serial = start.elapsed();

        println!("Serial: \t {:?}", serial);

        let start = Instant::now();
        fbm.euler_maruyama(10.0, 0.0, 1.0, 125, 1, true, fbm.hurst);
        let parallel = start.elapsed();

        println!("Parallel: \t {:?}", parallel);

        // Just checking that `parallel = true` actually works.
        // To see the output of this "test", run:
        // cargo test test_process -- --nocapture
    }

    #[cfg(feature = "seedable")]
    #[test]
    fn test_seedable_maruyama() {
        let fbm = FractionalBrownianMotion::new(0.7);

        let output_first_seed =
            fbm.seedable_euler_maruyama(10.0, 0.0, 1.0, 125, 10000, true, 123456789, 0.7);
        println!("First seed: \t {:?}", output_first_seed.paths[0][125]);

        let output_same_seed =
            fbm.seedable_euler_maruyama(10.0, 0.0, 1.0, 125, 10000, true, 123456789, 0.7);
        println!("Same seed: \t {:?}", output_same_seed.paths[0][125]);

        // Check that using the same seed gives the same output.
        assert_eq!(output_first_seed.paths, output_same_seed.paths);

        let output_different_seed =
            fbm.seedable_euler_maruyama(10.0, 0.0, 1.0, 125, 10000, true, 987654321, 0.7);
        println!("Different seed: {:?}", output_different_seed.paths[0][125]);

        // Check that using a different seed gives a different output.
        assert_ne!(output_first_seed.paths, output_different_seed.paths);

        // To see the output of this "test", run:
        // cargo test test_process -- --nocapture
    }
}
