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

fn is_lunar_new_year(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::January && (day == 22 || day == 23 || day == 24)
        || year == 2005 && month == Month::February && (day == 9 || day == 10 || day == 11)
        || year == 2006 && month == Month::January && (day == 28 || day == 29 || day == 30)
        || year == 2007 && month == Month::February && (day == 17 || day == 18 || day == 19)
        || year == 2008 && month == Month::February && (day == 7 || day == 8 || day == 9)
        || year == 2009 && month == Month::January && (day == 26 || day == 27 || day == 28)
        || year == 2010 && month == Month::February && (day == 13 || day == 14 || day == 15)
        || year == 2011 && month == Month::February && (day == 2 || day == 3 || day == 4)
        || year == 2012 && month == Month::January && (day == 23 || day == 24 || day == 25)
        || year == 2013 && month == Month::February && (day == 10 || day == 11 || day == 12)
        || year == 2014 && month == Month::January && (day == 31 || day == 1 || day == 2)
        || year == 2015 && month == Month::February && (day == 19 || day == 20 || day == 21)
        || year == 2016 && month == Month::February && (day == 8 || day == 9 || day == 10)
        || year == 2017 && month == Month::January && (day == 28 || day == 29 || day == 30)
        || year == 2018 && month == Month::February && (day == 16 || day == 17 || day == 18)
        || year == 2019 && month == Month::February && (day == 5 || day == 6 || day == 7)
        || year == 2020 && month == Month::January && (day == 25 || day == 27 || day == 28)
    {
        return true;
    }

    false
}

fn is_buddhas_birthday(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::May && day == 26
        || year == 2005 && month == Month::May && day == 16
        || year == 2006 && month == Month::May && day == 5
        || year == 2007 && month == Month::May && day == 24
        || year == 2008 && month == Month::May && day == 12
        || year == 2009 && month == Month::May && day == 2
        || year == 2010 && month == Month::May && day == 21
        || year == 2011 && month == Month::May && day == 10
        || year == 2012 && month == Month::May && day == 28
        || year == 2013 && month == Month::May && day == 17
        || year == 2014 && month == Month::May && day == 6
        || year == 2015 && month == Month::May && day == 25
        || year == 2016 && month == Month::May && day == 14
        || year == 2017 && month == Month::May && day == 3
        || year == 2018 && month == Month::May && day == 22
        || year == 2019 && month == Month::May && day == 12
        || year == 2020 && month == Month::May && day == 30
    {
        return true;
    }

    false
}

fn is_ching_ming_festival(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::April && day == 5
        || year == 2005 && month == Month::April && day == 5
        || year == 2006 && month == Month::April && day == 5
        || year == 2007 && month == Month::April && day == 5
        || year == 2008 && month == Month::April && day == 4
        || year == 2009 && month == Month::April && day == 4
        || year == 2010 && month == Month::April && day == 5
        || year == 2011 && month == Month::April && day == 5
        || year == 2012 && month == Month::April && day == 4
        || year == 2013 && month == Month::April && day == 4
        || year == 2014 && month == Month::April && day == 5
        || year == 2015 && month == Month::April && day == 5
        || year == 2016 && month == Month::April && day == 4
        || year == 2017 && month == Month::April && day == 4
        || year == 2018 && month == Month::April && day == 5
        || year == 2019 && month == Month::April && day == 5
        || year == 2020 && month == Month::April && day == 4
    {
        return true;
    }

    false
}

fn is_tuen_ng_festival(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::June && day == 22
        || year == 2005 && month == Month::June && day == 11
        || year == 2006 && month == Month::June && day == 1
        || year == 2007 && month == Month::June && day == 19
        || year == 2008 && month == Month::June && day == 9
        || year == 2009 && month == Month::June && day == 28
        || year == 2010 && month == Month::June && day == 16
        || year == 2011 && month == Month::June && day == 6
        || year == 2012 && month == Month::June && day == 23
        || year == 2013 && month == Month::June && day == 12
        || year == 2014 && month == Month::June && day == 2
        || year == 2015 && month == Month::June && day == 20
        || year == 2016 && month == Month::June && day == 9
        || year == 2017 && month == Month::May && day == 30
        || year == 2018 && month == Month::June && day == 18
        || year == 2019 && month == Month::June && day == 7
        || year == 2020 && month == Month::June && day == 25
    {
        return true;
    }

    false
}

fn is_mid_autumn_festival(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::September && day == 28
        || year == 2005 && month == Month::September && day == 18
        || year == 2006 && month == Month::October && day == 7
        || year == 2007 && month == Month::September && day == 26
        || year == 2008 && month == Month::September && day == 15
        || year == 2009 && month == Month::October && day == 3
        || year == 2010 && month == Month::September && day == 22
        || year == 2011 && month == Month::September && day == 12
        || year == 2012 && month == Month::September && day == 30
        || year == 2013 && month == Month::September && day == 19
        || year == 2014 && month == Month::September && day == 9
        || year == 2015 && month == Month::September && day == 27
        || year == 2016 && month == Month::September && day == 15
        || year == 2017 && month == Month::October && day == 5
        || year == 2018 && month == Month::September && day == 25
        || year == 2019 && month == Month::September && day == 14
        || year == 2020 && month == Month::October && day == 1
    {
        return true;
    }

    false
}

fn is_chung_yeung_festival(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::October && day == 28
        || year == 2005 && month == Month::October && day == 18
        || year == 2006 && month == Month::October && day == 7
        || year == 2007 && month == Month::October && day == 26
        || year == 2008 && month == Month::October && day == 16
        || year == 2009 && month == Month::October && day == 26
        || year == 2010 && month == Month::October && day == 16
        || year == 2011 && month == Month::October && day == 5
        || year == 2012 && month == Month::October && day == 23
        || year == 2013 && month == Month::October && day == 14
        || year == 2014 && month == Month::October && day == 2
        || year == 2015 && month == Month::October && day == 21
        || year == 2016 && month == Month::October && day == 10
        || year == 2017 && month == Month::October && day == 28
        || year == 2018 && month == Month::October && day == 17
        || year == 2019 && month == Month::October && day == 7
        || year == 2020 && month == Month::October && day == 26
    {
        return true;
    }

    false
}

fn is_second_day_after_christmas(year: i32, day: u8, month: Month) -> bool {
    if year == 2004 && month == Month::December && day == 27
        || year == 2005 && month == Month::December && day == 27
        || year == 2006 && month == Month::December && day == 27
        || year == 2007 && month == Month::December && day == 27
        || year == 2008 && month == Month::December && day == 29
        || year == 2009 && month == Month::December && day == 28
        || year == 2010 && month == Month::December && day == 27
        || year == 2011 && month == Month::December && day == 27
        || year == 2012 && month == Month::December && day == 27
        || year == 2013 && month == Month::December && day == 27
        || year == 2014 && month == Month::December && day == 29
        || year == 2015 && month == Month::December && day == 28
        || year == 2016 && month == Month::December && day == 27
        || year == 2017 && month == Month::December && day == 27
        || year == 2018 && month == Month::December && day == 27
        || year == 2019 && month == Month::December && day == 27
        || year == 2020 && month == Month::December && day == 28
    {
        return true;
    }

    false
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
