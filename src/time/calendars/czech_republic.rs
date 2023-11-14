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
pub struct CzechRepublic;

impl Calendar for CzechRepublic {
    fn name(&self) -> &'static str {
        "Czech Republic"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::CZECHIA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XPRA
    }

    /// Czech Republic holidays:
    ///     - New Year's Day
    ///     - Good Friday
    ///     - Easter Monday
    ///     - Labour Day
    ///     - Liberation Day
    ///     - SS. Cyril and Methodius
    ///     - Jan Hus Day
    ///     - Czech Statehood Day
    ///     - Independence Day
    ///     - Struggle for Freedom and Democracy Day
    ///     - Christmas Eve
    ///     - Christmas
    ///     - St. Stephen
    ///     - Miscellaneous (2004-01-02, 2004-12-31)
    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || (d == 1 && m == Month::January)
            || (dd == em - 3 && y >= 2016)
            || (dd == em)
            || (d == 1 && m == Month::May)
            || (d == 8 && m == Month::May)
            || (d == 5 && m == Month::July)
            || (d == 6 && m == Month::July)
            || (d == 28 && m == Month::September)
            || (d == 28 && m == Month::October)
            || (d == 17 && m == Month::November)
            || (d == 24 && m == Month::December)
            || (d == 25 && m == Month::December)
            || (d == 26 && m == Month::December)
            || (d == 2 && m == Month::January && y == 2004)
            || (d == 31 && m == Month::December && y == 2004)
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
