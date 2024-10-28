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

use crate::time::calendar::Calendar;
use crate::time::utilities::unpack_date;
use time::{Date, Month, Weekday};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// HongKong national holiday calendar.
pub struct SingaporeCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for SingaporeCalendar {
    fn name(&self) -> &'static str {
        "Singapore"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::SINGAPORE
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XSES
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day
            ((d == 1 || ((d == 2) && wd == Weekday::Monday)) && m == Month::January)
                // Good Friday
                || (yd == em - 3)
                // Labor Day
                || ((d == 1 || ((d == 2) && wd == Weekday::Monday)) && m == Month::May)
                // National Day
                || ((d == 9 || ((d == 10) && wd == Weekday::Monday)) && m == Month::August)
                // Christmas Day
                || (d == 25 || ((d == 26) && wd == Weekday::Monday) && m == Month::December)
                // Chinese New Year
                || self.is_chinese_new_year(y, m, d)
                // Hari Raya Haji
                || self.is_hari_raya_haji(y, m, d)
                // Hari Raya Puasa
                || self.is_hari_raya_puasa(y, m, d)
                // Vesak Day
                || self.is_vesak_day(y, m, d)
                // Deepavali
                || self.is_deepavali(y, m, d)
        ) {
            return true;
        }

        false
    }
}

impl Default for SingaporeCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl SingaporeCalendar {
    /// Create a new instance of the Hong Kong calendar.
    pub fn new() -> Self {
        Self
    }

    #[allow(overlapping_range_endpoints)]
    fn is_chinese_new_year(&self, year: i32, month: Month, day: u8) -> bool {
        use Month::{February, January};

        matches!(
            (year, day, month),
            (2004, 22..=23, January)
                | (2005, 9..=10, February)
                | (2006, 29..=30, January)
                | (2007, 18..=19, February)
                | (2008, 7..=8, February)
                | (2009, 26..=27, January)
                | (2010, 14..=15, February)
                | (2011, 3..=4, February)
                | (2012, 23..=24, January)
                | (2013, 10..=12, February)
                | (2014, 31 | 1, January)
                | (2015, 19..=20, February)
                | (2016, 8..=9, February)
                | (2017, 28..=29, January)
                | (2018, 16..=17, February)
                | (2019, 5..=6, February)
                | (2020, 25..=27, January)
                | (2021, 12..=13, February)
                | (2022, 1..=2, February)
                | (2023, 22..=24, January)
                | (2024, 10..=12, February)
        )
    }

    #[allow(overlapping_range_endpoints)]
    fn is_hari_raya_haji(&self, year: i32, month: Month, day: u8) -> bool {
        use Month::*;

        matches!(
            (year, day, month),
            (2004, 1..=2, February)
                | (2005, 21, January)
                | (2006, 10, January)
                | (2007, 19, December)
                | (2008, 8, December)
                | (2009, 27, November)
                | (2010, 17, November)
                | (2011, 6..=7, November)
                | (2012, 26, October)
                | (2013, 15, October)
                | (2014, 5..=6, October)
                | (2015, 24, September)
                | (2016, 12, September)
                | (2017, 1, September)
                | (2018, 22, August)
                | (2019, 11..=12, August)
                | (2020, 31, July)
                | (2021, 20, July)
                | (2022, 10..=11, July)
                | (2023, 29, June)
                | (2024, 17, June)
        )
    }

    #[allow(overlapping_range_endpoints)]
    fn is_hari_raya_puasa(&self, year: i32, month: Month, day: u8) -> bool {
        use Month::*;

        matches!(
            (year, day, month),
            (2004, 14..=15, November)
                | (2005, 3, November)
                | (2006, 24, October)
                | (2007, 13, October)
                | (2008, 1, October)
                | (2009, 20..=21, September)
                | (2010, 10, September)
                | (2011, 30, August)
                | (2012, 19..=20, August)
                | (2013, 8, August)
                | (2014, 28, July)
                | (2015, 17, July)
                | (2016, 6, July)
                | (2017, 25..=26, June)
                | (2018, 15, June)
                | (2019, 5, June)
                | (2020, 24..=25, May)
                | (2021, 13, May)
                | (2022, 3, May)
                | (2023, 22, April)
                | (2024, 10, April)
        )
    }

    #[allow(overlapping_range_endpoints)]
    fn is_vesak_day(&self, year: i32, month: Month, day: u8) -> bool {
        use Month::*;

        matches!(
            (year, day, month),
            (2004, 2, June)
                | (2005, 22..=23, October)
                | (2006, 12, May)
                | (2007, 31, May)
                | (2008, 19, May)
                | (2009, 9, May)
                | (2010, 28, May)
                | (2011, 17, May)
                | (2012, 5, May)
                | (2013, 24, May)
                | (2014, 13, May)
                | (2015, 1, June)
                | (2016, 21, May)
                | (2017, 10, May)
                | (2018, 29, May)
                | (2019, 19..=20, May)
                | (2020, 7, May)
                | (2021, 26, May)
                | (2022, 15..=16, May)
                | (2023, 2, June)
                | (2024, 22, May)
        )
    }

    #[allow(overlapping_range_endpoints)]
    fn is_deepavali(&self, year: i32, month: Month, day: u8) -> bool {
        use Month::*;

        matches!(
            (year, day, month),
            (2004, 11, November)
                | (2005, 1, November)
                | (2006, 21, October)
                | (2007, 8, November)
                | (2008, 27, October)
                | (2009, 15..=16, November)
                | (2010, 5, November)
                | (2011, 26, October)
                | (2012, 13, November)
                | (2013, 2, November)
                | (2014, 22, October)
                | (2015, 10, November)
                | (2016, 29, October)
                | (2017, 18, October)
                | (2018, 6, November)
                | (2019, 27..=28, October)
                | (2020, 14, November)
                | (2021, 4, November)
                | (2022, 24, October)
                | (2023, 12..=13, November)
                | (2024, 31, October)
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_singapore {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_name() {
        let calendar = SingaporeCalendar;
        assert_eq!(calendar.name(), "Singapore");
    }

    #[test]
    fn test_is_regular_business_day() {
        let calendar = SingaporeCalendar::new();

        assert!(calendar.is_business_day(date!(2022 - 1 - 3)));
        assert!(calendar.is_business_day(date!(2022 - 4 - 14)));
        assert!(calendar.is_business_day(date!(2022 - 8 - 8)));
        assert!(calendar.is_business_day(date!(2022 - 5 - 17)));
        assert!(!calendar.is_business_day(date!(2022 - 7 - 11)));
        assert!(!calendar.is_business_day(date!(2022 - 2 - 2)));
        assert!(!calendar.is_business_day(date!(2022 - 5 - 3)));
        assert!(!calendar.is_business_day(date!(2022 - 12 - 24)));
    }

    #[test]
    fn test_is_holiday() {
        let calendar = SingaporeCalendar::new();

        assert!(calendar.is_holiday(date!(2022 - 1 - 1)));
        assert!(calendar.is_holiday(date!(2022 - 4 - 15)));
        assert!(calendar.is_holiday(date!(2022 - 5 - 1)));
        assert!(calendar.is_holiday(date!(2022 - 8 - 9)));
        assert!(calendar.is_holiday(date!(2022 - 12 - 25)));
        assert!(calendar.is_holiday(date!(2022 - 2 - 1)));
        assert!(calendar.is_holiday(date!(2022 - 7 - 10)));
        assert!(calendar.is_holiday(date!(2022 - 5 - 16)));
    }
}
