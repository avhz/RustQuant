// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::utilities::is_last_day_of_february;
use crate::time::DayCountConvention;
use time::Date;

impl DayCountConvention {
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
    use crate::assert_approx_equal;
    use crate::time::DayCountConvention;
    use crate::RUSTQUANT_EPSILON;
    use time::macros::date;

    #[test]
    fn thirty_e_365() {
        let start_date = date!(2011 - 06 - 17);
        let end_date = date!(2012 - 12 - 30);

        let dcf = DayCountConvention::day_count_factor_thirty_e_365(start_date, end_date);

        assert_approx_equal!(dcf, 1.515_068_493, RUSTQUANT_EPSILON);
    }
}
