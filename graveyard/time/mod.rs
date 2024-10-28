// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Time and date functionality.

/// The core `Calendar` trait.
pub mod calendar;
pub use calendar::*;

/// Constants for calendars and holidays.
#[allow(dead_code)]
pub mod constants;
pub use constants::*;

/// Calendars implemented for specific countries.
#[allow(unused_parens)]
pub mod countries;
pub use countries::*;

/// Date rolling conventions and methods.
pub mod date_rolling;
pub use date_rolling::*;

/// Day counting conventions and methods.
pub mod day_counting;
pub use day_counting::*;

/// Frequency of payments.
pub mod frequency;
pub use frequency::*;

/// The `Holiday` trait.
pub mod holiday;
pub use holiday::*;

/// Utility functions for working with dates and times.
pub mod utilities;
pub use utilities::*;

/// The `Schedule` type.
pub mod schedule;
pub use schedule::*;

/// Date generation rules.
pub mod date_generation;
pub use date_generation::*;

/// Stub generation rules.
pub mod stub_generation;
pub use stub_generation::*;
