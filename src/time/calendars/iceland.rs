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

/// Iceland calendar.
pub struct Iceland;

impl Calendar for Iceland {
    fn name(&self) -> &'static str {
        "Iceland"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::ICELAND
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XICE
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // New Year's Day
            || (d == 1 && m == Month::January)
            // Maundy Thursday
            || (dd == em - 4)
            // Good Friday
            || (dd == em - 3)
            // Easter Monday
            || (dd == em)
            // First Day of Summer (first Thursday after 18th of April)
            || (w == Weekday::Thursday && (19..=25).contains(&d) && m == Month::April)
            // Labor Day
            || (d == 1 && m == Month::May)
            // Ascension Day
            || (dd == em + 38)
            // Whit Monday
            || (dd == em + 49)
            // Icelandic Republic Day
            || (d == 17 && m == Month::June)
            // Commerce Day (first Monday of August)
            || (d <= 7 && w == Weekday::Monday && m == Month::August)
            // Christmas Eve
            || (d == 24 && m == Month::December)
            // Christmas
            || (d == 25 && m == Month::December)
            // Boxing Day
            || (d == 26 && m == Month::December)
            // New Year's Eve
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
mod test_iceland {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Iceland;
        assert_eq!(calendar.name(), "Iceland");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Iceland;
        let sat = datetime!(2024-01-13 12:00:00 UTC);
        let sun = datetime!(2024-01-14 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Iceland;
        let new_years_day = datetime!(2024-01-01 12:00:00 UTC);
        let maudy_thursday = datetime!(2024-03-28 12:00:00 UTC);
        let first_day_of_summer = datetime!(2024-04-25 12:00:00 UTC);
        let labour_day = datetime!(2024-05-01 12:00:00 UTC);
        let ascension_day = datetime!(2024-05-09 12:00:00 UTC);
        let independence_day = datetime!(2024-06-17 12:00:00 UTC);
        let commerce_day = datetime!(2024-08-05 12:00:00 UTC);
        let christmas = datetime!(2024-12-25 12:00:00 UTC);
        let new_years_eve = datetime!(2024-12-31 12:00:00 UTC);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(maudy_thursday));
        assert!(!calendar.is_business_day(first_day_of_summer));
        assert!(!calendar.is_business_day(labour_day));
        assert!(!calendar.is_business_day(ascension_day));
        assert!(!calendar.is_business_day(independence_day));
        assert!(!calendar.is_business_day(commerce_day));
        assert!(!calendar.is_business_day(christmas));
        assert!(!calendar.is_business_day(new_years_eve));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = Iceland;
        let regular_day1 = datetime!(2024-01-17 12:00:00 UTC);
        let regular_day2 = datetime!(2024-07-08 12:00:00 UTC);
        let regular_day3 = datetime!(2024-11-18 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
