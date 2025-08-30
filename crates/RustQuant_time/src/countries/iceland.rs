// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::unpack_date;
use time::{Date, Month, Weekday};

pub(crate) fn is_holiday_impl_iceland(date: Date) -> bool {
    let (_y, m, d, wd, yd, em) = unpack_date(date, false);

    if (
        // New Year's Day
        (d == 1 && m == Month::January)
            // Maundy Thursday
            || (yd == em - 4)
            // Good Friday
            || (yd == em - 3)
            // Easter Monday
            || (yd == em)
            // First Day of Summer (first Thursday after 18th of April)
            || (wd == Weekday::Thursday && (19..=25).contains(&d) && m == Month::April)
            // Labor Day
            || (d == 1 && m == Month::May)
            // Ascension Day
            || (yd == em + 38)
            // Whit Monday
            || (yd == em + 49)
            // Icelandic Republic Day
            || (d == 17 && m == Month::June)
            // Commerce Day (first Monday of August)
            || (d <= 7 && wd == Weekday::Monday && m == Month::August)
            // Christmas Eve
            || (d == 24 && m == Month::December)
            // Christmas
            || (d == 25 && m == Month::December)
            // Boxing Day
            || (d == 26 && m == Month::December)
            // New Year's Eve
            || (d == 31 && m == Month::December)
    ) {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_iceland {
    use crate::{Calendar, Market};
    use time::macros::date;

    const CALENDAR: Calendar = Calendar::new(Market::Iceland);

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let sat = date!(2024 - 01 - 13);
        let sun = date!(2024 - 01 - 14);
        assert!(!CALENDAR.is_business_day(sat));
        assert!(!CALENDAR.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let new_years_day = date!(2024 - 01 - 01);
        let maudy_thursday = date!(2024 - 03 - 28);
        let first_day_of_summer = date!(2024 - 04 - 25);
        let labour_day = date!(2024 - 05 - 01);
        let ascension_day = date!(2024 - 05 - 09);
        let independence_day = date!(2024 - 06 - 17);
        let commerce_day = date!(2024 - 08 - 05);
        let christmas = date!(2024 - 12 - 25);
        let new_years_eve = date!(2024 - 12 - 31);

        assert!(!CALENDAR.is_business_day(new_years_day));
        assert!(!CALENDAR.is_business_day(maudy_thursday));
        assert!(!CALENDAR.is_business_day(first_day_of_summer));
        assert!(!CALENDAR.is_business_day(labour_day));
        assert!(!CALENDAR.is_business_day(ascension_day));
        assert!(!CALENDAR.is_business_day(independence_day));
        assert!(!CALENDAR.is_business_day(commerce_day));
        assert!(!CALENDAR.is_business_day(christmas));
        assert!(!CALENDAR.is_business_day(new_years_eve));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let regular_day1 = date!(2024 - 01 - 17);
        let regular_day2 = date!(2024 - 07 - 08);
        let regular_day3 = date!(2024 - 11 - 18);

        assert!(CALENDAR.is_business_day(regular_day1));
        assert!(CALENDAR.is_business_day(regular_day2));
        assert!(CALENDAR.is_business_day(regular_day3));
    }
}
