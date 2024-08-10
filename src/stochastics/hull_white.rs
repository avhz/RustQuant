// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{models::hull_white::HullWhite, stochastics::process::StochasticProcess};

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
    use crate::stochastics::StochasticProcessConfig;
    use crate::{assert_approx_equal, math::*};
    // fn theta_t(_t: f64) -> f64 {
    //     0.5
    // }

    #[test]
    fn test_hull_white() {
        let alpha = 2.0;
        let theta = 0.5;
        let sigma = 2.0;

        let hw = HullWhite::new(alpha, sigma, theta);
        let config = StochasticProcessConfig::new(10.0, 0.0, 1.0, 150, 1000, false);

        let output = hw.euler_maruyama(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
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
    }
}
