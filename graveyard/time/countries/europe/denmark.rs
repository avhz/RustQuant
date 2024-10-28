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

/// Denmark national holiday calendar.
pub struct DenmarkCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for DenmarkCalendar {
    fn name(&self) -> &'static str {
        "Denmark"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::DENMARK
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XCSE
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, _, yd, em) = unpack_date(date, false);

        if (
            // Maundy Thursday
            (yd == em - 4)

            // Good Friday
            || (yd == em - 3)

            // Easter Monday
            || (yd == em)

            // General Prayer Day
            || (yd == em + 25 && y <= 2023)

            // Ascension
            || (yd == em + 38)

            // Day after Ascension
            || (yd == em + 39 && y >= 2009)

            // Whit Monday
            || (yd == em + 49)

            // New Year's Day
            || (d == 1 && m == Month::January)

            // Constitution Day, June 5th
            || (d == 5 && m == Month::June)

            // Christmas Eve
            || (d == 24 && m == Month::December)

            // Christmas
            || (d == 25 && m == Month::December)

            // Boxing Day
            || (d == 26 && m == Month::December)

            // New Year's Eve
            || (d == 31 && m == Month::December)
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
mod test_denmark {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = DenmarkCalendar;
        assert_eq!(calendar.name(), "Denmark");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = DenmarkCalendar;
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);

        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = DenmarkCalendar;
        let new_years_day = date!(2023 - 01 - 01);
        let maunday_thursday = date!(2023 - 04 - 06);
        let good_friday = date!(2023 - 04 - 07);
        let easter_monday = date!(2023 - 04 - 10);
        let general_prayer_day = date!(2023 - 05 - 05);
        let ascension = date!(2023 - 05 - 18);
        let day_after_ascension = date!(2023 - 05 - 19);
        let whit_monday = date!(2023 - 05 - 29);
        let constitution_day = date!(2023 - 06 - 05);
        let christmas_eve = date!(2023 - 12 - 24);
        let christmas = date!(2023 - 12 - 25);
        let boxing_day = date!(2023 - 12 - 26);
        let new_years_eve = date!(2023 - 12 - 31);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(maunday_thursday));
        assert!(!calendar.is_business_day(good_friday));
        assert!(!calendar.is_business_day(easter_monday));
        assert!(!calendar.is_business_day(general_prayer_day));
        assert!(!calendar.is_business_day(ascension));
        assert!(!calendar.is_business_day(day_after_ascension));
        assert!(!calendar.is_business_day(whit_monday));
        assert!(!calendar.is_business_day(constitution_day));
        assert!(!calendar.is_business_day(christmas_eve));
        assert!(!calendar.is_business_day(christmas));
        assert!(!calendar.is_business_day(boxing_day));
        assert!(!calendar.is_business_day(new_years_eve));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = DenmarkCalendar;
        let regular_day1 = date!(2023 - 03 - 15);
        let regular_day2 = date!(2023 - 07 - 11);
        let regular_day3 = date!(2023 - 09 - 15);
        let regular_day4 = date!(2008 - 05 - 02); // Day after ascension (before 2009)
        let regular_day5 = date!(2024 - 04 - 26); // General Prayer Day (after 2023)

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
        assert!(calendar.is_business_day(regular_day4));
        assert!(calendar.is_business_day(regular_day5));
    }
}
