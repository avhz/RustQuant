// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::calendar::Calendar;
use crate::utilities::unpack_date;
use time::{Date, Month};
use RustQuant_iso::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Netherlands national holiday calendar.
pub struct NetherlandsCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for NetherlandsCalendar {
    fn new() -> Self {
        Self
    }
    
    fn name(&self) -> &'static str {
        "Netherlands"
    }

    fn country_code(&self) -> ISO_3166 {
        NETHERLANDS
    }

    fn market_identifier_code(&self) -> ISO_10383 {
        XAMS
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

   if (
            // New Year's Day
            d == 1 && m == Month::January
            
            // Good Friday
            || yd == em - 3
            
            // Easter Sunday
            || yd == em - 1
            
            // Easter Monday
            || yd == em
            
            // King's Day (April 27, or April 26 if 27th is a Sunday)
            || (m == Month::April && (
                (d == 27 && wd.number_from_monday() != 7) ||  // 27th April, unless it's a Sunday
                (d == 26 && wd.number_from_monday() == 6)     // 26th April, but only if it's a Saturday (i.e., 27th is Sunday)
            ))
            
            // Liberation Day (every year, but only a day off every 5 years from 2020)
            || (d == 5 && m == Month::May && (y % 5 == 0 && y >= 2020))
            
            // Ascension Day
            || yd == em + 38
            
            // Whit Sunday
            || yd == em + 48
            
            // Whit Monday
            || yd == em + 49
            
            // Christmas Day
            || d == 25 && m == Month::December
          
            // Boxing Day
            || d == 26 && m == Month::December
        ) {
            return true;
        }
        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_netherlands_holidays_2024() {
        let calendar = NetherlandsCalendar;

        // 2024 Holidays
        assert!(calendar.is_holiday(date!(2024-01-01)), "New Year's Day 2024");
        assert!(calendar.is_holiday(date!(2024-03-29)), "Good Friday 2024");
        assert!(calendar.is_holiday(date!(2024-03-31)), "Easter Sunday 2024");
        assert!(calendar.is_holiday(date!(2024-04-01)), "Easter Monday 2024");
        assert!(calendar.is_holiday(date!(2024-04-27)), "King's Day 2024");
        assert!(!calendar.is_holiday(date!(2024-05-05)), "Liberation Day 2024 (not a day off)");
        assert!(calendar.is_holiday(date!(2024-05-09)), "Ascension Day 2024");
        assert!(calendar.is_holiday(date!(2024-05-19)), "Whit Sunday 2024");
        assert!(calendar.is_holiday(date!(2024-05-20)), "Whit Monday 2024");
        assert!(calendar.is_holiday(date!(2024-12-25)), "Christmas Day 2024");
        assert!(calendar.is_holiday(date!(2024-12-26)), "Boxing Day 2024");

        // non-holiday dates in 2024
        assert!(!calendar.is_holiday(date!(2024-01-02)), "January 2, 2024");
        assert!(!calendar.is_holiday(date!(2024-04-28)), "April 28, 2024");
        assert!(!calendar.is_holiday(date!(2024-12-24)), "December 24, 2024");
    }

    #[test]
    fn test_netherlands_holidays_2025() {
        let calendar = NetherlandsCalendar;

        // 2025 Holidays
        assert!(calendar.is_holiday(date!(2025-01-01)), "New Year's Day 2025");
        assert!(calendar.is_holiday(date!(2025-04-18)), "Good Friday 2025");
        assert!(calendar.is_holiday(date!(2025-04-20)), "Easter Sunday 2025");
        assert!(calendar.is_holiday(date!(2025-04-21)), "Easter Monday 2025");
        assert!(calendar.is_holiday(date!(2025-04-26)), "King's Day 2025");
        assert!(calendar.is_holiday(date!(2025-05-05)), "Liberation Day 2025 (day off in 2025)");
        assert!(calendar.is_holiday(date!(2025-05-29)), "Ascension Day 2025");
        assert!(calendar.is_holiday(date!(2025-06-08)), "Whit Sunday 2025");
        assert!(calendar.is_holiday(date!(2025-06-09)), "Whit Monday 2025");
        assert!(calendar.is_holiday(date!(2025-12-25)), "Christmas Day 2025");
        assert!(calendar.is_holiday(date!(2025-12-26)), "Boxing Day 2025");

        // non-holiday dates in 2025
        assert!(!calendar.is_holiday(date!(2025-01-02)), "January 2, 2025");
        assert!(!calendar.is_holiday(date!(2025-04-27)), "April 27, 2025");
        assert!(!calendar.is_holiday(date!(2025-12-24)), "December 24, 2025");
    }

    #[test]
    fn test_liberation_day() {
        let calendar = NetherlandsCalendar;

        assert!(!calendar.is_holiday(date!(2023-05-05)), "Liberation Day 2023 (not a day off)");
        assert!(!calendar.is_holiday(date!(2024-05-05)), "Liberation Day 2024 (not a day off)");
        assert!(calendar.is_holiday(date!(2025-05-05)), "Liberation Day 2025 (day off)");
        assert!(!calendar.is_holiday(date!(2026-05-05)), "Liberation Day 2026 (not a day off)");
        assert!(!calendar.is_holiday(date!(2029-05-05)), "Liberation Day 2029 (not a day off)");
        assert!(calendar.is_holiday(date!(2030-05-05)), "Liberation Day 2030 (day off)");
    }

    #[test]
    fn test_kings_day() {
        let calendar = NetherlandsCalendar;

        // King's Day, including Sunday
        assert!(calendar.is_holiday(date!(2024-04-27)), "King's Day 2024 (Saturday)");
        assert!(calendar.is_holiday(date!(2025-04-26)), "King's Day 2025 (Saturday, 27th is Sunday)");
        assert!(calendar.is_holiday(date!(2026-04-27)), "King's Day 2026 (Monday)");
        assert!(calendar.is_holiday(date!(2027-04-27)), "King's Day 2027 (Tuesday)");
    }
}
