#![deny(missing_docs)]

use crate::process::StochasticProcess;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
pub struct OrnsteinUhlenbeck {
    /// The long-run mean ($\mu$).
    pub mu: f64,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: f64,

    /// Mean reversion parameter ($\theta$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub theta: f64,
}

impl OrnsteinUhlenbeck {
    /// Create a new Ornstein-Uhlenbeck process.
    pub fn new(mu: f64, sigma: f64, theta: f64) -> Self {
        Self { mu, sigma, theta }
    }
}

impl StochasticProcess for OrnsteinUhlenbeck {
    fn drift(&self, x: f64) -> f64 {
        (*self).theta * ((*self).mu - x)
    }

    fn diffusion(&self, _x: f64) -> f64 {
        (*self).sigma
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
    fn test_ornstein_uhlenbeck() -> Result<(), Box<dyn std::error::Error>> {
        let ou = OrnsteinUhlenbeck::new(0.15, 0.45, 0.01);

        let output = (&ou).euler_maruyama(10.0, 0.0, 0.5, 1000, 2);

        let file1 = "./Images/OU_new1.png";
        plot_vector((&output.trajectories[0]).clone(), file1).unwrap();

        let file2 = "./Images/OU_new2.png";
        plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
