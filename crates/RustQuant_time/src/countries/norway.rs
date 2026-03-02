use crate::utilities::unpack_date;
use time::{Date, Month};

pub(crate) fn is_holiday_impl_norway(date: Date) -> bool {
    let (y, m, d, _, yd, em) = unpack_date(date, false);

    if
    // New Year's Day
    (d == 1 && m == Month::January)
        // Maundy Thursday
        || yd == em - 4
        // Good Friday
        || yd == em - 3
        // Easter Sunday
        || yd == em - 1
        // Easter Monday
        || yd == em
        // Labour Day
        || (d == 1 && m == Month::May)
        // Constitution Day
        || (d == 17 && m == Month::May)
        // Ascension Day
        || yd == em + 38
        // Whit Sunday
        || yd == em + 48
        // Whit Monday
        || yd == em + 49
        // Christmas Day
        || (d == 25 && m == Month::December)
        // Boxing Day
        || (d == 26 && m == Month::December)
        // One-off: National day of mourning after Utøya attacks (2011-08-22)
        || (y == 2011 && m == Month::August && d == 22)
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{Calendar, Market};
    use time::macros::date;

    const CALENDAR: Calendar = Calendar::new(Market::Norway);

    #[test]
    fn test_norway_holidays_2025() {
        assert!(CALENDAR.is_holiday(date!(2025 - 01 - 01))); // New Year
        assert!(CALENDAR.is_holiday(date!(2025 - 04 - 17))); // Maundy Thursday
        assert!(CALENDAR.is_holiday(date!(2025 - 04 - 18))); // Good Friday
        assert!(CALENDAR.is_holiday(date!(2025 - 04 - 20))); // Easter Sunday
        assert!(CALENDAR.is_holiday(date!(2025 - 04 - 21))); // Easter Monday
        assert!(CALENDAR.is_holiday(date!(2025 - 05 - 01))); // Labour Day
        assert!(CALENDAR.is_holiday(date!(2025 - 05 - 17))); // Constitution Day
        assert!(CALENDAR.is_holiday(date!(2025 - 05 - 29))); // Ascension Day
        assert!(CALENDAR.is_holiday(date!(2025 - 06 - 08))); // Whit Sunday
        assert!(CALENDAR.is_holiday(date!(2025 - 06 - 09))); // Whit Monday
        assert!(CALENDAR.is_holiday(date!(2025 - 12 - 25))); // Christmas
        assert!(CALENDAR.is_holiday(date!(2025 - 12 - 26))); // Boxing Day
    }

    #[test]
    fn test_norway_business_days_2025() {
        assert!(!CALENDAR.is_holiday(date!(2025 - 01 - 02)));
        assert!(!CALENDAR.is_holiday(date!(2025 - 03 - 17)));
        assert!(!CALENDAR.is_holiday(date!(2025 - 11 - 12)));
    }
}
