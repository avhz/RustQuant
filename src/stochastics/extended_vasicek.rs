// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::extended_vasicek::ExtendedVasicek;
use crate::stochastics::process::StochasticProcess;

impl StochasticProcess for ExtendedVasicek {
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
mod tests_extended_vasicek {
    use super::*;
    use crate::{assert_approx_equal, math::*};

    // fn alpha_t(_t: f64) -> f64 {
    //     2.0
    // }
    // fn theta_t(_t: f64) -> f64 {
    //     0.5
    // }

    #[test]
    fn test_extended_vasicek() {
        let sigma = 2.0;
        let alpha = 2.0;
        let theta = 0.5;

        let ev = ExtendedVasicek::new(alpha, sigma, theta);

        let output = ev.euler_maruyama(10.0, 0.0, 1.0, 150, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
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
    }
}
