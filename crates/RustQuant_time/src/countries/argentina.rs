// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::*;
use time::{Date, Month, Weekday};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) fn is_holiday_impl_argentina(date: Date) -> bool {
    let (_, m, d, wd, yd, em) = unpack_date(date, false);

    if
    // New Year's Day
    is_christmas_day(date)
            // Holy Thursday
            || (yd == em-4)
            // Good Friday
            || (yd == em-3)
            // Labour Day
            || (d == 1 && m == Month::May)
            // May Revolution
            || (d == 25 && m == Month::May)
            // Death of General Manuel Belgrano
            || ((15..=21).contains(&d) && wd == Weekday::Monday && m == Month::June)
            // Independence Day
            || (d == 9 && m == Month::July)
            // Death of General José de San Martín
            || ((15..=21).contains(&d) && wd == Weekday::Monday && m == Month::August)
            // Columbus Day
            || ((d == 10 || d == 11 || d == 12 || d == 15 || d == 16) && wd == Weekday::Monday && m == Month::October)
            // Immaculate Conception
            || (d == 8 && m == Month::December)
            // Christmas Eve
            || is_christmas_eve(date)
            // New Year's Eve
            || is_new_years_eve(date)
    {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
