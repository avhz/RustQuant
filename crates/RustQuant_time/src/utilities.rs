// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines general calendar and holiday related functions.

use crate::{calendar::Calendar, constants::EASTER_MONDAYS};
use time::{
    util::{days_in_year, days_in_year_month, is_leap_year},
    Date, Duration, Error, Month, Weekday,
};

/// Unpacks a `Date` into a tuple in the following form:
///
/// ```ignore
/// (
///     y,      // Year
///     m,      // Month (January - December)
///     d,      // Day of month (1 - 31)
///     wd,     // Weekday (Monday-Sunday)
///     yd,     // Day of year (1 - 365)
///     em,     // Easter Monday
/// )
/// ```
pub fn unpack_date(date: Date, is_orthodox: bool) -> (i32, Month, u8, Weekday, u16, u16) {
    let y = date.year();
    let m = date.month();
    let d = date.day();

    let wd = date.weekday();
    let yd = date.ordinal();

    let em = easter_monday(y as usize, is_orthodox);

    (y, m, d, wd, yd, em)
}

/// Returns the Easter Monday for the given year.
fn easter_monday(year: usize, is_orthodox: bool) -> u16 {
    EASTER_MONDAYS[usize::from(is_orthodox)][year - 1901]
}

/// Compute a default year fraction between two dates.
///
/// The default year fraction is computed as the
/// actual number of days between two dates divided by 365.25.
/// This attempts to account for leap years.
pub fn year_fraction(start: Date, end: Date) -> f64 {
    days_between(start, end) as f64 / 365.25
}

/// Checks if date is a weekend.
pub fn is_weekend(date: Date) -> bool {
    let w = date.weekday();

    w == time::Weekday::Saturday || w == time::Weekday::Sunday
}

/// Check if the date is a weekday.
pub fn is_weekday(date: Date) -> bool {
    !is_weekend(date)
}

/// Function to get a list of the years in a range of `Dates`.
pub fn get_years_in_range(start: Date, end: Date) -> Vec<i32> {
    (start.year()..=end.year()).collect()
}

/// Retrieve today's date (local time).
///
/// # Warning:
/// Depending on where the code is physically run, the date may be different.
/// For example, two machines calling this function at precisely the same time in
/// Australia and the United States will get different dates.
pub fn today() -> Date {
    time::OffsetDateTime::now_utc().date()
}

/// Function to get the number of days for each year in a range of `Dates`.
///
/// ```
/// use time::{Date, Month};
/// use RustQuant::time::utilities::get_days_in_years_in_range;
///
/// let start = Date::from_calendar_date(2023, Month::July, 1).unwrap();
/// let end = Date::from_calendar_date(2025, Month::January, 1).unwrap();
///
/// let days_in_years = get_days_in_years_in_range(start, end);
///
/// assert_eq!(days_in_years, vec![365, 366, 365]);
/// ```
pub fn get_days_in_years_in_range(start: Date, end: Date) -> Vec<u16> {
    get_years_in_range(start, end)
        .iter()
        .map(|&y| days_in_year(y))
        .collect()
}

/// Function to check if a range of years contains a leap year.
pub fn contains_leap_year(start: Date, end: Date) -> bool {
    get_years_in_range(start, end)
        .iter()
        .any(|&y| is_leap_year(y))
}

/// Function to compute the number of days between two dates.
pub fn days_between(start: Date, end: Date) -> i64 {
    ((end - start).abs()).whole_days()
}

/// Function to get the number of leap years in a range of `Dates`.
///
/// ```
/// use time::{Date, Month};
/// use RustQuant::time::utilities::leap_year_count;
///
/// let start = Date::from_calendar_date(2023, Month::July, 1).unwrap();
/// let end = Date::from_calendar_date(2025, Month::January, 1).unwrap();
///
/// let number_of_leap_years = leap_year_count(start, end);
///
/// assert_eq!(number_of_leap_years, 1);
/// ```
pub fn leap_year_count(start: Date, end: Date) -> i64 {
    get_years_in_range(start, end)
        .iter()
        .filter(|&y| is_leap_year(*y))
        .collect::<Vec<&i32>>()
        .len() as i64
}

/// Function to check if Date is first day of the month.
pub fn is_first_day_of_month(date: Date) -> bool {
    date.day() == 1
}

/// Function to check if Date is last day of the month.
pub fn is_last_day_of_month(date: Date) -> bool {
    date.day() == days_in_year_month(date.year(), date.month())
}

/// Function to check if date is the last day of February.
pub fn is_last_day_of_february(date: Date) -> bool {
    let last_day_of_feb_non_leap =
        date.month() == Month::February && date.day() == 28 && !is_leap_year(date.year());
    let last_day_of_feb_leap =
        date.month() == Month::February && date.day() == 29 && is_leap_year(date.year());

    last_day_of_feb_non_leap || last_day_of_feb_leap
}

/// Function to get the next business day for a given date and calendar.
pub fn next_business_day<C: Calendar>(date: Date, calendar: &C) -> Date {
    let mut new_date = date;

    while !calendar.is_business_day(new_date) {
        new_date = new_date.next_day().unwrap();
    }

    new_date
}

/// Function to get the previous business day for a given date and calendar.
pub fn previous_business_day<C: Calendar>(date: Date, calendar: &C) -> Date {
    let mut new_date = date;

    while !calendar.is_business_day(new_date) {
        new_date = new_date.previous_day().unwrap();
    }

    new_date
}

/// Function to generate a sequence of dates from a start date, end date.
pub fn date_sequence(start: Date, end: Date) -> Vec<Date> {
    let mut dates = Vec::with_capacity((end - start).whole_days() as usize);
    let mut current_date = start;

    while current_date <= end {
        dates.push(current_date);
        current_date += Duration::days(1);
    }

    dates
}

