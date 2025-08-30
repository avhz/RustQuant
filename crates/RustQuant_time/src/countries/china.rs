// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::unpack_date;
use time::{Date, Month};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) fn is_holiday_impl_china(date: Date) -> bool {
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
                || (y == 2008 && (6..=12).contains(&d) && m == Month::February)
                || (y == 2009 && (26..=30).contains(&d) && m == Month::January)
                || (y == 2010 && (15..=19).contains(&d) && m == Month::February)
                || (y == 2011 && (2..=8).contains(&d) && m == Month::February)
                || (y == 2012 && (23..=28).contains(&d) && m == Month::January)
                || (y == 2013 && (11..=15).contains(&d) && m == Month::February)
                || (y == 2014 && d >= 31 && m == Month::January)
                || (y == 2014 && d <= 6 && m == Month::February)
                || (y == 2015 && (18..=24).contains(&d) && m == Month::February)
                || (y == 2016 && (8..=12).contains(&d) && m == Month::February)
                || (y == 2017 && ((d >= 27 && m == Month::January) || (d <= 2 && m == Month::February)))
                || (y == 2018 && ((15..=21).contains(&d) && m == Month::February))
                || (y == 2019 && (4..=8).contains(&d) && m == Month::February)
                || (y == 2020 && (d == 24 || (27..=31).contains(&d)) && m == Month::January)
                || (y == 2021 && (d == 11 || d == 12 || d == 15 || d == 16 || d == 17) && m == Month::February)
                || (y == 2022 && ((d == 31 && m == Month::January) || (d <= 4 && m == Month::February)))
                || (y == 2023 && (23..=27).contains(&d) && m == Month::January)

                // Ching Ming Festival
                || (y <= 2008 && d == 4 && m == Month::April)
                || (y == 2009 && d == 6 && m == Month::April)
                || (y == 2010 && d == 5 && m == Month::April)
                || (y == 2011 && (3..=5).contains(&d) && m == Month::April)
                || (y == 2012 && (2..=4).contains(&d) && m == Month::April)
                || (y == 2013 && (4..=5).contains(&d) && m == Month::April)
                || (y == 2014 && d == 7 && m == Month::April)
                || (y == 2015 && (5..=6).contains(&d) && m == Month::April)
                || (y == 2016 && d == 4 && m == Month::April)
                || (y == 2017 && (3..=4).contains(&d) && m == Month::April)
                || (y == 2018 && (5..=6).contains(&d) && m == Month::April)
                || (y == 2019 && d == 5 && m == Month::April)
                || (y == 2020 && d == 6 && m == Month::April)
                || (y == 2021 && d == 5 && m == Month::April)
                || (y == 2022 && (4..=5).contains(&d) && m == Month::April)
                || (y == 2023 && d == 5 && m == Month::April)

                // Labor Day
                || (y <= 2007 && (1..=7).contains(&d) && m == Month::May)
                || (y == 2008 && (1..=2).contains(&d) && m == Month::May)
                || (y == 2009 && d == 1 && m == Month::May)
                || (y == 2010 && d == 3 && m == Month::May)
                || (y == 2011 && d == 2 && m == Month::May)
                || (y == 2012 && ((d == 30 && m == Month::April) || (d == 1 && m == Month::May)))
                || (y == 2013 && ((d >= 29 && m == Month::April) || (d == 1 && m == Month::May)))
                || (y == 2014 && (1..=3).contains(&d) && m == Month::May)
                || (y == 2015 && d == 1 && m == Month::May)
                || (y == 2016 && (1..=2).contains(&d) && m == Month::May)
                || (y == 2017 && d == 1 && m == Month::May)
                || (y == 2018 && ((d == 30 && m == Month::April) || (d == 1 && m == Month::May)))
                || (y == 2019 && (1..=3).contains(&d) && m == Month::May)
                || (y == 2020 && (d == 1 || d == 4 || d == 5) && m == Month::May)
                || (y == 2021 && (d == 3 || d == 4 || d == 5) && m == Month::May)
                || (y == 2022 && (2..=4).contains(&d) && m == Month::May)
                || (y == 2023 && (1..=3).contains(&d) && m == Month::May)

                // Tuen Ng Festival
                || (y <= 2008 && d == 9 && m == Month::June)
                || (y == 2009 && (d == 28 || d == 29) && m == Month::May)
                || (y == 2010 && (14..=16).contains(&d) && m == Month::June)
                || (y == 2011 && (4..=6).contains(&d) && m == Month::June)
                || (y == 2012 && (22..=24).contains(&d) && m == Month::June)
                || (y == 2013 && (10..=12).contains(&d) && m == Month::June)
                || (y == 2014 && d == 2 && m == Month::June)
                || (y == 2015 && d == 22 && m == Month::June)
                || (y == 2016 && (9..=10).contains(&d) && m == Month::June)
                || (y == 2017 && (29..=30).contains(&d) && m == Month::May)
                || (y == 2018 && d == 18 && m == Month::June)
                || (y == 2019 && d == 7 && m == Month::June)
                || (y == 2020 && (25..=26).contains(&d) && m == Month::June)
                || (y == 2021 && d == 14 && m == Month::June)
                || (y == 2022 && d == 3 && m == Month::June)
                || (y == 2023 && (22..=23).contains(&d) && m == Month::June)

                // Mid-Autumn Festival
                || (y <= 2008 && d == 15 && m == Month::September)
                || (y == 2010 && (22..=24).contains(&d) && m == Month::September)
                || (y == 2011 && (10..=12).contains(&d) && m == Month::September)
                || (y == 2012 && d == 30 && m == Month::September)
                || (y == 2013 && (19..=20).contains(&d) && m == Month::September)
                || (y == 2014 && d == 8 && m == Month::September)
                || (y == 2015 && d == 27 && m == Month::September)
                || (y == 2016 && (15..=16).contains(&d) && m == Month::September)
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
