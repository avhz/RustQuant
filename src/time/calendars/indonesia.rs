// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use time::{Month, OffsetDateTime};

/// Indonesia calendar.
pub struct Indonesia;

impl Calendar for Indonesia {
    fn name(&self) -> &'static str {
        "Indonesia"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::INDONESIA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XIDX
    }

    fn is_business_day(&self, date: OffsetDateTime) -> bool {
        let (_, d, m, y, dd) = self.unpack_date(date);
        let em = Self::easter_monday(y as usize, false);

        if Self::is_weekend(date)
            // New Year's Day
            || (d == 1 && m == Month::January)
            // Ascension of the Prophet Muhammad
            || is_ascension_day_of_prophet_muhammad(y,d,m)
            // Lunar New Year
            || is_lunar_new_year(y,d,m)
            // Hindu New year
            || is_hindu_new_year(y,d,m)
            // Good Friday
            || (dd == em - 3)
            // Eid-ul-Fitar
            || is_eid_ul_fitar(y,d,m)
            // Labor Day
            || (d == 1 && m == Month::May)
            // Ascension Day of Jesus Christ
            || (dd == em + 38)
            || (y == 2007 && m == Month::May && d==18)
            || (y == 2008 && m == Month::May && d==2)
            || (y == 2024 && m == Month::May && d==10)
            // Vesak Day
            || is_vesak_day(y,d,m)
            // Pancasila Day
            || (d == 1 && m == Month::June)
            // Eid-ul-Adha
            || is_eid_ul_adha(y,d,m)
            // Muharram
            || is_muharram(y,d,m)
            // Independence Day
            || (d==17 && m==Month::August)
            // Birth of Prophet Muhammad
            || is_birth_of_prophet_muhammad(y,d,m)
            // Christmas
            || (d == 25 && m==Month::December)
            // Boxing Day
            || (d == 26 && m == Month::December)
        {
            return false;
        }

        true
    }
}

