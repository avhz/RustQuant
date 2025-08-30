// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::*;
use time::{Date, Month};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) fn is_holiday_impl_austria(date: Date) -> bool {
    let (y, m, d, _, yd, em) = unpack_date(date, false);

    if (
        // New Year's Day
        (d == 1 && m == Month::January) ||

            // Epiphany
            (d == 6 && m == Month::January) ||

            // Easter Monday
            (yd == em) ||

            // Ascension Thurday 
            (yd == em+38) ||

            // Whit Monday
            (yd == em+49) ||

            // Corpus Christi
            (yd == em+59) ||

            // Labour Day
            (d == 1 && m == Month::May) ||

            // Assumption
            (d == 15 && m == Month::August) ||

            // National Holiday since 1967
            (d == 26 && m == Month::October && y >= 1967) ||

            // National Holiday 1919-1934
            (d == 12 && m == Month::November && (1919..=1934).contains(&y)) ||

            // All Saints' Day
            (d == 1 && m == Month::November) ||

            // Immaculate Conception
            (d == 8 && m == Month::December) ||

            // Christmas
            (d == 25 && m == Month::December) ||

            // St. Stephen
            (d == 26 && m == Month::December)
    ) {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
