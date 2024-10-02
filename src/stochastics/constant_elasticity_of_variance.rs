// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::constant_elasticity_of_variance::ConstantElasticityOfVariance;
use crate::stochastics::process::StochasticProcess;

impl StochasticProcess for ConstantElasticityOfVariance {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        assert!(self.elasticity.0(t) >= 0.0 && self.elasticity.0(t) <= 1.0);

        self.sigma.0(t) * x.powf(self.elasticity.0(t))
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0), self.elasticity.0(0.0)]
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_cev {
    use super::*;
    use crate::{math::*, stochastics::StochasticProcessConfig};

    #[test]
    fn test_cev_process_euler_maruyama() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 100, 100, false, None);
        let output = cev.euler_maruyama(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();

        // Make a PR if you know the mean and variance of the CEV process.

        // assert_approx_equal!(
        //     E_XT,
        //     10. * (-0.01 * 0.5_f64).exp() + 0.15 * (1. - (-0.01 * 0.5_f64).exp()),
        //     0.5
        // );

        // assert_approx_equal!(
        //     V_XT,
        //     10. * (0.45 * 0.45 / 0.01) * ((-0.01 * 0.5_f64).exp() - (-2. * 0.01 * 0.5_f64).exp())
        //         + (0.15 * 0.45 * 0.45 / (2. * 0.01)) * (1. - (-0.01 * 0.5_f64).exp()).powi(2),
        //     0.5
        // );
    }

    #[test]
    fn test_cev_euler_maruyama_seeded() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 100, 100, false, Some(1337));
        let output = cev.euler_maruyama(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();

        // Make a PR if you know the mean and variance of the CEV process.

        // assert_approx_equal!(
        //     E_XT,
        //     10. * (-0.01 * 0.5_f64).exp() + 0.15 * (1. - (-0.01 * 0.5_f64).exp()),
        //     0.5
        // );

        // assert_approx_equal!(
        //     V_XT,
        //     10. * (0.45 * 0.45 / 0.01) * ((-0.01 * 0.5_f64).exp() - (-2. * 0.01 * 0.5_f64).exp())
        //         + (0.15 * 0.45 * 0.45 / (2. * 0.01)) * (1. - (-0.01 * 0.5_f64).exp()).powi(2),
        //     0.5
        // );
    }

    #[test]
    fn test_cev_process_milstein() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 100, 100, false, None);
        let output = cev.milstein(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();
    }

    #[test]
    fn test_cev_process_milstein_seeded() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 100, 100, false, Some(1337));
        let output = cev.milstein(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();
    }

    #[test]
    fn test_cev_process_strang_splitting() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 100, 100, false, None);
        let output = cev.euler_maruyama(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();
    }

    #[test]
    fn test_cev_process_strang_splitting_seeded() {
        let cev = ConstantElasticityOfVariance::new(0.05, 0.9, 0.45);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 100, 100, false, None);
        let output = cev.strang_splitting(&config);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let _E_XT = X_T.mean();
        let _V_XT = X_T.variance();
    }
}