#[allow(clippy::unnested_or_patterns)]
fn is_ascension_day_of_prophet_muhammad(year: i32, day: u8, month: Month) -> bool {
    matches!(
        (year, day, month),
        (2000, 26, Month::October)
            | (2001, 15, Month::October)
            | (2002, 4, Month::October)
            | (2003, 24, Month::September)
            | (2004, 12, Month::September)
            | (2005, 1, Month::September)
            | (2006, 22, Month::August)
            | (2007, 11, Month::August)
            | (2008, 31, Month::July)
            | (2009, 20, Month::July)
            | (2010, 9, Month::July)
            | (2011, 29, Month::June)
            | (2012, 17, Month::June)
            | (2013, 6, Month::June)
            | (2014, 27, Month::May)
            | (2015, 16, Month::May)
            | (2016, 6, Month::May)
            | (2017, 24, Month::April)
            | (2018, 14, Month::April)
            | (2019, 3, Month::April)
            | (2020, 22, Month::March)
            | (2021, 11, Month::March)
            | (2022, 28, Month::February)
            | (2023, 18, Month::February)
            | (2024, 8, Month::February)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_lunar_new_year(year: i32, day: u8, month: Month) -> bool {
    use Month::{February, January};
    matches!(
        (year, day, month),
        (2000, 5, February)
            | (2001, 24, January)
            | (2002, 12, February)
            | (2003, 1, February)
            | (2004, 22, January)
            | (2005, 9, February)
            | (2006, 30, January)
            | (2007, 19, February)
            | (2008, 7, February)
            | (2009, 26, January)
            | (2010, 15, February)
            | (2011, 3, February)
            | (2012, 23, January)
            | (2013, 11, February)
            | (2014, 31, January)
            | (2015, 19, February)
            | (2016, 8, February)
            | (2017, 28, January)
            | (2018, 16, February)
            | (2019, 5, February)
            | (2020, 25, January)
            | (2021, 12, February)
            | (2022, 1, February)
            | (2023, 23, January)
            | (2024, 9, February)
            | (2025, 29, January)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_hindu_new_year(year: i32, day: u8, month: Month) -> bool {
    use Month::March;
    matches!(
        (year, day, month),
        (2006, 30, March)
            | (2007, 19, March)
            | (2008, 7, March)
            | (2009, 26, March)
            | (2010, 16, March)
            | (2011, 5, March)
            | (2012, 23, March)
            | (2013, 12, March)
            | (2014, 31, March)
            | (2015, 21, March)
            | (2016, 9, March)
            | (2017, 28, March)
            | (2018, 17, March)
            | (2019, 7, March)
            | (2020, 25, March)
            | (2021, 14, March)
            | (2022, 3, March)
            | (2023, 22, March)
            | (2023, 23, March)
            | (2024, 11, March)
            | (2024, 12, March)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_eid_ul_fitar(year: i32, day: u8, month: Month) -> bool {
    matches!(
        (year, day, month),
        (2006, 23, Month::October)
            | (2007, 12, Month::October)
            | (2008, 29, Month::September)
            | (2008, 30, Month::September)
            | (2010, 9, Month::September)
            | (2015, 16, Month::July)
            | (2016, 4, Month::July)
            | (2016, 5, Month::July)
            | (2018, 11, Month::June)
            | (2018, 12, Month::June)
            | (2019, 3, Month::June)
            | (2019, 4, Month::June)
            | (2021, 12, Month::May)
            | (2022, 29, Month::April)
            | (2023, 19, Month::April)
            | (2023, 20, Month::April)
            | (2024, 8, Month::April)
            | (2024, 9, Month::April)
            | (2006, 26, Month::October)
            | (2007, 15, Month::October)
            | (2008, 3, Month::October)
            | (2009, 23, Month::September)
            | (2010, 13, Month::September)
            | (2014, 30, Month::July)
            | (2015, 20, Month::July)
            | (2016, 8, Month::July)
            | (2018, 18, Month::June)
            | (2018, 13, Month::June)
            | (2019, 7, Month::June)
            | (2019, 6, Month::June)
            | (2019, 5, Month::June)
            | (2020, 25, Month::May)
            | (2020, 24, Month::May)
            | (2021, 14, Month::May)
            | (2021, 17, Month::May)
            | (2021, 13, Month::May)
            | (2022, 2, Month::May)
            | (2022, 3, Month::May)
            | (2022, 4, Month::May)
            | (2023, 21, Month::April)
            | (2023, 24, Month::April)
            | (2023, 22, Month::April)
            | (2023, 23, Month::April)
            | (2024, 12, Month::April)
            | (2024, 15, Month::April)
            | (2024, 11, Month::April)
            | (2024, 10, Month::April)
            | (2025, 1, Month::April)
            | (2025, 2, Month::April)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_vesak_day(year: i32, day: u8, month: Month) -> bool {
    use Month::{June, May};
    matches!(
        (year, day, month),
        (2007, 1, June)
            | (2008, 19, May)
            | (2008, 20, May)
            | (2009, 9, May)
            | (2010, 28, May)
            | (2011, 17, May)
            | (2012, 6, May)
            | (2013, 25, May)
            | (2014, 15, May)
            | (2015, 2, June)
            | (2016, 22, May)
            | (2017, 11, May)
            | (2018, 29, May)
            | (2019, 19, May)
            | (2020, 7, May)
            | (2021, 26, May)
            | (2022, 16, May)
            | (2023, 4, June)
            | (2023, 2, June)
            | (2024, 23, May)
            | (2024, 24, May)
            | (2025, 13, May)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_eid_ul_adha(year: i32, day: u8, month: Month) -> bool {
    use Month::{August, July, June};
    matches!(
        (year, day, month),
        (2019, 11, August)
            | (2020, 31, July)
            | (2021, 20, July)
            | (2022, 10, July)
            | (2023, 29, June)
            | (2024, 17, June)
            | (2024, 18, June)
            | (2025, 7, June)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_muharram(year: i32, day: u8, month: Month) -> bool {
    matches!(
        (year, day, month),
        (2000, 6, Month::April)
            | (2001, 26, Month::March)
            | (2002, 15, Month::March)
            | (2003, 5, Month::March)
            | (2004, 22, Month::February)
            | (2005, 10, Month::February)
            | (2006, 31, Month::January)
            | (2007, 20, Month::January)
            | (2008, 10, Month::January)
            | (2008, 29, Month::December)
            | (2009, 18, Month::December)
            | (2010, 7, Month::December)
            | (2011, 27, Month::November)
            | (2012, 15, Month::November)
            | (2013, 5, Month::November)
            | (2014, 25, Month::October)
            | (2015, 14, Month::October)
            | (2016, 2, Month::October)
            | (2017, 21, Month::September)
            | (2018, 11, Month::September)
            | (2019, 1, Month::September)
            | (2020, 20, Month::August)
            | (2021, 10, Month::August)
            | (2021, 11, Month::August)
            | (2022, 30, Month::July)
            | (2023, 19, Month::July)
            | (2024, 7, Month::July)
            | (2025, 27, Month::June)
    )
}

#[allow(clippy::unnested_or_patterns)]
fn is_birth_of_prophet_muhammad(year: i32, day: u8, month: Month) -> bool {
    matches!(
        (year, day, month),
        (2006, 10, Month::April)
            | (2007, 31, Month::March)
            | (2008, 20, Month::March)
            | (2009, 9, Month::March)
            | (2010, 26, Month::February)
            | (2011, 15, Month::February)
            | (2012, 5, Month::February)
            | (2013, 24, Month::January)
            | (2014, 14, Month::January)
            | (2015, 3, Month::January)
            | (2015, 24, Month::December)
            | (2016, 12, Month::December)
            | (2017, 1, Month::December)
            | (2018, 20, Month::November)
            | (2019, 9, Month::November)
            | (2020, 29, Month::October)
            | (2021, 19, Month::October)
            | (2021, 20, Month::October)
            | (2022, 8, Month::October)
            | (2023, 28, Month::September)
            | (2024, 15, Month::September)
            | (2025, 5, Month::September)
    )
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_indonesia {
    use super::*;
    use time::macros::datetime;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = Indonesia;
        assert_eq!(calendar.name(), "Indonesia");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = Indonesia;
        let sat = datetime!(2024-04-27 12:00:00 UTC);
        let sun = datetime!(2024-04-28 12:00:00 UTC);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = Indonesia;
        let new_years_day = datetime!(2024-01-01 12:00:00 UTC);
        let ascension_of_the_prophet_muhammad = datetime!(2024-02-08 12:00:00 UTC);
        let lunar_new_year = datetime!(2024-02-09 12:00:00 UTC);
        let hindu_new_year = datetime!(2024-03-11 12:00:00 UTC);
        let hindu_new_year_2 = datetime!(2024-03-12 12:00:00 UTC);
        let eid_ul_fitar = datetime!(2024-04-08 12:00:00 UTC);
        let eid_ul_fitar_2 = datetime!(2024-04-15 12:00:00 UTC);
        let labor_day = datetime!(2024-05-01 12:00:00 UTC);
        let ascension_of_jesus_christ = datetime!(2024-05-09 12:00:00 UTC);
        let ascension_of_jesus_christ_2 = datetime!(2024-05-10 12:00:00 UTC);
        let vesak_day = datetime!(2024-05-24 12:00:00 UTC);
        let pancasila_day = datetime!(2024-06-01 12:00:00 UTC);
        let eid_ul_adha = datetime!(2024-06-17 12:00:00 UTC);
        let eid_ul_adha_2 = datetime!(2024-06-18 12:00:00 UTC);
        let muharram = datetime!(2024-07-07 12:00:00 UTC);
        let independence_day = datetime!(2024-08-17 12:00:00 UTC);
        let birth_of_prophet_muhammad = datetime!(2024-09-15 12:00:00 UTC);
        let christmas = datetime!(2024-12-25 12:00:00 UTC);
        let boxing_day = datetime!(2024-12-26 12:00:00 UTC);

        assert!(!calendar.is_business_day(new_years_day));
        assert!(!calendar.is_business_day(ascension_of_the_prophet_muhammad));
        assert!(!calendar.is_business_day(lunar_new_year));
        assert!(!calendar.is_business_day(hindu_new_year));
        assert!(!calendar.is_business_day(hindu_new_year_2));
        assert!(!calendar.is_business_day(eid_ul_fitar));
        assert!(!calendar.is_business_day(eid_ul_fitar_2));
        assert!(!calendar.is_business_day(labor_day));
        assert!(!calendar.is_business_day(ascension_of_jesus_christ));
        assert!(!calendar.is_business_day(ascension_of_jesus_christ_2));
        assert!(!calendar.is_business_day(vesak_day));
        assert!(!calendar.is_business_day(pancasila_day));
        assert!(!calendar.is_business_day(eid_ul_adha));
        assert!(!calendar.is_business_day(eid_ul_adha_2));
        assert!(!calendar.is_business_day(muharram));
        assert!(!calendar.is_business_day(independence_day));
        assert!(!calendar.is_business_day(birth_of_prophet_muhammad));
        assert!(!calendar.is_business_day(christmas));
        assert!(!calendar.is_business_day(boxing_day));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = Indonesia;
        let regular_day1 = datetime!(2024-06-19 12:00:00 UTC);
        let regular_day2 = datetime!(2024-07-03 12:00:00 UTC);
        let regular_day3 = datetime!(2024-11-07 12:00:00 UTC);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
