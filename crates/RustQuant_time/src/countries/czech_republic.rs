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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) fn is_holiday_impl_czech_republic(date: Date) -> bool {
    let (y, m, d, _wd, yd, em) = unpack_date(date, false);

    if (
        // New Year's Day
        (d == 1 && m == Month::January)

            // Good Friday
            || (yd == em - 3 && y >= 2016)

            // Easter Monday
            || (yd == em)

            // Labour Day
            || (d == 1 && m == Month::May)

            // Liberation Day
            || (d == 8 && m == Month::May)

            // SS. Cyril and Methodius
            || (d == 5 && m == Month::July)

            // Jan Hus Day
            || (d == 6 && m == Month::July)

            // Czech Statehood Day
            || (d == 28 && m == Month::September)

            // Independence Day
            || (d == 28 && m == Month::October)

            // Struggle for Freedom and Democracy Day
            || (d == 17 && m == Month::November)

            // Christmas Eve
            || (d == 24 && m == Month::December)

            // Christmas
            || (d == 25 && m == Month::December)

            // St. Stephen
            || (d == 26 && m == Month::December)

            // Miscellaneous
            || (d == 2 && m == Month::January && y == 2004)
            || (d == 31 && m == Month::December && y == 2004)
    ) {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
