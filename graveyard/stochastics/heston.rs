// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{models::Heston, stochastics::StochasticProcess};

impl StochasticProcess for Heston {
    fn drift(&self, _x: f64, _t: f64) -> f64 {
        todo!()
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        todo!()
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        todo!()
    }

    fn parameters(&self) -> Vec<f64> {
        vec![
            self.initial_variance.0(0.0),
            self.long_run_variance.0(0.0),
            self.mean_reversion_rate.0(0.0),
            self.correlation.0(0.0),
            self.volatility_of_volatility.0(0.0),
        ]
    }
}
