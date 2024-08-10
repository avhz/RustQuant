// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Monte-Carlo pricer trait.

use crate::stochastics::{StochasticProcess, StochasticProcessConfig};

/// Monte-Carlo pricer trait.
pub trait MonteCarloPricer<S>
where
    S: StochasticProcess,
{
    /// Price the instrument using a Monte-Carlo method.
    fn price_monte_carlo(&self, process: S, config: StochasticProcessConfig, rate: f64) -> f64;
}
