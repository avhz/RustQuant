// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::models::arithmetic_brownian_motion::ArithmeticBrownianMotion;
use crate::process::StochasticProcess;
use crate::ModelParameter;

/// Struct containing the Arithmetic Brownian Motion parameters.
pub struct ArithmeticBrownianMotion {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,
}

impl ArithmeticBrownianMotion {
    /// Create a new Arithmetic Brownian Motion process.
    pub fn new(mu: impl Into<ModelParameter>, sigma: impl Into<ModelParameter>) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
        }
    }
}

impl StochasticProcess for ArithmeticBrownianMotion {
    fn drift(&self, _x: f64, t: f64) -> f64 {
        // mu dt
        self.mu.0(t)
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        // sigma dW_t
        self.sigma.0(t)
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0)]
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_abm {
    use super::*;
    use crate::StochasticProcessConfig;
    use RustQuant_math::*;
    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn test_arithmetic_brownian_motion() {
        let abm = ArithmeticBrownianMotion::new(0.05, 0.9);
        let config = StochasticProcessConfig::new(10.0, 0.0, 0.5, 125, 1000, false);
        let output = abm.euler_maruyama(&config);

        // let file1 = "./images/ABM1.png";
        // plot_vector((&output.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./images/ABM2.png";
        // plot_vector((&output.trajectories[1]).clone(), file2)

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = X_0 + mu * T
        assert_approx_equal!(E_XT, 10.0 + 0.05 * 0.5, 0.1);
        // V[X_T] = sigma^2 * T
        assert_approx_equal!(V_XT, 0.9 * 0.9 * 0.5, 0.1);
    }
}
