// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use time::{Month, OffsetDateTime};

/// Germany calendar.
pub struct Germany;

impl Calendar for Germany {
    fn name(&self) -> &'static str {
        "Germany"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::GERMANY
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XFRA
    }

    /// Germany holidays:
    ///     - New Year's Day
    ///     - Good Friday
    ///     - Easter Monday
    ///     - Ascension Thursday
    ///     - Whit Monday
    ///     - Corpus Christi
    ///     - Labour Day
    ///     - National Day
    ///     - Christmas Eve
    ///     - Christmas
    ///     - Boxing Day
    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || d == 1 && m == Month::January
            || dd == em - 3
            || dd == em
            || dd == em + 38
            || dd == em + 49
            || dd == em + 59
            || d == 1 && m == Month::May
            || d == 3 && m == Month::October
            || d == 24 && m == Month::December
            || d == 25 && m == Month::December
            || d == 26 && m == Month::December
        {
            return false;
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {}
