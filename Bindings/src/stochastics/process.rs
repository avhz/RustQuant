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

use rand::prelude::Distribution;
use rayon::prelude::*;
use statrs::distribution::Normal;

/// Struct to contain the time points and path values of the process.
pub struct Trajectories {
    /// Vector of time points.
    pub times: Vec<f64>,
    /// Vector of process trajectories.
    pub paths: Vec<Vec<f64>>,
}

/// Trait to implement stochastic processes.
pub trait StochasticProcess: Sync {
    /// Base method for the process' drift.
    fn drift(&self, x: f64) -> f64;

    /// Base method for the process' diffusion.
    fn diffusion(&self, x: f64) -> f64;

    /// Base method for the process' jump term (if applicable).
    fn jump(&self, x: f64) -> f64;

    /// Euler-Maruyama discretisation scheme.
    ///
    /// # Arguments:
    /// * `x_0` - The process' initial value at `t_0`.
    /// * `t_0` - The initial time point.
    /// * `t_n` - The terminal time point.
    /// * `n_steps` - The number of time steps between `t_0` and `t_n`.
    /// * `m_paths` - How many process trajectories to simulate.
    /// * `parallel` - Run in parallel or not (recommended for > 1000 paths).
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
            let mut rng = rand::thread_rng();
            let scale = dt.sqrt();
            let dW: Vec<f64> = Normal::new(0.0, 1.0)
                .unwrap()
                .sample_iter(&mut rng)
                .take(n_steps)
                .map(|z| z * scale)
                .collect();

            for t in 0..n_steps {
                path[t + 1] = path[t] + self.drift(path[t]) * dt + self.diffusion(path[t]) * dW[t];
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
    use crate::stochastics::GeometricBrownianMotion;
    use std::time::Instant;

    #[test]
    fn test_euler_maruyama() {
        let gbm = GeometricBrownianMotion::new(0.05, 0.9);

        let start = Instant::now();
        (&gbm).euler_maruyama(10.0, 0.0, 1.0, 125, 10000, false);
        let serial = start.elapsed();

        println!("Serial: \t {:?}", serial);

        let start = Instant::now();
        (&gbm).euler_maruyama(10.0, 0.0, 1.0, 125, 10000, true);
        let parallel = start.elapsed();

        println!("Parallel: \t {:?}", parallel);

        // Just checking that `parallel = true` actually works.
        // To see the output of this "test", run:
        // cargo test test_process -- --nocapture
    }
}
