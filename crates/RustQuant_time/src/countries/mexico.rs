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

pub(crate) fn is_holiday_impl_mexico(date: Date) -> bool {
    let (y, m, d, wd, _, _) = unpack_date(date, false);

    if
    // New Year's Day
    (d == 1 && m == Month::January)
			// Constitution day (first Monday in February)
			|| (d <= 7 && m == Month::February && wd == Weekday::Monday)
			// Benito Juárez Birthday (third Monday in March)
			|| (m == Month::March && (15..=21).contains(&d) && wd == Weekday::Monday)
			// Labour Day
			|| (d == 1 && m == Month::May)
			// Independence day
			|| (d == 16 && m == Month::September)
			// Revolution Day (third Monday in November)
			|| ((15..=21).contains(&d) && wd == Weekday::Monday && m == Month::November)
			// President transition every 6 years (2018, 2024, 2030, ....)
			|| (d == 1 && m == Month::October && (y - 2018) % 6 == 0)
			// Christmas Eve
			|| (d == 25 && m == Month::December)
    {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_mexico_calendar {
    use crate::{Calendar, Market};
    use time::macros::date;

    const CALENDAR: Calendar = Calendar::new(Market::Mexico);

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
        let new_years_day = date!(2024 - 01 - 01);
        let constitution_day = date!(2024 - 02 - 05); // First Monday of February
        let benito_juarez_day = date!(2024 - 03 - 18); // Third Monday of March
        let labour_day = date!(2024 - 05 - 01);
        let independence_day = date!(2024 - 09 - 16);
        let revolution_day = date!(2024 - 11 - 18); // Third Monday in November
        let presidential_transition_day = date!(2030 - 10 - 01); // This might need adjustment
        let christmas = date!(2023 - 12 - 25);

        assert!(!CALENDAR.is_business_day(new_years_day));
        assert!(!CALENDAR.is_business_day(constitution_day));
        assert!(!CALENDAR.is_business_day(benito_juarez_day));
        assert!(!CALENDAR.is_business_day(labour_day));
        assert!(!CALENDAR.is_business_day(independence_day));
        assert!(!CALENDAR.is_business_day(revolution_day));
        assert!(!CALENDAR.is_business_day(presidential_transition_day));
        assert!(!CALENDAR.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let regular_day1 = date!(2023 - 03 - 15);
        let regular_day2 = date!(2023 - 08 - 15);
        let regular_day3 = date!(2023 - 10 - 25);

        assert!(CALENDAR.is_business_day(regular_day1));
        assert!(CALENDAR.is_business_day(regular_day2));
        assert!(CALENDAR.is_business_day(regular_day3));
    }
}
