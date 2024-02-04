// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::black_derman_toy::BlackDermanToy;
use crate::stochastics::process::StochasticProcess;

impl StochasticProcess for BlackDermanToy {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.theta.0(t) + diff(&self.sigma.0, t) / self.sigma.0(t) * x
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        self.sigma.0(t)
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

/// Central different differentiation
pub(crate) fn diff(f: &(dyn Fn(f64) -> f64 + Send + Sync), t: f64) -> f64 {
    // Arbitrary choice here...
    let eps = match t == 0. {
        // pretty arbitrary choice here
        true => f64::EPSILON.powf(1.0 / 3.0),
        false => f64::EPSILON.powf(1.0 / 3.0) * t,
    };
    (f(t + eps) - f(t - eps)) / (2.0 * eps)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_black_derman_toy {
    use super::*;
    use crate::statistics::*;

    // fn theta_t(_t: f64) -> f64 {
    //     1.5
    // }
    // fn sigma_t(_t: f64) -> f64 {
    //     0.13
    // }

    #[test]
    fn test_black_derman_toy_constant_sigma() {
        let sigma = 0.13;
        let theta = 1.5;

        let hw = BlackDermanToy::new(sigma, theta);

        let output = hw.euler_maruyama(0.13, 0.0, 1.0, 100, 100, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let E_XT = X_T.mean();
        assert!(E_XT.exp() >= 0.);
        // println!("Final expected short rate: {}", E_XT);
    }

    #[test]
    fn test_black_derman_toy_varying_sigma() {
        let sigma = 0.13;
        let theta = 1.5;

        let hw = BlackDermanToy::new(sigma, theta);

        let output = hw.euler_maruyama(0.13, 0.0, 1.0, 100, 1000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let E_XT = X_T.mean();
        assert!(E_XT.exp() >= 0.);
        // println!("Final expected short rate: {}", E_XT);
    }
}
