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

pub(crate) fn is_holiday_impl_brazil(date: Date) -> bool {
    let (y, m, d, _, yd, em) = unpack_date(date, false);

    if (
        // New Year's Day
        (d == 1 && m == Month::January)

            // Tiradentes Day
            || (d == 21 && m == Month::April)

            // Labor Day
            || (d == 1 && m == Month::May)

            // Independence Day
            || (d == 7 && m == Month::September)

            // Nossa Sra. Aparecida Day
            || (d == 12 && m == Month::October)

            // All Souls Day
            || (d == 2 && m == Month::November)

            // Republic Day
            || (d == 15 && m == Month::November)

            // Black Awareness Day
            || (d == 20 && m == Month::November && y >= 2024)

            // Christmas
            || (d == 25 && m == Month::December)

            // Passion of Christ
            || (yd == em-3)

            // Carnival
            || (yd == em-49 || yd == em-48)

            // Corpus Christi
            || (yd == em+59)
    ) {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
