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

/// Struct containing the Geometric Brownian Bridge parameters.
/// The Geometric Brownian Bridge is a stochastic process that models a path-dependent option.
/// It is a modification of the Geometric Brownian Motion where the end value is known.
pub struct GeometricBrownianBridge {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,
    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,
    /// The known end value of the process.
    pub end_value: f64,
    /// The known end time of the process.
    pub end_time: f64,
}

impl GeometricBrownianBridge {
    /// Create a new Geometric Brownian Bridge process.
    /// # Arguments
    /// * `mu` - The drift ($\mu$) in percentage.
    /// * `sigma` - The volatility ($\sigma$) in percentage.
    /// * `end_value` - The known end value of the process.
    /// * `end_time` - The known end time of the process.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        end_value: f64,
        end_time: f64,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            end_value,
            end_time,
        }
    }
}

impl StochasticProcess for GeometricBrownianBridge {
    /// The drift function for the Geometric Brownian Bridge.
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x + (self.end_value.ln() - x.ln()) / (self.end_time - t) * x
    }

    /// The diffusion function for the Geometric Brownian Bridge.
    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        self.sigma.0(t) * x
    }

    /// The jump function for the Geometric Brownian Bridge.
    /// As the Geometric Brownian Bridge does not have a jump term, this always returns None.
    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }

    fn parameters(&self) -> Vec<f64> {
        vec![
            self.mu.0(0.0),
            self.sigma.0(0.0),
            self.end_value,
            self.end_time,
        ]
    }
}

#[cfg(test)]
mod tests_gbm_bridge {
    use super::*;
    use crate::{StochasticProcessConfig, StochasticScheme};
    use RustQuant_math::*;
    use RustQuant_utils::assert_approx_equal;

    /// Test the Geometric Brownian Bridge process.
    #[test]
    fn test_geometric_brownian_motion_bridge() {
        let gbm = GeometricBrownianBridge::new(0.05, 0.9, 10.0, 0.5);

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
        assert_approx_equal!(E_XT, 10.0, 0.5);
        // V[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(V_XT, 0.0, 0.5);
    }
}
