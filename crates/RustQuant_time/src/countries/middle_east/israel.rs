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

use heca_lib::prelude::HebrewMonth;
use time::{Date, Weekday};

use chrono::Utc;
use chrono::offset::TimeZone;
use heca_lib::HebrewDate;

use crate::calendar::Calendar;
use crate::utilities::unpack_date;
use RustQuant_iso::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// CONSTANTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const JEWISH_HOLIDAYS: [(u8, u8); 19] = [
    (12, 29),     // Jewish new year (Rosh Hashana) I
    (1, 1),     // Jewish new year (Rosh Hashana) II
    (1, 2),     // Jewish new year (Rosh Hashana) II
    (1, 9),    // Yom Kippur I
    (1, 10),    // Yom Kippur II
    (1, 14),    // Sukkot I 
    (1, 15),    // Sukkot II 
    (1, 22),    // Simchat Torah I
    (1, 23),    // Simchat Torah II
    (6, 14),    // Purim 
    (7, 14),    // Passover I
    (7, 15),    // Passover II
    (7, 20),    // Passover two I
    (7, 21),    // Passover two II
    (8, 5),     // Memorial day
    (8, 6),     // Independence day
    (9, 5),     // Shavut I
    (9, 6),     // Shavut I
    (11, 9),    // Tisha Be'av
];

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Israel a national holiday calendar.
pub struct IsraelCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


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

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);
        let m = m as u8;

        // Jewish weekend (Friday, Saturday)
        if (wd == Weekday::Friday || wd == Weekday::Saturday) {
            return true;
        }

        let hebrew_date: HebrewDate = Utc.with_ymd_and_hms(y.into(), m.into(), d.into(), 0, 0, 0).unwrap().try_into().unwrap();
        let month = match hebrew_date.month() {
            HebrewMonth::Tishrei => 1,
            HebrewMonth::Cheshvan => 2,
            HebrewMonth::Kislev => 3,
            HebrewMonth::Teves => 4,
            HebrewMonth::Shvat => 5,
            HebrewMonth::Adar => 6,
            HebrewMonth::Adar1 => 6,
            HebrewMonth::Adar2 => 100, // Adar 2 is a leap-year month and never has a holiday. 100 is an arbitrary escape value.
            HebrewMonth::Nissan => 7,
            HebrewMonth::Iyar => 8,
            HebrewMonth::Sivan => 9,
            HebrewMonth::Tammuz => 10,
            HebrewMonth::Av => 11,
            HebrewMonth::Elul => 12,
        };

        let date_tuple: (u8, u8) = (month, hebrew_date.day().get() as u8); 
        println!("{:?}", hebrew_date);
        println!("{:?}", date_tuple);

        JEWISH_HOLIDAYS.contains(&date_tuple)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_israel {
    use super::*;
    use time::macros::date;

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
        let purim = date!(2024 - 03 - 24); // Purim holiday 2024
        let sukkot = date!(2024 - 10 - 17); // Sukkot holiday 2024
        let passover_23 = date!(2023 - 4 - 05); // Passover eve 2023
        let passover_24 = date!(2023 - 4 - 22); // Passover eve 2024

        assert!(!calendar.is_business_day(purim));
        assert!(!calendar.is_business_day(sukkot));
        assert!(!calendar.is_business_day(passover_23));
        assert!(!calendar.is_business_day(passover_24));
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
