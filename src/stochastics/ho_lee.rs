// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containing the Ho-Lee process parameters.
pub struct HoLee {
    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: f64,
    /// Non-negative time-varying mean reversion function ($\theta_t$)
    pub theta_t: fn(f64) -> f64,
}

impl HoLee {
    /// Create a new Ho-Lee process.
    pub fn new(sigma: f64, theta_t: fn(f64) -> f64) -> Self {
        assert!(sigma >= 0.0);
        // TODO assert theta_t is non-negative function
        Self { sigma, theta_t }
    }
}

impl StochasticProcess for HoLee {
    fn drift(&self, _x: f64, t: f64) -> f64 {
        (self.theta_t)(t)
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        self.sigma
    }

    fn jump(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_ho_lee {
    use super::*;
    use crate::{assert_approx_equal, utilities::*};

    // Test a simple case where theta_t is constant
    // Should add tests of simple analytically tractable case
    fn theta_t(_t: f64) -> f64 {
        2.0
    }
    #[test]
    fn test_ho_lee() -> Result<(), Box<dyn std::error::Error>> {
        let hl = HoLee::new(1.6, theta_t);

        // X_0 = 10.0
        // T = 1.0
        let output = hl.euler_maruyama(10.0, 0.0, 1.0, 125, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        let V_XT = variance(&X_T, VarianceType::Sample);

        // This case reduces to arithmetic brownian motion..
        // E[X_T] = X_0 + theta_T * T
        assert_approx_equal!(E_XT, 10.0 + 2.0 * 1.0, 0.5);
        // Same here
        // V[X_T] = sigma^2 * T
        assert_approx_equal!(V_XT, 1.6 * 1.6 * 1.0, 0.5);

        std::result::Result::Ok(())
    }
}
