// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::calendar::Calendar;
use crate::utilities::unpack_date;
use icu;
use time::{Date, Weekday};
use RustQuant_iso::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// CONSTANTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const JEWISH_HOLIDAYS: [(u8, u8); 16] = [
    (12, 29), // Jewish new year (Rosh Hashana) I
    (1, 1),   // Jewish new year (Rosh Hashana) II
    (1, 2),   // Jewish new year (Rosh Hashana) II
    (1, 9),   // Yom Kippur I
    (1, 10),  // Yom Kippur II
    (1, 14),  // Sukkot I
    (1, 15),  // Sukkot II
    (1, 22),  // Simchat Torah I
    (1, 23),  // Simchat Torah II
    (6, 14),  // Purim
    (7, 14),  // Passover I
    (7, 15),  // Passover II
    (7, 20),  // Passover two I
    (7, 21),  // Passover two II
    (9, 5),   // Shavut I
    (9, 6),   // Shavut I
];

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Israel national holiday calendar.
pub struct IsraelCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl IsraelCalendar {
    /// Hebrew weekend is Friday and Saturday,
    /// as opposed to Saturday and Sunday in the Gregorian calendar.
    fn is_weekend(&self, date: Date) -> bool {
        let wd = date.weekday();
        wd == Weekday::Friday || wd == Weekday::Saturday
    }
}

impl Calendar for IsraelCalendar {
    fn name(&self) -> &'static str {
        "Israel"
    }

    fn country_code(&self) -> ISO_3166 {
        ISRAEL
    }

    fn market_identifier_code(&self) -> ISO_10383 {
        XTAE
    }

    fn is_business_day(&self, date: Date) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);
        let m = m as u8;
        let iso_date = icu::calendar::Date::try_new_iso_date(y, m, d)
            .expect("Failed to initialize ISO Date instance for constructing Hebrew date.");

        let hebrew_date = iso_date.to_calendar(icu::calendar::hebrew::Hebrew);
        let mut hebrew_month = hebrew_date.month().ordinal as u8;
        let hebrew_day = hebrew_date.day_of_month().0 as u8;

        if hebrew_date.is_in_leap_year() && hebrew_month > 7 {
            hebrew_month -= 1;
        }

        // Check if the date is Independence Day or Memorial Day.
        let is_independence_or_memorial_day = matches!(
            &(hebrew_month, hebrew_day, wd),
            (8, 3..=4, Weekday::Thursday)
                | (8, 2..=3, Weekday::Wednesday)
                | (8, 5, Weekday::Monday)
                | (8, 6, Weekday::Tuesday)
                | (8, 5, Weekday::Wednesday)
                | (8, 4, Weekday::Tuesday)
        );

        // Check if the date is Tisha Beav.
        let is_tisha_beav = matches!(
            &(hebrew_month, hebrew_day, wd),
            (11, 10, Weekday::Sunday) | (11, 9, Weekday::Saturday) | (11, 9, _)
        );

        JEWISH_HOLIDAYS.contains(&(hebrew_month, hebrew_day))
            || is_independence_or_memorial_day
            || is_tisha_beav
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_israel {
    use super::*;
    use time::macros::date;
    use time::Month;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = IsraelCalendar;
        assert_eq!(calendar.name(), "Israel");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = IsraelCalendar;
        let fri = date!(2023 - 01 - 27);
        let sat = date!(2023 - 01 - 28);
        assert!(!calendar.is_business_day(fri));
        assert!(!calendar.is_business_day(sat));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = IsraelCalendar;
        let holidays = vec![
            (2024, 3, 24),  // Purim
            (2024, 4, 22),  // Passover Eve
            (2024, 4, 23),  // Passover
            (2024, 4, 28),  // Passover II Eve
            (2024, 4, 29),  // Passover II
            (2024, 5, 13),  // Memorial Day
            (2024, 5, 14),  // Independence Day
            (2024, 6, 11),  // Pentecost (Shavuot) Eve
            (2024, 6, 12),  // Pentecost (Shavuot)
            (2024, 8, 13),  // Fast Day (Tisha B'Av)
            (2024, 10, 3),  // Jewish New Year I
            (2024, 10, 4),  // Jewish New Year II
            (2024, 10, 11), // Yom Kippur Eve
            (2024, 10, 17), // Feast of Tabernacles (Sukkoth)
            (2024, 10, 24), // Rejoicing of the Law (Simchat Tora)
            (2025, 3, 14),  // Purim
            (2025, 4, 13),  // Passover
            (2025, 6, 2),   // Pentecost (Shavuot)
            (2025, 8, 3),   // Fast Day (Tisha B'Av)
            (2025, 9, 23),  // Jewish New Year I
            (2025, 9, 24),  // Jewish New Year II
            (2025, 10, 2),  // Yom Kippur
            (2025, 10, 7),  // Feast of Tabernacles (Sukkoth)
            (2025, 10, 14), // Rejoicing of the Law (Simchat Tora)
            (2015, 3, 5),   // Purim
            (2015, 4, 10),  // Passover II
            (2015, 4, 23),  // Independence Day
            (2015, 5, 24),  // Pentecost (Shavuot)
            (2015, 7, 26),  // Fast Day
            (2015, 9, 14),  // Jewish New Year I
            (2015, 9, 15),  // Jewish New Year II
            (2015, 9, 23),  // Yom Kippur
            (2015, 9, 28),  // Feast of Tabernacles (Sukkoth)
            (2015, 10, 5),  // Rejoicing of the Law (Simchat Tora)
            (2018, 3, 1),   // Purim
            (2018, 4, 6),   // Passover II
            (2018, 4, 19),  // Independence Day
            (2018, 5, 20),  // Pentecost (Shavuot)
            (2018, 7, 22),  // Fast Day
            (2018, 9, 10),  // Jewish New Year I
            (2018, 9, 11),  // Jewish New Year II
            (2018, 9, 18),  // Yom Kippur Eve
            (2018, 9, 19),  // Yom Kippur
            (2018, 9, 24),  // Feast of Tabernacles (Sukkoth)
            (2018, 10, 1),  // Rejoicing of the Law (Simchat Tora)
            (2017, 3, 12),  // Purim
            (2017, 4, 11),  // Passover 1
            (2017, 4, 17),  // Passover II
            (2017, 5, 2),   // Independence Day
            (2017, 5, 31),  // Pentecost (Shavuot)
            (2017, 8, 1),   // Fast Day
            (2017, 9, 21),  // Jewish New Year I
            (2017, 9, 22),  // Jewish New Year II
            (2017, 9, 29),  // Yom Kippur Eve
            (2017, 10, 5),  // Feast of Tabernacles (Sukkoth)
            (2017, 10, 12), // Rejoicing of the Law (Simchat Tora)
        ];
        for (y, m, d) in holidays {
            let date = Date::from_calendar_date(y, Month::try_from(m).unwrap(), d).unwrap();
            assert!(!calendar.is_business_day(date));
        }
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = IsraelCalendar;
        let regular_day1 = date!(2021 - 08 - 04);
        let regular_day2 = date!(2024 - 04 - 09);
        let regular_day3 = date!(2023 - 11 - 27);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
