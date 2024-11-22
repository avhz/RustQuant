// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines Australian holidays and calendars.

use crate::calendar::Calendar;
use crate::utilities::unpack_date;
use time::{Date, Month, Weekday};
use RustQuant_iso::*;

/// Australian national holiday calendar.
#[derive(Debug, Clone, Copy)]
pub struct AustraliaCalendar;

impl AustraliaCalendar {
    /// Creates a new instance of the Australian calendar.
    pub fn new() -> Self {
        Self
    }
}

impl Calendar for AustraliaCalendar {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &'static str {
        "Australia"
    }

    fn country_code(&self) -> ISO_3166 {
        AUSTRALIA
    }

    fn market_identifier_code(&self) -> ISO_10383 {
        XASX
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if
        // New Year's Day (possibly moved to Monday)
        ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m == Month::January)
            // Australia Day, January 26th (possibly moved to Monday)
            || ((d == 26 || ((d == 27 || d == 28) && wd == Weekday::Monday)) && m == Month::January)
            // Good Friday
            || (yd == em - 3)
            // Easter Monday
            || (yd == em)
            // ANZAC Day
            || (d == 25 && m == Month::April)
            // Queen's Birthday, second Monday in June
            || ((d > 7 && d <= 14) && wd == Weekday::Monday && m == Month::June)
            // Bank Holiday, first Monday in August
            || (d <= 7 && wd == Weekday::Monday && m == Month::August)
            // Labour Day, first Monday in October
            || (d <= 7 && wd == Weekday::Monday && m == Month::October)
            // Christmas, December 25th (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
            // Boxing Day, December 26th (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
            // National Day of Mourning for Her Majesty, September 22 (only 2022)
            || (d == 22 && m == Month::September && y == 2022)
        {
            return true;
        }

        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_australia {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = AustraliaCalendar;
        assert_eq!(calendar.name(), "Australia");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = AustraliaCalendar;
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = AustraliaCalendar;
        let new_years_day = date!(2023 - 01 - 01);
        let australia_day = date!(2023 - 01 - 26);
        let anzac_day = date!(2023 - 04 - 25);
        let christmas = date!(2023 - 12 - 25);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(australia_day));
        assert!(!calendar.is_business_day(anzac_day));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = AustraliaCalendar;
        let regular_day1 = date!(2023 - 03 - 01);
        let regular_day2 = date!(2023 - 07 - 12);
        let regular_day3 = date!(2023 - 11 - 17);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
