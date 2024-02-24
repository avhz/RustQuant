// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{
    models::geometric_brownian_motion::GeometricBrownianMotion,
    stochastics::process::StochasticProcess,
};

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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_gbm {
    use super::*;
    use crate::{assert_approx_equal, math::*};

    #[test]
    fn test_geometric_brownian_motion() {
        let gbm = GeometricBrownianMotion::new(0.05, 0.9);

        let output = gbm.euler_maruyama(10.0, 0.0, 0.5, 125, 10000, false);

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
