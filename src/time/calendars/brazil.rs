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

/// Brazil calendar.
pub struct Brazil;

impl Calendar for Brazil {
    fn name(&self) -> &'static str {
        "Brazil"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // New Year's Day
            || (d == 1 && m == Month::January)
            // Tiradentes Day
            || (d == 21 && m == Month::April)
            // Labor Day
            || (d == 1 && m == Month::May)
            // Independence Day
            || (d == 7 && m == Month::September)
            // Nossa Sra. Aparecida Day
            || (d == 12 && m == Month::October)
            // All Souls Day
            || (d == 2 && m ==Month:: November)
            // Republic Day
            || (d == 15 && m == Month::November)
            // Christmas
            || (d == 25 && m == Month::December)
            // Passion of Christ
            || (dd == em-3)
            // Carnival
            || (dd == em-49 || dd == em-48)
            // Corpus Christi
            || (dd == em+59)
        {
            return false;
        }

        return true;
    }
}
