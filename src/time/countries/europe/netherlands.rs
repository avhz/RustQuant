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

/// Netherlands national holiday calendar.
pub struct NetherlandsCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for NetherlandsCalendar {
    fn name(&self) -> &'static str {
        "Netherlands"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::NETHERLANDS
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XAMS
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day
            d == 1 && m == Month::January
            
            // Good Friday
            || yd == em - 2
            
            // Easter Sunday
            || yd == em
            
            // Easter Monday
            || yd == em + 1
            
            // King's Day (April 27, or April 26 if 27th is a Sunday)
            || (d == 27 && m == Month::April) || (d == 26 && m == Month::April && wd.number_from_monday() == 7)
            
            // Liberation Day (every year, but only a day off every 5 years from 2020)
            || (d == 5 && m == Month::May && (y % 5 == 0 && y >= 2020))
            
            // Ascension Day
            || yd == em + 39
            
            // Whit Sunday
            || yd == em + 49
            
            // Whit Monday
            || yd == em + 50
            
            // Christmas Day
            || d == 25 && m == Month::December
          
            // Boxing Day
            || d == 26 && m == Month::December
        ) {
            return true;
        }
        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
