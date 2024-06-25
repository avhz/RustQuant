// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines a custom calendar type.

use std::collections::BTreeMap;

use crate::time::{CalendarMetadata, DateRollingConvention, DayCountConvention, Holiday};
use time::Date;

/// Custom calendar type.
pub struct CustomCalendar {
    /// Start date of the calendar.
    pub start: Date,
    /// End date of the calendar.
    pub end: Date,
    /// Metadata of the calendar.
    pub metadata: CalendarMetadata,

    /// Holidays in the calendar.
    pub holidays: BTreeMap<Date, Holiday>,

    /// Day count convention.
    pub day_count_convention: DayCountConvention,
    /// Date rolling convention.
    pub date_rolling_convention: DateRollingConvention,
}

impl CustomCalendar {
    // / Create a new custom calendar.
    // pub fn new(start: Date, end: Date, metadata: CalendarMetadata) -> Self {
    //     Self {
    //         start,
    //         end,
    //         metadata,
    //     }
    // }

    /// Add a vector of holidays to the calendar.
    pub fn add_holidays(&mut self, holidays: Vec<Holiday>) {
        for holiday in holidays {
            self.holidays.insert(holiday.date, holiday);
        }
    }
}
