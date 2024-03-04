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
pub struct HongKongCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for HongKongCalendar {
    fn name(&self) -> &'static str {
        "Hong Kong"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::HONG_KONG
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XHKG
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day
            ((d == 1 || ((d == 2) && wd == Weekday::Monday)) && m == Month::January)
                // Good Friday
                || (yd == em-3)
                // Easter Monday
                || (yd == em)
                // Labor Day
                || ((d == 1 || ((d == 2) && wd == Weekday::Monday)) && m == Month::May)
                // SAR Establishment Day
                || ((d == 1 || ((d == 2) && wd == Weekday::Monday)) && m == Month::July)
                // National Day
                || ((d == 1 || ((d == 2) && wd == Weekday::Monday)) && m == Month::October)
                // Christmas Day
                || (d == 25 && m == Month::December)
                // Boxing Day
                || (d == 26 && m == Month::December)
                // Lunar New Year
                || self.is_lunar_new_year(y, m, d)
                // Buddha's birthday
                || self.is_buddhas_birthday(y, m,d)
                // Ching Ming Festival
                || self.is_ching_ming_festival(y, m,d)
                // Tuen Ng festival
                || self.is_tuen_ng_festival(y, m,d)
                // Mid-autumn festival
                || self.is_mid_autumn_festival(y, m,d)
                // Chung Yeung festival
                || self.is_chung_yeung_festival(y, m,d)
                // Second day after Christmas
                || self.is_second_day_after_christmas(y, m,d)
        ) {
            return true;
        }

        false
    }
}

impl Default for HongKongCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl HongKongCalendar {
    /// Create a new instance of the Hong Kong calendar.
    pub fn new() -> Self {
        Self
    }

    #[allow(overlapping_range_endpoints)]
    fn is_lunar_new_year(&self, year: i32, month: Month, day: u8) -> bool {
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

    fn is_buddhas_birthday(&self, year: i32, month: Month, day: u8) -> bool {
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

    fn is_ching_ming_festival(&self, year: i32, month: Month, day: u8) -> bool {
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

    fn is_tuen_ng_festival(&self, year: i32, month: Month, day: u8) -> bool {
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

    fn is_mid_autumn_festival(&self, year: i32, month: Month, day: u8) -> bool {
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

    fn is_chung_yeung_festival(&self, year: i32, month: Month, day: u8) -> bool {
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

    fn is_second_day_after_christmas(&self, year: i32, month: Month, day: u8) -> bool {
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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
