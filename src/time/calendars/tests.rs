// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod calendar_tests {
    use crate::time::*;
    use time::{macros::datetime, Month, OffsetDateTime, Weekday};

    const DATE: OffsetDateTime = datetime!(2023-11-14 0:00 UTC);

    const ARGENTINA: Argentina = Argentina;
    const AUSTRALIA: Australia = Australia;
    const AUSTRIA: Austria = Austria;
    const BOTSWANA: Botswana = Botswana;
    const BRAZIL: Brazil = Brazil;
    const CANADA: Canada = Canada;
    const CHILE: Chile = Chile;
    const CZECH_REPUBLIC: CzechRepublic = CzechRepublic;
    const DENMARK: Denmark = Denmark;
    const FINLAND: Finland = Finland;
    const FRANCE: France = France;
    const GERMANY: Germany = Germany;
    const HONG_KONG: HongKong = HongKong;
    const UNITED_KINGDOM: UnitedKingdom = UnitedKingdom;
    const UNITED_STATES: UnitedStates = UnitedStates;

    #[test]
    fn test_is_business_day() {
        assert!(ARGENTINA.is_business_day(DATE));
        assert!(AUSTRALIA.is_business_day(DATE));
        assert!(AUSTRIA.is_business_day(DATE));
        assert!(BOTSWANA.is_business_day(DATE));
        assert!(BRAZIL.is_business_day(DATE));
        assert!(CANADA.is_business_day(DATE));
        assert!(CHILE.is_business_day(DATE));
        assert!(CZECH_REPUBLIC.is_business_day(DATE));
        assert!(DENMARK.is_business_day(DATE));
        assert!(FINLAND.is_business_day(DATE));
        assert!(FRANCE.is_business_day(DATE));
        assert!(GERMANY.is_business_day(DATE));
        assert!(HONG_KONG.is_business_day(DATE));
        assert!(UNITED_KINGDOM.is_business_day(DATE));
        assert!(UNITED_STATES.is_business_day(DATE));
    }

    #[rustfmt::skip]
    #[test]
    fn test_country_code() {
        assert_eq!(ARGENTINA.country_code(), crate::iso::ARGENTINA);
        assert_eq!(AUSTRALIA.country_code(), crate::iso::AUSTRALIA);
        assert_eq!(AUSTRIA.country_code(), crate::iso::AUSTRIA);
        assert_eq!(BOTSWANA.country_code(), crate::iso::BOTSWANA);
        assert_eq!(BRAZIL.country_code(), crate::iso::BRAZIL);
        assert_eq!(CANADA.country_code(), crate::iso::CANADA);
        assert_eq!(CHILE.country_code(), crate::iso::CHILE);
        assert_eq!(CZECH_REPUBLIC.country_code(), crate::iso::CZECH_REPUBLIC);
        assert_eq!(DENMARK.country_code(), crate::iso::DENMARK);
        assert_eq!(FINLAND.country_code(), crate::iso::FINLAND);
        assert_eq!(FRANCE.country_code(), crate::iso::FRANCE);
        assert_eq!(GERMANY.country_code(), crate::iso::GERMANY);
        assert_eq!(HONG_KONG.country_code(), crate::iso::HONG_KONG);
        assert_eq!(UNITED_KINGDOM.country_code(), crate::iso::UNITED_KINGDOM_OF_GREAT_BRITAIN_AND_NORTHERN_IRELAND);
        assert_eq!(UNITED_STATES.country_code(), crate::iso::UNITED_STATES_OF_AMERICA);
    }

    #[test]
    fn test_market_identifier_code() {
        assert_eq!(ARGENTINA.market_identifier_code(), crate::iso::XBUE);
        assert_eq!(AUSTRALIA.market_identifier_code(), crate::iso::XASX);
        assert_eq!(AUSTRIA.market_identifier_code(), crate::iso::EXAA);
        assert_eq!(BOTSWANA.market_identifier_code(), crate::iso::XBOT);
        assert_eq!(BRAZIL.market_identifier_code(), crate::iso::BVMF);
        assert_eq!(CANADA.market_identifier_code(), crate::iso::XCNQ);
        assert_eq!(CHILE.market_identifier_code(), crate::iso::XSGO);
        assert_eq!(CZECH_REPUBLIC.market_identifier_code(), crate::iso::XPRA);
        assert_eq!(DENMARK.market_identifier_code(), crate::iso::XCSE);
        assert_eq!(FINLAND.market_identifier_code(), crate::iso::XHEL);
        assert_eq!(FRANCE.market_identifier_code(), crate::iso::XPAR);
        assert_eq!(GERMANY.market_identifier_code(), crate::iso::XFRA);
        assert_eq!(HONG_KONG.market_identifier_code(), crate::iso::XHKG);
        assert_eq!(UNITED_KINGDOM.market_identifier_code(), crate::iso::XLON);
        assert_eq!(UNITED_STATES.market_identifier_code(), crate::iso::XNYS);
    }

    #[rustfmt::skip]
    #[test]
    fn test_unpack_date() {
        assert_eq!(ARGENTINA.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(AUSTRALIA.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(AUSTRIA.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(BOTSWANA.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(BRAZIL.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(CANADA.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(CHILE.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(CZECH_REPUBLIC.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(DENMARK.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(FINLAND.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(FRANCE.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(GERMANY.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(HONG_KONG.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(UNITED_KINGDOM.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
        assert_eq!(UNITED_STATES.unpack_date(DATE), (Weekday::Tuesday, 14, Month::November, 2023, 318));
    }

    // #[test]
    // fn test_easter_monday() {
    //     assert_eq!(EASTER_MONDAYS[0][0], 98);
    //     assert_eq!(EASTER_MONDAYS[1][0], 91);
    // }

    // #[test]
    // fn test_is_weekend() {
    //     let date = OffsetDateTime::from_unix_timestamp(1614556800);

    //     assert_eq!(
    //         crate::time::calendars::UnitedStates::is_weekend(date),
    //         false
    //     );
    // }
}
