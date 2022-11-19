#![deny(missing_docs)]

use crate::process::StochasticProcess;

/// Struct containin the Geometric Brownian Motion parameters.
pub struct BrownianMotion {}

impl BrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new() -> Self {
        Self {}
    }
}

impl StochasticProcess for BrownianMotion {
    fn drift(&self, x: f64) -> f64 {
        0_f64
    }

    fn diffusion(&self, x: f64) -> f64 {
        1_f64
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
    fn test_brownian_motion() -> Result<(), Box<dyn std::error::Error>> {
        let bm = BrownianMotion::new();

        let output = (&bm).euler_maruyama(10.0, 0.0, 0.5, 1000, 2);

        let file1 = "./Images/BM1.png";
        plot_vector((&output.trajectories[0]).clone(), file1).unwrap();

        let file2 = "./Images/BM2.png";
        plot_vector((&output.trajectories[1]).clone(), file2)
    }
}
