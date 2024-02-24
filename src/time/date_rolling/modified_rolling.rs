// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::calendar::Calendar;
use crate::time::DateRollingConvention;
use time::Date;

impl DateRollingConvention {
    /// Adjust (roll) the date according: Modified rolling convention.
    pub(crate) fn roll_date_modified_rolling<C: Calendar>(date: Date, calendar: &C) -> Date {
        let mut new_date = date;

        while !calendar.is_business_day(new_date) {
            new_date = new_date.next_day().unwrap();
        }

        new_date
    }
}
