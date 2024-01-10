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

/// Hungary calendar.
pub struct Hungary;

impl Calendar for Hungary {
    fn name(&self) -> &'static str {
        "Hungary"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::HUNGARY
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XBUD
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // New Year's Day
            || (d == 1 && m == Month::January)
            // 1848 Revolution Memorial Day
            || (d == 15 && m == Month::March)
            // Good Friday
            || (dd == em - 3)
            // Easter Monday
            || (dd == em)
            // Labor Day / May Day
            || (d == 1 && m == Month::May)
            // Whit Monday
            || (dd == em + 49)
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
mod test_hungary {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Hungary;
        assert_eq!(calendar.name(), "Hungary");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Hungary;
        let sat = datetime!(2024-01-13 12:00:00 UTC);
        let sun = datetime!(2024-01-14 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Hungary;
        let new_years_day = datetime!(2024-01-01 12:00:00 UTC);
        let revolution_1848_day = datetime!(2024-03-15 12:00:00 UTC);
        let labour_day = datetime!(2024-05-01 12:00:00 UTC);
        let national_holiday = datetime!(2024-08-20 12:00:00 UTC);
        let revolution_1956_day = datetime!(2024-10-23 12:00:00 UTC);
        let christmas = datetime!(2023-12-25 12:00:00 UTC);
        let second_christmas_day = datetime!(2023-12-26 12:00:00 UTC);

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
        let calendar = Hungary;
        let regular_day1 = datetime!(2024-03-07 12:00:00 UTC);
        let regular_day2 = datetime!(2024-07-02 12:00:00 UTC);
        let regular_day3 = datetime!(2024-12-11 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
