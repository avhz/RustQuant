// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
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

/// Trait for converting a month to a usize.
/// Needed so that we can perform arithmetic on months.
trait MonthNumeric {
    fn as_usize(&self) -> usize;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS/METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl MonthNumeric for time::Month {
    fn as_usize(&self) -> usize {
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
    pub fn day_count_factor(
        start: OffsetDateTime,
        end: OffsetDateTime,
        convention: &DayCountConvention,
    ) -> f64 {
        // THIS FUNCTION NEEDS WORK.

        let start_month = start.month().as_usize();
        let end_month = end.month().as_usize();

        let days = (end - start).whole_days() as f64;
        let months = (end_month - start_month) as f64;
        let years = (end.year() - start.year()) as f64;

        match convention {
            DayCountConvention::Actual365 => days / 365.0,
            DayCountConvention::Actual364 => days / 364.0,
            DayCountConvention::Actual360 => days / 360.0,
            DayCountConvention::Thirty360 => {
                ((30 - start.day()).max(0) as f64
                    + (end.day()).min(30) as f64
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

    #[test]
    fn test_daycount_factor() {
        let mut dc = DayCounter::new(
            datetime!(2022-01-01 0:00 UTC),
            datetime!(2023-06-02 0:00 UTC),
            DayCountConvention::Actual365,
        );

        assert_approx_equal!(dc.day_count_factor, 1.416438, 1e-6);

        dc.change_convention(DayCountConvention::Actual360);

        assert_approx_equal!(dc.day_count_factor, 1.436111, 1e-6);

        dc.change_convention(DayCountConvention::Actual364);

        assert_approx_equal!(dc.day_count_factor, 1.420329, 1e-6);

        dc.change_convention(DayCountConvention::Thirty360);

        assert_approx_equal!(dc.day_count_factor, 1.419444, 1e-6);
    }

    #[test]
    fn test_day_counts() {
        let start = datetime!(2022-01-01 0:00 UTC);
        let end = datetime!(2023-06-02 0:00 UTC);

        let dc = DayCounter::new(start, end, DayCountConvention::Actual365);

        assert_eq!(dc.day_count_business, 370);
        assert_eq!(dc.day_count_calendar, 517);
    }
}
