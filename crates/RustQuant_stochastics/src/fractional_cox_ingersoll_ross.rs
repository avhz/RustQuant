// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::process::{StochasticProcessConfig, StochasticProcess, Trajectories};
use crate::fractional_process::{fractional_monte_carlo, FractionalProcessGeneratorMethod};
use crate::model_parameter::ModelParameter;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
pub struct FractionalCoxIngersollRoss {
    /// The long-run mean ($\mu$).
    pub mu: ModelParameter,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: ModelParameter,

    /// Mean reversion parameter ($\theta$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub theta: ModelParameter,

    /// Hurst parameter of the process.
    /// The Hurst parameter is a measure of the long-term memory of the process.
    pub hurst: f64,

    /// Method to generate Fractional Gaussian Noise.
    pub method: FractionalProcessGeneratorMethod,
}

impl FractionalCoxIngersollRoss {
    /// Create a new Ornstein-Uhlenbeck process.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        theta: impl Into<ModelParameter>,
        hurst: f64,
        method: FractionalProcessGeneratorMethod,
    ) -> Self {
        assert!((0.0..=1.0).contains(&hurst));
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            theta: theta.into(),
            hurst,
            method,
        }
    }
}

impl StochasticProcess for FractionalCoxIngersollRoss {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.theta.0(t) * (self.mu.0(t) - x)
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        self.sigma.0(t) * x.sqrt()
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        Some(0.0)
    }

    fn parameters(&self) -> Vec<f64> {
        vec![
            self.mu.0(0.0),
            self.sigma.0(0.0),
            self.theta.0(0.0),
            self.hurst,
        ]
    }

    fn monte_carlo(&self, config: &StochasticProcessConfig) -> Trajectories {
        fractional_monte_carlo(self, config, &self.method, self.hurst)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_fractional_cir {
    use super::*;
    use crate::{fractional_ornstein_uhlenbeck::FractionalOrnsteinUhlenbeck, StochasticScheme};

    #[test]
    #[ignore = "Hard to test."]
    fn test_fractional_cir() -> Result<(), Box<dyn std::error::Error>> {
        let fou = FractionalOrnsteinUhlenbeck::new(
            0.15,
            0.45,
            0.01,
            0.7,
            FractionalProcessGeneratorMethod::FFT,
        );

        let config = StochasticProcessConfig::new(
            10.0, 0.0, 0.5, 100, StochasticScheme::EulerMaruyama, 100, false, None
        );

        #[allow(dead_code)]
        let _output = fou.monte_carlo(&config);

        std::result::Result::Ok(())
    }
}
