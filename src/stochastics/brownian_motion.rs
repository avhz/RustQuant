// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::brownian_motion::BrownianMotion;
use crate::stochastics::process::StochasticProcess;

impl StochasticProcess for BrownianMotion {
    fn drift(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        1.0
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod sde_tests {
    // use std::time::Instant;

    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    #[test]
    fn test_brownian_motion() {
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

        let output_serial = bm.euler_maruyama(0.0, 0.0, 0.5, 100, 1000, false);
        // let output_parallel = (&bm).euler_maruyama(10.0, 0.0, 0.5, 100, 10, true);

        // let file1 = "./images/BM1.png";
        // plot_vector((&output_serial.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./images/BM2.png";
        // plot_vector((&output_serial.trajectories[1]).clone(), file2).unwrap();
        // let file2 = "./images/BM3_parallel.png";
        // plot_vector((&output_parallel.trajectories[0]).clone(), file2)

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output_serial
            .paths
            .iter()
            .filter_map(|v| v.last().copied())
            .collect();

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = 0
        assert_approx_equal!(E_XT, 0.0, 0.5);
        // V[X_T] = T
        assert_approx_equal!(V_XT, 0.5, 0.5);
    }
}
