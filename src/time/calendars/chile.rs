// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use time::{Month, OffsetDateTime, Weekday};

/// Chile calendar.
pub struct Chile;

impl Calendar for Chile {
    fn name(&self) -> &'static str {
        "Chile"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::ISO_3166 {
            alpha_2: "CL",
            alpha_3: "CHL",
            numeric: "152",
        }
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // New Year's Day
            || (d == 1 && m == Month::January)
            || (d == 2 && m == Month::January && w == Weekday::Monday && y > 2016)
            // Good Friday
            || (dd == em-3)
            // Easter Saturday
            || (dd == em-2)
            // Labour Day
            || (d == 1 && m == Month::May)
            // Navy Day
            || (d == 21 && m == Month::May)
            // Day of Aboriginal People
            || (d == 21 && m == Month::June && y >= 2021)
            // St. Peter and St. Paul
            || ((26..=29).contains(&d) && m == Month::June && w == Weekday::Monday)
            || (d == 2 && m == Month::July && w == Weekday::Monday)
            // Our Lady of Mount Carmel
            || (d == 16 && m == Month::July)
            // Assumption Day
            || (d == 15 && m == Month::August)
            // Independence Day
            || (d == 17 && m == Month::September && ((w == Weekday::Monday && y >= 2007) || (w == Weekday::Friday && y > 2016)))
            || (d == 18 && m == Month::September)
            // Army Day
            || (d == 19 && m == Month::September)
            || (d == 20 && m == Month::September && w == Weekday::Friday && y >= 2007)
            // Discovery of Two Worlds
            || ((9..=12).contains(&d) && m == Month::October && w == Weekday::Monday)
            || (d == 15 && m == Month::October && w == Weekday::Monday)
            // Reformation Day
            || (((d == 27 && m == Month::October && w == Weekday::Friday)
                || (d == 31 && m == Month::October && w != Weekday::Tuesday && w != Weekday::Wednesday)
                || (d == 2 && m == Month::November && w == Weekday::Friday)) && y >= 2008)
            // All Saints' Day
            || (d == 1 && m == Month::November)
            // Immaculate Conception
            || (d == 8 && m == Month::December)
            // Christmas Day
            || (d == 25 && m == Month::December)
        {
            return false;
        }

        true
    }
}
