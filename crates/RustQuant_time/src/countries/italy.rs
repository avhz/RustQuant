// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::unpack_date;
use time::{Date, Month};

pub(crate) fn is_holiday_impl_italy(date: Date) -> bool {
    let (_, m, d, _, yd, em) = unpack_date(date, false);

    // New Year's Day
    if (d == 1 && m == Month::January)
            // Epiphany
            || (d == 6 && m == Month::January)
            // Easter Monday
            || (yd == em)
            // Liberation Day
            || (d == 25 && m == Month::April)
            // Labour Day
            || (d == 1 && m == Month::May)
            // Republic Day
            || (d == 2 && m == Month::June)
            // Assumption of Mary
            || (d == 15 && m == Month::August)
            // All Saints' Day
            || (d == 1 && m == Month::November)
            // Immaculate Conception
            || (d == 8 && m == Month::December)
            // Christmas Day
            || (d == 25 && m == Month::December)
            // St. Stephen's Day
            || (d == 26 && m == Month::December)
    {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_italy {
    use crate::{Calendar, Market};
    use time::macros::date;

    const CALENDAR: Calendar = Calendar::new(Market::Italy);

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let sat = date!(2024 - 08 - 24);
        let sun = date!(2024 - 08 - 25);

        assert!(!CALENDAR.is_business_day(sat));
        assert!(!CALENDAR.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays in Italy.
    #[test]
    fn test_is_public_holiday() {
        let new_years_day = date!(2024 - 01 - 01);
        let epiphany = date!(2024 - 01 - 06);
        let liberation_day = date!(2024 - 04 - 25);
        let labour_day = date!(2024 - 05 - 01);
        let republic_day = date!(2024 - 06 - 02);
        let assumption = date!(2024 - 08 - 15);
        let all_saints_day = date!(2024 - 11 - 01);
        let immaculate_conception = date!(2024 - 12 - 08);
        let christmas = date!(2024 - 12 - 25);
        let st_stephen = date!(2024 - 12 - 26);

        assert!(!CALENDAR.is_business_day(new_years_day));
        assert!(!CALENDAR.is_business_day(epiphany));
        assert!(!CALENDAR.is_business_day(liberation_day));
        assert!(!CALENDAR.is_business_day(labour_day));
        assert!(!CALENDAR.is_business_day(republic_day));
        assert!(!CALENDAR.is_business_day(assumption));
        assert!(!CALENDAR.is_business_day(all_saints_day));
        assert!(!CALENDAR.is_business_day(immaculate_conception));
        assert!(!CALENDAR.is_business_day(christmas));
        assert!(!CALENDAR.is_business_day(st_stephen));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days in Italy.
    #[test]
    fn test_is_regular_business_day() {
        let regular_day1 = date!(2024 - 03 - 15);
        let regular_day2 = date!(2024 - 07 - 11);
        let regular_day3 = date!(2024 - 09 - 16);
        let day_before_new_year = date!(2025 - 12 - 31); // Not a holiday in Italy
        let day_after_epiphany = date!(2025 - 01 - 07); // Not a holiday in Italy

        assert!(CALENDAR.is_business_day(regular_day1));
        assert!(CALENDAR.is_business_day(regular_day2));
        assert!(CALENDAR.is_business_day(regular_day3));
        assert!(CALENDAR.is_business_day(day_before_new_year));
        assert!(CALENDAR.is_business_day(day_after_epiphany));
    }
}
