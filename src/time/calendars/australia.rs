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

/// Australiann settlement calendar.
/// See: <https://www.australia.gov.au/public-holidays>
pub struct Australia;

impl Calendar for Australia {
    fn name(&self) -> &'static str {
        "Australia"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
                // New Year's Day (possibly moved to Monday)
                || ((d == 1 || ((d == 2 || d == 3) && w == Weekday::Monday)) && m == Month::January)
                // Australia Day, January 26th (possibly moved to Monday)
                || ((d == 26 || ((d == 27 || d == 28) && w == Weekday::Monday)) && m == Month::January)
                // Good Friday
                || (dd == em - 3)
                // Easter Monday
                || (dd == em)
                // ANZAC Day, April 25th
                || (d == 25 && m == Month::April)
                // Queen's Birthday, second Monday in June
                || ((d > 7 && d <= 14) && w == Weekday::Monday && m == Month::June)
                // Bank Holiday, first Monday in August
                || (d <= 7 && w == Weekday::Monday && m == Month::August)
                // Labour Day, first Monday in October
                || (d <= 7 && w == Weekday::Monday && m == Month::October)
                // Christmas, December 25th (possibly moved to Monday or Tuesday)
                || ((d == 25 || (d == 27 && (w == Weekday::Monday || w == Weekday::Tuesday))) && m == Month::December)
                // Boxing Day, December 26th (possibly moved to Monday or Tuesday)
                || ((d == 26 || (d == 28 && (w == Weekday::Monday || w == Weekday::Tuesday))) && m == Month::December)
                // National Day of Mourning for Her Majesty, September 22 (only 2022)
                || (d == 22 && m == Month::September && y == 2022)
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
mod test_australia {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Australia;
        assert_eq!(calendar.name(), "Australia");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Australia;
        let sat = datetime!(2023-08-26 12:00:00 UTC);
        let sun = datetime!(2023-08-27 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Australia;
        let new_years_day = datetime!(2023 - 01 - 01 12:00:00 UTC);
        let australia_day = datetime!(2023 - 01 - 26 12:00:00 UTC);
        let anzac_day = datetime!(2023 - 04 - 25 12:00:00 UTC);
        let christmas = datetime!(2023 - 12 - 25 12:00:00 UTC);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(australia_day));
        assert!(!calendar.is_business_day(anzac_day));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = Australia;
        let regular_day1 = datetime!(2023-03-01 12:00:00 UTC);
        let regular_day2 = datetime!(2023-07-12 12:00:00 UTC);
        let regular_day3 = datetime!(2023-11-17 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
