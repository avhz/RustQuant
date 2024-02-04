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
    fn drift(&self, x: f64, t: f64) -> f64 {
        todo!()
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        todo!()
    }

    fn jump(&self, x: f64, t: f64) -> Option<f64> {
        todo!()
    }
}
