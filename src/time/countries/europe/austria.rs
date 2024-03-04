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

/// Austria national holiday calendar.
pub struct AustriaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for AustriaCalendar {
    fn name(&self) -> &'static str {
        "Austria"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::AUSTRIA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::EXAA
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, _, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day
            (d == 1 && m == Month::January) ||

            // Epiphany
            (d == 6 && m == Month::January) ||

            // Easter Monday
            (yd == em) ||

            // Ascension Thurday 
            (yd == em+38) ||

            // Whit Monday
            (yd == em+49) ||

            // Corpus Christi
            (yd == em+59) ||

            // Labour Day
            (d == 1 && m == Month::May) ||

            // Assumption
            (d == 15 && m == Month::August) ||

            // National Holiday since 1967
            (d == 26 && m == Month::October && y >= 1967) ||

            // National Holiday 1919-1934
            (d == 12 && m == Month::November && (1919..=1934).contains(&y)) ||

            // All Saints' Day
            (d == 1 && m == Month::November) ||

            // Immaculate Conception
            (d == 8 && m == Month::December) ||

            // Christmas
            (d == 25 && m == Month::December) ||

            // St. Stephen
            (d == 26 && m == Month::December)
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
mod test_austria {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = AustriaCalendar;
        assert_eq!(calendar.name(), "Austria");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = AustriaCalendar;
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = AustriaCalendar;
        let new_years_day = date!(2023 - 01 - 01);
        let epiphany = date!(2023 - 01 - 06);
        let labour_day = date!(2023 - 05 - 01);
        let national_holiday = date!(2023 - 10 - 26);
        let christmas = date!(2023 - 12 - 25);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(epiphany));
        assert!(!calendar.is_business_day(labour_day));
        assert!(!calendar.is_business_day(national_holiday));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = AustriaCalendar;
        let regular_day1 = date!(2023 - 02 - 01);
        let regular_day2 = date!(2023 - 07 - 12);
        let regular_day3 = date!(2023 - 11 - 17);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
