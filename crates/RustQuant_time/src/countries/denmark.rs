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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) fn is_holiday_impl_denmark(date: Date) -> bool {
    let (y, m, d, _, yd, em) = unpack_date(date, false);

    if (
        // Maundy Thursday
        (yd == em - 4)

            // Good Friday
            || (yd == em - 3)

            // Easter Monday
            || (yd == em)

            // General Prayer Day
            || (yd == em + 25 && y <= 2023)

            // Ascension
            || (yd == em + 38)

            // Day after Ascension
            || (yd == em + 39 && y >= 2009)

            // Whit Monday
            || (yd == em + 49)

            // New Year's Day
            || (d == 1 && m == Month::January)

            // Constitution Day, June 5th
            || (d == 5 && m == Month::June)

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
mod test_denmark {
    use crate::{Calendar, Market};
    use time::macros::date;

    const CALENDAR: Calendar = Calendar::new(Market::Denmark);

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let sat = date!(2023 - 08 - 26);
        let sun = date!(2023 - 08 - 27);

        assert!(!CALENDAR.is_business_day(sat));
        assert!(!CALENDAR.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let new_years_day = date!(2023 - 01 - 01);
        let maunday_thursday = date!(2023 - 04 - 06);
        let good_friday = date!(2023 - 04 - 07);
        let easter_monday = date!(2023 - 04 - 10);
        let general_prayer_day = date!(2023 - 05 - 05);
        let ascension = date!(2023 - 05 - 18);
        let day_after_ascension = date!(2023 - 05 - 19);
        let whit_monday = date!(2023 - 05 - 29);
        let constitution_day = date!(2023 - 06 - 05);
        let christmas_eve = date!(2023 - 12 - 24);
        let christmas = date!(2023 - 12 - 25);
        let boxing_day = date!(2023 - 12 - 26);
        let new_years_eve = date!(2023 - 12 - 31);

        assert!(!CALENDAR.is_business_day(new_years_day));
        assert!(!CALENDAR.is_business_day(maunday_thursday));
        assert!(!CALENDAR.is_business_day(good_friday));
        assert!(!CALENDAR.is_business_day(easter_monday));
        assert!(!CALENDAR.is_business_day(general_prayer_day));
        assert!(!CALENDAR.is_business_day(ascension));
        assert!(!CALENDAR.is_business_day(day_after_ascension));
        assert!(!CALENDAR.is_business_day(whit_monday));
        assert!(!CALENDAR.is_business_day(constitution_day));
        assert!(!CALENDAR.is_business_day(christmas_eve));
        assert!(!CALENDAR.is_business_day(christmas));
        assert!(!CALENDAR.is_business_day(boxing_day));
        assert!(!CALENDAR.is_business_day(new_years_eve));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let regular_day1 = date!(2023 - 03 - 15);
        let regular_day2 = date!(2023 - 07 - 11);
        let regular_day3 = date!(2023 - 09 - 15);
        let regular_day4 = date!(2008 - 05 - 02); // Day after ascension (before 2009)
        let regular_day5 = date!(2024 - 04 - 26); // General Prayer Day (after 2023)

        assert!(CALENDAR.is_business_day(regular_day1));
        assert!(CALENDAR.is_business_day(regular_day2));
        assert!(CALENDAR.is_business_day(regular_day3));
        assert!(CALENDAR.is_business_day(regular_day4));
        assert!(CALENDAR.is_business_day(regular_day5));
    }
}
