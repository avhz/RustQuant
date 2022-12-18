#![deny(missing_docs)]

use crate::stochastics::*;

/// Struct containin the Geometric Brownian Motion parameters.
pub struct GeometricBrownianMotion {
    /// The drift ($\mu$) in percentage.
    pub mu: f64,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: f64,
}

impl GeometricBrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new(mu: f64, sigma: f64) -> Self {
        assert!(sigma >= 0.0);
        Self { mu, sigma }
    }
}

impl StochasticProcess for GeometricBrownianMotion {
    fn drift(&self, x: f64) -> f64 {
        self.mu * x
    }

    fn diffusion(&self, x: f64) -> f64 {
        self.sigma * x
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::*;

    #[test]
    fn test_geometric_brownian_motion() -> Result<(), Box<dyn std::error::Error>> {
        let gbm = GeometricBrownianMotion::new(0.05, 0.9);

        let output = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 100, 2, false);

        let file1 = "./Images/GBM1.png";
        plot_vector((&output.trajectories[0]).clone(), file1).unwrap();

        let file2 = "./Images/GBM2.png";
        plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
