use crate::stochastics::*;

use crate::statistics::{Distribution as LocalDistribution, Gaussian, Poisson};
use rand::prelude::Distribution;
use rayon::prelude::*;
use statrs::distribution::Normal;

/// Struct containing the Merton Jump Diffusion parameters.
/// The Merton Jump Diffusion is a stochastic process that models a path-dependent option.
/// It is a modification of the Geometric Brownian Motion where the end value is known.
pub struct MertonJumpDiffusion {
    /// The drift ($\mu$) in percentage.
    mu: TimeDependent,

    /// The volatility ($\sigma$) in percentage.
    sigma: TimeDependent,

    /// The jump intensity ($\lambda$) in percentage.
    lambda: TimeDependent,

    /// The Gaussian distribution for the jump size.
    gaussian: Gaussian,
}

impl MertonJumpDiffusion {
    /// Create a new Merton Jump Diffusion process.
    /// # Arguments
    /// * `mu` - The drift ($\mu$) in percentage.
    /// * `sigma` - The volatility ($\sigma$) in percentage.
    /// * `lambda` - The jump intensity ($\lambda$) in percentage.
    /// * `m` - The mean of the Gaussian distribution for the jump size.
    /// * `v` - The variance of the Gaussian distribution for the jump size.
    pub fn new(
        mu: impl Into<TimeDependent>,
        sigma: impl Into<TimeDependent>,
        lambda: impl Into<TimeDependent>,
        m: f64,
        v: f64,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            lambda: lambda.into(),
            gaussian: Gaussian::new(m, v),
        }
    }
}

impl StochasticProcess for MertonJumpDiffusion {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        self.sigma.0(t) * x
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        self.gaussian.sample(1).unwrap().first().cloned()
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
            let mut rng = rand::thread_rng();
            let scale = dt.sqrt();
            let dW: Vec<f64> = Normal::new(0.0, 1.0)
                .unwrap()
                .sample_iter(&mut rng)
                .take(n_steps)
                .map(|z| z * scale)
                .collect();

            let jumps = Poisson::new(self.lambda.0(0.0) * dt)
                .sample(n_steps)
                .unwrap();

            for t in 0..n_steps {
                if jumps[t] > 0.0 {
                    path[t + 1] = path[t]
                        + self.drift(path[t], times[t]) * dt
                        + self.diffusion(path[t], times[t]) * dW[t]
                        + self.jump(path[t], times[t]).unwrap_or(0.0);
                } else {
                    path[t + 1] = path[t]
                        + self.drift(path[t], times[t]) * dt
                        + self.diffusion(path[t], times[t]) * dW[t]
                }
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
mod tests_gbm_bridge {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    #[test]
    fn test_geometric_brownian_motion_bridge() -> Result<(), Box<dyn std::error::Error>> {
        let mjd = MertonJumpDiffusion::new(0.05, 0.9, 1.0, 0.0, 0.3);

        let output = mjd.euler_maruyama(10.0, 0.0, 0.5, 125, 10000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        println!("X_T = {:?}", X_T);

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(E_XT, 10. * (0.05 * 0.5_f64).exp(), 0.5);
        // V[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(
            V_XT,
            10. * 10. * (2. * 0.05 * 0.5_f64).exp() * ((0.9 * 0.9 * 0.5_f64).exp() - 1.),
            5.0
        );

        std::result::Result::Ok(())
    }
}
