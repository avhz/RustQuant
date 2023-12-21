// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Time and date functionality.

pub use crate::time::{
    calendar::*,
    calendars::{
        argentina::*, australia::*, austria::*, botswana::*, brazil::*, canada::*, chile::*,
        china::*, czech_republic::*, denmark::*, finland::*, france::*, germany::*, hong_kong::*,
        united_kingdom::*, united_states::*,
    },
    constants::*,
    conventions::*,
    daycount::*,
    schedule::*,
};

/// Calendar definitions.
pub mod calendar;
/// Date/time constants
pub mod constants;
/// Day count and business day conventions.
pub mod conventions;
/// Daycount definitions.
pub mod daycount;
/// Scheduling definitions.
pub mod schedule;

/// Calendar definitions for settlement purposes.
pub mod calendars {
    /// Argentina settlement calendar.
    pub mod argentina;
    /// Australian settlement calendar.
    pub mod australia;
    /// Austrian settlement calendar.
    pub mod austria;
    /// Botswana settlement calendar.
    pub mod botswana;
    /// Brazil settlement calendar.
    pub mod brazil;
    /// Canadian settlement calendar.
    pub mod canada;
    /// Chile settlement calendar.
    pub mod chile;
    /// China settlement calendar.
    pub mod china;
    /// Czech Republic settlement calendar.
    pub mod czech_republic;
    /// Denmark settlement calendar.
    pub mod denmark;
    /// Finland settlement calendar.
    pub mod finland;
    /// France settlement calendar.
    pub mod france;
    /// Germany settlement calendar.
    pub mod germany;
    /// Hong Kong settlement calendar.
    pub mod hong_kong;
    /// Calendar test module.
    mod tests;
    /// UK settlement calendar.
    pub mod united_kingdom;
    /// USA settlement calendar.
    pub mod united_states;
}
