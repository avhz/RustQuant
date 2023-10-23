// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containing the extended Vasicek process parameters.
pub struct ExtendedVasicek {
    /// Mean function ($\mu(t)$)
    pub alpha_t: fn(f64) -> f64,
    /// Non-negative diffusion, or instantaneous time-varying volatility ($\sigma$).
    pub sigma: f64,
    /// Mean reversion function ($\theta(t)$)
    pub theta_t: fn(f64) -> f64,
}

impl ExtendedVasicek {
    /// Create a new Hull-White process.
    pub fn new(alpha_t: fn(f64) -> f64, sigma: f64, theta_t: fn(f64) -> f64) -> Self {
        Self {
            alpha_t,
            sigma,
            theta_t,
        }
    }
}

impl StochasticProcess for ExtendedVasicek {
    fn drift(&self, x: f64, t: f64) -> f64 {
        (self.theta_t)(t) - (self.alpha_t)(t) * x
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        self.sigma
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_extended_vasicek {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    fn alpha_t(_t: f64) -> f64 {
        2.0
    }
    fn theta_t(_t: f64) -> f64 {
        0.5
    }
    #[test]
    fn test_extended_vasicek() -> Result<(), Box<dyn std::error::Error>> {
        let sig = 2.0;
        let alpha = alpha_t(1.0);
        let theta = theta_t(1.0);

        let ev = ExtendedVasicek::new(alpha_t, sig, theta_t);

        let output = ev.euler_maruyama(10.0, 0.0, 1.0, 150, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = X_T.mean();
        // Note these tests are identical to the Hull-White
        // E[X_T] = X_0*exp(-alpha_t)(t) T) X_0 + (theta/alpha_t)(t))(1- exp(-alpha_t)(t) * T))
        // Expectation with constant reduces to Hull-White
        assert_approx_equal!(
            E_XT,
            (-alpha * 1.0_f64).exp() * 10.0 + (theta / alpha) * (1.0 - alpha * 1.0_f64).exp(),
            0.25
        );

        std::result::Result::Ok(())
    }
}
