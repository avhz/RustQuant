// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for computing day count factors.

use super::conventions::DayCountConvention;
use time::{Duration, OffsetDateTime};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Day counter struct.
/// This struct is used to compute:
///     - Day count factor.
///     - Business day count.
///     - Calendar day count.
pub struct DayCounter {
    /// Day count factor (fraction of year between two dates).
    pub day_count_factor: f64,
    /// Business day count.
    pub day_count_business: i64,
    /// Calendar day count.
    pub day_count_calendar: i64,
    /// The start date of the day count.
    pub start: OffsetDateTime,
    /// The end date of the day count.
    pub end: OffsetDateTime,
    /// The day count convention.
    pub convention: DayCountConvention,
}

/// Trait for converting a month to a isize.
/// Needed so that we can perform arithmetic on months.
trait MonthNumeric {
    fn as_isize(&self) -> isize;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS/METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl MonthNumeric for time::Month {
    fn as_isize(&self) -> isize {
        match *self {
            time::Month::January => 1,
            time::Month::February => 2,
            time::Month::March => 3,
            time::Month::April => 4,
            time::Month::May => 5,
            time::Month::June => 6,
            time::Month::July => 7,
            time::Month::August => 8,
            time::Month::September => 9,
            time::Month::October => 10,
            time::Month::November => 11,
            time::Month::December => 12,
        }
    }
}

impl DayCounter {
    /// New day counter.
    #[must_use]
    pub fn new(start: OffsetDateTime, end: OffsetDateTime, convention: DayCountConvention) -> Self {
        let day_count_factor = Self::day_count_factor(start, end, &convention);
        let day_count_business = Self::day_count_business(start, end);
        let day_count_calendar = Self::day_count_calendar(start, end);

        DayCounter {
            day_count_factor,
            day_count_business,
            day_count_calendar,
            start,
            end,
            convention,
        }
    }

    /// Change the day count convention.
    pub fn change_convention(&mut self, convention: DayCountConvention) {
        self.convention = convention;
        self.day_count_factor = Self::day_count_factor(self.start, self.end, &self.convention);
    }

    /// Compute the day count factor between two dates.
    ///
    /// # Arguments
    ///
    /// * `start` - The start date (optional and defaults to today).
    /// * `end` - The end date.
    /// * `convention` - The day count convention.
    #[must_use]
    pub fn day_count_factor(
        start: OffsetDateTime,
        end: OffsetDateTime,
        convention: &DayCountConvention,
    ) -> f64 {
        // THIS FUNCTION NEEDS WORK.

        // let start = start.unwrap_or(OffsetDateTime::now_utc());

        let start_month = start.month().as_isize();
        let end_month = end.month().as_isize();

        let days = (end - start).whole_days() as f64;
        let months = (end_month - start_month) as f64;
        let years = f64::from(end.year() - start.year());

        match convention {
            DayCountConvention::Actual365 => days / 365.0,
            DayCountConvention::Actual364 => days / 364.0,
            DayCountConvention::Actual360 => days / 360.0,
            DayCountConvention::Thirty360 => {
                (f64::from((30 - start.day()).max(0))
                    + f64::from((end.day()).min(30))
                    + 360.0 * years
                    + 30.0 * (months - 1.0))
                    / 360.0
            } // DayCountConvention::Thirty360_BondBasis => {}
              // DayCountConvention::Thirty360_US => {}
              // DayCountConvention::ThirtyE360 => {}
              // DayCountConvention::ThirtyE360_ISDA => {}
              // DayCountConvention::ActualActual_ICMA => {}
              // DayCountConvention::ActualActual_ISDA => {}
              // DayCountConvention::Actual365L => {}
              // DayCountConvention::ActualActual_AFB => {}
              // DayCountConvention::OneOne => {}
        }
    }

