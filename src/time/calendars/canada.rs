// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::{is_weekend, Calendar};
use time::{Month, OffsetDateTime, Weekday};

/// Canadian settlement calendar.
pub struct Canada;

impl Calendar for Canada {
    fn name(&self) -> &'static str {
        "Canada"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let w = date.weekday();
        let d = date.day();
        let m = date.month();
        let y = date.year();
        let dd = date.ordinal(); // Day of the year

        let em = crate::time::easter_monday(y as usize, false); // assuming you have a similar easter_monday function

        if is_weekend(date)
            || ((d == 1 || ((d == 2 || d == 3) && w == Weekday::Monday)) && m == Month::January)
            || ((d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::February && y >= 2008)
            || (dd == em - 3)
            || (d > 17 && d <= 24 && w == Weekday::Monday && m == Month::May)
            || ((d == 1 || ((d == 2 || d == 3) && w == Weekday::Monday)) && m == Month::July)
            || (d <= 7 && w == Weekday::Monday && m == Month::August)
            || (d <= 7 && w == Weekday::Monday && m == Month::September)
            || (((d == 30 && m == Month::September)
                || (d <= 2 && m == Month::October && w == Weekday::Monday))
                && y >= 2021)
            || (d > 7 && d <= 14 && w == Weekday::Monday && m == Month::October)
            || ((d == 11 || ((d == 12 || d == 13) && w == Weekday::Monday)) && m == Month::November)
            || ((d == 25 || (d == 27 && (w == Weekday::Monday || w == Weekday::Tuesday)))
                && m == Month::December)
            || ((d == 26 || (d == 28 && (w == Weekday::Monday || w == Weekday::Tuesday)))
                && m == Month::December)
        {
            return false;
        }

        true
    }
}
