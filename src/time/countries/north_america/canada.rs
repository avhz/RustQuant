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

/// Canada national holiday calendar.
pub struct CanadaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for CanadaCalendar {
    fn name(&self) -> &'static str {
        "Canada"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::CANADA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XCNQ
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day (possibly moved to Monday)
            ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m == Month::January)

            // Family Day (third Monday in February, since 2008)
            || ((15..=21).contains(&d) && wd == Weekday::Monday && m == Month::February && y >= 2008)

            // Good Friday
            || (yd == em-3)

            // The Monday on or preceding 24 May (Victoria Day)
            || (d > 17 && d <= 24 && wd == Weekday::Monday && m == Month::May)

            // July 1st, possibly moved to Monday (Canada Day)
            || ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m==Month::July)

            // first Monday of August (Provincial Holiday)
            || (d <= 7 && wd == Weekday::Monday && m == Month::August)

            // first Monday of September (Labor Day)
            || (d <= 7 && wd == Weekday::Monday && m == Month::September)

            // September 30th, possibly moved to Monday
            // (National Day for Truth and Reconciliation, since 2021)
            || (((d == 30 && m == Month::September) || (d <= 2 && m == Month::October && wd == Weekday::Monday)) && y >= 2021)

            // second Monday of October (Thanksgiving Day)
            || (d > 7 && d <= 14 && wd == Weekday::Monday && m == Month::October)

            // November 11th (possibly moved to Monday)
            || ((d == 11 || ((d == 12 || d == 13) && wd == Weekday::Monday)) && m == Month::November)

            // Christmas (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)

            // Boxing Day (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
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
mod test_canada {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = CanadaCalendar;
        assert_eq!(calendar.name(), "Canada");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = CanadaCalendar;
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = CanadaCalendar;
        let new_years_day = date!(2023 - 01 - 01);
        let family_day = date!(2023 - 02 - 20); // 3rd Monday of February
        let canada_day = date!(2023 - 07 - 01);
        let thanksgiving = date!(2023 - 10 - 09); // 2nd Monday in October
        let christmas = date!(2023 - 12 - 25);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(family_day));
        assert!(!calendar.is_business_day(canada_day));
        assert!(!calendar.is_business_day(thanksgiving));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = CanadaCalendar;
        let regular_day1 = date!(2023 - 03 - 01);
        let regular_day2 = date!(2023 - 07 - 12);
        let regular_day3 = date!(2023 - 11 - 17);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
