// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::utilities::contains_leap_year;
use crate::time::DayCountConvention;
use time::Date;

impl DayCountConvention {
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
}

#[cfg(test)]
mod TESTS_actual_constant {
    use crate::assert_approx_equal;
    use crate::time::DayCountConvention;
    use crate::RUSTQUANT_EPSILON;
    use time::macros::date;

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
