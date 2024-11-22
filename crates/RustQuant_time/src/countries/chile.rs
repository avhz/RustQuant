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

use crate::calendar::Calendar;
use crate::utilities::unpack_date;
use time::{Date, Month, Weekday};
use RustQuant_iso::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Chile national holiday calendar.
pub struct ChileCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for ChileCalendar {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &'static str {
        "Chile"
    }

    fn country_code(&self) -> ISO_3166 {
        CHILE
    }

    fn market_identifier_code(&self) -> ISO_10383 {
        XSGO
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day
            (d == 1 && m == Month::January) || (d == 2 && m == Month::January && wd == Weekday::Monday && y > 2016)

                // Good Friday
                || (yd == em - 3)
                // Easter Saturday
                || (yd == em - 2)

                // Labour Day
                || (d == 1 && m == Month::May)

                // Navy Day
                || (d == 21 && m == Month::May)

                // Day of Aboriginal People
                || (d == 21 && m == Month::June && y >= 2021)

                // St. Peter and St. Paul
                || ((26..=29).contains(&d) && m == Month::June && wd == Weekday::Monday)
                || (d == 2 && m == Month::July && wd == Weekday::Monday)

                // Our Lady of Mount Carmel
                || (d == 16 && m == Month::July)

                // Assumption Day
                || (d == 15 && m == Month::August)

                // Independence Day
                || (d == 17 && m == Month::September && ((wd == Weekday::Monday && y >= 2007)
                || (wd == Weekday::Friday && y > 2016)))
                || (d == 18 && m == Month::September)

                // Army Day
                || (d == 19 && m == Month::September)
                || (d == 20 && m == Month::September && wd == Weekday::Friday && y >= 2007)

                // Discovery of Two Worlds
                || ((9..=12).contains(&d) && m == Month::October && wd == Weekday::Monday)
                || (d == 15 && m == Month::October && wd == Weekday::Monday)

                // Reformation Day
                || (((d == 27 && m == Month::October && wd == Weekday::Friday)
                || (d == 31 && m == Month::October && wd != Weekday::Tuesday && wd != Weekday::Wednesday)
                || (d == 2 && m == Month::November && wd == Weekday::Friday)) && y >= 2008)

                // All Saints' Day
                || (d == 1 && m == Month::November)

                // Immaculate Conception
                || (d == 8 && m == Month::December)

                // Christmas Day
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
