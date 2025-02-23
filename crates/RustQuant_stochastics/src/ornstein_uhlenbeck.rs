// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::model_parameter::ModelParameter;
use crate::process::StochasticProcess;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
pub struct OrnsteinUhlenbeck {
    /// The long-run mean ($\mu$).
    pub mu: ModelParameter,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: ModelParameter,

    /// Mean reversion parameter ($\theta$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub theta: ModelParameter,
}

impl OrnsteinUhlenbeck {
    /// Create a new Ornstein-Uhlenbeck process.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        theta: impl Into<ModelParameter>,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            theta: theta.into(),
        }
    }
}

impl StochasticProcess for OrnsteinUhlenbeck {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.theta.0(t) * (self.mu.0(t) - x)
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        self.sigma.0(t)
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0), self.theta.0(0.0)]
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_ornstein_uhlenbeck {
    use super::*;
    use crate::{StochasticProcessConfig, StochasticScheme};
    use RustQuant_math::*;
    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn test_ornstein_uhlenbeck() {
        let ou = OrnsteinUhlenbeck::new(0.15, 0.45, 0.01);

        let config = StochasticProcessConfig::new(
            10.0, 0.0, 0.5, 100, StochasticScheme::EulerMaruyama,100, false, None
        );
        let output = ou.monte_carlo(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = https://en.wikipedia.org/wiki/Ornstein%E2%80%93Uhlenbeck_process
        assert_approx_equal!(
            E_XT,
            10. * (-0.01 * 0.5_f64).exp() + 0.15 * (1. - (-0.01 * 0.5_f64).exp()),
            0.5
        );
        // V[X_T] = https://en.wikipedia.org/wiki/Ornstein%E2%80%93Uhlenbeck_process
        assert_approx_equal!(
            V_XT,
            (0.45 * 0.45 / (2. * 0.01)) * (1. - (-2. * 0.01 * 0.5_f64).exp()),
            0.5
        );

        // let file1 = "./images/OU1.png";
        // plot_vector((&output.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./images/OU2.png";
        // plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
