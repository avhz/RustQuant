// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::{is_weekend, Calendar};
use time::{Month, OffsetDateTime};

/// Austrian settlement calendar.
pub struct Austria;

impl Calendar for Austria {
    fn name(&self) -> &'static str {
        "Austria"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let d = date.day();
        let m = date.month();
        let y = date.year();
        let dd = date.ordinal(); // Day of the year

        let em = crate::time::easter_monday(y as usize, false);

        if is_weekend(date)
                // New Year's Day
                || (d == 1 && m == Month::January)
                // Epiphany
                || (d == 6 && m == Month::January)
                // Easter Monday
                || (dd == em)
                // Ascension Thursday
                || (dd == em + 38)
                // Whit Monday
                || (dd == em + 49)
                // Corpus Christi
                || (dd == em + 59)
                // Labour Day
                || (d == 1 && m == Month::May)
                // Assumption
                || (d == 15 && m == Month::August)
                // National Holiday since 1967
                || (d == 26 && m == Month::October && y >= 1967)
                // National Holiday 1919-1934
                || (d == 12 && m == Month::November && (1919..=1934).contains(&y))
                // All Saints' Day
                || (d == 1 && m == Month::November)
                // Immaculate Conception
                || (d == 8 && m == Month::December)
                // Christmas
                || (d == 25 && m == Month::December)
                // St. Stephen
                || (d == 26 && m == Month::December)
        {
            return false;
        }

        true
    }
}
