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

/// India national holiday calendar.
pub struct IndiaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for IndiaCalendar {
    fn name(&self) -> &'static str {
        "India"
    }

    fn country_code(&self) -> crate::iso::ISO_3166 {
        crate::iso::INDIA
    }

    fn market_identifier_code(&self) -> crate::iso::ISO_10383 {
        crate::iso::XBOM
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, _wd, yd, em) = unpack_date(date, false);

        if (
            // Republic Day
            (d == 26 && m == Month::January)
            // Mahashivratri
            || self.is_mahashivratri(y, d, m)
            // Holi
            || self.is_holi(y, d, m)
            // Good Friday
            || (yd == em - 3)
            // Eid-ul-Fitar
            || self.is_eid_ul_fitar(y,d,m)
            // Rama Navami
            || self.is_rama_navami(y,d,m)
            // Mahavir Jayanti
            || self.is_mahavir_jayanti(y,d,m)
            // Maharashtra Day
            || (d == 1 && m == Month::May)
            // Bakri Id
            || self.is_bakri_id(y,d,m)
            // Muharram
            || self.is_muharram(y,d,m)
            // Independence Day
            || (d == 15 && m == Month::August)
            // Gandhi Jayanti
            || (d == 2 && m == Month::October)
            // Dussehra
            || self.is_dussehra(y,d,m)
            // Diwali
            || self.is_diwali(y,d,m)
            // Gurunanak Jayanti
            || self.is_gurunanak_jayanti(y,d,m)
            // Christmas
            || (d == 25 && m == Month::December)
        ) {
            return true;
        }

        false
    }
}

impl IndiaCalendar {
    fn is_mahashivratri(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::{February, March};

        matches!(
            (year, day, month),
            (2000, 4, March)
                | (2001, 21, February)
                | (2002, 12, March)
                | (2003, 1, March)
                | (2004, 18, February)
                | (2005, 8, March)
                | (2006, 26, February)
                | (2007, 16, February)
                | (2008, 6, March)
                | (2009, 23, February)
                | (2010, 12, February)
                | (2011, 2, March)
                | (2012, 20, February)
                | (2013, 10, March)
                | (2014, 27, February)
                | (2015, 17, February)
                | (2016, 7, March)
                | (2017, 24, February)
                | (2018, 13, February)
                | (2019, 4, March)
                | (2020, 21, February)
                | (2021, 11, March)
                | (2022, 1, March)
                | (2023, 18, February)
                | (2024, 8, March)
                | (2025, 26, February)
                | (2026, 15, February)
                | (2027, 6, March)
                | (2028, 23, February)
                | (2029, 11, February)
        )
    }

    fn is_holi(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::March;
        matches!(
            (year, day, month),
            (2000, 20, March)
                | (2001, 10, March)
                | (2002, 29, March)
                | (2003, 18, March)
                | (2004, 7, March)
                | (2005, 26, March)
                | (2006, 15, March)
                | (2007, 4, March)
                | (2008, 22, March)
                | (2009, 11, March)
                | (2010, 1, March)
                | (2011, 20, March)
                | (2012, 8, March)
                | (2013, 27, March)
                | (2014, 17, March)
                | (2015, 6, March)
                | (2016, 24, March)
                | (2017, 13, March)
                | (2018, 2, March)
                | (2019, 21, March)
                | (2020, 10, March)
                | (2021, 29, March)
                | (2022, 18, March)
                | (2023, 8, March)
                | (2024, 25, March)
                | (2025, 14, March)
                | (2026, 4, March)
                | (2027, 22, March)
                | (2028, 11, March)
                | (2029, 1, March)
        )
    }

    fn is_eid_ul_fitar(&self, year: i32, day: u8, month: Month) -> bool {
        matches!(
            (year, day, month),
            (2000, 8, Month::January)
                | (2000, 28, Month::December)
                | (2001, 17, Month::December)
                | (2002, 6, Month::December)
                | (2003, 26, Month::November)
                | (2004, 14, Month::November)
                | (2005, 4, Month::November)
                | (2006, 24, Month::October)
                | (2007, 13, Month::October)
                | (2008, 2, Month::October)
                | (2009, 21, Month::September)
                | (2010, 10, Month::September)
                | (2011, 31, Month::August)
                | (2012, 20, Month::August)
                | (2013, 9, Month::August)
                | (2014, 29, Month::July)
                | (2015, 19, Month::July)
                | (2016, 6, Month::July)
                | (2017, 26, Month::June)
                | (2018, 15, Month::June)
                | (2019, 5, Month::June)
                | (2020, 25, Month::May)
                | (2021, 14, Month::May)
                | (2022, 3, Month::May)
                | (2023, 22, Month::April)
                | (2024, 11, Month::April)
                | (2025, 31, Month::March)
                | (2026, 21, Month::March)
                | (2027, 10, Month::March)
                | (2028, 27, Month::February)
                | (2029, 15, Month::February)
        )
    }

