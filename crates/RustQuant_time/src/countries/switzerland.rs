use crate::utilities::unpack_date;
use time::{Date, Month};

pub(crate) fn is_holiday_impl_switzerland(date: Date) -> bool {
    let (_, m, d, _, yd, em) = unpack_date(date, false);

    is_new_year(m, d)
        || is_berchtholdstag(m, d)
        || is_good_friday(yd, em)
        || is_easter_monday(yd, em)
        || is_labour_day(m, d)
        || is_ascension_day(yd, em)
        || is_whit_monday(yd, em)
        || is_national_holiday(m, d)
        || is_christmas_eve(m, d)
        || is_christmas_day(m, d)
        || is_st_stephens_day(m, d)
        || is_new_year_eve(m, d)
}

#[inline]
fn is_new_year(month: Month, day: u8) -> bool {
    month == Month::January && day == 1
}

#[inline]
fn is_berchtholdstag(month: Month, day: u8) -> bool {
    month == Month::January && day == 2
}

#[inline]
fn is_good_friday(yd: u16, em: u16) -> bool {
    yd == em - 3
}

#[inline]
fn is_easter_monday(yd: u16, em: u16) -> bool {
    yd == em
}

#[inline]
fn is_labour_day(month: Month, day: u8) -> bool {
    month == Month::May && day == 1
}

#[inline]
fn is_ascension_day(yd: u16, em: u16) -> bool {
    yd == em + 38
}

#[inline]
fn is_whit_monday(yd: u16, em: u16) -> bool {
    yd == em + 49
}

#[inline]
fn is_national_holiday(month: Month, day: u8) -> bool {
    month == Month::August && day == 1
}

#[inline]
fn is_christmas_eve(month: Month, day: u8) -> bool {
    month == Month::December && day == 24
}

#[inline]
fn is_christmas_day(month: Month, day: u8) -> bool {
    month == Month::December && day == 25
}

#[inline]
fn is_st_stephens_day(month: Month, day: u8) -> bool {
    month == Month::December && day == 26
}

#[inline]
fn is_new_year_eve(month: Month, day: u8) -> bool {
    month == Month::December && day == 31
}

#[cfg(test)]
mod tests_switzerland {
    use crate::{Calendar, Market};

    const CALENDAR: Calendar = Calendar::new(Market::Switzerland);

    #[test]
    fn new_years_day_2025() {
        let date = time::macros::date!(2025 - 01 - 01);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn whit_monday_2025() {
        let date = time::macros::date!(2025 - 06 - 09);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn regular_day_2025() {
        let date = time::macros::date!(2025 - 03 - 17);
        assert!(CALENDAR.is_business_day(date));
    }

    #[test]
    fn national_holiday_2025() {
        let date = time::macros::date!(2025 - 08 - 01);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn good_friday_2025() {
        let date = time::macros::date!(2025 - 04 - 18);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn berchtholdstag_2025() {
        let date = time::macros::date!(2025 - 01 - 02);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn ascension_2025() {
        let date = time::macros::date!(2025 - 05 - 29);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn christmas_2025() {
        let date = time::macros::date!(2025 - 12 - 25);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn st_stephens_day_2025() {
        let date = time::macros::date!(2025 - 12 - 26);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn new_years_eve_2025() {
        let date = time::macros::date!(2025 - 12 - 31);
        assert!(!CALENDAR.is_business_day(date));
    }

    #[test]
    fn christmas_eve_2025() {
        let date = time::macros::date!(2025 - 12 - 24);
        assert!(!CALENDAR.is_business_day(date));
    }
}