/// Function to get the first day of the month.
pub fn get_first_day_of_month(year: i32, month: Month) -> Result<Weekday, Error> {
    Ok(Date::from_calendar_date(year, month, 1)?.weekday())
}

/// Function to get the last day of the month.
pub fn get_last_day_of_month(year: i32, month: Month) -> Result<Weekday, Error> {
    let date = Date::from_calendar_date(year, month, 1)?;

    let last_day = date + Duration::days(days_in_year_month(year, month) as i64);

    Ok(last_day.weekday())
}

/// Function to get the date of the first Monday of the month.
pub fn get_first_monday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Monday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Monday)),
    }
}

/// Function to get the date of the first Tuesday of the month.
pub fn get_first_tuesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Tuesday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Tuesday)),
    }
}

/// Function to get the date of the first Wednesday of the month.
pub fn get_first_wednesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Wednesday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Wednesday)),
    }
}

/// Function to get the date of the first Thursday of the month.
pub fn get_first_thursday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Thursday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Thursday)),
    }
}

/// Function to get the date of the first Friday of the month.
pub fn get_first_friday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Friday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Friday)),
    }
}

/// Function to get the date of the first Saturday of the month.
pub fn get_first_saturday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Saturday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Saturday)),
    }
}

/// Function to get the date of the first Sunday of the month.
pub fn get_first_sunday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Sunday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Sunday)),
    }
}

/// Function to get the date of the last Monday of the month.
pub fn get_last_monday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Monday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Monday)),
    }
}

/// Function to get the date of the last Tuesday of the month.
pub fn get_last_tuesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Tuesday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Tuesday)),
    }
}

/// Function to get the date of the last Wednesday of the month.
pub fn get_last_wednesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Wednesday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Wednesday)),
    }
}

/// Function to get the date of the last Thursday of the month.
pub fn get_last_thursday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Thursday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Thursday)),
    }
}

/// Function to get the date of the last Friday of the month.
pub fn get_last_friday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Friday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Friday)),
    }
}

/// Function to get the date of the last Saturday of the month.
pub fn get_last_saturday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Saturday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Saturday)),
    }
}

/// Function to get the date of the last Sunday of the month.
pub fn get_last_sunday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Sunday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Sunday)),
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_utilities {
    use super::*;

    #[test]
    fn test_first_x_day_of_month() {
        let y = 2024;
        let m = Month::January;

        assert_eq!(
            get_first_monday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 1).unwrap()
        );
        assert_eq!(
            get_first_tuesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 2).unwrap()
        );
        assert_eq!(
            get_first_wednesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 3).unwrap()
        );
        assert_eq!(
            get_first_thursday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 4).unwrap()
        );
        assert_eq!(
            get_first_friday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 5).unwrap()
        );
        assert_eq!(
            get_first_saturday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 6).unwrap()
        );
        assert_eq!(
            get_first_sunday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 7).unwrap()
        );
    }

    #[test]
    fn test_last_x_day_of_month() {
        let y = 2024;
        let m = Month::January;

        assert_eq!(
            get_last_monday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 29).unwrap()
        );
        assert_eq!(
            get_last_tuesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 30).unwrap()
        );
        assert_eq!(
            get_last_wednesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 31).unwrap()
        );
        assert_eq!(
            get_last_thursday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 25).unwrap()
        );
        assert_eq!(
            get_last_friday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 26).unwrap()
        );
        assert_eq!(
            get_last_saturday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 27).unwrap()
        );
        assert_eq!(
            get_last_sunday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 28).unwrap()
        );
    }

    #[test]
    fn test_get_first_day_of_month() {
        assert_eq!(
            get_first_day_of_month(2024, Month::January).unwrap(),
            Weekday::Monday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::February).unwrap(),
            Weekday::Thursday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::March).unwrap(),
            Weekday::Friday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::April).unwrap(),
            Weekday::Monday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::May).unwrap(),
            Weekday::Wednesday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::June).unwrap(),
            Weekday::Saturday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::July).unwrap(),
            Weekday::Monday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::August).unwrap(),
            Weekday::Thursday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::September).unwrap(),
            Weekday::Sunday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::October).unwrap(),
            Weekday::Tuesday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::November).unwrap(),
            Weekday::Friday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::December).unwrap(),
            Weekday::Sunday
        );
    }

    #[test]
    fn test_get_first_monday_of_month() {
        assert_eq!(
            get_first_monday_of_month(2024, Month::January).unwrap(),
            Date::from_calendar_date(2024, Month::January, 1).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::February).unwrap(),
            Date::from_calendar_date(2024, Month::February, 5).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::March).unwrap(),
            Date::from_calendar_date(2024, Month::March, 4).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::April).unwrap(),
            Date::from_calendar_date(2024, Month::April, 1).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::May).unwrap(),
            Date::from_calendar_date(2024, Month::May, 6).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::June).unwrap(),
            Date::from_calendar_date(2024, Month::June, 3).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::July).unwrap(),
            Date::from_calendar_date(2024, Month::July, 1).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::August).unwrap(),
            Date::from_calendar_date(2024, Month::August, 5).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::September).unwrap(),
            Date::from_calendar_date(2024, Month::September, 2).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::October).unwrap(),
            Date::from_calendar_date(2024, Month::October, 7).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::November).unwrap(),
            Date::from_calendar_date(2024, Month::November, 4).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::December).unwrap(),
            Date::from_calendar_date(2024, Month::December, 2).unwrap()
        );
    }
}
