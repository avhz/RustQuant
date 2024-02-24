// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::calendar::Calendar;
use crate::time::utilities::{next_business_day, previous_business_day};
use crate::time::DateRollingConvention;
use time::Date;

impl DateRollingConvention {
    /// Adjust (roll) the date according: Modified following convention.
    pub(crate) fn roll_date_modified_following<C: Calendar>(date: Date, calendar: &C) -> Date {
        let mut new_date = next_business_day(date, calendar);

        if new_date.month() != date.month() {
            new_date = previous_business_day(date, calendar);
        }

        new_date
    }
}
