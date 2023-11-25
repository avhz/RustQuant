// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use time::{Month, OffsetDateTime};

/// Czech Republic calendar.
pub struct Denmark;

impl Calendar for Denmark {
    fn name(&self) -> &'static str {
        "Denmark"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::DENMARK
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XCSE
    }

    /// Denmark holidays:
    ///     - Maunday Thursday
    ///     - Good Friday
    ///     - Easter Monday
    ///     - General Prayer Day
    ///     - Ascension
    ///     - Day after Ascension
    ///     - Whit Monday
    ///     - New Year's Day
    ///     - Constitution Day, June 5th
    ///     - Christmas Eve
    ///     - Christmas
    ///     - Boxing Day
    ///     - New Year's Eve
    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            || (dd == em - 4)
            || (dd == em - 3)
            || (dd == em)
            || (dd == em + 25 && y <= 2023)
            || (dd == em + 38)
            || (dd == em + 39 && y >= 2009)
            || (dd == em + 49)
            || (d == 1 && m == Month::January)
            || (d == 5 && m == Month::June)
            || (d == 24 && m == Month::December)
            || (d == 25 && m == Month::December)
            || (d == 26 && m == Month::December)
            || (d == 31 && m == Month::December)
        {
            return false;
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_denmark {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Denmark;
        assert_eq!(calendar.name(), "Denmark");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Denmark;
        let sat = datetime!(2023-08-26 12:00:00 UTC);
        let sun = datetime!(2023-08-27 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Denmark;
        let new_years_day = datetime!(2023-01-01 12:00:00 UTC);
        let maunday_thursday = datetime!(2023-04-06 12:00:00 UTC);
        let good_friday = datetime!(2023-04-07 12:00:00 UTC);
        let easter_monday = datetime!(2023-04-10 12:00:00 UTC);
        let general_prayer_day = datetime!(2023-05-05 12:00:00 UTC);
        let ascension = datetime!(2023-05-18 12:00:00 UTC);
        let day_after_ascension = datetime!(2023-05-19 12:00:00 UTC);
        let whit_monday = datetime!(2023-05-29 12:00:00 UTC);
        let constitution_day = datetime!(2023-06-05 12:00:00 UTC);
        let christmas_eve = datetime!(2023-12-24 12:00:00 UTC);
        let christmas = datetime!(2023-12-25 12:00:00 UTC);
        let boxing_day = datetime!(2023-12-26 12:00:00 UTC);
        let new_years_eve = datetime!(2023-12-31 12:00:00 UTC);

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
        let calendar = Denmark;
        let regular_day1 = datetime!(2023-03-15 12:00:00 UTC);
        let regular_day2 = datetime!(2023-07-11 12:00:00 UTC);
        let regular_day3 = datetime!(2023-09-15 12:00:00 UTC);
        let regular_day4 = datetime!(2008-05-02 12:00:00 UTC); // Day after ascension (before 2009)
        let regular_day5 = datetime!(2024-04-26 12:00:00 UTC); // General Prayer Day (after 2023)

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
        assert!(calendar.is_business_day(regular_day4));
        assert!(calendar.is_business_day(regular_day5));
    }
}
