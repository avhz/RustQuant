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

/// Struct containing the Geometric Brownian Motion parameters.
pub struct GeometricBrownianMotion {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,
}

impl GeometricBrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new(mu: impl Into<ModelParameter>, sigma: impl Into<ModelParameter>) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
        }
    }

    /// Return the parameters as a `Vec<f64>`.
    pub fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0)]
    }

    /// Unpack the parameters from a Vec<f64>.
    pub fn unpack(&self) -> (f64, f64) {
        let p = self.parameters();

        (p[0], p[1])
    }
}

impl StochasticProcess for GeometricBrownianMotion {
    fn drift(&self, x: f64, t: f64) -> f64 {
        // mu X_t dt
        self.mu.0(t) * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        // sigma X_t dW_t
        self.sigma.0(t) * x
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0)]
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_gbm {
    use super::*;
    use crate::{StochasticProcessConfig, StochasticScheme};
    use RustQuant_math::*;
    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn test_geometric_brownian_motion() {
        let gbm = GeometricBrownianMotion::new(0.05, 0.9);

        let config = StochasticProcessConfig::new(
            10.0, 0.0, 0.5, 125, StochasticScheme::EulerMaruyama, 10000, false, None
        );
        let output = gbm.monte_carlo(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

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

        // let file1 = "./images/GBM1.png";
        // plot_vector((&output.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./images/GBM2.png";
        // plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
