// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::{is_weekend, Calendar};
use time::{Month, OffsetDateTime, Weekday};

/// Australiann settlement calendar.
/// See: https://www.australia.gov.au/public-holidays
pub struct Australia;

impl Calendar for Australia {
    fn name(&self) -> &'static str {
        "Australia"
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
                // Australia Day, January 26th (possibly moved to Monday)
                || ((d == 26 || ((d == 27 || d == 28) && w == Weekday::Monday)) && m == Month::January)
                // Good Friday
                || (dd == em - 3)
                // Easter Monday
                || (dd == em)
                // ANZAC Day, April 25th
                || (d == 25 && m == Month::April)
                // Queen's Birthday, second Monday in June
                || ((d > 7 && d <= 14) && w == Weekday::Monday && m == Month::June)
                // Bank Holiday, first Monday in August
                || (d <= 7 && w == Weekday::Monday && m == Month::August)
                // Labour Day, first Monday in October
                || (d <= 7 && w == Weekday::Monday && m == Month::October)
                // Christmas, December 25th (possibly moved to Monday or Tuesday)
                || ((d == 25 || (d == 27 && (w == Weekday::Monday || w == Weekday::Tuesday))) && m == Month::December)
                // Boxing Day, December 26th (possibly moved to Monday or Tuesday)
                || ((d == 26 || (d == 28 && (w == Weekday::Monday || w == Weekday::Tuesday))) && m == Month::December)
                // National Day of Mourning for Her Majesty, September 22 (only 2022)
                || (d == 22 && m == Month::September && y == 2022)
        {
            return false;
        }

        true
    }
}
