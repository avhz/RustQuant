// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::process::{StochasticProcess, Trajectories, StochasticProcessConfig};
use crate::fractional_process::{simulate_fractional_stochastic_process, FractionalProcessGeneratorMethod};

/// Struct containing the Fractional Brownian Motion parameters.
#[derive(Debug)]
pub struct FractionalBrownianMotion {
    /// Hurst parameter of the process.
    pub hurst: f64,

    /// Method used to generate the process.
    pub method: FractionalProcessGeneratorMethod,
}

impl Default for FractionalBrownianMotion {
    fn default() -> Self {
        Self::new(0.5, FractionalProcessGeneratorMethod::FFT)
    }
}

impl FractionalBrownianMotion {
    /// Create a new Fractional Brownian Motion process.
    ///
    /// # Panics
    ///
    /// Will panic if Hurst parameter is not in [0, 1].
    #[must_use]
    pub fn new(hurst: f64, method: FractionalProcessGeneratorMethod) -> Self {
        assert!((0.0..=1.0).contains(&hurst));

        Self { hurst, method }
    }
}

impl StochasticProcess for FractionalBrownianMotion {
    fn drift(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        1.0
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }

    fn parameters(&self) -> Vec<f64> {
        vec![self.hurst]
    }

    fn generate(&self, config: &StochasticProcessConfig) -> Trajectories {
        simulate_fractional_stochastic_process(self, config, &self.method, self.hurst)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


#[cfg(test)]
mod test_fractional_brownian_motion {
    use super::*;
    use crate::{StochasticProcessConfig, StochasticScheme};
    use RustQuant_math::*;
    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn test_brownian_motion() {
        let fbm = FractionalBrownianMotion::new(0.7, FractionalProcessGeneratorMethod::FFT);
        let config = StochasticProcessConfig::new(
            0.0, 0.0, 0.5,100, StochasticScheme::EulerMaruyama,1000, false, None
        );
        let output_serial = fbm.generate(&config);
        // let output_parallel = (&bm).euler_maruyama(10.0, 0.0, 0.5, 100, 10, true);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output_serial
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        // E[X_T] = 0
        assert_approx_equal!(X_T.clone().mean(), 0.0, 0.5);
        // V[X_T] = T
        assert_approx_equal!(X_T.clone().variance(), 0.5, 0.5);
    }
}