    fn is_rama_navami(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::{April, March};
        matches!(
            (year, day, month),
            (2000, 12, April)
                | (2001, 2, April)
                | (2002, 21, April)
                | (2003, 11, April)
                | (2004, 30, March)
                | (2005, 18, April)
                | (2006, 6, April)
                | (2007, 26, March)
                | (2008, 13, April)
                | (2009, 3, April)
                | (2010, 24, March)
                | (2011, 12, April)
                | (2012, 1, April)
                | (2013, 19, April)
                | (2014, 8, April)
                | (2015, 28, March)
                | (2016, 15, April)
                | (2017, 4, April)
                | (2018, 25, March)
                | (2019, 13, April)
                | (2020, 2, April)
                | (2021, 21, April)
                | (2022, 10, April)
                | (2023, 30, March)
                | (2024, 17, April)
                | (2025, 6, April)
                | (2026, 26, March)
                | (2027, 15, April)
                | (2028, 3, April)
                | (2029, 22, April)
        )
    }

    fn is_mahavir_jayanti(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::{April, March};
        matches!(
            (year, day, month),
            (2005, 22, April)
                | (2006, 11, April)
                | (2007, 31, March)
                | (2008, 18, April)
                | (2009, 7, April)
                | (2010, 28, April)
                | (2011, 16, April)
                | (2012, 5, April)
                | (2013, 24, April)
                | (2014, 13, April)
                | (2015, 2, April)
                | (2016, 20, April)
                | (2017, 9, April)
                | (2018, 29, March)
                | (2019, 17, April)
                | (2020, 6, April)
                | (2021, 25, April)
                | (2022, 14, April)
                | (2023, 4, April)
                | (2024, 21, April)
                | (2025, 10, April)
        )
    }

    fn is_bakri_id(&self, year: i32, day: u8, month: Month) -> bool {
        matches!(
            (year, day, month),
            (2000, 16, Month::March)
                | (2001, 6, Month::March)
                | (2002, 23, Month::February)
                | (2003, 12, Month::February)
                | (2004, 2, Month::February)
                | (2005, 21, Month::January)
                | (2006, 11, Month::January)
                | (2007, 20, Month::December)
                | (2008, 9, Month::December)
                | (2009, 28, Month::November)
                | (2010, 17, Month::November)
                | (2011, 7, Month::November)
                | (2012, 27, Month::October)
                | (2013, 16, Month::October)
                | (2014, 6, Month::October)
                | (2015, 25, Month::September)
                | (2016, 13, Month::September)
                | (2017, 2, Month::September)
                | (2018, 22, Month::August)
                | (2019, 12, Month::August)
                | (2020, 1, Month::August)
                | (2021, 21, Month::July)
                | (2022, 10, Month::July)
                | (2023, 29, Month::June)
                | (2024, 17, Month::June)
                | (2025, 7, Month::June)
                | (2026, 28, Month::May)
                | (2027, 17, Month::May)
                | (2028, 6, Month::May)
                | (2029, 25, Month::April)
        )
    }

    fn is_muharram(&self, year: i32, day: u8, month: Month) -> bool {
        matches!(
            (year, day, month),
            (2007, 30, Month::January)
                | (2009, 28, Month::December)
                | (2010, 17, Month::December)
                | (2011, 6, Month::December)
                | (2012, 25, Month::November)
                | (2013, 14, Month::November)
                | (2014, 4, Month::November)
                | (2015, 24, Month::October)
                | (2016, 12, Month::October)
                | (2017, 1, Month::October)
                | (2018, 21, Month::September)
                | (2019, 10, Month::September)
                | (2020, 30, Month::August)
                | (2021, 20, Month::August)
                | (2022, 9, Month::August)
                | (2023, 29, Month::July)
                | (2024, 17, Month::July)
                | (2025, 6, Month::July)
                | (2026, 26, Month::June)
                | (2027, 16, Month::June)
                | (2028, 4, Month::June)
                | (2029, 25, Month::May)
        )
    }

    fn is_dussehra(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::{October, September};
        matches!(
            (year, day, month),
            (2000, 7, October)
                | (2001, 26, October)
                | (2002, 15, October)
                | (2003, 5, October)
                | (2004, 22, October)
                | (2005, 12, October)
                | (2006, 2, October)
                | (2007, 21, October)
                | (2008, 9, October)
                | (2009, 28, September)
                | (2010, 17, October)
                | (2011, 6, October)
                | (2012, 24, October)
                | (2013, 13, October)
                | (2014, 3, October)
                | (2015, 22, October)
                | (2016, 11, October)
                | (2017, 30, September)
                | (2018, 19, October)
                | (2019, 8, October)
                | (2020, 25, October)
                | (2021, 15, October)
                | (2022, 5, October)
                | (2023, 24, October)
                | (2024, 12, October)
                | (2025, 2, October)
                | (2026, 20, October)
                | (2027, 9, October)
                | (2028, 27, September)
                | (2029, 16, October)
        )
    }

