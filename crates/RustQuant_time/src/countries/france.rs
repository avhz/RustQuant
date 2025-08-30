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

pub(crate) fn is_holiday_impl_france(date: Date) -> bool {
    let (_y, m, d, _wd, yd, em) = unpack_date(date, false);

    if (
        // Jour de l'An
        (d == 1 && m == Month::January)
            // Lundi de Paques
            || (yd == em)
            // Fete du Travail
            || (d == 1 && m == Month::May)
            // Victoire 1945
            || (d == 8 && m == Month::May)
            // Ascension
            || (d == 10 && m == Month::May)
            // Pentecote
            || (d == 21 && m == Month::May)
            // Fete nationale
            || (d == 14 && m == Month::July)
            // Assomption
            || (d == 15 && m == Month::August)
            // Toussaint
            || (d == 1 && m == Month::November)
            // Armistice 1918
            || (d == 11 && m == Month::November)
            // Noel
            || (d == 25 && m == Month::December)
    ) {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
