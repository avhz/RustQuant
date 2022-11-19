#![deny(missing_docs)]

use crate::process::StochasticProcess;

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
    use crate::plot::plot_vector;

    #[test]
    fn test_gbm_new() -> Result<(), Box<dyn std::error::Error>> {
        let gbm = GeometricBrownianMotion::new(0.05, 0.9);

        let output = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 1000, 2);

        let file1 = "./Images/GBM1.png";
        plot_vector((&output.trajectories[0]).clone(), file1).unwrap();

        let file2 = "./Images/GBM2.png";
        plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
