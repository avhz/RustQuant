// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines all global currencies as per the ISO 4217 standard.

use crate::instruments::fx::currency::Currency;
use crate::iso::iso_4217::ISO_4217;

macro_rules! generate_currencies {
    ($(
        $identifier:ident,
        $name:literal,
        $symbol:literal,
        $alphabetic_code:literal,
        $numeric_code:literal,
        $minor:literal,
        $fractions:literal
    );* $(;)?) => {
        $(
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
        )*
    };
}

generate_currencies! {
    AED, "United Arab Emirates Dirham", "د.إ", "AED", "784", 2, 100;
    AFN, "Afghan Afghani", "؋", "AFN", "971", 2, 100;
    ALL, "Albanian Lek", "L", "ALL", "008", 2, 100;
    AMD, "Armenian Dram", "֏", "AMD", "051", 2, 100;
    ANG, "Netherlands Antillean Guilder", "ƒ", "ANG", "532", 2, 100;
    AOA, "Angolan Kwanza", "Kz", "AOA", "973", 2, 100;
    ARS, "Argentine Peso", "AR$", "ARS", "032", 2, 100;
    AUD, "Australian Dollar", "AU$", "AUD", "036", 2, 100;
    AWG, "Aruban Florin", "ƒ", "AWG", "533", 2, 100;
    AZN, "Azerbaijani Manat", "₼", "AZN", "944", 2, 100;
    BAM, "Bosnia and Herzegovina Convertible Mark", "KM", "BAM", "977", 2, 100;
    BBD, "Barbados Dollar", "Bds$", "BBD", "052", 2, 100;
    BDT, "Bangladeshi Taka", "৳", "BDT", "050", 2, 100;
    BGN, "Bulgarian Lev", "лв", "BGN", "975", 2, 100;
    BHD, "Bahraini Dinar", "BD", "BHD", "048", 3, 1000;
    BIF, "Burundian Franc", "FBu", "BIF", "108", 0, 1;
    BMD, "Bermudian Dollar", "BD$", "BMD", "060", 2, 100;
    BND, "Brunei Dollar", "BN$", "BND", "096", 2, 100;
    BOB, "Boliviano", "Bs", "BOB", "068", 2, 100;
    BRL, "Brazilian Real", "R$", "BRL", "986", 2, 100;
    BSD, "Bahamian Dollar", "B$", "BSD", "044", 2, 100;
    BTN, "Bhutanese Ngultrum", "Nu.", "BTN", "064", 2, 100;
    BWP, "Botswana Pula", "P", "BWP", "072", 2, 100;
    BYN, "Belarusian Ruble", "Br", "BYN", "933", 2, 100;
    BZD, "Belize Dollar", "BZ$", "BZD", "084", 2, 100;
    CAD, "Canadian Dollar", "CA$", "CAD", "124", 2, 100;
    CDF, "Congolese Franc", "FC", "CDF", "976", 2, 100;
    CHF, "Swiss Franc", "CHF", "CHF", "756", 2, 100;
    CLP, "Chilean Peso", "CL$", "CLP", "152", 0, 1;
    COP, "Colombian Peso", "CO$", "COP", "170", 2, 100;
    CRC, "Costa Rican Colon", "₡", "CRC", "188", 2, 100;
    CUC, "Cuban Convertible Peso", "$", "CUC", "931", 2, 100;
    CUP, "Cuban Peso", "$MN", "CUP", "192", 2, 100;
    CVE, "Cape Verdean Escudo", "Esc", "CVE", "132", 2, 100;
    CZK, "Czech Koruna", "Kč", "CZK", "203", 2, 100;
    DJF, "Djiboutian Franc", "Fdj", "DJF", "262", 0, 1;
    DKK, "Danish Krone", "kr", "DKK", "208", 2, 100;
    DOP, "Dominican Peso", "RD$", "DOP", "214", 2, 100;
    DZD, "Algerian Dinar", "دج", "DZD", "012", 2, 100;
    EGP, "Egyptian Pound", "E£", "EGP", "818", 2, 100;
    ERN, "Eritrean Nakfa", "Nfk", "ERN", "232", 2, 100;
    ETB, "Ethiopian Birr", "Br", "ETB", "230", 2, 100;
    EUR, "Euro", "€", "EUR", "978", 2, 100;
    FJD, "Fijian Dollar", "FJ$", "FJD", "242", 2, 100;
    FKP, "Falkland Islands Pound", "FK£", "FKP", "238", 2, 100;
    GBP, "Pound Sterling", "£", "GBP", "826", 2, 100;
    GEL, "Georgian Lari", "₾", "GEL", "981", 2, 100;
    GHS, "Ghanaian Cedi", "GH₵", "GHS", "936", 2, 100;
    GIP, "Gibraltar Pound", "£", "GIP", "292", 2, 100;
    GMD, "Gambian Dalasi", "D", "GMD", "270", 2, 100;
    GNF, "Guinean Franc", "FG", "GNF", "324", 0, 1;
    GTQ, "Guatemalan Quetzal", "Q", "GTQ", "320", 2, 100;
    GYD, "Guyanese Dollar", "GY$", "GYD", "328", 2, 100;
    HKD, "Hong Kong Dollar", "HK$", "HKD", "344", 2, 100;
    HNL, "Honduran Lempira", "L", "HNL", "340", 2, 100;
    HRK, "Croatian Kuna", "kn", "HRK", "191", 2, 100;
    HTG, "Haitian Gourde", "G", "HTG", "332", 2, 100;
    HUF, "Hungarian Forint", "Ft", "HUF", "348", 2, 100;
    IDR, "Indonesian Rupiah", "Rp", "IDR", "360", 2, 100;
    ILS, "Israeli New Shekel", "₪", "ILS", "376", 2, 100;
    INR, "Indian Rupee", "₹", "INR", "356", 2, 100;
    IQD, "Iraqi Dinar", "ع.د", "IQD", "368", 3, 1000;
    IRR, "Iranian Rial", "﷼", "IRR", "364", 2, 100;
    ISK, "Icelandic Króna", "kr", "ISK", "352", 0, 1;
    JMD, "Jamaican Dollar", "J$", "JMD", "388", 2, 100;
    JOD, "Jordanian Dinar", "JD", "JOD", "400", 3, 1000;
    JPY, "Japanese Yen", "¥", "JPY", "392", 0, 1;
    KES, "Kenyan Shilling", "KSh", "KES", "404", 2, 100;
    KGS, "Kyrgyzstani Som", "лв", "KGS", "417", 2, 100;
    KHR, "Cambodian Riel", "៛", "KHR", "116", 2, 100;
    KMF, "Comoro Franc", "CF", "KMF", "174", 0, 1;
    KPW, "North Korean Won", "₩", "KPW", "408", 2, 100;
    KRW, "South Korean Won", "₩", "KRW", "410", 0, 1;
    KWD, "Kuwaiti Dinar", "KD", "KWD", "414", 3, 1000;
    KYD, "Cayman Islands Dollar", "KY$", "KYD", "136", 2, 100;
    KZT, "Kazakhstani Tenge", "₸", "KZT", "398", 2, 100;
    LAK, "Lao Kip", "₭", "LAK", "418", 2, 100;
    LBP, "Lebanese Pound", "L£", "LBP", "422", 2, 100;
    LKR, "Sri Lankan Rupee", "Rs", "LKR", "144", 2, 100;
    LRD, "Liberian Dollar", "L$", "LRD", "430", 2, 100;
    LSL, "Lesotho Loti", "M", "LSL", "426", 2, 100;
    LYD, "Libyan Dinar", "LD", "LYD", "434", 3, 1000;
    MAD, "Moroccan Dirham", "MAD", "MAD", "504", 2, 100;
    MDL, "Moldovan Leu", "MDL", "MDL", "498", 2, 100;
    MGA, "Malagasy Ariary", "Ar", "MGA", "969", 2, 100;
    MKD, "Macedonian Denar", "ден", "MKD", "807", 2, 100;
    MMK, "Myanmar Kyat", "K", "MMK", "104", 2, 100;
    MNT, "Mongolian Tögrög", "₮", "MNT", "496", 2, 100;
    MOP, "Macanese Pataca", "MOP$", "MOP", "446", 2, 100;
    MRO, "Mauritanian Ouguiya", "UM", "MRO", "478", 2, 100;
    MUR, "Mauritian Rupee", "Rs", "MUR", "480", 2, 100;
    MVR, "Maldivian Rufiyaa", "Rf", "MVR", "462", 2, 100;
    MWK, "Malawian Kwacha", "MK", "MWK", "454", 2, 100;
    MXN, "Mexican Peso", "MX$", "MXN", "484", 2, 100;
    MYR, "Malaysian Ringgit", "RM", "MYR", "458", 2, 100;
    MZN, "Mozambican Metical", "MT", "MZN", "943", 2, 100;
    NAD, "Namibian Dollar", "N$", "NAD", "516", 2, 100;
    NGN, "Nigerian Naira", "₦", "NGN", "566", 2, 100;
    NIO, "Nicaraguan Córdoba", "C$", "NIO", "558", 2, 100;
    NOK, "Norwegian Krone", "kr", "NOK", "578", 2, 100;
    NPR, "Nepalese Rupee", "Rs", "NPR", "524", 2, 100;
    NZD, "New Zealand Dollar", "NZ$", "NZD", "554", 2, 100;
    OMR, "Omani Rial", "OMR", "OMR", "512", 3, 1000;
    PAB, "Panamanian Balboa", "B/.", "PAB", "590", 2, 100;
    PEN, "Peruvian Sol", "S/.", "PEN", "604", 2, 100;
    PGK, "Papua New Guinean Kina", "K", "PGK", "598", 2, 100;
    PHP, "Philippine Peso", "₱", "PHP", "608", 2, 100;
    PKR, "Pakistani Rupee", "Rs", "PKR", "586", 2, 100;
    PLN, "Polish Złoty", "zł", "PLN", "985", 2, 100;
    PYG, "Paraguayan Guarani", "₲", "PYG", "600", 0, 1;
    QAR, "Qatari Riyal", "QR", "QAR", "634", 2, 100;
    RON, "Romanian Leu", "lei", "RON", "946", 2, 100;
    RSD, "Serbian Dinar", "din", "RSD", "941", 2, 100;
    CNY, "Renminbi (Chinese) Yuan", "¥", "CNY", "156", 2, 100;
    RUB, "Russian Ruble", "₽", "RUB", "643", 2, 100;
    RWF, "Rwandan Franc", "RF", "RWF", "646", 0, 1;
    SAR, "Saudi Riyal", "SR", "SAR", "682", 2, 100;
    SBD, "Solomon Islands Dollar", "SI$", "SBD", "090", 2, 100;
    SCR, "Seychelles Rupee", "SR", "SCR", "690", 2, 100;
    SDG, "Sudanese Pound", "SDG", "SDG", "938", 2, 100;
    SEK, "Swedish Krona/Kronor", "kr", "SEK", "752", 2, 100;
    SGD, "Singapore Dollar", "S$", "SGD", "702", 2, 100;
    SHP, "Saint Helena Pound", "£", "SHP", "654", 2, 100;
    SLE, "Sierra Leonean (new) Leone", "Le", "SLE", "925", 2, 100;
    SLL, "Sierra Leonean (old) Leone", "Le", "SLL", "694", 2, 100;
    SOS, "Somali Shilling", "Sh", "SOS", "706", 2, 100;
    SRD, "Surinamese Dollar", "SR$", "SRD", "968", 2, 100;
    SSP, "South Sudanese Pound", "SSP", "SSP", "728", 2, 100;
    STN, "São Tomé and Príncipe Dobra", "Db", "STN", "930", 2, 100;
    SVC, "Salvadoran Colón", "₡", "SVC", "222", 2, 100;
    SYP, "Syrian Pound", "LS", "SYP", "760", 2, 100;
    SZL, "Swazi Lilangeni", "E", "SZL", "748", 2, 100;
    THB, "Thai Baht", "฿", "THB", "764", 2, 100;
    TJS, "Tajikistani Somoni", "SM", "TJS", "972", 2, 100;
    TMT, "Turkmenistan Manat", "T", "TMT", "934", 2, 100;
    TND, "Tunisian Dinar", "DT", "TND", "788", 3, 1000;
    TOP, "Tongan Paʻanga", "T$", "TOP", "776", 2, 100;
    TRY, "Turkish Lira", "₺", "TRY", "949", 2, 100;
    TTD, "Trinidad and Tobago Dollar", "TT$", "TTD", "780", 2, 100;
    TWD, "New Taiwan Dollar", "NT$", "TWD", "901", 2, 100;
    TZS, "Tanzanian Shilling", "TSh", "TZS", "834", 2, 100;
    UAH, "Ukrainian Hryvnia", "₴", "UAH", "980", 2, 100;
    UGX, "Ugandan Shilling", "USh", "UGX", "800", 0, 1;
    USD, "United States Dollar", "$", "USD", "840", 2, 100;
    UYU, "Uruguayan Peso", "$U", "UYU", "858", 2, 100;
    UZS, "Uzbekistan Som", "лв", "UZS", "860", 2, 100;
    VES, "Venezuelan Bolívar Soberano", "Bs", "VES", "928", 2, 100;
    VND, "Vietnamese Đồng", "₫", "VND", "704", 0, 1;
    VUV, "Vanuatu Vatu", "VT", "VUV", "548", 0, 1;
    WST, "Samoan Tālā", "WS$", "WST", "882", 2, 100;
    XAF, "CFA Franc BEAC", "FCFA", "XAF", "950", 0, 1;
    XCD, "East Caribbean Dollar", "EC$", "XCD", "951", 2, 100;
    XOF, "CFA Franc BCEAO", "CFA", "XOF", "952", 0, 1;
    XPF, "CFP Franc", "₣", "XPF", "953", 0, 1;
    YER, "Yemeni Rial", "YR", "YER", "886", 2, 100;
    ZAR, "South African Rand", "R", "ZAR", "710", 2, 100;
    ZMW, "Zambian Kwacha", "ZK", "ZMW", "967", 2, 100;
    ZWL, "Zimbabwean Dollar", "Z$", "ZWL", "932", 2, 100;
}
