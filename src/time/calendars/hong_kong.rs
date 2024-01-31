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

/// Hong Kong calendar.
pub struct HongKong;

impl Calendar for HongKong {
    fn name(&self) -> &'static str {
        "Hong Kong"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::HONG_KONG
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XHKG
    }

    #[allow(clippy::match_overlapping_arm)]
    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (w, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
                // New Year's Day
                || ((d == 1 || ((d == 2) && w == Weekday::Monday)) && m == Month::January)
                // Good Friday
                || (dd == em-3)
                // Easter Monday
                || (dd == em)
                // Labor Day
                || ((d == 1 || ((d == 2) && w == Weekday::Monday)) && m == Month::May)
                // SAR Establishment Day
                || ((d == 1 || ((d == 2) && w == Weekday::Monday)) && m == Month::July)
                // National Day
                || ((d == 1 || ((d == 2) && w == Weekday::Monday)) && m == Month::October)
                // Christmas Day
                || (d == 25 && m == Month::December)
                // Boxing Day
                || (d == 26 && m == Month::December)
                // Lunar New Year
                || is_lunar_new_year(y, d, m)
                // Buddha's birthday
                || is_buddhas_birthday(y, d, m)
                // Ching Ming Festival
                || is_ching_ming_festival(y, d, m)
                // Tuen Ng festival
                || is_tuen_ng_festival(y, d, m)
                // Mid-autumn festival
                || is_mid_autumn_festival(y, d, m)
                // Chung Yeung festival
                || is_chung_yeung_festival(y, d, m)
                // Second day after Christmas
                || is_second_day_after_christmas(y, d, m)
        {
            return false;
        }

        // if y == 2004 {
        //     if
        //     // Lunar New Year
        //     ((d==22 || d==23 || d==24) && m == Month::January)
        //         // Ching Ming Festival
        //         || (d == 5 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 26 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 22 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 29 && m == Month::September)
        //         // Chung Yeung
        //         || (d == 22 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2005 {
        //     if
        //     // Lunar New Year
        //     ((d==9 || d==10 || d==11) && m == Month::February)
        //         // Ching Ming Festival
        //         || (d == 5 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 16 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 11 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 19 && m == Month::September)
        //         // Chung Yeung festival
        //         || (d == 11 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2006 {
        //     if
        //     // Lunar New Year
        //     ((d >= 28 && d <= 31) && m == Month::January)
        //         // Ching Ming Festival
        //         || (d == 5 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 5 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 31 && m == Month::May)
        //         // Mid-autumn festival
        //         || (d == 7 && m == Month::October)
        //         // Chung Yeung festival
        //         || (d == 30 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2007 {
        //     if
        //     // Lunar New Year
        //     ((d >= 17 && d <= 20) && m == Month::February)
        //         // Ching Ming Festival
        //         || (d == 5 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 24 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 19 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 26 && m == Month::September)
        //         // Chung Yeung festival
        //         || (d == 19 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2008 {
        //     if
        //     // Lunar New Year
        //     ((d >= 7 && d <= 9) && m == Month::February)
        //         // Ching Ming Festival
        //         || (d == 4 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 12 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 9 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 15 && m == Month::September)
        //         // Chung Yeung festival
        //         || (d == 7 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2009 {
        //     if ((d >= 26 && d <= 28) && m == Month::January) // Lunar New Year
        //         || (d == 4 && m == Month::April) // Ching Ming Festival
        //         || (d == 2 && m == Month::May) // Buddha's birthday
        //         || (d == 28 && m == Month::May) // Tuen Ng festival
        //         || (d == 3 && m == Month::October) // Mid-autumn festival
        //         || (d == 26 && m == Month::October)
        //     // Chung Yeung festival
        //     {
        //         return false;
        //     }
        // }

        // if y == 2010 {
        //     if
        //     // Lunar New Year
        //     ((d == 15 || d == 16) && m == Month::February)
        //         // Ching Ming Festival
        //         || (d == 6 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 21 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 16 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 23 && m == Month::September)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // Ching Ming Festival
        // // Buddha's birthday
        // // Tuen Ng festival
        // // Mid-autumn festival
        // // Chung Yeung festival
        // // Second day after Christmas
        // if y == 2011 {
        //     if ((d == 3 || d == 4) && m == Month::February)
        //         || (d == 5 && m == Month::April)
        //         || (d == 10 && m == Month::May)
        //         || (d == 6 && m == Month::June)
        //         || (d == 13 && m == Month::September)
        //         || (d == 5 && m == Month::October)
        //         || (d == 27 && m == Month::December)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2012 {
        //     if
        //     // Lunar New Year
        //     (d >= 23 && d <= 25 && m == Month::January)
        //         // Ching Ming Festival
        //         || (d == 4 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 10 && m == Month::May)
        //         // Mid-autumn festival
        //         || (d == 1 && m == Month::October)
        //         // Chung Yeung festival
        //         || (d == 23 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2013 {
        //     if
        //     // Lunar New Year
        //     (d >= 11 && d <= 13 && m == Month::February)
        //         // Ching Ming Festival
        //         || (d == 4 && m == Month::April)
        //         // Buddha's birthday
        //         || (d == 17 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 12 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 20 && m == Month::September)
        //         // Chung Yeung festival
        //         || (d == 14 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // if y == 2014 {
        //     if
        //     // Lunar New Year
        //     ((d == 31 && m == Month::January) || (d <= 3 && m == Month::February))
        //         // Buddha's birthday
        //         || (d == 6 && m == Month::May)
        //         // Tuen Ng festival
        //         || (d == 2 && m == Month::June)
        //         // Mid-autumn festival
        //         || (d == 9 && m == Month::September)
        //         // Chung Yeung festival
        //         || (d == 2 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // The day following Easter Monday
        // // Buddha's birthday
        // // Tuen Ng festival
        // // The 70th anniversary day of the victory of the Chinese
        // // people's war of resistance against Japanese aggression
        // // Mid-autumn festival
        // // Chung Yeung festival
        // if y == 2015 {
        //     if ((d == 19 && m == Month::February) || (d == 20 && m == Month::February))
        //         || (d == 7 && m == Month::April)
        //         || (d == 25 && m == Month::May)
        //         || (d == 20 && m == Month::June)
        //         || (d == 3 && m == Month::September)
        //         || (d == 28 && m == Month::September)
        //         || (d == 21 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // Ching Ming Festival
        // // Tuen Ng festival
        // // Mid-autumn festival
        // // Chung Yeung festival
        // // Second day after Christmas
        // if y == 2016 {
        //     if ((d >= 8 && d <= 10) && m == Month::February)
        //         || (d == 4 && m == Month::April)
        //         || (d == 9 && m == Month::June)
        //         || (d == 16 && m == Month::September)
        //         || (d == 10 && m == Month::October)
        //         || (d == 27 && m == Month::December)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // Ching Ming Festival
        // // Buddha's birthday
        // // Tuen Ng festival
        // // Mid-autumn festival
        // if y == 2017 {
        //     if ((d == 30 || d == 31) && m == Month::January)
        //         || (d == 4 && m == Month::April)
        //         || (d == 3 && m == Month::May)
        //         || (d == 30 && m == Month::May)
        //         || (d == 5 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // Ching Ming Festival
        // // Buddha's birthday
        // // Tuen Ng festival
        // // Mid-autumn festival
        // // Chung Yeung festival
        // if y == 2018 {
        //     if ((d == 16 && m == Month::February) || (d == 19 && m == Month::February))
        //         || (d == 5 && m == Month::April)
        //         || (d == 22 && m == Month::May)
        //         || (d == 18 && m == Month::June)
        //         || (d == 25 && m == Month::September)
        //         || (d == 17 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // Ching Ming Festival
        // // Tuen Ng festival
        // // Chung Yeung festival
        // if y == 2019 {
        //     if ((d >= 5 && d <= 7) && m == Month::February)
        //         || (d == 5 && m == Month::April)
        //         || (d == 7 && m == Month::June)
        //         || (d == 7 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        // // Lunar New Year
        // // Ching Ming Festival
        // // Buddha's birthday
        // // Tuen Ng festival
        // // Mid-autumn festival
        // // Chung Yeung festival
        // if y == 2020 {
        //     if ((d == 27 || d == 28) && m == Month::January)
        //         || (d == 4 && m == Month::April)
        //         || (d == 30 && m == Month::April)
        //         || (d == 25 && m == Month::June)
        //         || (d == 2 && m == Month::October)
        //         || (d == 26 && m == Month::October)
        //     {
        //         return false;
        //     }
        // }

        true
    }
}

