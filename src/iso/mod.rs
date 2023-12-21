// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! ISO codes module.

#[allow(clippy::module_name_repetitions)]
pub use iso_10383::*;
#[allow(clippy::module_name_repetitions)]
pub use iso_3166::*;
#[allow(clippy::module_name_repetitions)]
pub use iso_4217::*;

/// ISO 10383 market identifier codes module.
#[allow(clippy::module_name_repetitions)]
pub mod iso_10383;
/// ISO 3166 country codes module.
#[allow(clippy::module_name_repetitions)]
pub mod iso_3166;
/// ISO 4217 currency codes module.
#[allow(clippy::module_name_repetitions)]
pub mod iso_4217;
