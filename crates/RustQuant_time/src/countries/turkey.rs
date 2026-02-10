// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::unpack_date;
use time::{Date, Month};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Turkey calendar.
pub(crate) fn is_holiday_impl_turkey(date: Date) -> bool {
    let (_y, m, d, _wd, _yd, _em) = unpack_date(date, false);

    // Borsa Istanbul (BIST) Holiday Calendar
    // Source: Official Public Holidays in Turkey

    // Fixed Public Holidays (Gregorian Calendar)
    if (d == 1 && m == Month::January)                  // New Year's Day
        || (d == 23 && m == Month::April)               // National Sovereignty and Children's Day
        || (d == 1 && m == Month::May)                  // Labor and Solidarity Day
        || (d == 19 && m == Month::May)                 // Commemoration of Atatürk, Youth and Sports Day
        || (d == 15 && m == Month::July)                // Democracy and National Unity Day
        || (d == 30 && m == Month::August)              // Victory Day
        || (d == 29 && m == Month::October)
    // Republic Day
    {
        return true;
    }

    false
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::{Calendar, Market};
    use time::macros::date;

    #[test]
    fn test_turkey_holidays() {
        let calendar = Calendar::new(Market::Turkey);

        // Republic Day
        assert!(calendar.is_holiday(date!(2024 - 10 - 29)));

        // Victory Day
        assert!(calendar.is_holiday(date!(2024 - 08 - 30)));

        // Regular business day
        assert!(calendar.is_business_day(date!(2024 - 01 - 02)));

        // Weekend check (Example: Jan 6, 2024 was Saturday)
        assert!(calendar.is_weekend(date!(2024 - 01 - 06)));
    }
}
