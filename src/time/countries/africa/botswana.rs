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
use time::{Date, Month, Weekday};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Botswana national holiday calendar.
pub struct BotswanaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for BotswanaCalendar {
    fn name(&self) -> &'static str {
        "Botswana"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::BOTSWANA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XBOT
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (_, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day (possibly moved to Monday or Tuesday)
            ((d == 1 || (d == 2 && wd == Weekday::Monday) || (d == 3 && wd == Weekday::Tuesday))  && m == Month::January) ||

            // Good Friday
            (yd == em - 3) ||

            // Easter Monday
            (yd == em) ||

            // Labour Day, May 1st (possibly moved to Monday)
            ((d == 1 || (d == 2 && wd == Weekday::Monday)) && m == Month::May) ||

            // Ascension
            (yd == em + 38) ||

            // Sir Seretse Khama Day, July 1st (possibly moved to Monday)
            ((d == 1 || (d == 2 && wd == Weekday::Monday)) && m == Month::July) ||

            // Presidents' Day (third Monday of July)
            ((15..=21).contains(&d) && wd == Weekday::Monday && m == Month::July) ||

            // Independence Day, September 30th (possibly moved to Monday)
            ((d == 30 && m == Month::September) || (d == 1  && wd == Weekday::Monday && m == Month::October)) ||

            // Botswana Day, October 1st (possibly moved to Monday or Tuesday)
            ((d == 1 || (d == 2 && wd == Weekday::Monday) || (d == 3 && wd == Weekday::Tuesday))  && m == Month::October) ||

            // Christmas
            (d == 25 && m == Month::December) ||

            // Boxing Day (possibly moved to Monday)
            ((d == 26 || (d == 27 && wd == Weekday::Monday)) && m == Month::December)
        ) {
            return true;
        }

        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
