// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines Australian holidays and calendars.

use crate::time::calendar::Calendar;
use crate::time::utilities::unpack_date;
use time::{Date, Month, Weekday};

/// New Zealand national holiday calendar.
pub struct NewZealandCalendar;

impl Calendar for NewZealandCalendar {
    fn name(&self) -> &'static str {
        "New Zealand"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::NEW_ZEALAND
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XNZE
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if
        // New Year's Day (possibly moved to Monday or Tuesday)
        ((d == 1 || (d == 3 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::January)
            // Day after New Year's Day (possibly moved to Mon or Tuesday)
            || ((d == 2 || (d == 4 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::January)
            // Anniversary Day, Monday nearest January 22nd
            || ((19..=25).contains(&d) && wd == Weekday::Monday && m == Month::January)
            // Waitangi Day. February 6th ("Mondayised" since 2013)
            || (d == 6 && m == Month::February)
            || ((d == 7 || d == 8) && wd == Weekday::Monday && m == Month::February && y > 2013)
            // Good Friday
            || (yd == em - 3)
            // Easter Monday
            || (yd == em)
            // ANZAC Day. April 25th ("Mondayised" since 2013) 
            || (d == 25 && m == Month::April)
            || ((d == 26 || d == 27) && wd == Weekday::Monday && m == Month::April && y > 2013)
            // Queen's Birthday, first Monday in June
            || (d <= 7 && wd == Weekday::Monday && m == Month::June)
            // Labour Day, fourth Monday in October
            || ((22..=28).contains(&d) && wd == Weekday::Monday && m == Month::October)
            // Christmas, December 25th (possibly Monday or Tuesday)
            || ((d == 25 || (d == 27 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
            // Boxing Day, December 26th (possibly Monday or Tuesday)
            || ((d == 26 || (d == 28 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
            // Matariki, it happens on Friday in June or July
            // official calendar released by the NZ government for the
            // next 30 years
            || (d == 20 && m == Month::June && y == 2025)
            || (d == 21 && m == Month::June && (y == 2030 || y == 2052))
            || (d == 24 && m == Month::June && (y == 2022 || y == 2033 || y == 2044))
            || (d == 25 && m == Month::June && (y == 2027 || y == 2038 || y == 2049))
            || (d == 28 && m == Month::June && y == 2024)
            || (d == 29 && m == Month::June && (y == 2035 || y == 2046))
            || (d == 30 && m == Month::June && y == 2051)
            || (d == 2  && m == Month::July && y == 2032)
            || (d == 3  && m == Month::July && (y == 2043 || y == 2048))
            || (d == 6  && m == Month::July && (y == 2029 || y == 2040))
            || (d == 7  && m == Month::July && (y == 2034 || y == 2045))
            || (d == 10 && m == Month::July && (y == 2026 || y == 2037))
            || (d == 11 && m == Month::July && (y == 2031 || y == 2042))
            || (d == 14 && m == Month::July && (y == 2023 || y == 2028))
            || (d == 15 && m == Month::July && (y == 2039 || y == 2050))
            || (d == 18 && m == Month::July && y == 2036)
            || (d == 19 && m == Month::July && (y == 2041 || y == 2047))
        {
            return true;
        }

        false
    }
}
