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
use time::{Date, Month};
use RustQuant_iso::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Hungary national holiday calendar.
pub struct HungaryCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for HungaryCalendar {
    fn name(&self) -> &'static str {
        "Hungary"
    }

    fn country_code(&self) -> ISO_3166 {
        HUNGARY
    }

    fn market_identifier_code(&self) -> ISO_10383 {
        XBUD
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (_y, m, d, _wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day
            (d == 1 && m == Month::January)
            // 1848 Revolution Memorial Day
            || (d == 15 && m == Month::March)
            // Good Friday
            || (yd == em - 3)
            // Easter Monday
            || (yd == em)
            // Labor Day / May Day
            || (d == 1 && m == Month::May)
            // Whit Monday
            || (yd == em + 49)
            // Hungary National Day
            || (d == 20 && m == Month::August)
            // 1956 Revolution Memorial Day
            || (d == 23 && m == Month::October)
            // All Saints' Day
            || (d == 1 && m == Month::November)
            // Christmas
            || (d == 25 && m == Month::December)
            // Second Day of Christmas
            || (d == 26 && m == Month::December)
        ) {
            return true;
        }

        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_hungary {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = HungaryCalendar;
        assert_eq!(calendar.name(), "Hungary");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = HungaryCalendar;
        let sat = date!(2024 - 01 - 13);
        let sun = date!(2024 - 01 - 14);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = HungaryCalendar;
        let new_years_day = date!(2024 - 01 - 01);
        let revolution_1848_day = date!(2024 - 03 - 15);
        let labour_day = date!(2024 - 05 - 01);
        let national_holiday = date!(2024 - 08 - 20);
        let revolution_1956_day = date!(2024 - 10 - 23);
        let christmas = date!(2024 - 12 - 25);
        let second_christmas_day = date!(2024 - 12 - 26);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(revolution_1848_day));
        assert!(!calendar.is_business_day(labour_day));
        assert!(!calendar.is_business_day(national_holiday));
        assert!(!calendar.is_business_day(revolution_1956_day));
        assert!(!calendar.is_business_day(christmas));
        assert!(!calendar.is_business_day(second_christmas_day));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = HungaryCalendar;
        let regular_day1 = date!(2024 - 03 - 07);
        let regular_day2 = date!(2024 - 07 - 02);
        let regular_day3 = date!(2024 - 12 - 11);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
