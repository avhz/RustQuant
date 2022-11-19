#![deny(missing_docs)]

//! Module that contains the struct `Trajectories` and trait `StochasticProcess`.
//!
//! `Trajectories` is the return type of all the stochastic processes.
//! `StochasticProcess` is the base trait for all the stochastic processes.
//! Currently only autonomous stochastic processes are implemented.
//! Autonomous refers to processes where the drift and diffusion
//! do not explicitly depend on the time `t`.

/// Struct to contain the time points and path values of the process.
pub struct Trajectories {
    /// Vector of time points.
    pub time: Vec<f64>,
    /// Vector of process trajectories (can have more than one trajectory).
    pub trajectories: Vec<Vec<f64>>,
}

/// Trait to implement stochastic processes.
pub trait StochasticProcess {
    /// Base method for the process' drift.
    fn drift(&self, x: f64) -> f64;

    /// Base method for the process' diffusion.
    fn diffusion(&self, x: f64) -> f64;

    /// Euler-Maruyama discretisation scheme.
    ///
    /// # Arguments:
    /// * `x_0` - The process' initial value at `t_0`.
    /// * `t_0` - The initial time point.
    /// * `t_n` - The terminal time point.
    /// * `n` - The number of time steps between `t_0` and `t_n`.
    /// * `sims` - How many process trajectories to simulate.
    fn euler_maruyama(&self, x_0: f64, t_0: f64, t_n: f64, n: usize, sims: usize) -> Trajectories {
        use rand::prelude::Distribution;

        let dt: f64 = (t_n - t_0) / (n as f64);

        // Initialise empty paths.
        let mut paths = vec![vec![0.0; n + 1]; sims];
        let mut times = vec![0.0; n + 1];

        // Fill time points.
        times[0] = t_0;
        times[n] = t_n;
        for t in 1..(n + 1) {
            times[t] = t_0 + dt * (t as f64);
        }

        // Generate trajectories:
        for i in 0..sims {
            // Set up rng.
            let mut rng = rand::thread_rng();
            let increments: Vec<f64> = match statrs::distribution::Normal::new(0.0, 1.0) {
                Ok(dist) => dist,
                Err(_) => panic!("Please check the parameters ..."),
            }
            .sample_iter(&mut rng)
            .take(n)
            .map(|x| (x as f64) * dt.sqrt())
            .collect();

            paths[i][0] = x_0;

            for t in 0..n {
                paths[i][t + 1] = paths[i][t]
                    + self.drift(paths[i][t]) * (times[t + 1] - times[t])
                    + self.diffusion(paths[i][t]) * increments[t];
            }
        }

        Trajectories {
            time: times,
            trajectories: paths,
        }
    }
}
