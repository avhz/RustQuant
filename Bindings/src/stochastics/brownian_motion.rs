// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;

/// Struct containin the Geometric Brownian Motion parameters.
#[derive(Debug)]
pub struct BrownianMotion {}

impl Default for BrownianMotion {
    fn default() -> Self {
        Self::new()
    }
}

impl BrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new() -> Self {
        Self {}
    }
}

impl StochasticProcess for BrownianMotion {
    fn drift(&self, _x: f64) -> f64 {
        0.0
    }

    fn diffusion(&self, _x: f64) -> f64 {
        1.0
    }

    fn jump(&self, _x: f64) -> f64 {
        0.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod sde_tests {
    // use std::time::Instant;

    use super::*;
    use crate::{assert_approx_equal, utilities::*};

    #[test]
    fn test_brownian_motion() -> Result<(), Box<dyn std::error::Error>> {
        let bm = BrownianMotion::new();

        // AT LEAST 100 PATHS BEFORE PARALLEL IS WORTH IT.
        // for _steps in [1, 10, 100, 1000] {
        //     for paths in [1, 10, 100, 1000] {
        //         let start_serial = Instant::now();
        //         (&bm).euler_maruyama(10.0, 0.0, 0.5, 1000, paths, false);
        //         let duration_serial = start_serial.elapsed();

        //         let start_parallel = Instant::now();
        //         (&bm).euler_maruyama(10.0, 0.0, 0.5, 1000, paths, true);
        //         let duration_parallel = start_parallel.elapsed();

        //         println!(
        //             "{},{},{:?},{:?}",
        //             1000,
        //             paths,
        //             duration_serial.as_micros(),
        //             duration_parallel.as_micros()
        //         );
        //     }
        // }
        // assert!(1 == 2);

        let output_serial = (&bm).euler_maruyama(0.0, 0.0, 0.5, 100, 1000, false);
        // let output_parallel = (&bm).euler_maruyama(10.0, 0.0, 0.5, 100, 10, true);

        // let file1 = "./Images/BM1.png";
        // plot_vector((&output_serial.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./Images/BM2.png";
        // plot_vector((&output_serial.trajectories[1]).clone(), file2).unwrap();
        // let file2 = "./Images/BM3_parallel.png";
        // plot_vector((&output_parallel.trajectories[0]).clone(), file2)

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output_serial
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = mean(&X_T, MeanType::Arithmetic);
        let V_XT = variance(&X_T, VarianceType::Sample);
        // E[X_T] = 0
        assert_approx_equal!(E_XT, 0.0, 0.5);
        // V[X_T] = T
        assert_approx_equal!(V_XT, 0.5, 0.5);

        std::result::Result::Ok(())
    }
}
