// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containing the Hull-White process parameters.
pub struct HullWhite {
    /// Long run mean ($\alpha)
    pub alpha: TimeDependent,

    /// Non-negative diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: TimeDependent,

    /// Mean reversion function (non-negative) ($\theta(t)$)
    pub theta: TimeDependent,
}

impl HullWhite {
    /// Create a new Hull-White process.
    pub fn new(
        alpha: impl Into<TimeDependent>,
        sigma: impl Into<TimeDependent>,
        theta: impl Into<TimeDependent>,
    ) -> Self {
        Self {
            alpha: alpha.into(),
            sigma: sigma.into(),
            theta: theta.into(),
        }
    }
}

impl StochasticProcess for HullWhite {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.theta.0(t) - (self.alpha.0(t) * x)
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        self.sigma.0(t)
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_hull_white {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    // fn theta_t(_t: f64) -> f64 {
    //     0.5
    // }

    #[test]
    fn test_hull_white() -> Result<(), Box<dyn std::error::Error>> {
        let alpha = 2.0;
        let theta = 0.5;
        let sigma = 2.0;

        let hw = HullWhite::new(alpha, sigma, theta);

        let output = hw.euler_maruyama(10.0, 0.0, 1.0, 150, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = X_T.mean();
        // E[X_T] = X_0*exp(-alpha T) X_0 + (theta_t/alpha)(1- exp(-alpha * T))
        assert_approx_equal!(
            E_XT,
            (-alpha * 1.0_f64).exp() * 10.0 + (theta / alpha) * (1.0 - (-alpha * 1.0_f64).exp()),
            0.25
        );

        // No closed form solution for variance that I know of...
        // Have to take it on faith that it works
        std::result::Result::Ok(())
    }
}
