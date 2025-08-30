// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{contains_leap_year, days_between, is_last_day_of_february, leap_year_count};
use std::fmt;
use time::{util::is_leap_year, Date, Month};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Day count conventions.
///
/// From Wikipedia (<https://en.wikipedia.org/wiki/Day_count_convention>):
/// """
/// In finance, a day count convention determines how interest accrues
/// over time for a variety of investments, including bonds, notes,
/// loans, mortgages, medium-term notes, swaps, and forward rate agreements (FRAs).
/// This determines the number of days between two coupon payments,
/// thus calculating the amount transferred on payment dates and also the
/// accrued interest for dates between payments. The day count is also
/// used to quantify periods of time when discounting a cash-flow to its
/// present value. When a security such as a bond is sold between interest
/// payment dates, the seller is eligible to some fraction of the coupon amount.
/// """
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum DayCountConvention {
    /// The '1/1' day count, which always returns a day count of 1.
    One_One,

    /// The 'Act/360' day count, which divides the actual number of days by 360.
    Actual_360,

    /// The 'Act/364' day count, which divides the actual number of days by 364.
    Actual_364,

    /// The 'Act/364' day count, which divides the actual number of days by 366.
    Actual_366,

    /// The 'Act/365.25' day count, which divides the actual number of days by 365.25.
    Actual_365_25,

    /// The 'Act/365 Actual' day count, which divides the actual number of days
    /// by 366 if a leap day is contained, or by 365 if not.
    Actual_365_Actual,

    /// The 'Act/365F' day count, which divides the actual number of days by 365 (fixed).
    Actual_365_Fixed,

    /// The 'Act/365L' day count, which divides the actual number of days by 365 or 366.
    Actual_365_Leap,

    /// The 'Act/Act AFB' day count, which divides the actual number of days by 366
    /// if a leap day is contained, or by 365 if not, with additional rules for periods over one year.
    Actual_Actual_AFB,

    /// The 'Act/Act ICMA' day count, which divides the actual number of days by
    /// the actual number of days in the coupon period multiplied by the frequency.
    Actual_Actual_ICMA,

    /// The 'Act/Act ISDA' day count, which divides the actual number of days in a
    /// leap year by 366 and the actual number of days in a standard year by 365.
    Actual_Actual_ISDA,

    /// The 'NL/360' day count, which divides the actual number of days omitting leap days by 360.
    No_Leap_360,

    /// The 'NL/365' day count, which divides the actual number of days omitting leap days by 365.
    No_Leap_365,

    /// The '30/360 ISDA' day count, which treats input day-of-month 31 specially.
    Thirty_360_ISDA,

    /// The '30E/360' day count, which treats input day-of-month 31 specially.
    Thirty_E_360,

    /// The '30E/360 ISDA' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_E_360_ISDA,

    /// The '30E/365' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_E_365,

    /// The '30E+/360' day count, which treats input day-of-month 31 specially.
    Thirty_E_Plus_360,

    /// The '30U/360' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_U_360,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS/METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl DayCountConvention {
    /// Entry point for day count factor calculation.
    #[rustfmt::skip]
    pub fn day_count_factor(&self, start_date: Date, end_date: Date) -> f64 {
        match self {
            Self::One_One               => Self::day_count_factor_one_one(start_date, end_date),
            Self::Actual_360            => Self::day_count_factor_actual_360(start_date, end_date),
            Self::Actual_364            => Self::day_count_factor_actual_364(start_date, end_date),
            Self::Actual_366            => Self::day_count_factor_actual_366(start_date, end_date),
            Self::Actual_365_25         => Self::day_count_factor_actual_365_25(start_date, end_date),
            Self::Actual_365_Actual     => Self::day_count_factor_actual_365_actual(start_date, end_date),
            Self::Actual_365_Fixed      => Self::day_count_factor_actual_365_fixed(start_date, end_date),
            Self::Actual_365_Leap       => Self::day_count_factor_actual_365_leap(start_date, end_date),
            Self::Actual_Actual_AFB     => Self::day_count_factor_actual_actual_afb(start_date, end_date),
            Self::Actual_Actual_ICMA    => Self::day_count_factor_actual_actual_icma(start_date, end_date),
            Self::Actual_Actual_ISDA    => Self::day_count_factor_actual_actual_isda(start_date, end_date),
            Self::No_Leap_360           => Self::day_count_factor_nl_360(start_date, end_date),
            Self::No_Leap_365           => Self::day_count_factor_nl_365(start_date, end_date),
            Self::Thirty_360_ISDA       => Self::day_count_factor_thirty_360_isda(start_date, end_date),
            Self::Thirty_E_360          => Self::day_count_factor_thirty_e_360(start_date, end_date),
            Self::Thirty_E_360_ISDA     => Self::day_count_factor_thirty_e_360_isda(start_date, end_date),
            Self::Thirty_E_365          => Self::day_count_factor_thirty_e_365(start_date, end_date),
            Self::Thirty_E_Plus_360     => Self::day_count_factor_thirty_e_plus_360(start_date, end_date),
            Self::Thirty_U_360          => Self::day_count_factor_thirty_u_360(start_date, end_date),
        }
    }

    // Actual/Actual AFB day count factor calculation.
    pub(crate) fn day_count_factor_actual_actual_afb(start_date: Date, end_date: Date) -> f64 {
        let (y1, _y2) = (start_date.year(), end_date.year());
        let (_m1, m2) = (start_date.month(), end_date.month());
        let (_d1, d2) = (start_date.day(), end_date.day());

        let stub_date = if Date::from_calendar_date(y1, m2, d2).unwrap() < start_date {
            Date::from_calendar_date(y1 + 1, m2, d2).unwrap()
        } else {
            Date::from_calendar_date(y1, m2, d2).unwrap()
        };

        let initial_stub_days = (stub_date - start_date).whole_days() as f64;
        // let final_stub_days = (end_date - stub_date).whole_days() as f64;
        let final_stub_years = (end_date.year() - stub_date.year()) as f64;
        let initial_stub_contains_leap = contains_leap_year(start_date, stub_date);

        match initial_stub_contains_leap {
            true => final_stub_years + initial_stub_days / 366.0,
            false => (end_date - start_date).whole_days() as f64 / 365.0,
        }
    }

    // Actual/Actual ICMA day count factor calculation.
    pub(crate) fn day_count_factor_actual_actual_icma(_start_date: Date, _end_date: Date) -> f64 {
        todo!()
    }

    // Actual/Actual ISDA day count factor calculation.
    pub(crate) fn day_count_factor_actual_actual_isda(start_date: Date, end_date: Date) -> f64 {
        if start_date == end_date {
            return 0.0;
        }

        let (y1, y2) = (start_date.year(), end_date.year());

        let (dib1, dib2) = (
            if is_leap_year(y1) { 366.0 } else { 365.0 },
            if is_leap_year(y2) { 366.0 } else { 365.0 },
        );

        let mut sum: f64 = (y2 - y1 - 1) as f64;

        sum += days_between(
            start_date,
            Date::from_calendar_date(y1 + 1, Month::January, 1).unwrap(),
        ) as f64
            / dib1;

        sum += days_between(
            Date::from_calendar_date(y2, Month::January, 1).unwrap(),
            end_date,
        ) as f64
            / dib2;

        sum
    }

    // NL/360 day count factor calculation.
    pub(crate) fn day_count_factor_nl_360(start_date: Date, end_date: Date) -> f64 {
        let day_count = (end_date - start_date).whole_days() as f64;
        let leap_years = leap_year_count(start_date, end_date) as f64;

        (day_count - leap_years) / 360.0
    }

    // NL/365 day count factor calculation.
    pub(crate) fn day_count_factor_nl_365(start_date: Date, end_date: Date) -> f64 {
        let day_count = (end_date - start_date).whole_days() as f64;
        let leap_years = leap_year_count(start_date, end_date) as f64;

        (day_count - leap_years) / 365.0
    }

    // One/One day count factor calculation.
    pub(crate) fn day_count_factor_one_one(_start_date: Date, _end_date: Date) -> f64 {
        1.0
    }

    // Actual/360 day count factor calculation.
    pub(crate) fn day_count_factor_actual_360(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 360.0
    }

    // Actual/364 day count factor calculation.
    pub(crate) fn day_count_factor_actual_364(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 364.0
    }

    // Actual/365.25 day count factor calculation.
    pub(crate) fn day_count_factor_actual_365_25(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 365.25
    }

    // Actual/365 Actual day count factor calculation.
    pub(crate) fn day_count_factor_actual_365_actual(start_date: Date, end_date: Date) -> f64 {
        match contains_leap_year(start_date, end_date) {
            true => (end_date - start_date).whole_days() as f64 / 366.0,
            false => (end_date - start_date).whole_days() as f64 / 365.0,
        }
    }

    // Actual/365F day count factor calculation.
    pub(crate) fn day_count_factor_actual_365_fixed(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 365.0
    }

    // Actual/365L day count factor calculation.
    pub(crate) fn day_count_factor_actual_365_leap(start_date: Date, end_date: Date) -> f64 {
        match contains_leap_year(start_date, end_date) {
            true => (end_date - start_date).whole_days() as f64 / 366.0,
            false => (end_date - start_date).whole_days() as f64 / 365.0,
        }
    }

    // Actual/366 day count factor calculation.
    pub(crate) fn day_count_factor_actual_366(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 366.0
    }

    // 30/360 ISDA day count factor calculation.
    pub(crate) fn day_count_factor_thirty_360_isda(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 {
            d1 = 30;
        }

        if d1 == 30 && d2 == 31 {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E/360 day count factor calculation.
    pub(crate) fn day_count_factor_thirty_e_360(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 {
            d1 = 30;
        }

        if d2 == 31 {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E/360 ISDA day count factor calculation.
    pub(crate) fn day_count_factor_thirty_e_360_isda(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 || is_last_day_of_february(start_date) {
            d1 = 30;
        }

        if d2 == 31 || is_last_day_of_february(end_date) {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E+/360 day count factor calculation.
    pub(crate) fn day_count_factor_thirty_e_plus_360(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (mut y2, mut m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 {
            d1 = 30;
        }

        if d2 == 31 {
            (y2, m2, d2) = Self::thirty_360_unpack_date(end_date.next_day().unwrap());
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30U/360 day count factor calculation.
    pub(crate) fn day_count_factor_thirty_u_360(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 || is_last_day_of_february(start_date) {
            d1 = 30;
        }

        if d2 == 31 && d1 == 30 || is_last_day_of_february(end_date) {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E/365 day count factor calculation.
    pub(crate) fn day_count_factor_thirty_e_365(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 || is_last_day_of_february(start_date) {
            d1 = 30;
        }

        if d2 == 31 || is_last_day_of_february(end_date) {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 365.0
    }

    /// Function to comput the 30/360 numerator.
    pub(crate) fn thirty_360_numerator(
        y1: i32,
        y2: i32,
        m1: i32,
        m2: i32,
        d1: i32,
        d2: i32,
    ) -> f64 {
        (360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)) as f64
    }

    /// Function to unpack the date components for 30/360 calculation.
    pub(crate) fn thirty_360_unpack_date(date: Date) -> (i32, i32, i32) {
        (date.year(), date.month() as i32, date.day() as i32)
    }
}

// UNIT TESTS

#[cfg(test)]
mod TESTS_thirty_360 {
    use crate::DayCountConvention;
    use time::macros::date;
    use RustQuant_utils::assert_approx_equal;
    use RustQuant_utils::RUSTQUANT_EPSILON;

    #[test]
    fn thirty_e_365() {
        let start_date = date!(2011 - 06 - 17);
        let end_date = date!(2012 - 12 - 30);

        let dcf = DayCountConvention::day_count_factor_thirty_e_365(start_date, end_date);

        assert_approx_equal!(dcf, 1.515_068_493, RUSTQUANT_EPSILON);
    }
}

#[cfg(test)]
mod TESTS_actual_constant {
    use crate::DayCountConvention;
    use time::macros::date;
    use RustQuant_utils::assert_approx_equal;
    use RustQuant_utils::RUSTQUANT_EPSILON;

    #[test]
    fn actual_365_25() {
        // Test cases from QuantLib.
        let test_dates: Vec<time::Date> = vec![
            date!(2002 - 02 - 1),
            date!(2002 - 02 - 4),
            date!(2003 - 05 - 16),
            date!(2003 - 12 - 17),
            date!(2004 - 12 - 17),
            date!(2005 - 12 - 19),
            date!(2006 - 01 - 02),
            date!(2006 - 03 - 13),
            date!(2006 - 05 - 15),
            date!(2006 - 03 - 17),
            date!(2006 - 05 - 15),
            date!(2006 - 07 - 26),
            date!(2007 - 06 - 28),
            date!(2009 - 09 - 16),
            date!(2016 - 07 - 26),
        ];

        let expected: Vec<f64> = vec![
            0.0082135523613963,
            1.27583846680356,
            0.588637919233402,
            1.00205338809035,
            1.00479123887748,
            0.0383299110198494,
            0.191649555099247,
            0.172484599589322,
            -0.161533196440794,
            0.161533196440794,
            0.197125256673511,
            0.922655715263518,
            2.22039698836413,
            6.85831622176591,
        ];

        for i in 1..test_dates.len() {
            let dcf = DayCountConvention::day_count_factor_actual_365_25(
                test_dates[i - 1],
                test_dates[i],
            );

            assert_approx_equal!(dcf, expected[i - 1], RUSTQUANT_EPSILON);
        }
    }

    #[test]
    fn actual_366() {
        // Test cases from QuantLib.
        let test_dates: Vec<time::Date> = vec![
            date!(2002 - 02 - 1),
            date!(2002 - 02 - 4),
            date!(2003 - 05 - 16),
            date!(2003 - 12 - 17),
            date!(2004 - 12 - 17),
            date!(2005 - 12 - 19),
            date!(2006 - 01 - 02),
            date!(2006 - 03 - 13),
            date!(2006 - 05 - 15),
            date!(2006 - 03 - 17),
            date!(2006 - 05 - 15),
            date!(2006 - 07 - 26),
            date!(2007 - 06 - 28),
            date!(2009 - 09 - 16),
            date!(2016 - 07 - 26),
        ];

        let expected: Vec<f64> = vec![
            0.00819672131147541,
            1.27322404371585,
            0.587431693989071,
            1.0000000000000,
            1.00273224043716,
            0.0382513661202186,
            0.191256830601093,
            0.172131147540984,
            -0.16120218579235,
            0.16120218579235,
            0.19672131147541,
            0.920765027322404,
            2.21584699453552,
            6.84426229508197,
        ];

        for i in 1..test_dates.len() {
            let dcf =
                DayCountConvention::day_count_factor_actual_366(test_dates[i - 1], test_dates[i]);

            assert_approx_equal!(dcf, expected[i - 1], RUSTQUANT_EPSILON);
        }
    }
}

#[cfg(test)]
mod TESTS_actual_actual {
    use crate::DayCountConvention;
    use time::macros::date;
    use RustQuant_utils::assert_approx_equal;
    use RustQuant_utils::RUSTQUANT_EPSILON;

    const DATE_1: time::Date = date!(2003 - 11 - 1);
    const DATE_2: time::Date = date!(2004 - 5 - 1);

    #[test]
    fn actual_actual_isda() {
        // Test cases from QuantLib.
        assert_approx_equal!(
            DayCountConvention::day_count_factor_actual_actual_isda(DATE_1, DATE_2),
            0.497724380567,
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn actual_actual_icma() {
        // Test cases from QuantLib.
        assert_approx_equal!(
            DayCountConvention::day_count_factor_actual_actual_isda(DATE_1, DATE_2),
            0.497724380567,
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn actual_actual_afb() {
        // Test cases from QuantLib.
        assert_approx_equal!(
            DayCountConvention::day_count_factor_actual_actual_afb(DATE_1, DATE_2),
            0.497267759563,
            RUSTQUANT_EPSILON
        );
    }
}
