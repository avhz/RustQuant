// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containin the Arithmetic Brownian Motion parameters.
pub struct ArithmeticBrownianMotion {
    /// The drift ($\mu$) in percentage.
    pub mu: f64,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: f64,
}

impl ArithmeticBrownianMotion {
    /// Create a new Arithmetic Brownian Motion process.
    pub fn new(mu: f64, sigma: f64) -> Self {
        assert!(sigma >= 0.0);
        Self { mu, sigma }
    }
}

impl StochasticProcess for ArithmeticBrownianMotion {
    fn drift(&self, _x: f64) -> f64 {
        // mu dt
        self.mu
    }

    fn diffusion(&self, _x: f64) -> f64 {
        // sigma dW_t
        self.sigma
    }

    fn jump(&self, _x: f64) -> f64 {
        0.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_abm {
    use super::*;
    use crate::{assert_approx_equal, utilities::*};

    #[test]
    fn test_arithmetic_brownian_motion() -> Result<(), Box<dyn std::error::Error>> {
        let abm = ArithmeticBrownianMotion::new(0.05, 0.9);

        let output = (&abm).euler_maruyama(10.0, 0.0, 0.5, 125, 1000, false);

        // let file1 = "./Images/ABM1.png";
        // plot_vector((&output.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./Images/ABM2.png";
        // plot_vector((&output.trajectories[1]).clone(), file2)

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        let V_XT = variance(&X_T, VarianceType::Sample);
        // E[X_T] = X_0 + mu * T
        assert_approx_equal!(E_XT, 10.0 + 0.05 * 0.5, 0.1);
        // V[X_T] = sigma^2 * T
        assert_approx_equal!(V_XT, 0.9 * 0.9 * 0.5, 0.1);

        std::result::Result::Ok(())
    }
}
