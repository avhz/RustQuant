// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::model_parameter::ModelParameter;
use crate::process::{StochasticProcess, StochasticProcessConfig};
use crate::simulation::simulate_stochatic_process;
use RustQuant_math::Gaussian;
use RustQuant_math::Distribution as LocalDistribution;

/// Struct containing the Merton Jump Diffusion parameters.
/// The Merton Jump Diffusion is a stochastic process that models a path-dependent option.
/// It is a modification of the Geometric Brownian Motion where the end value is known.
pub struct MertonJumpDiffusion {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,

    /// The jump intensity ($\lambda$) in percentage.
    pub lambda: ModelParameter,

    /// The Gaussian distribution for the jump size.
    pub gaussian: Gaussian,
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
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        lambda: impl Into<ModelParameter>,
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
        self.gaussian.sample(1).unwrap().first().copied()
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0), self.lambda.0(0.0)]
    }

    fn generate(&self, config: &StochasticProcessConfig) -> crate::process::Trajectories {
        simulate_stochatic_process(self, config, Some(self.lambda.0(0.0)), None)
    }
}

#[cfg(test)]
mod tests_gbm_bridge {
    use super::*;
    use crate::{StochasticProcessConfig, StochasticScheme};
    use RustQuant_math::*;
    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn test_geometric_brownian_motion_bridge() {
        let mjd = MertonJumpDiffusion::new(0.05, 0.9, 1.0, 0.0, 0.3);
        let config = StochasticProcessConfig::new(
            10.0, 0.0, 0.5, 125, StochasticScheme::EulerMaruyama, 10000, false, None
        );
        let output = mjd.generate(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
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
    }
}
