// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::utilities::leap_year_count;
use crate::time::DayCountConvention;
use time::Date;

impl DayCountConvention {
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
}
