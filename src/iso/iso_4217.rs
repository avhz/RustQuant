// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains implementations of 150+ currencies,
//! defined according to the ISO 4217 standard.
//! It currently needs to be proof-read and tested.

use std::fmt;
use std::fmt::Formatter;

pub use unformatted::*;

/// ISO 4217 codes enum.
///
/// Format:
///     - First two letters are the ISO 3166-1 alpha-2 country code. e.g. US = United States
///     - Third letter is the first letter of the currency name. e.g. USD = United States Dollar
///     - The number is the ISO numeric code. e.g. 840 = USD
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct ISO_4217 {
    /// The ISO 4217 alphabetic code.
    pub alphabetic: &'static str,

    /// The ISO 4217 numeric code.
    pub numeric: &'static str,
}

impl ISO_4217 {
    /// Create a new ISO 4217 code.
    #[must_use]
    pub fn new(alphabetic: &'static str, numeric: &'static str) -> Self {
        Self {
            alphabetic,
            numeric,
        }
    }

    /// Get the ISO 4217 alphabetic code.
    #[must_use]
    pub fn alphabetic(&self) -> &str {
        self.alphabetic
    }

    /// Get the ISO 4217 numeric code.
    #[must_use]
    pub fn numeric(&self) -> &'static str {
        self.numeric
    }
}

impl Eq for ISO_4217 {}

impl PartialEq for ISO_4217 {
    fn eq(&self, other: &Self) -> bool {
        self.alphabetic == other.alphabetic && self.numeric == other.numeric
    }
}

impl fmt::Display for ISO_4217 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Alphabetic: {}, Numeric: {}",
            self.alphabetic, self.numeric
        )
    }
}

macro_rules! generate_currency {
    ($identifier:ident, $name:literal, $symbol:literal, $alphabetic_code:literal, $numeric_code:literal, $minor:literal, $fractions:literal) => {
        #[doc = concat!(" ", $name, " (", $alphabetic_code, ")")]
        pub const $identifier: Currency = Currency {
            name: $name,
            symbol: $symbol,
            code: ISO_4217 {
                alphabetic: $alphabetic_code,
                numeric: $numeric_code,
            },
            minor: $minor,
            fractions: $fractions,
        };
    };
}