    /// Compute the business day count between two dates.
    /// This is the number of days between two dates, excluding weekends.
    /// Obviously, for this to be really useful, we need to have a calendar
    /// that tells us which days are weekends, and also which are holidays.
    #[must_use]
    pub fn day_count_business(mut start: OffsetDateTime, end: OffsetDateTime) -> i64 {
        let mut count = 0;
        while start <= end {
            match start.weekday() {
                time::Weekday::Saturday | time::Weekday::Sunday => {}
                _ => count += 1,
            }
            start += Duration::days(1);
        }
        count
    }

    /// Compute the calendar day count between two dates.
    /// This is the number of days between two dates.
    #[must_use]
    pub fn day_count_calendar(start: OffsetDateTime, end: OffsetDateTime) -> i64 {
        (end - start).whole_days()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_daycount {
    use super::*;
    use crate::assert_approx_equal;
    use time::macros::datetime;

    use std::f64::EPSILON as EPS;

    #[test]
    fn test_daycount_factor() {
        let mut dc = DayCounter::new(
            datetime!(2022-01-01 0:00 UTC),
            datetime!(2023-06-02 0:00 UTC),
            DayCountConvention::Actual365,
        );

        assert_approx_equal!(dc.day_count_factor, 1.416_438_356_164_383_6, EPS);

        dc.change_convention(DayCountConvention::Actual360);

        assert_approx_equal!(dc.day_count_factor, 1.436_111_111_111_111_1, EPS);

        dc.change_convention(DayCountConvention::Actual364);

        assert_approx_equal!(dc.day_count_factor, 1.420_329_670_329_670_4, EPS);

        dc.change_convention(DayCountConvention::Thirty360);

        assert_approx_equal!(dc.day_count_factor, 1.419_444_444_444_444_5, EPS);
    }

    #[test]
    fn test_day_counts() {
        let start = datetime!(2022-01-01 0:00 UTC);
        let end = datetime!(2023-06-02 0:00 UTC);

        let dc = DayCounter::new(start, end, DayCountConvention::Actual365);

        assert_eq!(dc.day_count_business, 370);
        assert_eq!(dc.day_count_calendar, 517);
    }
    #[test]
    fn test_thirty360_convention_same_day_same_month_different_years() {
        let start_date = datetime!(2022-02-15 0:00 UTC);
        let end_date = datetime!(2023-02-15 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, 1.0, EPS);
    }

    #[test]
    fn test_thirty360_convention_same_day_different_month_same_year() {
        let start_date = datetime!(2023-05-15 0:00 UTC);
        let end_date = datetime!(2023-11-15 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, 0.5, EPS);
    }

    #[test]
    fn test_thirty360_convention_different_day_same_month_same_year() {
        let start_date = datetime!(2023-09-15 0:00 UTC);
        let end_date = datetime!(2023-09-30 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, 0.041_666_666_666_666_664, EPS);
    }

    #[test]
    fn test_thirty360_convention_31_day_same_month_same_year() {
        let start_date = datetime!(2023-10-15 0:00 UTC);
        let end_date = datetime!(2023-10-31 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, 0.041_666_666_666_666_664, EPS);
    }

    #[test]
    fn test_thirty360_convention_different_day_different_month_same_year() {
        let start_date = datetime!(2023-03-15 0:00 UTC);
        let end_date = datetime!(2023-08-31 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, 0.458_333_333_333_333_3, EPS);
    }

    #[test]
    fn test_thirty360_convention_end_day_less_than_start_day_same_month() {
        let start_date = datetime!(2023-07-30 0:00 UTC);
        let end_date = datetime!(2023-07-15 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, -0.041_666_666_666_666_664, EPS);
    }

    #[test]
    fn test_thirty360_convention_end_day_less_than_start_day_different_month() {
        let start_date = datetime!(2023-07-30 0:00 UTC);
        let end_date = datetime!(2023-12-15 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, 0.375, EPS);
    }

    #[test]
    fn test_thirty360_convention_end_month_less_than_start_month() {
        let start_date = datetime!(2023-06-30 0:00 UTC);
        let end_date = datetime!(2023-04-15 0:00 UTC);
        let result = DayCounter::new(start_date, end_date, DayCountConvention::Thirty360);
        assert_approx_equal!(result.day_count_factor, -0.208_333_333_333_333_34, EPS);
    }
}
