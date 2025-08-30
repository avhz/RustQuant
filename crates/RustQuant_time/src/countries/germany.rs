// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::unpack_date;
use time::{Date, Month};

pub(crate) fn is_holiday_impl_germany(date: Date) -> bool {
    let (_y, m, d, _wd, yd, em) = unpack_date(date, false);

    if (
        // New Year's Day
        d == 1 && m == Month::January

            // Good Friday
            || yd == em - 3

            // Easter Monday
            || yd == em

            // Ascension Thursday
            || yd == em + 38

            // Whit Monday
            || yd == em + 49

            // Corpus Christi
            || yd == em + 59

            // Labour Day
            || d == 1 && m == Month::May

            // National Day
            || d == 3 && m == Month::October

            // Christmas Eve
            || d == 24 && m == Month::December

            // Christmas
            || d == 25 && m == Month::December

            // Boxing Day
            || d == 26 && m == Month::December
    ) {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
