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

/// Austrian settlement calendar.
pub struct Austria;

impl Calendar for Austria {
    fn name(&self) -> &'static str {
        "Austria"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::ISO_3166 {
            alpha_2: "AT",
            alpha_3: "AUT",
            numeric: "040",
        }
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
                // New Year's Day
                || (d == 1 && m == Month::January)
                // Epiphany
                || (d == 6 && m == Month::January)
                // Easter Monday
                || (dd == em)
                // Ascension Thursday
                || (dd == em + 38)
                // Whit Monday
                || (dd == em + 49)
                // Corpus Christi
                || (dd == em + 59)
                // Labour Day
                || (d == 1 && m == Month::May)
                // Assumption
                || (d == 15 && m == Month::August)
                // National Holiday since 1967
                || (d == 26 && m == Month::October && y >= 1967)
                // National Holiday 1919-1934
                || (d == 12 && m == Month::November && (1919..=1934).contains(&y))
                // All Saints' Day
                || (d == 1 && m == Month::November)
                // Immaculate Conception
                || (d == 8 && m == Month::December)
                // Christmas
                || (d == 25 && m == Month::December)
                // St. Stephen
                || (d == 26 && m == Month::December)
        {
            return false;
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS for Austria
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_austria {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Austria;
        assert_eq!(calendar.name(), "Austria");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Austria;
        let sat = datetime!(2023-08-26 12:00:00 UTC);
        let sun = datetime!(2023-08-27 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Austria;
        let new_years_day = datetime!(2023-01-01 12:00:00 UTC);
        let epiphany = datetime!(2023-01-06 12:00:00 UTC);
        let labour_day = datetime!(2023-05-01 12:00:00 UTC);
        let national_holiday = datetime!(2023-10-26 12:00:00 UTC);
        let christmas = datetime!(2023-12-25 12:00:00 UTC);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(epiphany));
        assert!(!calendar.is_business_day(labour_day));
        assert!(!calendar.is_business_day(national_holiday));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = Austria;
        let regular_day1 = datetime!(2023-02-01 12:00:00 UTC);
        let regular_day2 = datetime!(2023-07-12 12:00:00 UTC);
        let regular_day3 = datetime!(2023-11-17 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
