// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::{is_weekend, Calendar};
use time::{Month, OffsetDateTime, Weekday};

/// United Kingdom settlement calendar.
pub struct UnitedKingdom;

impl Calendar for UnitedKingdom {
    fn name(&self) -> &'static str {
        "United Kingdom"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let w = date.weekday();
        let d = date.day();
        let m = date.month();
        let y = date.year();
        let dd = date.ordinal(); // Day of the year

        let em = crate::time::easter_monday(y as usize, false);

        if is_weekend(date)
            // New Year's Day (possibly moved to Monday)
            || ((d == 1 || ((d == 2 || d == 3) && w == Weekday::Monday)) && m == Month::January)
            // Good Friday
            || (dd == em - 3)
            // Easter Monday
            || (dd == em)
            // Bank Holidays
            || is_bank_holiday(d, w, m, y)
            // Christmas (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (w == Weekday::Monday || w == Weekday::Tuesday))) && m == Month::December)
            // Boxing Day (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (w == Weekday::Monday || w == Weekday::Tuesday))) && m == Month::December)
            // December 31st, 1999 only
            || (d == 31 && m == Month::December && y == 1999)
        {
            return false;
        }

        true
    }
}

fn is_bank_holiday(d: u8, w: Weekday, m: Month, y: i32) -> bool {
    // first Monday of May (Early May Bank Holiday)
    // moved to May 8th in 1995 and 2020 for V.E. day
    (d <= 7 && w == Weekday::Monday && m == Month::May && y != 1995 && y != 2020)
        || (d == 8 && m == Month::May && (y == 1995 || y == 2020))
        // last Monday of May (Spring Bank Holiday)
        // moved to in 2002, 2012 and 2022 for the Golden, Diamond and Platinum
        // Jubilee with an additional holiday
        || (d >= 25 && w == Weekday::Monday && m == Month::May && y != 2002 && y != 2012 && y != 2022)
        || ((d == 3 || d == 4) && m == Month::June && y == 2002)
        || ((d == 4 || d == 5) && m == Month::June && y == 2012)
        || ((d == 2 || d == 3) && m == Month::June && y == 2022)
        // last Monday of August (Summer Bank Holiday)
        || (d >= 25 && w == Weekday::Monday && m == Month::August)
        // April 29th, 2011 only (Royal Wedding Bank Holiday)
        || (d == 29 && m == Month::April && y == 2011)
        // September 19th, 2022 only (The Queen's Funeral Bank Holiday)
        || (d == 19 && m == Month::September && y == 2022)
        // May 8th, 2023 (King Charles III Coronation Bank Holiday)
        || (d == 8 && m == Month::May && y == 2023)
}
