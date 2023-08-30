// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::OffsetDateTime;

/// Calendar trait.
/// The calendars follow generic settlement rules, not the exchange holiday rules.
pub trait Calendar {
    /// Name of the calendar.
    fn name(&self) -> &'static str;

    /// Check if the date is a business day.
    fn is_business_day(&self, date: OffsetDateTime) -> bool;
}

/// Holiday type.
/// This simply returns the name of the holiday.
pub struct Holiday(pub &'static str);

/// Returns the Easter Monday for the given year.
pub fn easter_monday(year: usize, is_orthodox: bool) -> u16 {
    let index = if is_orthodox { 1 } else { 0 };

    super::EASTER_MONDAYS[index][year - 1901]
}

/// Checks if date is a weekend.
pub fn is_weekend(date: OffsetDateTime) -> bool {
    let w = date.weekday();

    w == time::Weekday::Saturday || w == time::Weekday::Sunday
}
