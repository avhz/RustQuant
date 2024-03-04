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
use time::{Date, Month};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// China national holiday calendar.
pub struct ChinaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for ChinaCalendar {
    fn name(&self) -> &'static str {
        "China"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::CHINA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XSHG
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, _wd, _yd, _em) = unpack_date(date, false);

        if (
            // New Year's Day
            (d == 1 && m == Month::January)
            || (y == 2005 && d == 3 && m == Month::January)
            || (y == 2006 && (d == 2 || d == 3) && m == Month::January)
            || (y == 2007 && d <= 3 && m == Month::January)
            || (y == 2007 && d == 31 && m == Month::December)
            || (y == 2009 && d == 2 && m == Month::January)
            || (y == 2011 && d == 3 && m == Month::January)
            || (y == 2012 && (d == 2 || d == 3) && m == Month::January)
            || (y == 2013 && d <= 3 && m == Month::January)
            || (y == 2014 && d == 1 && m == Month::January)
            || (y == 2015 && d <= 3 && m == Month::January)
            || (y == 2017 && d == 2 && m == Month::January)
            || (y == 2018 && d == 1 && m == Month::January)
            || (y == 2018 && d == 31 && m == Month::December)
            || (y == 2019 && d == 1 && m == Month::January)
            || (y == 2020 && d == 1 && m == Month::January)
            || (y == 2021 && d == 1 && m == Month::January)
            || (y == 2022 && d == 3 && m == Month::January)
            || (y == 2023 && d == 2 && m == Month::January)

            // Chinese New Year
            || (y == 2004 && (19..=28).contains(&d) && m == Month::January)
            || (y == 2005 && (7..=15).contains(&d) && m == Month::February)
            || (y == 2006 && ((d >= 26 && m == Month::January) ||
                            (d <= 3 && m == Month::February)))
            || (y == 2007 && (17..=25).contains(&d) && m == Month::February)
            || (y == 2008 && d >= 6 && d <= 12 && m == Month::February)
            || (y == 2009 && d >= 26 && d <= 30 && m == Month::January)
            || (y == 2010 && d >= 15 && d <= 19 && m == Month::February)
            || (y == 2011 && d >= 2 && d <= 8 && m == Month::February)
            || (y == 2012 && d >= 23 && d <= 28 && m == Month::January)
            || (y == 2013 && d >= 11 && d <= 15 && m == Month::February)
            || (y == 2014 && d >= 31 && m == Month::January)
            || (y == 2014 && d <= 6 && m == Month::February)
            || (y == 2015 && d >= 18 && d <= 24 && m == Month::February)
            || (y == 2016 && d >= 8 && d <= 12 && m == Month::February)
            || (y == 2017 && ((d >= 27 && m == Month::January) || (d <= 2 && m == Month::February)))
            || (y == 2018 && (d >= 15 && d <= 21 && m == Month::February))
            || (y == 2019 && d >= 4 && d <= 8 && m == Month::February)
            || (y == 2020 && (d == 24 || (d >= 27 && d <= 31)) && m == Month::January)
            || (y == 2021 && (d == 11 || d == 12 || d == 15 || d == 16 || d == 17) && m == Month::February)
            || (y == 2022 && ((d == 31 && m == Month::January) || (d <= 4 && m == Month::February)))
            || (y == 2023 && d >= 23 && d <= 27 && m == Month::January)

            // Ching Ming Festival
            || (y <= 2008 && d == 4 && m == Month::April)
            || (y == 2009 && d == 6 && m == Month::April)
            || (y == 2010 && d == 5 && m == Month::April)
            || (y == 2011 && d >=3 && d <= 5 && m == Month::April)
            || (y == 2012 && d >= 2 && d <= 4 && m == Month::April)
            || (y == 2013 && d >= 4 && d <= 5 && m == Month::April)
            || (y == 2014 && d == 7 && m == Month::April)
            || (y == 2015 && d >= 5 && d <= 6 && m == Month::April)
            || (y == 2016 && d == 4 && m == Month::April)
            || (y == 2017 && d >= 3 && d <= 4 && m == Month::April)
            || (y == 2018 && d >= 5 && d <= 6 && m == Month::April)
            || (y == 2019 && d == 5 && m == Month::April)
            || (y == 2020 && d == 6 && m == Month::April)
            || (y == 2021 && d == 5 && m == Month::April)
            || (y == 2022 && d >= 4 && d <= 5 && m == Month::April)
            || (y == 2023 && d == 5 && m == Month::April)

            // Labor Day
            || (y <= 2007 && d >= 1 && d <= 7 && m == Month::May)
            || (y == 2008 && d >= 1 && d <= 2 && m == Month::May)
            || (y == 2009 && d == 1 && m == Month::May)
            || (y == 2010 && d == 3 && m == Month::May)
            || (y == 2011 && d == 2 && m == Month::May)
            || (y == 2012 && ((d == 30 && m == Month::April) || (d == 1 && m == Month::May)))
            || (y == 2013 && ((d >= 29 && m == Month::April) || (d == 1 && m == Month::May)))
            || (y == 2014 && d >= 1 && d <=3 && m == Month::May)
            || (y == 2015 && d == 1 && m == Month::May)
            || (y == 2016 && d >= 1 && d <=2 && m == Month::May)
            || (y == 2017 && d == 1 && m == Month::May)
            || (y == 2018 && ((d == 30 && m == Month::April) || (d == 1 && m == Month::May)))
            || (y == 2019 && d >= 1 && d <=3 && m == Month::May)
            || (y == 2020 && (d == 1 || d == 4 || d == 5) && m == Month::May)
            || (y == 2021 && (d == 3 || d == 4 || d == 5) && m == Month::May)
            || (y == 2022 && d >= 2 && d <= 4 && m == Month::May)
            || (y == 2023 && d >= 1 && d <= 3 && m == Month::May)

            // Tuen Ng Festival
            || (y <= 2008 && d == 9 && m == Month::June)
            || (y == 2009 && (d == 28 || d == 29) && m == Month::May)
            || (y == 2010 && d >= 14 && d <= 16 && m == Month::June)
            || (y == 2011 && d >= 4 && d <= 6 && m == Month::June)
            || (y == 2012 && d >= 22 && d <= 24 && m == Month::June)
            || (y == 2013 && d >= 10 && d <= 12 && m == Month::June)
            || (y == 2014 && d == 2 && m == Month::June)
            || (y == 2015 && d == 22 && m == Month::June)
            || (y == 2016 && d >= 9 && d <= 10 && m == Month::June)
            || (y == 2017 && d >= 29 && d <= 30 && m == Month::May)
            || (y == 2018 && d == 18 && m == Month::June)
            || (y == 2019 && d == 7 && m == Month::June)
            || (y == 2020 && d >= 25 && d <= 26 && m == Month::June)
            || (y == 2021 && d == 14 && m == Month::June)
            || (y == 2022 && d == 3 && m == Month::June)
            || (y == 2023 && d >= 22 && d <= 23 && m == Month::June)

            // Mid-Autumn Festival
            || (y <= 2008 && d == 15 && m == Month::September)
            || (y == 2010 && d >= 22 && d <= 24 && m == Month::September)
            || (y == 2011 && d >= 10 && d <= 12 && m == Month::September)
            || (y == 2012 && d == 30 && m == Month::September)
            || (y == 2013 && d >= 19 && d <= 20 && m == Month::September)
            || (y == 2014 && d == 8 && m == Month::September)
            || (y == 2015 && d == 27 && m == Month::September)
            || (y == 2016 && d >= 15 && d <= 16 && m == Month::September)
            || (y == 2018 && d == 24 && m == Month::September)
            || (y == 2019 && d == 13 && m == Month::September)
            || (y == 2021 && (d == 20 || d == 21) && m == Month::September)
            || (y == 2022 && d == 12 && m == Month::September)
            || (y == 2023 && d == 29 && m == Month::September)

            // National Day
            || (y <= 2007 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2008 && ((d >= 29 && m == Month::September) || (d <= 3 && m == Month::October)))
            || (y == 2009 && (1..=8).contains(&d) && m == Month::October)
            || (y == 2010 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2011 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2012 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2013 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2014 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2015 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2016 && (3..=7).contains(&d) && m == Month::October)
            || (y == 2017 && (2..=6).contains(&d) && m == Month::October)
            || (y == 2018 && (1..=5).contains(&d) && m == Month::October)
            || (y == 2019 && (1..=7).contains(&d) && m == Month::October)
            || (y == 2020 && (1..=2).contains(&d) && m == Month::October)
            || (y == 2020 && (5..=8).contains(&d) && m == Month::October)
            || (y == 2021 && (d == 1 || d == 4 || d == 5 || d == 6 || d == 7) && m == Month::October)
            || (y == 2022 && (3..=7).contains(&d) && m == Month::October)
            || (y == 2023 && (2..=6).contains(&d) && m == Month::October)
            
            // 70th anniversary of the victory of anti-Japanese war
            || (y == 2015 && (3..=4).contains(&d) && m == Month::September)
        ) {
            return true;
        }

        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
