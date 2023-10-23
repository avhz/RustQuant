use crate::stochastics::*;
pub struct CompoundPoisson {
    lambda: f64,
    jump_distribution: Box<dyn Fn() -> f64 + Sync>,
}

impl CompoundPoisson {
    pub fn new(lambda: f64, jump_distribution: Box<dyn Fn() -> f64 + Sync>) -> Self {
        Self {
            lambda,
            jump_distribution,
        }
    }
}

impl StochasticProcess for CompoundPoisson {
    fn drift(&self, _x: f64, _t: f64) -> f64 {
        self.lambda * ((self.jump_distribution)() - 1.0)
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        0.0 // No diffusion term
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        Some((self.jump_distribution)())
    }
}

#[cfg(test)]
mod tests_compound_poisson {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    fn gaussian_sample() -> f64 {
        1.0
    }

    #[test]
    fn tests_compound_poisson() -> Result<(), Box<dyn std::error::Error>> {
        let cp = CompoundPoisson::new(1000., Box::new(gaussian_sample));

        let output = cp.euler_maruyama(10.0, 0.0, 0.5, 125, 10000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        println!("X_T = {:?}", X_T);

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(E_XT, 10.0, 0.5);
        // V[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(
            V_XT,
            0.0,
            0.5
        );

        std::result::Result::Ok(())
    }
}