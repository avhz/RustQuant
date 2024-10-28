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

/// United States of America national holiday calendar.
pub struct UnitedStatesCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for UnitedStatesCalendar {
    fn name(&self) -> &'static str {
        "United States of America"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::UNITED_STATES_OF_AMERICA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XNYS
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (
            // New Year's Day (possibly moved to Monday if on Sunday)
            ((d == 1 || (d == 2 && wd == Weekday::Monday)) && m == Month::January)

            // (or to Friday if on Saturday)
            || (d == 31 && wd == Weekday::Friday && m == Month::December)

            // Martin Luther King's birthday (third Monday in January)
            || ((15..=21).contains(&d) && wd == Weekday::Monday && m == Month::January && y >= 1983)

            // Washington's birthday (third Monday in February)
            || self.is_washington_birthday(date)

            // Memorial Day (last Monday in May)
            || self.is_memorial_day(date)

            // Juneteenth (Monday if Sunday or Friday if Saturday)
            || self.is_juneteenth(date, true)

            // Independence Day (Monday if Sunday or Friday if Saturday)
            || ((d == 4 || (d == 5 && wd == Weekday::Monday) || (d == 3 && wd == Weekday::Friday)) && m == Month::July)

            // Labor Day (first Monday in September)
            || self.is_labor_day(date)

            // Columbus Day (second Monday in October)
            || self.is_columbus_day(date)

            // Veteran's Day (Monday if Sunday or Friday if Saturday)
            || self.is_veterans_day(date)

            // Thanksgiving Day (fourth Thursday in November)
            || ((22..=28).contains(&d) && wd == Weekday::Thursday && m == Month::November)

            // Christmas (Monday if Sunday or Friday if Saturday)
            || ((d == 25 || (d == 26 && wd == Weekday::Monday) || (d == 24 && wd == Weekday::Friday)) && m == Month::December)
        ) {
            return true;
        }

        false
    }
}

impl Default for UnitedStatesCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitedStatesCalendar {
    /// Create a new instance of the United States of America calendar.
    pub fn new() -> Self {
        UnitedStatesCalendar
    }

    fn is_washington_birthday(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (y >= 1971) {
            // third Monday in February
            (15..=21).contains(&d) && wd == Weekday::Monday && m == Month::February
        } else {
            // February 22nd, possibly adjusted
            (d == 22 || (d == 23 && wd == Weekday::Monday) || (d == 21 && wd == Weekday::Friday))
                && m == Month::February
        }
    }

    fn is_memorial_day(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (y >= 1971) {
            // last Monday in May
            d >= 25 && wd == Weekday::Monday && m == Month::May
        } else {
            // May 30th, possibly adjusted
            (d == 30 || (d == 31 && wd == Weekday::Monday) || (d == 29 && wd == Weekday::Friday))
                && m == Month::May
        }
    }

    fn is_labor_day(&self, date: Date) -> bool {
        let (_, m, d, wd, _, _) = unpack_date(date, false);

        // first Monday in September
        d <= 7 && wd == Weekday::Monday && m == Month::September
    }

    fn is_columbus_day(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        // second Monday in October
        (8..=14).contains(&d) && wd == Weekday::Monday && m == Month::October && y >= 1971
    }

    fn is_veterans_day(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (y <= 1970 || y >= 1978) {
            // November 11th, adjusted
            (d == 11 || (d == 12 && wd == Weekday::Monday) || (d == 10 && wd == Weekday::Friday))
                && m == Month::November
        } else {
            // fourth Monday in October
            (22..=28).contains(&d) && wd == Weekday::Monday && m == Month::October
        }
    }

    fn is_juneteenth(&self, date: Date, move_to_friday: bool) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        // declared in 2021, but only observed by exchanges since 2022
        (d == 19
            || (d == 20 && wd == Weekday::Monday)
            || ((d == 18 && wd == Weekday::Friday) && move_to_friday))
            && m == Month::June
            && y >= 2022
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_united_states {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = UnitedStatesCalendar;
        assert_eq!(calendar.name(), "United States of America");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = UnitedStatesCalendar;
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = UnitedStatesCalendar;
        let new_years_day = date!(2023 - 01 - 01);
        let independence_day = date!(2023 - 07 - 04);
        let labor_day = date!(2023 - 09 - 04); // First Monday of September
        let thanksgiving = date!(2023 - 11 - 23); // Fourth Thursday of November
        let christmas = date!(2023 - 12 - 25);
        let washington_birthday = date!(2023 - 02 - 20); // This might need adjustment
        let memorial_day = date!(2023 - 05 - 29); // This might need adjustment
        let juneteenth = date!(2023 - 06 - 19);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(independence_day));
        assert!(!calendar.is_business_day(labor_day));
        assert!(!calendar.is_business_day(thanksgiving));
        assert!(!calendar.is_business_day(christmas));
        assert!(!calendar.is_business_day(washington_birthday));
        assert!(!calendar.is_business_day(memorial_day));
        assert!(!calendar.is_business_day(juneteenth));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = UnitedStatesCalendar;
        let regular_day1 = date!(2023 - 03 - 15);
        let regular_day2 = date!(2023 - 08 - 15);
        let regular_day3 = date!(2023 - 10 - 25);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
