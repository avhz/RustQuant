// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use time::{Month, OffsetDateTime, Weekday};

/// Argentina calendar.
pub struct Argentina;

impl Calendar for Argentina {
    fn name(&self) -> &'static str {
        "Argentina"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::ARGENTINA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XBUE
    }

    /// Argentina holidays:
    ///     - New Year's Day
    ///     - Holy Thursday
    ///     - Good Friday
    ///     - Labour Day
    ///     - May Revolution
    ///     - Death of General Manuel Belgrano
    ///     - Independence Day
    ///     - Death of General José de San Martín
    ///     - Columbus Day
    ///     - Immaculate Conception
    ///     - Christmas Eve
    ///     - New Year's Eve
    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || (d == 1 && m == Month::January)
            || (dd == em - 4)
            || (dd == em - 3)
            || (d == 1 && m == Month::May)
            || (d == 25 && m == Month::May)
            || ((15..=21).contains(&d) && w == Weekday::Monday && m == Month::June)
            || (d == 9 && m == Month::July)
            || ((15..=21).contains(&d) && w == Weekday::Monday && m == Month::August)
            || ((d == 10 || d == 11 || d == 12 || d == 15 || d == 16)
                && w == Weekday::Monday
                && m == Month::October)
            || (d == 8 && m == Month::December)
            || (d == 24 && m == Month::December)
            || ((d == 31 || (d == 30 && w == Weekday::Friday)) && m == Month::December)
        {
            return false;
        }

        true
    }
}
