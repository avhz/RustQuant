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

/// UnitedKingdom national holiday calendar.
pub struct UnitedKingdomCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for UnitedKingdomCalendar {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &'static str {
        "United Kingdom"
    }

    fn country_code(&self) -> ISO_3166 {
        UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND
    }

    fn market_identifier_code(&self) -> ISO_10383 {
        XLON
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day (possibly moved to Monday)
            ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m == Month::January)
             // Good Friday
             || (yd == em - 3)
             // Easter Monday
             || (yd == em)
             // Bank Holidays
             || self.is_bank_holiday(d, wd, m, y)
             // Christmas (possibly moved to Monday or Tuesday)
             || ((d == 25 || (d == 27 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
             // Boxing Day (possibly moved to Monday or Tuesday)
             || ((d == 26 || (d == 28 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
             // December 31st, 1999 only
             || (d == 31 && m == Month::December && y == 1999)
        ) {
            return true;
        }

        false
    }
}

impl UnitedKingdomCalendar {
    fn is_bank_holiday(&self, d: u8, w: Weekday, m: Month, y: i32) -> bool {
        // first Monday of May (Early May Bank Holiday)
        // moved to May 8th in 1995 and 2020 for V.E. day
        (d <= 7 && w == Weekday::Monday && m == Month::May && y != 1995 && y != 2020)
            || (d == 8 && m == Month::May && (y == 1995 || y == 2020))
            // last Monday of May (Spring Bank Holiday)
            // moved to in 2002, 2012 and 2022 for the Golden, Diamond and Platinum
            // Jubilee with an additional holiday
            || (d >= 25 && w == Weekday::Monday && m == Month::May && y != 2002 && y != 2012 && y != 2022)
            || ((d == 3 || d == 4) && m == Month::June && y == 2002)
            || ((d == 4 || d == 5) && m == Month::June && y == 2012)
            || ((d == 2 || d == 3) && m == Month::June && y == 2022)
            // last Monday of August (Summer Bank Holiday)
            || (d >= 25 && w == Weekday::Monday && m == Month::August)
            // April 29th, 2011 only (Royal Wedding Bank Holiday)
            || (d == 29 && m == Month::April && y == 2011)
            // September 19th, 2022 only (The Queen's Funeral Bank Holiday)
            || (d == 19 && m == Month::September && y == 2022)
            // May 8th, 2023 (King Charles III Coronation Bank Holiday)
            || (d == 8 && m == Month::May && y == 2023)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_united_kingdom {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = UnitedKingdomCalendar;
        assert_eq!(calendar.name(), "United Kingdom");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = UnitedKingdomCalendar;
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = UnitedKingdomCalendar;
        let new_years_day = date!(2023 - 01 - 01);
        let good_friday = date!(2023 - 04 - 07); // This date might need adjustment based on easter calculation
        let bank_holiday_may = date!(2023 - 05 - 01); // First Monday of May
        let coronation_day = date!(2023 - 05 - 08); // King Charles III Coronation Bank Holiday
        let christmas = date!(2023 - 12 - 25);
        let boxing_day = date!(2023 - 12 - 26);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(good_friday));
        assert!(!calendar.is_business_day(bank_holiday_may));
        assert!(!calendar.is_business_day(coronation_day));
        assert!(!calendar.is_business_day(christmas));
        assert!(!calendar.is_business_day(boxing_day));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = UnitedKingdomCalendar;
        let regular_day1 = date!(2023 - 03 - 15);
        let regular_day2 = date!(2023 - 07 - 11);
        let regular_day3 = date!(2023 - 09 - 15);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
