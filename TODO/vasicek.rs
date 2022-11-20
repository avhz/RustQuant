#![deny(missing_docs)]

use crate::process::StochasticProcess;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
#[derive(Debug)]
pub struct Vasicek {
    /// Mean reversion parameter ($\alpha$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub alpha: f64,

    /// The long-run mean ($\beta$).
    pub beta: f64,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: f64,
}

impl Vasicek {
    /// Create a new Vasicek short rate process.
    pub fn new(alpha: f64, beta: f64, sigma: f64) -> Self {
        assert!(sigma >= 0.0);
        Self { alpha, beta, sigma }
    }
}

impl StochasticProcess for Vasicek {
    fn drift(&self, x: f64) -> f64 {
        self.alpha * (self.beta - x)
    }

    fn diffusion(&self, x: f64) -> f64 {
        self.sigma
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
    fn test_vasicek() -> Result<(), Box<dyn std::error::Error>> {
        let vasicek = Vasicek::new(0.15, 0.05, 0.2);

        let output = vasicek.euler_maruyama(10.0, 0.0, 0.5, 1000, 2, false);

        let file1 = "./Images/Vasicek1.png";
        plot_vector((&output.trajectories[0]).clone(), file1).unwrap();

        let file2 = "./Images/Vasicek2.png";
        plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
