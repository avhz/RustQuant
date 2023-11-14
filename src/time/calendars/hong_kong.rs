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
        {
            return false;
        }

        if y == 2004 {
            if
            // Lunar New Year
            ((d==22 || d==23 || d==24) && m == Month::January)
                // Ching Ming Festival
                || (d == 5 && m == Month::April)
                // Buddha's birthday
                || (d == 26 && m == Month::May)
                // Tuen Ng festival
                || (d == 22 && m == Month::June)
                // Mid-autumn festival
                || (d == 29 && m == Month::September)
                // Chung Yeung
                || (d == 22 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2005 {
            if
            // Lunar New Year
            ((d==9 || d==10 || d==11) && m == Month::February)
                // Ching Ming Festival
                || (d == 5 && m == Month::April)
                // Buddha's birthday
                || (d == 16 && m == Month::May)
                // Tuen Ng festival
                || (d == 11 && m == Month::June)
                // Mid-autumn festival
                || (d == 19 && m == Month::September)
                // Chung Yeung festival
                || (d == 11 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2006 {
            if
            // Lunar New Year
            ((d >= 28 && d <= 31) && m == Month::January)
                // Ching Ming Festival
                || (d == 5 && m == Month::April)
                // Buddha's birthday
                || (d == 5 && m == Month::May)
                // Tuen Ng festival
                || (d == 31 && m == Month::May)
                // Mid-autumn festival
                || (d == 7 && m == Month::October)
                // Chung Yeung festival
                || (d == 30 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2007 {
            if
            // Lunar New Year
            ((d >= 17 && d <= 20) && m == Month::February)
                // Ching Ming Festival
                || (d == 5 && m == Month::April)
                // Buddha's birthday
                || (d == 24 && m == Month::May)
                // Tuen Ng festival
                || (d == 19 && m == Month::June)
                // Mid-autumn festival
                || (d == 26 && m == Month::September)
                // Chung Yeung festival
                || (d == 19 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2008 {
            if
            // Lunar New Year
            ((d >= 7 && d <= 9) && m == Month::February)
                // Ching Ming Festival
                || (d == 4 && m == Month::April)
                // Buddha's birthday
                || (d == 12 && m == Month::May)
                // Tuen Ng festival
                || (d == 9 && m == Month::June)
                // Mid-autumn festival
                || (d == 15 && m == Month::September)
                // Chung Yeung festival
                || (d == 7 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2009 {
            if ((d >= 26 && d <= 28) && m == Month::January) // Lunar New Year
                || (d == 4 && m == Month::April) // Ching Ming Festival
                || (d == 2 && m == Month::May) // Buddha's birthday
                || (d == 28 && m == Month::May) // Tuen Ng festival
                || (d == 3 && m == Month::October) // Mid-autumn festival
                || (d == 26 && m == Month::October)
            // Chung Yeung festival
            {
                return false;
            }
        }

        if y == 2010 {
            if
            // Lunar New Year
            ((d == 15 || d == 16) && m == Month::February)
                // Ching Ming Festival
                || (d == 6 && m == Month::April)
                // Buddha's birthday
                || (d == 21 && m == Month::May)
                // Tuen Ng festival
                || (d == 16 && m == Month::June)
                // Mid-autumn festival
                || (d == 23 && m == Month::September)
            {
                return false;
            }
        }

        // Lunar New Year
        // Ching Ming Festival
        // Buddha's birthday
        // Tuen Ng festival
        // Mid-autumn festival
        // Chung Yeung festival
        // Second day after Christmas
        if y == 2011 {
            if ((d == 3 || d == 4) && m == Month::February)
                || (d == 5 && m == Month::April)
                || (d == 10 && m == Month::May)
                || (d == 6 && m == Month::June)
                || (d == 13 && m == Month::September)
                || (d == 5 && m == Month::October)
                || (d == 27 && m == Month::December)
            {
                return false;
            }
        }

        if y == 2012 {
            if
            // Lunar New Year
            (d >= 23 && d <= 25 && m == Month::January)
                // Ching Ming Festival
                || (d == 4 && m == Month::April)
                // Buddha's birthday
                || (d == 10 && m == Month::May)
                // Mid-autumn festival
                || (d == 1 && m == Month::October)
                // Chung Yeung festival
                || (d == 23 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2013 {
            if
            // Lunar New Year
            (d >= 11 && d <= 13 && m == Month::February)
                // Ching Ming Festival
                || (d == 4 && m == Month::April)
                // Buddha's birthday
                || (d == 17 && m == Month::May)
                // Tuen Ng festival
                || (d == 12 && m == Month::June)
                // Mid-autumn festival
                || (d == 20 && m == Month::September)
                // Chung Yeung festival
                || (d == 14 && m == Month::October)
            {
                return false;
            }
        }

        if y == 2014 {
            if
            // Lunar New Year
            ((d == 31 && m == Month::January) || (d <= 3 && m == Month::February))
                // Buddha's birthday
                || (d == 6 && m == Month::May)
                // Tuen Ng festival
                || (d == 2 && m == Month::June)
                // Mid-autumn festival
                || (d == 9 && m == Month::September)
                // Chung Yeung festival
                || (d == 2 && m == Month::October)
            {
                return false;
            }
        }

        // Lunar New Year
        // The day following Easter Monday
        // Buddha's birthday
        // Tuen Ng festival
        // The 70th anniversary day of the victory of the Chinese
        // people's war of resistance against Japanese aggression
        // Mid-autumn festival
        // Chung Yeung festival
        if y == 2015 {
            if ((d == 19 && m == Month::February) || (d == 20 && m == Month::February))
                || (d == 7 && m == Month::April)
                || (d == 25 && m == Month::May)
                || (d == 20 && m == Month::June)
                || (d == 3 && m == Month::September)
                || (d == 28 && m == Month::September)
                || (d == 21 && m == Month::October)
            {
                return false;
            }
        }

        // Lunar New Year
        // Ching Ming Festival
        // Tuen Ng festival
        // Mid-autumn festival
        // Chung Yeung festival
        // Second day after Christmas
        if y == 2016 {
            if ((d >= 8 && d <= 10) && m == Month::February)
                || (d == 4 && m == Month::April)
                || (d == 9 && m == Month::June)
                || (d == 16 && m == Month::September)
                || (d == 10 && m == Month::October)
                || (d == 27 && m == Month::December)
            {
                return false;
            }
        }

        // Lunar New Year
        // Ching Ming Festival
        // Buddha's birthday
        // Tuen Ng festival
        // Mid-autumn festival
        if y == 2017 {
            if ((d == 30 || d == 31) && m == Month::January)
                || (d == 4 && m == Month::April)
                || (d == 3 && m == Month::May)
                || (d == 30 && m == Month::May)
                || (d == 5 && m == Month::October)
            {
                return false;
            }
        }

        // Lunar New Year
        // Ching Ming Festival
        // Buddha's birthday
        // Tuen Ng festival
        // Mid-autumn festival
        // Chung Yeung festival
        if y == 2018 {
            if ((d == 16 && m == Month::February) || (d == 19 && m == Month::February))
                || (d == 5 && m == Month::April)
                || (d == 22 && m == Month::May)
                || (d == 18 && m == Month::June)
                || (d == 25 && m == Month::September)
                || (d == 17 && m == Month::October)
            {
                return false;
            }
        }

        // Lunar New Year
        // Ching Ming Festival
        // Tuen Ng festival
        // Chung Yeung festival
        if y == 2019 {
            if ((d >= 5 && d <= 7) && m == Month::February)
                || (d == 5 && m == Month::April)
                || (d == 7 && m == Month::June)
                || (d == 7 && m == Month::October)
            {
                return false;
            }
        }

        // Lunar New Year
        // Ching Ming Festival
        // Buddha's birthday
        // Tuen Ng festival
        // Mid-autumn festival
        // Chung Yeung festival
        if y == 2020 {
            if ((d == 27 || d == 28) && m == Month::January)
                || (d == 4 && m == Month::April)
                || (d == 30 && m == Month::April)
                || (d == 25 && m == Month::June)
                || (d == 2 && m == Month::October)
                || (d == 26 && m == Month::October)
            {
                return false;
            }
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {}
