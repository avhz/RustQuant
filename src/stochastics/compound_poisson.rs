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
    fn drift(&self, _x: f64, t: f64) -> f64 {
        self.lambda * ((self.jump_distribution)() - 1.0)
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        0.0 // No diffusion term
    }

    fn jump(&self, _x: f64, t: f64) -> Option<f64> {
        Some((self.jump_distribution)())
    }
}