#[rustfmt::skip]
mod unformatted {
    use crate::iso::iso_4217::ISO_4217;
    use crate::instruments::fx::currency::Currency;
    generate_currency!(AED, "United Arab Emirates Dirham", "د.إ", "AED", "784", 2, 100);
    generate_currency!(AFN, "Afghan Afghani", "؋", "AFN", "971", 2, 100);
    generate_currency!(ALL, "Albanian Lek", "L", "ALL", "008", 2, 100);
    generate_currency!(AMD, "Armenian Dram", "֏", "AMD", "051", 2, 100);
    generate_currency!(ANG, "Netherlands Antillean Guilder", "ƒ", "ANG", "532", 2, 100);
    generate_currency!(AOA, "Angolan Kwanza", "Kz", "AOA", "973", 2, 100);
    generate_currency!(ARS, "Argentine Peso", "AR$", "ARS", "032", 2, 100);
    generate_currency!(AUD, "Australian Dollar", "AU$", "AUD", "036", 2, 100);
    generate_currency!(AWG, "Aruban Florin", "ƒ", "AWG", "533", 2, 100);
    generate_currency!(AZN, "Azerbaijani Manat", "₼", "AZN", "944", 2, 100);
    generate_currency!(BAM, "Bosnia and Herzegovina Convertible Mark", "KM", "BAM", "977", 2, 100);
    generate_currency!(BBD, "Barbados Dollar", "Bds$", "BBD", "052", 2, 100);
    generate_currency!(BDT, "Bangladeshi Taka", "৳", "BDT", "050", 2, 100);
    generate_currency!(BGN, "Bulgarian Lev", "лв", "BGN", "975", 2, 100);
    generate_currency!(BHD, "Bahraini Dinar", "BD", "BHD", "048", 3, 1000);
    generate_currency!(BIF, "Burundian Franc", "FBu", "BIF", "108", 0, 1);
    generate_currency!(BMD, "Bermudian Dollar", "BD$", "BMD", "060", 2, 100);
    generate_currency!(BND, "Brunei Dollar", "BN$", "BND", "096", 2, 100);
    generate_currency!(BOB, "Boliviano", "Bs", "BOB", "068", 2, 100);
    generate_currency!(BRL, "Brazilian Real", "R$", "BRL", "986", 2, 100);
    generate_currency!(BSD, "Bahamian Dollar", "B$", "BSD", "044", 2, 100);
    generate_currency!(BTN, "Bhutanese Ngultrum", "Nu.", "BTN", "064", 2, 100);
    generate_currency!(BWP, "Botswana Pula", "P", "BWP", "072", 2, 100);
    generate_currency!(BYN, "Belarusian Ruble", "Br", "BYN", "933", 2, 100);
    generate_currency!(BZD, "Belize Dollar", "BZ$", "BZD", "084", 2, 100);
    generate_currency!(CAD, "Canadian Dollar", "CA$", "CAD", "124", 2, 100);
    generate_currency!(CDF, "Congolese Franc", "FC", "CDF", "976", 2, 100);
    generate_currency!(CHF, "Swiss Franc", "CHF", "CHF", "756", 2, 100);
    generate_currency!(CLP, "Chilean Peso", "CL$", "CLP", "152", 0, 1);
    generate_currency!(COP, "Colombian Peso", "CO$", "COP", "170", 2, 100);
    generate_currency!(CRC, "Costa Rican Colon", "₡", "CRC", "188", 2, 100);
    generate_currency!(CUC, "Cuban Convertible Peso", "$", "CUC", "931", 2, 100);
    generate_currency!(CUP, "Cuban Peso", "$MN", "CUP", "192", 2, 100);
    generate_currency!(CVE, "Cape Verdean Escudo", "Esc", "CVE", "132", 2, 100);
    generate_currency!(CZK, "Czech Koruna", "Kč", "CZK", "203", 2, 100);
    generate_currency!(DJF, "Djiboutian Franc", "Fdj", "DJF", "262", 0, 1);
    generate_currency!(DKK, "Danish Krone", "kr", "DKK", "208", 2, 100);
    generate_currency!(DOP, "Dominican Peso", "RD$", "DOP", "214", 2, 100);
    generate_currency!(DZD, "Algerian Dinar", "دج", "DZD", "012", 2, 100);
    generate_currency!(EGP, "Egyptian Pound", "E£", "EGP", "818", 2, 100);
    generate_currency!(ERN, "Eritrean Nakfa", "Nfk", "ERN", "232", 2, 100);
    generate_currency!(ETB, "Ethiopian Birr", "Br", "ETB", "230", 2, 100);
    generate_currency!(EUR, "Euro", "€", "EUR", "978", 2, 100);
    generate_currency!(FJD, "Fijian Dollar", "FJ$", "FJD", "242", 2, 100);
    generate_currency!(FKP, "Falkland Islands Pound", "FK£", "FKP", "238", 2, 100);
    generate_currency!(GBP, "Pound Sterling", "£", "GBP", "826", 2, 100);
    generate_currency!(GEL, "Georgian Lari", "₾", "GEL", "981", 2, 100);
    generate_currency!(GHS, "Ghanaian Cedi", "GH₵", "GHS", "936", 2, 100);
    generate_currency!(GIP, "Gibraltar Pound", "£", "GIP", "292", 2, 100);
    generate_currency!(GMD, "Gambian Dalasi", "D", "GMD", "270", 2, 100);
    generate_currency!(GNF, "Guinean Franc", "FG", "GNF", "324", 0, 1);
    generate_currency!(GTQ, "Guatemalan Quetzal", "Q", "GTQ", "320", 2, 100);
    generate_currency!(GYD, "Guyanese Dollar", "GY$", "GYD", "328", 2, 100);
    generate_currency!(HKD, "Hong Kong Dollar", "HK$", "HKD", "344", 2, 100);
    generate_currency!(HNL, "Honduran Lempira", "L", "HNL", "340", 2, 100);
    generate_currency!(HRK, "Croatian Kuna", "kn", "HRK", "191", 2, 100);
    generate_currency!(HTG, "Haitian Gourde", "G", "HTG", "332", 2, 100);
    generate_currency!(HUF, "Hungarian Forint", "Ft", "HUF", "348", 2, 100);
    generate_currency!(IDR, "Indonesian Rupiah", "Rp", "IDR", "360", 2, 100);
    generate_currency!(ILS, "Israeli New Shekel", "₪", "ILS", "376", 2, 100);
    generate_currency!(INR, "Indian Rupee", "₹", "INR", "356", 2, 100);
    generate_currency!(IQD, "Iraqi Dinar", "ع.د", "IQD", "368", 3, 1000);
    generate_currency!(IRR, "Iranian Rial", "﷼", "IRR", "364", 2, 100);
    generate_currency!(ISK, "Icelandic Króna", "kr", "ISK", "352", 0, 1);
    generate_currency!(JMD, "Jamaican Dollar", "J$", "JMD", "388", 2, 100);
    generate_currency!(JOD, "Jordanian Dinar", "JD", "JOD", "400", 3, 1000);
    generate_currency!(JPY, "Japanese Yen", "¥", "JPY", "392", 0, 1);
    generate_currency!(KES, "Kenyan Shilling", "KSh", "KES", "404", 2, 100);
    generate_currency!(KGS, "Kyrgyzstani Som", "лв", "KGS", "417", 2, 100);
    generate_currency!(KHR, "Cambodian Riel", "៛", "KHR", "116", 2, 100);
    generate_currency!(KMF, "Comoro Franc", "CF", "KMF", "174", 0, 1);
    generate_currency!(KPW, "North Korean Won", "₩", "KPW", "408", 2, 100);
    generate_currency!(KRW, "South Korean Won", "₩", "KRW", "410", 0, 1);
    generate_currency!(KWD, "Kuwaiti Dinar", "KD", "KWD", "414", 3, 1000);
    generate_currency!(KYD, "Cayman Islands Dollar", "KY$", "KYD", "136", 2, 100);
    generate_currency!(KZT, "Kazakhstani Tenge", "₸", "KZT", "398", 2, 100);
    generate_currency!(LAK, "Lao Kip", "₭", "LAK", "418", 2, 100);
    generate_currency!(LBP, "Lebanese Pound", "L£", "LBP", "422", 2, 100);
    generate_currency!(LKR, "Sri Lankan Rupee", "Rs", "LKR", "144", 2, 100);
    generate_currency!(LRD, "Liberian Dollar", "L$", "LRD", "430", 2, 100);
    generate_currency!(LSL, "Lesotho Loti", "M", "LSL", "426", 2, 100);
    generate_currency!(LYD, "Libyan Dinar", "LD", "LYD", "434", 3, 1000);
    generate_currency!(MAD, "Moroccan Dirham", "MAD", "MAD", "504", 2, 100);
    generate_currency!(MDL, "Moldovan Leu", "MDL", "MDL", "498", 2, 100);
    generate_currency!(MGA, "Malagasy Ariary", "Ar", "MGA", "969", 2, 100);
    generate_currency!(MKD, "Macedonian Denar", "ден", "MKD", "807", 2, 100);
    generate_currency!(MMK, "Myanmar Kyat", "K", "MMK", "104", 2, 100);
    generate_currency!(MNT, "Mongolian Tögrög", "₮", "MNT", "496", 2, 100);
    generate_currency!(MOP, "Macanese Pataca", "MOP$", "MOP", "446", 2, 100);
    generate_currency!(MRO, "Mauritanian Ouguiya", "UM", "MRO", "478", 2, 100);
    generate_currency!(MUR, "Mauritian Rupee", "Rs", "MUR", "480", 2, 100);
    generate_currency!(MVR, "Maldivian Rufiyaa", "Rf", "MVR", "462", 2, 100);
    generate_currency!(MWK, "Malawian Kwacha", "MK", "MWK", "454", 2, 100);
    generate_currency!(MXN, "Mexican Peso", "MX$", "MXN", "484", 2, 100);
    generate_currency!(MYR, "Malaysian Ringgit", "RM", "MYR", "458", 2, 100);
    generate_currency!(MZN, "Mozambican Metical", "MT", "MZN", "943", 2, 100);
    generate_currency!(NAD, "Namibian Dollar", "N$", "NAD", "516", 2, 100);
    generate_currency!(NGN, "Nigerian Naira", "₦", "NGN", "566", 2, 100);
    generate_currency!(NIO, "Nicaraguan Córdoba", "C$", "NIO", "558", 2, 100);
    generate_currency!(NOK, "Norwegian Krone", "kr", "NOK", "578", 2, 100);
    generate_currency!(NPR, "Nepalese Rupee", "Rs", "NPR", "524", 2, 100);
    generate_currency!(NZD, "New Zealand Dollar", "NZ$", "NZD", "554", 2, 100);
    generate_currency!(OMR, "Omani Rial", "OMR", "OMR", "512", 3, 1000);
    generate_currency!(PAB, "Panamanian Balboa", "B/.", "PAB", "590", 2, 100);
    generate_currency!(PEN, "Peruvian Sol", "S/.", "PEN", "604", 2, 100);
    generate_currency!(PGK, "Papua New Guinean Kina", "K", "PGK", "598", 2, 100);
    generate_currency!(PHP, "Philippine Peso", "₱", "PHP", "608", 2, 100);
    generate_currency!(PKR, "Pakistani Rupee", "Rs", "PKR", "586", 2, 100);
    generate_currency!(PLN, "Polish Złoty", "zł", "PLN", "985", 2, 100);
    generate_currency!(PYG, "Paraguayan Guarani", "₲", "PYG", "600", 0, 1);
    generate_currency!(QAR, "Qatari Riyal", "QR", "QAR", "634", 2, 100);
    generate_currency!(RON, "Romanian Leu", "lei", "RON", "946", 2, 100);
    generate_currency!(RSD, "Serbian Dinar", "din", "RSD", "941", 2, 100);
    generate_currency!(CNY, "Renminbi (Chinese) Yuan", "¥", "CNY", "156", 2, 100);
    generate_currency!(RUB, "Russian Ruble", "₽", "RUB", "643", 2, 100);
    generate_currency!(RWF, "Rwandan Franc", "RF", "RWF", "646", 0, 1);
    generate_currency!(SAR, "Saudi Riyal", "SR", "SAR", "682", 2, 100);
    generate_currency!(SBD, "Solomon Islands Dollar", "SI$", "SBD", "090", 2, 100);
    generate_currency!(SCR, "Seychelles Rupee", "SR", "SCR", "690", 2, 100);
    generate_currency!(SDG, "Sudanese Pound", "SDG", "SDG", "938", 2, 100);
    generate_currency!(SEK, "Swedish Krona/Kronor", "kr", "SEK", "752", 2, 100);
    generate_currency!(SGD, "Singapore Dollar", "S$", "SGD", "702", 2, 100);
    generate_currency!(SHP, "Saint Helena Pound", "£", "SHP", "654", 2, 100);
    generate_currency!(SLE, "Sierra Leonean (new) Leone", "Le", "SLE", "925", 2, 100);
    generate_currency!(SLL, "Sierra Leonean (old) Leone", "Le", "SLL", "694", 2, 100);
    generate_currency!(SOS, "Somali Shilling", "Sh", "SOS", "706", 2, 100);
    generate_currency!(SRD, "Surinamese Dollar", "SR$", "SRD", "968", 2, 100);
    generate_currency!(SSP, "South Sudanese Pound", "SSP", "SSP", "728", 2, 100);
    generate_currency!(STN, "São Tomé and Príncipe Dobra", "Db", "STN", "930", 2, 100);
    generate_currency!(SVC, "Salvadoran Colón", "₡", "SVC", "222", 2, 100);
    generate_currency!(SYP, "Syrian Pound", "LS", "SYP", "760", 2, 100);
    generate_currency!(SZL, "Swazi Lilangeni", "E", "SZL", "748", 2, 100);
    generate_currency!(THB, "Thai Baht", "฿", "THB", "764", 2, 100);
    generate_currency!(TJS, "Tajikistani Somoni", "SM", "TJS", "972", 2, 100);
    generate_currency!(TMT, "Turkmenistan Manat", "T", "TMT", "934", 2, 100);
    generate_currency!(TND, "Tunisian Dinar", "DT", "TND", "788", 3, 1000);
    generate_currency!(TOP, "Tongan Paʻanga", "T$", "TOP", "776", 2, 100);
    generate_currency!(TRY, "Turkish Lira", "₺", "TRY", "949", 2, 100);
    generate_currency!(TTD, "Trinidad and Tobago Dollar", "TT$", "TTD", "780", 2, 100);
    generate_currency!(TWD, "New Taiwan Dollar", "NT$", "TWD", "901", 2, 100);
    generate_currency!(TZS, "Tanzanian Shilling", "TSh", "TZS", "834", 2, 100);
    generate_currency!(UAH, "Ukrainian Hryvnia", "₴", "UAH", "980", 2, 100);
    generate_currency!(UGX, "Ugandan Shilling", "USh", "UGX", "800", 0, 1);
    generate_currency!(USD, "United States Dollar", "$", "USD", "840", 2, 100);
    generate_currency!(UYU, "Uruguayan Peso", "$U", "UYU", "858", 2, 100);
    generate_currency!(UZS, "Uzbekistan Som", "лв", "UZS", "860", 2, 100);
    generate_currency!(VES, "Venezuelan Bolívar Soberano", "Bs", "VES", "928", 2, 100);
    generate_currency!(VND, "Vietnamese Đồng", "₫", "VND", "704", 0, 1);
    generate_currency!(VUV, "Vanuatu Vatu", "VT", "VUV", "548", 0, 1);
    generate_currency!(WST, "Samoan Tālā", "WS$", "WST", "882", 2, 100);
    generate_currency!(XAF, "CFA Franc BEAC", "FCFA", "XAF", "950", 0, 1);
    generate_currency!(XCD, "East Caribbean Dollar", "EC$", "XCD", "951", 2, 100);
    generate_currency!(XOF, "CFA Franc BCEAO", "CFA", "XOF", "952", 0, 1);
    generate_currency!(XPF, "CFP Franc", "₣", "XPF", "953", 0, 1);
    generate_currency!(YER, "Yemeni Rial", "YR", "YER", "886", 2, 100);
    generate_currency!(ZAR, "South African Rand", "R", "ZAR", "710", 2, 100);
    generate_currency!(ZMW, "Zambian Kwacha", "ZK", "ZMW", "967", 2, 100);
    generate_currency!(ZWL, "Zimbabwean Dollar", "Z$", "ZWL", "932", 2, 100);
}
