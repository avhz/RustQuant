// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines a `Calendar` type and its methods.

use crate::utilities::is_weekend;
use time::Date;
use RustQuant_iso::{ISO_10383, ISO_3166};

/// Calendar metadata struct.
pub struct CalendarMetadata {
    /// Name of the calendar, typically the country name, but could also be
    /// a region/subdivision or a special calendar, such as a financial calendar (e.g. NYSE).
    pub name: &'static str,

    /// ISO 3166-1 country code.
    pub country_code: ISO_3166,

    /// ISO 10383 market identifier code.
    pub market_identifier_code: ISO_10383,
}

/// Calendar trait.
pub trait Calendar {
    /// Name of the calendar, typically the country name, but could also be
    /// a region/subdivision or a special calendar, such as a financial calendar (e.g. NYSE).
    fn name(&self) -> &'static str;

    /// Check if the date is a holiday (but not a weekend).
    /// This is the primary method to implement for a calendar.
    fn is_holiday(&self, date: Date) -> bool;

    /// Returns the ISO 3166-1 country code.
    fn country_code(&self) -> ISO_3166;

    /// Returns the ISO 10383 market identifier code.
    fn market_identifier_code(&self) -> ISO_10383;

    /// Check if the date is a business day.
    /// A business day is a day that is not a holiday and not a weekend.
    fn is_business_day(&self, date: Date) -> bool {
        !is_weekend(date) && !self.is_holiday(date)
    }

    /// Function to list all holidays for a given range of `Date`s.
    fn all_holidays_between(&self, start_date: Date, end_date: Date) -> Vec<Date> {
        let mut holidays = Vec::new();

        let mut temp_date = start_date;

        while temp_date <= end_date {
            if self.is_holiday(temp_date) {
                holidays.push(temp_date);
            }

            temp_date = temp_date.next_day().unwrap();
        }

        holidays.sort();

        holidays
    }

    /// Function to list all business days for a given range of `Date`s.
    fn all_business_days_between(&self, start_date: Date, end_date: Date) -> Vec<Date> {
        let mut business_days = Vec::new();

        let mut temp_date = start_date;

        while temp_date <= end_date {
            if self.is_business_day(temp_date) {
                business_days.push(temp_date);
            }

            temp_date = temp_date.next_day().unwrap();
        }

        business_days.sort();

        business_days
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl CalendarMetadata {
    /// New calendar metadata.
    pub fn new(
        name: &'static str,
        country_code: ISO_3166,
        market_identifier_code: ISO_10383,
    ) -> Self {
        Self {
            name,
            country_code,
            market_identifier_code,
        }
    }

    /// Returns the name of the calendar.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the ISO 3166-1 country code.
    pub fn country_code(&self) -> ISO_3166 {
        self.country_code
    }

    /// Returns the ISO 10383 market identifier code.
    pub fn market_identifier_code(&self) -> ISO_10383 {
        self.market_identifier_code
    }
}
