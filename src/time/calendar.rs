// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::{Month, OffsetDateTime, Weekday};

/// Calendar trait.
/// The calendars follow generic settlement rules, not the exchange holiday rules.
pub trait Calendar {
    /// Name of the calendar.
    fn name(&self) -> &'static str;

    /// Check if the date is a business day.
    fn is_business_day(&self, date: OffsetDateTime) -> bool;

    /// Returns the ISO 3166-1 country code.
    fn country_code(&self) -> crate::iso::ISO_3166;

    /// Returns the ISO 10383 market identifier code.
    fn market_identifier_code(&self) -> crate::iso::ISO_10383;

    /// Unpacks an OffsetDateTime into a tuple in the following form:
    ///
    /// ```ignore
    /// (
    ///     weekday,
    ///     day_of_month,
    ///     month,
    ///     year,
    ///     day_of_year
    /// )
    /// ```
    ///
    fn unpack_date(&self, date: OffsetDateTime) -> (Weekday, u8, Month, i32, u16) {
        let w = date.weekday(); // Weekday (Monday-Sunday)
        let d = date.day(); // Day of month (1-31)
        let m = date.month(); // Month (January-December)
        let y = date.year(); // Year
        let dd = date.ordinal(); // Day of year (1-365)

        (w, d, m, y, dd)
    }

    /// Returns the Easter Monday for the given year.
    fn easter_monday(year: usize, is_orthodox: bool) -> u16 {
        let index = if is_orthodox { 1 } else { 0 };

        super::EASTER_MONDAYS[index][year - 1901]
    }

    /// Checks if date is a weekend.
    fn is_weekend(date: OffsetDateTime) -> bool {
        let w = date.weekday();

        w == time::Weekday::Saturday || w == time::Weekday::Sunday
    }
}

/// Holiday type.
/// This simply returns the name of the holiday.
pub struct Holiday(pub &'static str);
