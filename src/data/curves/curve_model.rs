// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::Date;

/// A trait for curve models.
#[allow(clippy::module_name_repetitions)]
pub trait CurveModel {
    /// Returns the forward rate for a given date.
    fn forward_rate(&self, date: Date) -> f64;

    /// Returns the spot rate for a given date.
    fn spot_rate(&self, date: Date) -> f64;

    /// Returns the discount factor for a given date.
    fn discount_factor(&self, date: Date) -> f64;

    // /// Calibrates the model to a set of market rates.
    // #[must_use]
    // fn calibrate<C: Curve>(&self, curve: C) -> Self;
}
