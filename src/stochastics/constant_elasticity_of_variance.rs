// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::{StochasticProcess, TimeDependent};

/// Struct containing the CEV process parameters.
pub struct ConstantElasticityOfVariance {
    /// The long-run mean ($\mu$).
    pub mu: TimeDependent,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: TimeDependent,

    /// Elasticity parameter.
    /// Often denoted as $\beta$, $\rho$, or $\gamma$.
    /// Must be in the unit interval $[0, 1]$.
    pub elasticity: TimeDependent,
}

impl ConstantElasticityOfVariance {
    /// Create a new Cox-Ingersoll-Ross process.
    pub fn new(
        mu: impl Into<TimeDependent>,
        sigma: impl Into<TimeDependent>,
        elasticity: impl Into<TimeDependent>,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            elasticity: elasticity.into(),
        }
    }
}

impl StochasticProcess for ConstantElasticityOfVariance {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        assert!(self.elasticity.0(t) >= 0.0 && self.elasticity.0(t) <= 1.0);

        self.sigma.0(t) * x.powf(self.elasticity.0(t))
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_cev {
    use super::*;
    use crate::statistics::*;

    #[test]
    fn test_cev_process() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);

        let output = cev.euler_maruyama(10.0, 0.0, 0.5, 100, 100, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();

        // Make a PR if you know the mean and variance of the CEV process.

        // assert_approx_equal!(
        //     E_XT,
        //     10. * (-0.01 * 0.5_f64).exp() + 0.15 * (1. - (-0.01 * 0.5_f64).exp()),
        //     0.5
        // );

        // assert_approx_equal!(
        //     V_XT,
        //     10. * (0.45 * 0.45 / 0.01) * ((-0.01 * 0.5_f64).exp() - (-2. * 0.01 * 0.5_f64).exp())
        //         + (0.15 * 0.45 * 0.45 / (2. * 0.01)) * (1. - (-0.01 * 0.5_f64).exp()).powi(2),
        //     0.5
        // );
    }
}
