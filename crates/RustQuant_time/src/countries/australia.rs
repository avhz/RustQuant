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

pub(crate) fn is_holiday_impl_australia(date: Date) -> bool {
    let (y, m, d, wd, yd, em) = unpack_date(date, false);

    if
    // New Year's Day (possibly moved to Monday)
    ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m == Month::January)
            // Australia Day, January 26th (possibly moved to Monday)
            || ((d == 26 || ((d == 27 || d == 28) && wd == Weekday::Monday)) && m == Month::January)
            // Good Friday
            || (yd == em - 3)
            // Easter Monday
            || (yd == em)
            // ANZAC Day
            || (d == 25 && m == Month::April)
            // Queen's Birthday, second Monday in June
            || ((d > 7 && d <= 14) && wd == Weekday::Monday && m == Month::June)
            // Bank Holiday, first Monday in August
            || (d <= 7 && wd == Weekday::Monday && m == Month::August)
            // Labour Day, first Monday in October
            || (d <= 7 && wd == Weekday::Monday && m == Month::October)
            // Christmas, December 25th (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
            // Boxing Day, December 26th (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
            // National Day of Mourning for Her Majesty, September 22 (only 2022)
            || (d == 22 && m == Month::September && y == 2022)
    {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
