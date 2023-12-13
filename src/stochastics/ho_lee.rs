// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::{StochasticProcess, TimeDependent};

/// Struct containing the Ho-Lee process parameters.
pub struct HoLee {
    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: TimeDependent,

    /// Non-negative time-varying mean reversion function ($\theta_t$)
    pub theta: TimeDependent,
}

impl HoLee {
    /// Create a new Ho-Lee process.
    pub fn new(sigma: impl Into<TimeDependent>, theta: impl Into<TimeDependent>) -> Self {
        Self {
            sigma: sigma.into(),
            theta: theta.into(),
        }
    }
}

impl StochasticProcess for HoLee {
    fn drift(&self, _x: f64, t: f64) -> f64 {
        assert!(self.theta.0(t) >= 0.0);
        (self.theta.0)(t)
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
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
mod tests_ho_lee {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    // Test a simple case where theta_t is constant
    // Should add tests of simple analytically tractable case
    // fn theta_t(_t: f64) -> f64 {
    //     2.0
    // }

    #[test]
    fn test_ho_lee() {
        let hl = HoLee::new(1.6, 2.0);

        // X_0 = 10.0
        // T = 1.0
        let output = hl.euler_maruyama(10.0, 0.0, 1.0, 125, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();

        // This case reduces to arithmetic brownian motion..
        // E[X_T] = X_0 + theta_T * T
        assert_approx_equal!(E_XT, 10.0 + 2.0 * 1.0, 0.5);
        // Same here
        // V[X_T] = sigma^2 * T
        assert_approx_equal!(V_XT, 1.6 * 1.6 * 1.0, 0.5);
    }
}
