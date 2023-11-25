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

/// Czech Republic calendar.
pub struct Denmark;

impl Calendar for Denmark {
    fn name(&self) -> &'static str {
        "Denmark"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::DENMARK
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XCSE
    }

    /// Denmark holidays:
    ///     - Maunday Thursday
    ///     - Good Friday
    ///     - Easter Monday
    ///     - General Prayer Day
    ///     - Ascension
    ///     - Day after Ascension
    ///     - Whit Monday
    ///     - New Year's Day
    ///     - Constitution Day, June 5th
    ///     - Christmas Eve
    ///     - Christmas
    ///     - Boxing Day
    ///     - New Year's Eve
    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || (dd == em - 4)
            || (dd == em - 3)
            || (dd == em)
            || (dd == em + 25 && y <= 2023)
            || (dd == em + 38)
            || (dd == em + 39 && y >= 2009)
            || (dd == em + 49)
            || (d == 1 && m == Month::January)
            || (d == 5 && m == Month::June)
            || (d == 24 && m == Month::December)
            || (d == 25 && m == Month::December)
            || (d == 26 && m == Month::December)
            || (d == 31 && m == Month::December)
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
