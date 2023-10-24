use crate::stochastics::*;

/// Struct containing the Geometric Brownian Bridge parameters.
/// The Geometric Brownian Bridge is a stochastic process that models a path-dependent option.
/// It is a modification of the Geometric Brownian Motion where the end value is known.
pub struct GeometricBrownianBridge {
    /// The drift ($\mu$) in percentage.
    mu: TimeDependent,
    /// The volatility ($\sigma$) in percentage.
    sigma: TimeDependent,
    /// The known end value of the process.
    end_value: f64,
    /// The known end time of the process.
    end_time: f64,
}

impl GeometricBrownianBridge {
    /// Create a new Geometric Brownian Bridge process.
    /// # Arguments
    /// * `mu` - The drift ($\mu$) in percentage.
    /// * `sigma` - The volatility ($\sigma$) in percentage.
    /// * `end_value` - The known end value of the process.
    /// * `end_time` - The known end time of the process.
    pub fn new(
        mu: impl Into<TimeDependent>,
        sigma: impl Into<TimeDependent>,
        end_value: f64,
        end_time: f64,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            end_value,
            end_time,
        }
    }
}

impl StochasticProcess for GeometricBrownianBridge {
    /// The drift function for the Geometric Brownian Bridge.
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x + (self.end_value.ln() - x.ln()) / (self.end_time - t) * x
    }

    /// The diffusion function for the Geometric Brownian Bridge.
    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        self.sigma.0(t) * x
    }

    /// The jump function for the Geometric Brownian Bridge.
    /// As the Geometric Brownian Bridge does not have a jump term, this always returns None.
    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests_gbm_bridge {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    /// Test the Geometric Brownian Bridge process.
    #[test]
    fn test_geometric_brownian_motion_bridge() -> Result<(), Box<dyn std::error::Error>> {
        let gbm = GeometricBrownianBridge::new(0.05, 0.9, 10.0, 0.5);

        let output = gbm.euler_maruyama(10.0, 0.0, 0.5, 125, 10000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(E_XT, 10.0, 0.5);
        // V[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(V_XT, 0.0, 0.5);

        std::result::Result::Ok(())
    }
}
