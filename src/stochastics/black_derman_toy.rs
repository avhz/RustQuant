// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Sigma can either be varying or constant in the BDT model
/// The explicit distinction between the two is provided
/// to save on floating point computations, since the model
/// requires one to differentiate time-variant $\sigma(t)$.
///
/// Using a constant function in place of the Const variant
/// is not recommended for performance reasons.
pub enum Sigma {
    /// Sigma is constant
    Const(f64),
    /// Sigma is time-varying. Must be a positive function
    /// for non-negative time values
    Varying(fn(f64) -> f64),
}

/// Struct containing the Black-Derman-Toy process parameters.
pub struct BlackDermanToy {
    /// Instantaneous volatility
    pub sigma: Sigma,
    /// Value of underlying at option expiry
    pub theta_t: fn(f64) -> f64,
}

impl BlackDermanToy {
    /// Create a new Black-Derman-Toy process.
    pub fn new(sigma: Sigma, theta_t: fn(f64) -> f64) -> Self {
        match sigma {
            Sigma::Const(sigma) => {
                assert!(sigma >= 0.0);
                Self {
                    sigma: Sigma::Const(sigma),
                    theta_t,
                }
            }
            Sigma::Varying(sigma) => Self {
                // TODO add check for positivity of the function here...
                sigma: Sigma::Varying(sigma),
                theta_t,
            },
        }
    }
}

impl StochasticProcess for BlackDermanToy {
    fn drift(&self, x: f64, t: f64) -> f64 {
        match self.sigma {
            Sigma::Varying(sig) => (self.theta_t)(t) + (diff(sig, t) / sig(t)) * x,
            Sigma::Const(_) => (self.theta_t)(t),
        }
    }

    fn diffusion(&self, _x: f64, t: f64) -> f64 {
        match self.sigma {
            Sigma::Const(sig) => sig,
            Sigma::Varying(sig) => sig(t),
        }
    }

    fn jump(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }
}

/// Central different differentiation
pub(crate) fn diff(f: fn(f64) -> f64, t: f64) -> f64 {
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
    use crate::utilities::*;

    fn theta_t(_t: f64) -> f64 {
        1.5
    }
    fn sigma_t(_t: f64) -> f64 {
        0.13
    }
    #[test]
    fn test_black_derman_toy_constant_sigma() -> Result<(), Box<dyn std::error::Error>> {
        let sig = Sigma::Const(0.13);

        let hw = BlackDermanToy::new(sig, theta_t);

        let output = hw.euler_maruyama(0.13, 0.0, 1.0, 100, 100, false, None, None);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        assert!(E_XT.exp() >= 0.);
        // println!("Final expected short rate: {}", E_XT);

        std::result::Result::Ok(())
    }

    #[test]
    fn test_black_derman_toy_varying_sigma() -> Result<(), Box<dyn std::error::Error>> {
        let sig = Sigma::Varying(sigma_t);

        let hw = BlackDermanToy::new(sig, theta_t);

        let output = hw.euler_maruyama(0.13, 0.0, 1.0, 100, 1000, false, None, None);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        assert!(E_XT.exp() >= 0.);
        // println!("Final expected short rate: {}", E_XT);

        std::result::Result::Ok(())
    }
}
