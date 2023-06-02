// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for computing day counts.

use super::{constants::Frequency, conventions::DayCountConvention};
use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

/// Compute the day count factor between two dates.
pub fn day_count_factor(
    start: OffsetDateTime,
    end: OffsetDateTime,
    convention: DayCountConvention,
    frequency: Option<Frequency>,
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
                + 30.0 * (months - 1.0) as f64)
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_daycount {
    use super::*;
    use crate::assert_approx_equal;
    use time::macros::datetime;

    #[test]
    fn test_daycount() {
        let start = datetime!(2022-01-01 0:00 UTC);
        let end = datetime!(2023-06-02 0:00 UTC);

        assert_approx_equal!(
            day_count_factor(start, end, DayCountConvention::Actual365, None),
            1.416438,
            1e-6
        );
        assert_approx_equal!(
            day_count_factor(start, end, DayCountConvention::Actual360, None),
            1.436111,
            1e-6
        );
        assert_approx_equal!(
            day_count_factor(start, end, DayCountConvention::Actual364, None),
            1.420329,
            1e-6
        );
        assert_approx_equal!(
            day_count_factor(start, end, DayCountConvention::Thirty360, None),
            1.419444,
            1e-6
        );
    }
}
