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
        crate::iso::ISO_3166 {
            alpha_2: "AR",
            alpha_3: "ARG",
            numeric: "032",
        }
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // New Year's Day
            || (d == 1 && m == Month::January)
            // Holy Thursday
            || (dd == em-4)
            // Good Friday
            || (dd == em-3)
            // Labour Day
            || (d == 1 && m == Month::May)
            // May Revolution
            || (d == 25 && m == Month::May)
            // Death of General Manuel Belgrano
            || (d >= 15 && d <= 21 && w == Weekday::Monday && m == Month::June)
            // Independence Day
            || (d == 9 && m == Month::July)
            // Death of General José de San Martín
            || (d >= 15 && d <= 21 && w ==Weekday::Monday && m == Month::August)
            // Columbus Day
            || ((d == 10 || d == 11 || d == 12 || d == 15 || d == 16)
                && w == Weekday::Monday && m == Month::October)
            // Immaculate Conception
            || (d == 8 && m == Month::December)
            // Christmas Eve
            || (d == 24 && m == Month::December)
            // New Year's Eve
            || ((d == 31 || (d == 30 && w == Weekday::Friday)) && m == Month::December)
        {
            return false;
        }

        return true;
    }
}
