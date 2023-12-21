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

/// Canadian settlement calendar.
pub struct Canada;

impl Calendar for Canada {
    fn name(&self) -> &'static str {
        "Canada"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::CANADA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XCNQ
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || ((d == 1 || ((d == 2 || d == 3) && w == Weekday::Monday)) && m == Month::January)
            || ((15..=21).contains(&d) && w == Weekday::Monday && m == Month::February && y >= 2008)
            || (dd == em - 3)
            || (d > 17 && d <= 24 && w == Weekday::Monday && m == Month::May)
            || ((d == 1 || ((d == 2 || d == 3) && w == Weekday::Monday)) && m == Month::July)
            || (d <= 7 && w == Weekday::Monday && m == Month::August)
            || (d <= 7 && w == Weekday::Monday && m == Month::September)
            || (((d == 30 && m == Month::September)
                || (d <= 2 && m == Month::October && w == Weekday::Monday))
                && y >= 2021)
            || (d > 7 && d <= 14 && w == Weekday::Monday && m == Month::October)
            || ((d == 11 || ((d == 12 || d == 13) && w == Weekday::Monday)) && m == Month::November)
            || ((d == 25 || (d == 27 && (w == Weekday::Monday || w == Weekday::Tuesday)))
                && m == Month::December)
            || ((d == 26 || (d == 28 && (w == Weekday::Monday || w == Weekday::Tuesday)))
                && m == Month::December)
        {
            return false;
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS for Canada
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_canada {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Canada;
        assert_eq!(calendar.name(), "Canada");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Canada;
        let sat = datetime!(2023-08-26 12:00:00 UTC);
        let sun = datetime!(2023-08-27 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Canada;
        let new_years_day = datetime!(2023-01-01 12:00:00 UTC);
        let family_day = datetime!(2023-02-20 12:00:00 UTC); // 3rd Monday of February
        let canada_day = datetime!(2023-07-01 12:00:00 UTC);
        let thanksgiving = datetime!(2023-10-09 12:00:00 UTC); // 2nd Monday in October
        let christmas = datetime!(2023-12-25 12:00:00 UTC);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(family_day));
        assert!(!calendar.is_business_day(canada_day));
        assert!(!calendar.is_business_day(thanksgiving));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = Canada;
        let regular_day1 = datetime!(2023-03-01 12:00:00 UTC);
        let regular_day2 = datetime!(2023-07-12 12:00:00 UTC);
        let regular_day3 = datetime!(2023-11-17 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
