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
use reqwest::{blocking::Client, Error};
use serde::Deserialize;
use time::{Date, Weekday};
use RustQuant_iso::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Israel a national holiday calendar.
pub struct IsraelCalendar;

// Two structs to capture response
#[derive(Deserialize, Debug)]
struct ResponseData {
    items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Item {
    title: String,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn jewish_holiday(date: &String) -> Result<bool, Error> {
    let url = "https://www.hebcal.com/hebcal";

    let params = [
        ("v", "1"),
        ("cfg", "json"),
        ("maj", "on"),
        ("start", date),
        ("end", date),
    ];

    let response = Client::new().get(url).query(&params).send().map_err(|e| {
        eprintln!("Failed to send request: {}", e);
        e
    })?;

    let json: ResponseData = response.json().map_err(|e| {
        eprintln!("Failed to parse Json response: {}", e);
        e
    })?;

    Ok(!json.items.is_empty())
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

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);
        let m = m as u8;

        // Jewish weekend (Friday, Saturday)
        if (wd == Weekday::Friday || wd == Weekday::Saturday) {
            return true;
        }

        match jewish_holiday(&format!("{:04}-{:02}-{:02}", y, m, d)) {
            Ok(is_holiday) => is_holiday,
            Err(e) => {
                eprintln!("Error checking Jewish holiday: {}", e);
                false // default to non-holiday if there's an error
            }
        }
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
