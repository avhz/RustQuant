use crate::utilities::unpack_date;
use time::{Date, Month};

/// Turkey calendar.
pub fn is_holiday_impl_turkey(date: Date) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_turkey_holidays() {
        // Republic Day
        assert!(is_holiday_impl_turkey(date!(2024 - 10 - 29)));
        // Victory Day
        assert!(is_holiday_impl_turkey(date!(2024 - 08 - 30)));
        // Regular business day
        assert!(!is_holiday_impl_turkey(date!(2024 - 01 - 02)));
    }
}
