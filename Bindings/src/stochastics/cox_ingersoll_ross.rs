// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
#[derive(Debug)]
pub struct CoxIngersollRoss {
    /// The long-run mean ($\mu$).
    pub mu: f64,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: f64,

    /// Mean reversion parameter ($\theta$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub theta: f64,
}

impl CoxIngersollRoss {
    /// Create a new Cox-Ingersoll-Ross process.
    pub fn new(mu: f64, sigma: f64, theta: f64) -> Self {
        assert!(sigma >= 0.0);
        Self { mu, sigma, theta }
    }
}

impl StochasticProcess for CoxIngersollRoss {
    fn drift(&self, x: f64) -> f64 {
        self.theta * (self.mu - x)
    }

    fn diffusion(&self, x: f64) -> f64 {
        self.sigma * x.sqrt()
    }

    fn jump(&self, _x: f64) -> f64 {
        0.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_cir {
    use super::*;
    use crate::{assert_approx_equal, utilities::*};

    #[test]
    fn test_cox_ingersoll_ross() -> Result<(), Box<dyn std::error::Error>> {
        let cir = CoxIngersollRoss::new(0.15, 0.45, 0.01);

        let output = cir.euler_maruyama(10.0, 0.0, 0.5, 100, 100, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        let V_XT = variance(&X_T, VarianceType::Sample);
        // E[X_T] = https://en.wikipedia.org/wiki/Cox%E2%80%93Ingersoll%E2%80%93Ross_model
        assert_approx_equal!(
            E_XT,
            10. * (-0.01 * 0.5 as f64).exp() + 0.15 * (1. - (-0.01 * 0.5 as f64).exp()),
            0.5
        );
        // V[X_T] = see https://en.wikipedia.org/wiki/Cox%E2%80%93Ingersoll%E2%80%93Ross_model
        assert_approx_equal!(
            V_XT,
            10. * (0.45 * 0.45 / 0.01)
                * ((-0.01 * 0.5 as f64).exp() - (-2. * 0.01 * 0.5 as f64).exp())
                + (0.15 * 0.45 * 0.45 / (2. * 0.01)) * (1. - (-0.01 * 0.5 as f64).exp()).powi(2),
            0.5
        );

        // let file1 = "./Images/CIR1.png";
        // plot_vector((&output.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./Images/CIR2.png";
        // plot_vector((&output.trajectories[1]).clone(), file2)

        std::result::Result::Ok(())
    }
}
