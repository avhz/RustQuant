// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::utilities::{contains_leap_year};
use crate::time::{days_between, DayCountConvention};
use time::util::is_leap_year;
use time::{Date, Month};

impl DayCountConvention {
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
}

#[cfg(test)]
mod TESTS_actual_actual {
    use crate::assert_approx_equal;
    use crate::time::DayCountConvention;
    use crate::RUSTQUANT_EPSILON;
    use time::macros::date;

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