#[allow(clippy::unnested_or_patterns)]
fn is_lunar_new_year(year: i32, day: u8, month: Month) -> bool {
    use Month::{February, January};
    matches!(
        (year, day, month),
        (2004, 22..=24, January)
            | (2005, 9..=11, February)
            | (2006, 28..=30, January)
            | (2007, 17..=19, February)
            | (2008, 7..=9, February)
            | (2009, 26..=28, January)
            | (2010, 13..=15, February)
            | (2011, 2..=4, February)
            | (2012, 23..=25, January)
            | (2013, 10..=12, February)
            | (2014, 31 | 1 | 2, January)
            | (2015, 19..=21, February)
            | (2016, 8..=10, February)
            | (2017, 28..=30, January)
            | (2018, 16..=18, February)
            | (2019, 5..=7, February)
            | (2020, 25 | 27 | 28, January)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_buddhas_birthday(year: i32, day: u8, month: Month) -> bool {
    use Month::May;
    matches!(
        (year, day, month),
        (2004, 26, May)
            | (2005, 16, May)
            | (2006, 5, May)
            | (2007, 24, May)
            | (2008, 12, May)
            | (2009, 2, May)
            | (2010, 21, May)
            | (2011, 10, May)
            | (2012, 28, May)
            | (2013, 17, May)
            | (2014, 6, May)
            | (2015, 25, May)
            | (2016, 14, May)
            | (2017, 3, May)
            | (2018, 22, May)
            | (2019, 12, May)
            | (2020, 30, May)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_ching_ming_festival(year: i32, day: u8, month: Month) -> bool {
    use Month::April;
    matches!(
        (year, day, month),
        (2004, 5, April)
            | (2005, 5, April)
            | (2006, 5, April)
            | (2007, 5, April)
            | (2008, 4, April)
            | (2009, 4, April)
            | (2010, 5, April)
            | (2011, 5, April)
            | (2012, 4, April)
            | (2013, 4, April)
            | (2014, 5, April)
            | (2015, 5, April)
            | (2016, 4, April)
            | (2017, 4, April)
            | (2018, 5, April)
            | (2019, 5, April)
            | (2020, 4, April)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_tuen_ng_festival(year: i32, day: u8, month: Month) -> bool {
    use Month::{June, May};

    matches!(
        (year, day, month),
        (2004, 22, June)
            | (2005, 11, June)
            | (2006, 1, June)
            | (2007, 19, June)
            | (2008, 9, June)
            | (2009, 28, June)
            | (2010, 16, June)
            | (2011, 6, June)
            | (2012, 23, June)
            | (2013, 12, June)
            | (2014, 2, June)
            | (2015, 20, June)
            | (2016, 9, June)
            | (2017, 30, May)
            | (2018, 18, June)
            | (2019, 7, June)
            | (2020, 25, June)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_mid_autumn_festival(year: i32, day: u8, month: Month) -> bool {
    use Month::{October, September};
    matches!(
        (year, day, month),
        (2004, 28, September)
            | (2005, 18, September)
            | (2006, 7, October)
            | (2007, 26, September)
            | (2008, 15, September)
            | (2009, 3, October)
            | (2010, 22, September)
            | (2011, 12, September)
            | (2012, 30, September)
            | (2013, 19, September)
            | (2014, 9, September)
            | (2015, 27, September)
            | (2016, 15, September)
            | (2017, 5, October)
            | (2018, 25, September)
            | (2019, 14, September)
            | (2020, 1, October)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_chung_yeung_festival(year: i32, day: u8, month: Month) -> bool {
    use Month::October;
    matches!(
        (year, day, month),
        (2004, 28, October)
            | (2005, 18, October)
            | (2006, 7, October)
            | (2007, 26, October)
            | (2008, 16, October)
            | (2009, 26, October)
            | (2010, 16, October)
            | (2011, 5, October)
            | (2012, 23, October)
            | (2013, 14, October)
            | (2014, 2, October)
            | (2015, 21, October)
            | (2016, 10, October)
            | (2017, 28, October)
            | (2018, 17, October)
            | (2019, 7, October)
            | (2020, 26, October)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_second_day_after_christmas(year: i32, day: u8, month: Month) -> bool {
    use Month::December;
    matches!(
        (year, day, month),
        (2004, 27, December)
            | (2005, 27, December)
            | (2006, 27, December)
            | (2007, 27, December)
            | (2008, 29, December)
            | (2009, 28, December)
            | (2010, 27, December)
            | (2011, 27, December)
            | (2012, 27, December)
            | (2013, 27, December)
            | (2014, 29, December)
            | (2015, 28, December)
            | (2016, 27, December)
            | (2017, 27, December)
            | (2018, 27, December)
            | (2019, 27, December)
            | (2020, 28, December)
    )
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #[cfg(test)]
// mod test_hong_kong {
//     use super::*;
//     use time::macros::datetime;

//     const CALENDAR: HongKong = HongKong;
//     const YEARS: [i32; 16] = [
//         2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019,
//         2020,
//     ];

//     #[test]
//     fn test_metadata() {
//         assert_eq!(CALENDAR.name(), "Hong Kong");
//         assert_eq!(CALENDAR.country_code(), crate::iso::HONG_KONG);
//         assert_eq!(CALENDAR.market_identifier_code(), crate::iso::XHKG);
//     }

//     // Test Lunar New Year dates.
//     // Date::from_calendar_date(2020, Month::January, 1)?.midnight().assume_utc()
//     #[test]
//     fn test_lunar_new_year() {
//         for year in YEARS.iter() {
//             // let lunar_new_year =
//             //     OffsetDateTime::from_unix_timestamp((CALENDAR.lunar_new_year(year) * 86400) as i64);

//             let lunar_new_year = OffsetDateTime::from_calendar_date(
//                 year,
//                 Month::January,
//                 lunar_new_year.day() as u8,
//             )?;

//             assert!(!CALENDAR.is_business_day(lunar_new_year));
//         }
//     }

//     // Test to verify if weekends are not considered business days.
//     #[test]
//     fn test_is_weekend() {
//         let sat = datetime!(2023-08-26 12:00:00 UTC);
//         let sun = datetime!(2023-08-27 12:00:00 UTC);
//         let mon = datetime!(2023-08-28 12:00:00 UTC);

//         assert!(!CALENDAR.is_business_day(sat));
//         assert!(!CALENDAR.is_business_day(sun));

//         assert!(CALENDAR.is_business_day(mon));
//     }

//     // Test to verify if the is_business_day() method properly accounts for public holidays.
//     #[test]
//     fn test_is_public_holiday() {
//         let new_years_day = datetime!(2023 - 01 - 01 12:00:00 UTC);
//         let australia_day = datetime!(2023 - 01 - 26 12:00:00 UTC);
//         let anzac_day = datetime!(2023 - 04 - 25 12:00:00 UTC);
//         let christmas = datetime!(2023 - 12 - 25 12:00:00 UTC);

//         assert!(!CALENDAR.is_business_day(new_years_day));
//         assert!(!CALENDAR.is_business_day(australia_day));
//         assert!(!CALENDAR.is_business_day(anzac_day));
//         assert!(!CALENDAR.is_business_day(christmas));
//     }

//     // Test to verify if the is_business_day() method properly accounts for regular business days.
//     #[test]
//     fn test_is_regular_business_day() {
//         let calendar = Australia;
//         let regular_day1 = datetime!(2023-03-01 12:00:00 UTC);
//         let regular_day2 = datetime!(2023-07-12 12:00:00 UTC);
//         let regular_day3 = datetime!(2023-11-17 12:00:00 UTC);

//         assert!(calendar.is_business_day(regular_day1));
//         assert!(calendar.is_business_day(regular_day2));
//         assert!(calendar.is_business_day(regular_day3));
//     }
// }
