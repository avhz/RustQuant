// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::{is_weekend, Calendar};
use time::{Month, OffsetDateTime, Weekday};

/// United States settlement calendar.
pub struct UnitedStates;

fn is_washington_birthday(d: u8, m: Month, y: i32, w: Weekday) -> bool {
    if y >= 1971 {
        (15..=21).contains(&d) && w == Weekday::Monday && m == Month::February
    } else {
        (d == 22 || (d == 23 && w == Weekday::Monday) || (d == 21 && w == Weekday::Friday))
            && m == Month::February
    }
}

fn is_memorial_day(d: u8, m: Month, y: i32, w: Weekday) -> bool {
    if y >= 1971 {
        d >= 25 && w == Weekday::Monday && m == Month::May
    } else {
        (d == 30 || (d == 31 && w == Weekday::Monday) || (d == 29 && w == Weekday::Friday))
            && m == Month::May
    }
}

fn is_juneteenth(d: u8, m: Month, y: i32, w: Weekday) -> bool {
    (d == 19 || (d == 20 && w == Weekday::Monday) || (d == 18 && w == Weekday::Friday))
        && m == Month::June
        && y >= 2022
}

impl Calendar for UnitedStates {
    fn name(&self) -> &'static str {
        "United States"
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let w = date.weekday();
        let d = date.day();
        let m = date.month();
        let y = date.year();

        if is_weekend(date)
            || ((d == 1 || (d == 2 && w == Weekday::Monday)) && m == Month::January)
            || (d == 31 && w == Weekday::Friday && m == Month::December)
            || ((15..=21).contains(&d) && w == Weekday::Monday && m == Month::January && y >= 1983)
            || is_washington_birthday(d, m, y, w)
            || is_memorial_day(d, m, y, w)
            || is_juneteenth(d, m, y, w)
            || ((d == 4 || (d == 5 && w == Weekday::Monday) || (d == 3 && w == Weekday::Friday))
                && m == Month::July)
            || (d <= 7 && w == Weekday::Monday && m == Month::September)
            || ((8..=14).contains(&d) && w == Weekday::Monday && m == Month::October && y >= 1971)
            || ((d == 11 || (d == 12 && w == Weekday::Monday) || (d == 10 && w == Weekday::Friday))
                && m == Month::November)
            || ((22..=28).contains(&d) && w == Weekday::Thursday && m == Month::November)
            || ((d == 25 || (d == 26 && w == Weekday::Monday) || (d == 24 && w == Weekday::Friday))
                && m == Month::December)
        {
            return false;
        }

        true
    }
}
