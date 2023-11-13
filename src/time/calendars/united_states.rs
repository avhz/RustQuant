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

/// United States settlement calendar.
pub struct UnitedStates;

fn is_washington_birthday(d: u8, m: Month, y: i32, w: Weekday) -> bool {
    if y >= 1971 {
        (15..=21).contains(&d) && w == Weekday::Monday && m == Month::February
    } else {
        (d == 22 || (d == 23 && w == Weekday::Monday) || (d == 21 && w == Weekday::Friday))
            && m == Month::February
    }
}

fn is_memorial_day(d: u8, m: Month, y: i32, w: Weekday) -> bool {
    if y >= 1971 {
        d >= 25 && w == Weekday::Monday && m == Month::May
    } else {
        (d == 30 || (d == 31 && w == Weekday::Monday) || (d == 29 && w == Weekday::Friday))
            && m == Month::May
    }
}

fn is_juneteenth(d: u8, m: Month, y: i32, w: Weekday) -> bool {
    (d == 19 || (d == 20 && w == Weekday::Monday) || (d == 18 && w == Weekday::Friday))
        && m == Month::June
        && y >= 2022
}

impl Calendar for UnitedStates {
    fn name(&self) -> &'static str {
        "United States"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::ISO_3166 {
            alpha_2: "US",
            alpha_3: "USA",
            numeric: "840",
        }
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || ((d == 1 || (d == 2 && w == Weekday::Monday)) && m == Month::January)
            || (d == 31 && w == Weekday::Friday && m == Month::December)
            || ((15..=21).contains(&d) && w == Weekday::Monday && m == Month::January && y >= 1983)
            || is_washington_birthday(d, m, y, w)
            || is_memorial_day(d, m, y, w)
            || is_juneteenth(d, m, y, w)
            || ((d == 4 || (d == 5 && w == Weekday::Monday) || (d == 3 && w == Weekday::Friday))
                && m == Month::July)
            || (d <= 7 && w == Weekday::Monday && m == Month::September)
            || ((8..=14).contains(&d) && w == Weekday::Monday && m == Month::October && y >= 1971)
            || ((d == 11 || (d == 12 && w == Weekday::Monday) || (d == 10 && w == Weekday::Friday))
                && m == Month::November)
            || ((22..=28).contains(&d) && w == Weekday::Thursday && m == Month::November)
            || ((d == 25 || (d == 26 && w == Weekday::Monday) || (d == 24 && w == Weekday::Friday))
                && m == Month::December)
        {
            return false;
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS for United States
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_united_states {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = UnitedStates;
        assert_eq!(calendar.name(), "United States");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = UnitedStates;
        let sat = datetime!(2023-08-26 12:00:00 UTC);
        let sun = datetime!(2023-08-27 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = UnitedStates;
        let new_years_day = datetime!(2023-01-01 12:00:00 UTC);
        let independence_day = datetime!(2023-07-04 12:00:00 UTC);
        let labor_day = datetime!(2023-09-04 12:00:00 UTC); // First Monday of September
        let thanksgiving = datetime!(2023-11-23 12:00:00 UTC); // Fourth Thursday of November
        let christmas = datetime!(2023-12-25 12:00:00 UTC);
        let washington_birthday = datetime!(2023-02-20 12:00:00 UTC); // This might need adjustment
        let memorial_day = datetime!(2023-05-29 12:00:00 UTC); // This might need adjustment
        let juneteenth = datetime!(2023-06-19 12:00:00 UTC);

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
        let calendar = UnitedStates;
        let regular_day1 = datetime!(2023-03-15 12:00:00 UTC);
        let regular_day2 = datetime!(2023-08-15 12:00:00 UTC);
        let regular_day3 = datetime!(2023-10-25 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
