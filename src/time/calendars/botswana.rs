// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use time::{Month, OffsetDateTime, Weekday};

/// Botswana calendar.
pub struct Botswana;

impl Calendar for Botswana {
    fn name(&self) -> &'static str {
        "Botswana"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
        // New Year's Day (possibly moved to Monday or Tuesday)
        || ((d == 1 || (d == 2 && w == Weekday::Monday) || (d == 3 && w == Weekday::Tuesday)) 
            && m == Month::January)
        // Good Friday
        || (dd == em - 3)
        // Easter Monday
        || (dd == em)
        // Labour Day, May 1st (possibly moved to Monday)
        || ((d == 1 || (d == 2 && w == Weekday::Monday))
            && m == Month::May)
        // Ascension
        || (dd == em + 38)
        // Sir Seretse Khama Day, July 1st (possibly moved to Monday)
        || ((d == 1 || (d == 2 && w == Weekday::Monday))
            && m == Month::July)
        // Presidents' Day (third Monday of July)
        || ((d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::July)
        // Independence Day, September 30th (possibly moved to Monday)
        || ((d == 30 && m == Month::September) || 
            (d == 1  && w == Weekday::Monday && m == Month::October))
        // Botswana Day, October 1st (possibly moved to Monday or Tuesday)
        || ((d == 1 || (d == 2 && w == Weekday::Monday) || (d == 3 && w == Weekday::Tuesday)) 
            && m == Month::October)
        // Christmas
        || (d == 25 && m == Month::December)
        // Boxing Day (possibly moved to Monday)
        || ((d == 26 || (d == 27 && w == Weekday::Monday))
            && m == Month::December)
        {
            return false;
        }

        return true;
    }
}
