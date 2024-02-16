// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::calendar::Calendar;
use crate::time::utilities::unpack_date;
use time::{Date, Month};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// France national holiday calendar.
pub struct FranceCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for FranceCalendar {
    fn name(&self) -> &'static str {
        "France"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::FRANCE
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XPAR
    }

    fn is_holiday(&self, date: Date) -> bool {
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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
