// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains implementations of 150+ currencies,
//! defined according to the ISO 4217 standard.
//! It currently needs to be proof-read and tested.

use std::fmt;
use std::fmt::Formatter;

// pub use unformatted::*;

/// ISO 4217 codes enum.
///
/// Format:
///     - First two letters are the ISO 3166-1 alpha-2 country code. e.g. US = United States
///     - Third letter is the first letter of the currency name. e.g. USD = United States Dollar
///     - The number is the ISO numeric code. e.g. 840 = USD
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub struct ISO_4217 {
    /// The ISO 4217 alphabetic code.
    pub alphabetic: &'static str,

    /// The ISO 4217 numeric code.
    pub numeric: &'static str,
}

impl ISO_4217 {
    /// Create a new ISO 4217 code.
    #[must_use]
    pub fn new(alphabetic: &'static str, numeric: &'static str) -> Self {
        Self {
            alphabetic,
            numeric,
        }
    }

    /// Get the ISO 4217 alphabetic code.
    #[must_use]
    pub fn alphabetic(&self) -> &str {
        self.alphabetic
    }

    /// Get the ISO 4217 numeric code.
    #[must_use]
    pub fn numeric(&self) -> &'static str {
        self.numeric
    }
}

// impl PartialEq for ISO_4217 {
//     fn eq(&self, other: &Self) -> bool {
//         self.alphabetic == other.alphabetic && self.numeric == other.numeric
//     }
// }

impl fmt::Display for ISO_4217 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Alphabetic: {}, Numeric: {}",
            self.alphabetic, self.numeric
        )
    }
}
