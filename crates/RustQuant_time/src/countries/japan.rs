use crate::utilities::unpack_date;
use time::{Date, Month, Weekday};

pub(crate) fn is_holiday_impl_japan(date: Date) -> bool {
    let (y, m, d, wd, _, _) = unpack_date(date, false);

    // Fixed-date holidays (modern scope)
    let fixed =
        (m == Month::January && d == 1) // New Year's Day
        || (m == Month::February && d == 11) // National Foundation Day
        || (m == Month::February && d == 23 && y >= 2020) // Emperor's Birthday (Reiwa)
        || (m == Month::April && d == 29) // Showa Day
        || (m == Month::May && d == 3) // Constitution Memorial Day
        || (m == Month::May && d == 4) // Greenery Day
        || (m == Month::May && d == 5) // Children's Day
        || (m == Month::August && d == 11 && y >= 2016) // Mountain Day
        || (m == Month::November && d == 3) // Culture Day
        || (m == Month::November && d == 23); // Labor Thanksgiving Day

    // Happy Monday system holidays
    let happy_monday =
        // Coming of Age Day: 2nd Monday of January
        (m == Month::January && wd == Weekday::Monday && d >= 8 && d <= 14)
        // Marine Day: 3rd Monday of July
        || (m == Month::July && wd == Weekday::Monday && d >= 15 && d <= 21)
        // Respect for the Aged Day: 3rd Monday of September
        || (m == Month::September && wd == Weekday::Monday && d >= 15 && d <= 21)
        // Sports Day: 2nd Monday of October
        || (m == Month::October && wd == Weekday::Monday && d >= 8 && d <= 14);

    // Equinox holidays (standard approximation)
    let vernal =
        m == Month::March && d as i32 == vernal_equinox_day(y as i32);
    let autumnal =
        m == Month::September && d as i32 == autumnal_equinox_day(y as i32);

    // Substitute holidays: if holiday falls on Sunday, following Monday is holiday.
    let substitute_monday = wd == Weekday::Monday && is_recognized_holiday(date.previous_day().unwrap());

    fixed || happy_monday || vernal || autumnal || substitute_monday
}

fn is_recognized_holiday(date: Date) -> bool {
    let (y, m, d, wd, _, _) = unpack_date(date, false);

    let fixed =
        (m == Month::January && d == 1)
        || (m == Month::February && d == 11)
        || (m == Month::February && d == 23 && y >= 2020)
        || (m == Month::April && d == 29)
        || (m == Month::May && d == 3)
        || (m == Month::May && d == 4)
        || (m == Month::May && d == 5)
        || (m == Month::August && d == 11 && y >= 2016)
        || (m == Month::November && d == 3)
        || (m == Month::November && d == 23);

    let happy_monday =
        (m == Month::January && wd == Weekday::Monday && d >= 8 && d <= 14)
        || (m == Month::July && wd == Weekday::Monday && d >= 15 && d <= 21)
        || (m == Month::September && wd == Weekday::Monday && d >= 15 && d <= 21)
        || (m == Month::October && wd == Weekday::Monday && d >= 8 && d <= 14);

    let vernal = m == Month::March && d as i32 == vernal_equinox_day(y as i32);
    let autumnal = m == Month::September && d as i32 == autumnal_equinox_day(y as i32);

    fixed || happy_monday || vernal || autumnal
}

fn vernal_equinox_day(year: i32) -> i32 {
    (20.8431 + 0.242194 * (year - 1980) as f64 - ((year - 1980) / 4) as f64).floor() as i32
}

fn autumnal_equinox_day(year: i32) -> i32 {
    (23.2488 + 0.242194 * (year - 1980) as f64 - ((year - 1980) / 4) as f64).floor() as i32
}

#[cfg(test)]
mod tests {
    use crate::{Calendar, Market};
    use time::macros::date;

    const CALENDAR: Calendar = Calendar::new(Market::Japan);

    #[test]
    fn test_japan_holidays_2025() {
        assert!(CALENDAR.is_holiday(date!(2025 - 01 - 01))); // New Year
        assert!(CALENDAR.is_holiday(date!(2025 - 01 - 13))); // Coming of Age Day
        assert!(CALENDAR.is_holiday(date!(2025 - 02 - 11))); // National Foundation Day
        assert!(CALENDAR.is_holiday(date!(2025 - 03 - 20))); // Vernal Equinox
        assert!(CALENDAR.is_holiday(date!(2025 - 04 - 29))); // Showa Day
        assert!(CALENDAR.is_holiday(date!(2025 - 05 - 05))); // Children's Day
        assert!(CALENDAR.is_holiday(date!(2025 - 07 - 21))); // Marine Day
        assert!(CALENDAR.is_holiday(date!(2025 - 09 - 15))); // Respect for the Aged Day
        assert!(CALENDAR.is_holiday(date!(2025 - 09 - 23))); // Autumnal Equinox
        assert!(CALENDAR.is_holiday(date!(2025 - 10 - 13))); // Sports Day
        assert!(CALENDAR.is_holiday(date!(2025 - 11 - 03))); // Culture Day
        assert!(CALENDAR.is_holiday(date!(2025 - 11 - 24))); // Labor Thanksgiving substitute
    }

    #[test]
    fn test_regular_business_days_2025() {
        assert!(!CALENDAR.is_holiday(date!(2025 - 01 - 02)));
        assert!(!CALENDAR.is_holiday(date!(2025 - 06 - 10)));
        assert!(!CALENDAR.is_holiday(date!(2025 - 12 - 01)));
    }
}
