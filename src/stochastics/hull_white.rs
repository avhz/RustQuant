// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containing the Hull-White process parameters.
pub struct HullWhite {
    /// Long run mean ($\mu$)
    pub alpha: f64,
    /// Non-negative diffusion, or instantaneous time-varying volatility ($\sigma$).
    pub sigma_t: fn(f64) -> f64,
    /// Mean reversion function ($\theta_t$)
    pub theta: f64,
}

impl HullWhite {
    /// Create a new Hull-White process.
    pub fn new(alpha: f64, sigma_t: fn(f64) -> f64, theta: f64) -> Self {
        Self {
            alpha,
            sigma_t,
            theta,
        }
    }
}

impl StochasticProcess for HullWhite {
    fn drift(&self, x: f64, _t: f64) -> f64 {
        self.theta - (self.alpha * x)
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        (self.sigma_t)(t)
    }

    fn jump(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_hull_white {
    use super::*;
    use crate::{assert_approx_equal, utilities::*};

    fn sigma_t(_t: f64) -> f64 {
        2.0
    }
    #[test]
    fn test_hull_white() -> Result<(), Box<dyn std::error::Error>> {
        let alpha = 2.0;
        let theta = 0.3;
        let sig = sigma_t(1.0);

        let hw = HullWhite::new(alpha, sigma_t, theta);

        let output = hw.euler_maruyama(10.0, 0.0, 1.0, 150, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        // E[X_T] = X_0*exp(-alpha T) X_0 + (theta/alpha)(1- exp(-alpha * T))
        assert_approx_equal!(
            E_XT,
            (-alpha * 1.0 as f64).exp() * 10.0
                + (theta / alpha) * (1.0 - (-alpha * 1.0 as f64).exp()),
            0.25
        );

        // No closed form solution for variance that I know of...
        // Have to take it on faith that it works
        std::result::Result::Ok(())
    }
}