    fn is_diwali(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::{November, October};
        matches!(
            (year, day, month),
            (2000, 26, October)
                | (2001, 14, November)
                | (2002, 4, November)
                | (2003, 25, October)
                | (2004, 12, November)
                | (2005, 1, November)
                | (2006, 21, October)
                | (2007, 9, November)
                | (2008, 28, October)
                | (2009, 17, October)
                | (2010, 5, November)
                | (2011, 26, October)
                | (2012, 13, November)
                | (2013, 3, November)
                | (2014, 23, October)
                | (2015, 11, November)
                | (2016, 30, October)
                | (2017, 19, October)
                | (2018, 7, November)
                | (2019, 27, October)
                | (2020, 14, November)
                | (2021, 4, November)
                | (2022, 24, October)
                | (2023, 12, November)
                | (2024, 31, October)
                | (2025, 20, October)
                | (2026, 8, November)
                | (2027, 29, October)
                | (2028, 17, October)
                | (2029, 5, November)
        )
    }

    fn is_gurunanak_jayanti(&self, year: i32, day: u8, month: Month) -> bool {
        use Month::November;
        matches!(
            (year, day, month),
            (2005, 15, November)
                | (2006, 5, November)
                | (2007, 24, November)
                | (2008, 13, November)
                | (2009, 2, November)
                | (2010, 21, November)
                | (2011, 10, November)
                | (2012, 28, November)
                | (2013, 17, November)
                | (2014, 6, November)
                | (2015, 25, November)
                | (2016, 14, November)
                | (2017, 4, November)
                | (2018, 23, November)
                | (2019, 12, November)
                | (2020, 30, November)
                | (2021, 19, November)
                | (2022, 8, November)
                | (2023, 27, November)
                | (2024, 15, November)
                | (2025, 5, November)
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_india {
    use super::*;
    use time::macros::date;

    // Test to verify the name() method.
    #[test]
    fn test_name() {
        let calendar = IndiaCalendar;
        assert_eq!(calendar.name(), "India");
    }

    // Test to verify if weekends are not considered business days.
    #[test]
    fn test_is_weekend() {
        let calendar = IndiaCalendar;
        let sat = date!(2024 - 03 - 09);
        let sun = date!(2024 - 03 - 10);
        assert!(!calendar.is_business_day(sat));
        assert!(!calendar.is_business_day(sun));
    }

    // Test to verify if the is_business_day() method properly accounts for public holidays.
    #[test]
    fn test_is_public_holiday() {
        let calendar = IndiaCalendar;
        let republic_day = date!(2024 - 01 - 26);
        let mahashivratri = date!(2024 - 03 - 08);
        let holi = date!(2024 - 03 - 25);
        let good_friday = date!(2024 - 03 - 29);
        let eid_ul_fitar = date!(2024 - 04 - 11);
        let rama_navami = date!(2024 - 04 - 17);
        let mahavir_jayanti = date!(2024 - 04 - 21);
        let maharashtra_day = date!(2024 - 05 - 01);
        let bakri_id = date!(2024 - 06 - 17);
        let muharram = date!(2024 - 07 - 17);
        let independence_day = date!(2024 - 08 - 15);
        let gandhi_jayanti = date!(2024 - 10 - 02);
        let dussehra = date!(2023 - 10 - 24);
        let diwali = date!(2024 - 10 - 31);
        let gurunanak_jayanti = date!(2024 - 11 - 15);
        let christmas = date!(2024 - 12 - 25);

        assert!(!calendar.is_business_day(republic_day));
        assert!(!calendar.is_business_day(mahashivratri));
        assert!(!calendar.is_business_day(holi));
        assert!(!calendar.is_business_day(good_friday));
        assert!(!calendar.is_business_day(eid_ul_fitar));
        assert!(!calendar.is_business_day(rama_navami));
        assert!(!calendar.is_business_day(mahavir_jayanti));
        assert!(!calendar.is_business_day(maharashtra_day));
        assert!(!calendar.is_business_day(bakri_id));
        assert!(!calendar.is_business_day(muharram));
        assert!(!calendar.is_business_day(independence_day));
        assert!(!calendar.is_business_day(gandhi_jayanti));
        assert!(!calendar.is_business_day(dussehra));
        assert!(!calendar.is_business_day(diwali));
        assert!(!calendar.is_business_day(gurunanak_jayanti));
        assert!(!calendar.is_business_day(christmas));
    }

    // Test to verify if the is_business_day() method properly accounts for regular business days.
    #[test]
    fn test_is_regular_business_day() {
        let calendar = IndiaCalendar;
        let regular_day1 = date!(2024 - 03 - 22);
        let regular_day2 = date!(2024 - 10 - 30);
        let regular_day3 = date!(2024 - 12 - 09);

        assert!(calendar.is_business_day(regular_day1));
        assert!(calendar.is_business_day(regular_day2));
        assert!(calendar.is_business_day(regular_day3));
    }
}
