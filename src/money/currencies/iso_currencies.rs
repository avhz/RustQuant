// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains implementations of 150+ currencies,
//! defined according to the ISO 4217 standard.
//! It currently needs to be proof-read and tested.

use super::Currency;
use super::ISO_4217;

/// United Arab Emirates dirham (AED)
pub const AED: &'static Currency = &Currency {
    name: "United Arab Emirates Dirham",
    symbol: "د.إ",
    code: ISO_4217 {
        alphabetic: "AED",
        numeric: "784",
    },
    minor: 2,
    fractions: 100,
};

/// Afghan afghani (AFN)
pub const AFN: &'static Currency = &Currency {
    name: "Afghan Afghani",
    symbol: "؋",
    code: ISO_4217 {
        alphabetic: "AFN",
        numeric: "971",
    },
    minor: 2,
    fractions: 100,
};

/// Albanian lek (ALL)
pub const ALL: &'static Currency = &Currency {
    name: "Albanian Lek",
    symbol: "L",
    code: ISO_4217 {
        alphabetic: "ALL",
        numeric: "008",
    },
    minor: 2,
    fractions: 100,
};

/// Armenian dram (AMD)
pub const AMD: &'static Currency = &Currency {
    name: "Armenian Dram",
    symbol: "֏",
    code: ISO_4217 {
        alphabetic: "AMD",
        numeric: "051",
    },
    minor: 2,
    fractions: 100,
};

/// Netherlands Antillean guilder (ANG)
pub const ANG: &'static Currency = &Currency {
    name: "Netherlands Antillean Guilder",
    symbol: "ƒ",
    code: ISO_4217 {
        alphabetic: "ANG",
        numeric: "532",
    },
    minor: 2,
    fractions: 100,
};

/// Angolan kwanza (AOA)
pub const AOA: &'static Currency = &Currency {
    name: "Angolan Kwanza",
    symbol: "Kz",
    code: ISO_4217 {
        alphabetic: "AOA",
        numeric: "973",
    },
    minor: 2,
    fractions: 100,
};

/// Argentine peso (ARS)
pub const ARS: &'static Currency = &Currency {
    name: "Argentine Peso",
    symbol: "AR$",
    code: ISO_4217 {
        alphabetic: "ARS",
        numeric: "032",
    },
    minor: 2,
    fractions: 100,
};

/// Australian dollar (AUD)
pub const AUD: &'static Currency = &Currency {
    name: "Australian Dollar",
    symbol: "AU$",
    code: ISO_4217 {
        alphabetic: "AUD",
        numeric: "036",
    },
    minor: 2,
    fractions: 100,
};

/// Aruban florin (AWG)
pub const AWG: &'static Currency = &Currency {
    name: "Aruban Florin",
    symbol: "ƒ",
    code: ISO_4217 {
        alphabetic: "AWG",
        numeric: "533",
    },
    minor: 2,
    fractions: 100,
};

/// Azerbbaijani manat (AZN)
pub const AZN: &'static Currency = &Currency {
    name: "Azerbaijani Manat",
    symbol: "₼",
    code: ISO_4217 {
        alphabetic: "AZN",
        numeric: "944",
    },
    minor: 2,
    fractions: 100,
};

/// Bosnia and Herzegovina convertible mark (BAM)
pub const BAM: &'static Currency = &Currency {
    name: "Bosnia and Herzegovina Convertible Mark",
    symbol: "KM",
    code: ISO_4217 {
        alphabetic: "BAM",
        numeric: "977",
    },
    minor: 2,
    fractions: 100,
};

/// Barbados dollar (BBD)
pub const BBD: &'static Currency = &Currency {
    name: "Barbados Dollar",
    symbol: "Bds$",
    code: ISO_4217 {
        alphabetic: "BBD",
        numeric: "052",
    },
    minor: 2,
    fractions: 100,
};

/// Bangladeshi taka (BDT)
pub const BDT: &'static Currency = &Currency {
    name: "Bangladeshi Taka",
    symbol: "৳",
    code: ISO_4217 {
        alphabetic: "BDT",
        numeric: "050",
    },
    minor: 2,
    fractions: 100,
};

/// Bulgarian lev (BGN)
pub const BGN: &'static Currency = &Currency {
    name: "Bulgarian Lev",
    symbol: "лв",
    code: ISO_4217 {
        alphabetic: "BGN",
        numeric: "975",
    },
    minor: 2,
    fractions: 100,
};

/// Bahraini dinar (BHD)
pub const BHD: &'static Currency = &Currency {
    name: "Bahraini Dinar",
    symbol: "BD",
    code: ISO_4217 {
        alphabetic: "BHD",
        numeric: "048",
    },
    minor: 3,
    fractions: 1000,
};

/// Burundian franc (BIF)
pub const BIF: &'static Currency = &Currency {
    name: "Burundian Franc",
    symbol: "FBu",
    code: ISO_4217 {
        alphabetic: "BIF",
        numeric: "108",
    },
    minor: 0,
    fractions: 1,
};

/// Bermudian dollar (BMD)
pub const BMD: &'static Currency = &Currency {
    name: "Bermudian Dollar",
    symbol: "BD$",
    code: ISO_4217 {
        alphabetic: "BMD",
        numeric: "060",
    },
    minor: 2,
    fractions: 100,
};

/// Brunei dollar (BND)
pub const BND: &'static Currency = &Currency {
    name: "Brunei Dollar",
    symbol: "BN$",
    code: ISO_4217 {
        alphabetic: "BND",
        numeric: "096",
    },
    minor: 2,
    fractions: 100,
};

/// Boliviano (BOB)
pub const BOB: &'static Currency = &Currency {
    name: "Boliviano",
    symbol: "Bs",
    code: ISO_4217 {
        alphabetic: "BOB",
        numeric: "068",
    },
    minor: 2,
    fractions: 100,
};

/// Brazilian real (BRL)
pub const BRL: &'static Currency = &Currency {
    name: "Brazilian Real",
    symbol: "R$",
    code: ISO_4217 {
        alphabetic: "BRL",
        numeric: "986",
    },
    minor: 2,
    fractions: 100,
};

/// Bahamian dollar (BSD)
pub const BSD: &'static Currency = &Currency {
    name: "Bahamian Dollar",
    symbol: "B$",
    code: ISO_4217 {
        alphabetic: "BSD",
        numeric: "044",
    },
    minor: 2,
    fractions: 100,
};

/// Bhutanese ngultrum (BTN)
pub const BTN: &'static Currency = &Currency {
    name: "Bhutanese Ngultrum",
    symbol: "Nu.",
    code: ISO_4217 {
        alphabetic: "BTN",
        numeric: "064",
    },
    minor: 2,
    fractions: 100,
};

/// Botswana pula (BWP)
pub const BWP: &'static Currency = &Currency {
    name: "Botswana Pula",
    symbol: "P",
    code: ISO_4217 {
        alphabetic: "BWP",
        numeric: "072",
    },
    minor: 2,
    fractions: 100,
};

/// Belarusian ruble (BYN)
pub const BYN: &'static Currency = &Currency {
    name: "Belarusian Ruble",
    symbol: "Br",
    code: ISO_4217 {
        alphabetic: "BYN",
        numeric: "933",
    },
    minor: 2,
    fractions: 100,
};

/// Belize dollar (BZD)
pub const BZD: &'static Currency = &Currency {
    name: "Belize Dollar",
    symbol: "BZ$",
    code: ISO_4217 {
        alphabetic: "BZD",
        numeric: "084",
    },
    minor: 2,
    fractions: 100,
};

/// Canadian dollar (CAD)
pub const CAD: &'static Currency = &Currency {
    name: "Canadian Dollar",
    symbol: "CA$",
    code: ISO_4217 {
        alphabetic: "CAD",
        numeric: "124",
    },
    minor: 2,
    fractions: 100,
};

/// Congolese franc (CDF)
pub const CDF: &'static Currency = &Currency {
    name: "Congolese Franc",
    symbol: "FC",
    code: ISO_4217 {
        alphabetic: "CDF",
        numeric: "976",
    },
    minor: 2,
    fractions: 100,
};

/// Swiss franc (CHF)
pub const CHF: &'static Currency = &Currency {
    name: "Swiss Franc",
    symbol: "CHF",
    code: ISO_4217 {
        alphabetic: "CHF",
        numeric: "756",
    },
    minor: 2,
    fractions: 100,
};

/// Chilean peso (CLP)
pub const CLP: &'static Currency = &Currency {
    name: "Chilean Peso",
    symbol: "CL$",
    code: ISO_4217 {
        alphabetic: "CLP",
        numeric: "152",
    },
    minor: 0,
    fractions: 1,
};

/// Colombian peso (COP)
pub const COP: &'static Currency = &Currency {
    name: "Colombian Peso",
    symbol: "CO$",
    code: ISO_4217 {
        alphabetic: "COP",
        numeric: "170",
    },
    minor: 2,
    fractions: 100,
};

/// Costa Rican colon (CRC)
pub const CRC: &'static Currency = &Currency {
    name: "Costa Rican Colon",
    symbol: "₡",
    code: ISO_4217 {
        alphabetic: "CRC",
        numeric: "188",
    },
    minor: 2,
    fractions: 100,
};

/// Cuban convertible peso (CUC)
pub const CUC: &'static Currency = &Currency {
    name: "Cuban Convertible Peso",
    symbol: "$",
    code: ISO_4217 {
        alphabetic: "CUC",
        numeric: "931",
    },
    minor: 2,
    fractions: 100,
};

/// Cuban peso (CUP)
pub const CUP: &'static Currency = &Currency {
    name: "Cuban Peso",
    symbol: "$MN",
    code: ISO_4217 {
        alphabetic: "CUP",
        numeric: "192",
    },
    minor: 2,
    fractions: 100,
};

/// Cape Verdean escudo (CVE)
pub const CVE: &'static Currency = &Currency {
    name: "Cape Verdean Escudo",
    symbol: "Esc",
    code: ISO_4217 {
        alphabetic: "CVE",
        numeric: "132",
    },
    minor: 2,
    fractions: 100,
};

/// Czech koruna (CZK)
pub const CZK: &'static Currency = &Currency {
    name: "Czech Koruna",
    symbol: "Kč",
    code: ISO_4217 {
        alphabetic: "CZK",
        numeric: "203",
    },
    minor: 2,
    fractions: 100,
};

/// Djiboutian franc (DJF)
pub const DJF: &'static Currency = &Currency {
    name: "Djiboutian Franc",
    symbol: "Fdj",
    code: ISO_4217 {
        alphabetic: "DJF",
        numeric: "262",
    },
    minor: 0,
    fractions: 1,
};

/// Danish krone (DKK)
pub const DKK: &'static Currency = &Currency {
    name: "Danish Krone",
    symbol: "kr",
    code: ISO_4217 {
        alphabetic: "DKK",
        numeric: "208",
    },
    minor: 2,
    fractions: 100,
};

/// Dominican peso (DOP)
pub const DOP: &'static Currency = &Currency {
    name: "Dominican Peso",
    symbol: "RD$",
    code: ISO_4217 {
        alphabetic: "DOP",
        numeric: "214",
    },
    minor: 2,
    fractions: 100,
};

/// Algerian dinar (DZD)
pub const DZD: &'static Currency = &Currency {
    name: "Algerian Dinar",
    symbol: "دج",
    code: ISO_4217 {
        alphabetic: "DZD",
        numeric: "012",
    },
    minor: 2,
    fractions: 100,
};

/// Egyptian pound (EGP)
pub const EGP: &'static Currency = &Currency {
    name: "Egyptian Pound",
    symbol: "E£",
    code: ISO_4217 {
        alphabetic: "EGP",
        numeric: "818",
    },
    minor: 2,
    fractions: 100,
};

/// Eritrean nakfa (ERN)
pub const ERN: &'static Currency = &Currency {
    name: "Eritrean Nakfa",
    symbol: "Nfk",
    code: ISO_4217 {
        alphabetic: "ERN",
        numeric: "232",
    },
    minor: 2,
    fractions: 100,
};

/// Ethiopian birr (ETB)
pub const ETB: &'static Currency = &Currency {
    name: "Ethiopian Birr",
    symbol: "Br",
    code: ISO_4217 {
        alphabetic: "ETB",
        numeric: "230",
    },
    minor: 2,
    fractions: 100,
};

/// Euro (EUR)
pub const EUR: &'static Currency = &Currency {
    name: "Euro",
    symbol: "€",
    code: ISO_4217 {
        alphabetic: "EUR",
        numeric: "978",
    },
    minor: 2,
    fractions: 100,
};

/// Fijian dollar (FJD)
pub const FJD: &'static Currency = &Currency {
    name: "Fijian Dollar",
    symbol: "FJ$",
    code: ISO_4217 {
        alphabetic: "FJD",
        numeric: "242",
    },
    minor: 2,
    fractions: 100,
};

/// Falkland Islands pound (FKP)
pub const FKP: &'static Currency = &Currency {
    name: "Falkland Islands Pound",
    symbol: "FK£",
    code: ISO_4217 {
        alphabetic: "FKP",
        numeric: "238",
    },
    minor: 2,
    fractions: 100,
};

/// Pound sterling (GBP)
pub const GBP: &'static Currency = &Currency {
    name: "Pound Sterling",
    symbol: "£",
    code: ISO_4217 {
        alphabetic: "GBP",
        numeric: "826",
    },
    minor: 2,
    fractions: 100,
};

/// Georgian lari (GEL)
pub const GEL: &'static Currency = &Currency {
    name: "Georgian Lari",
    symbol: "₾",
    code: ISO_4217 {
        alphabetic: "GEL",
        numeric: "981",
    },
    minor: 2,
    fractions: 100,
};

/// Ghanaian cedi (GHS)
pub const GHS: &'static Currency = &Currency {
    name: "Ghanaian Cedi",
    symbol: "GH₵",
    code: ISO_4217 {
        alphabetic: "GHS",
        numeric: "936",
    },
    minor: 2,
    fractions: 100,
};

/// Gibraltar pound (GIP)
pub const GIP: &'static Currency = &Currency {
    name: "Gibraltar Pound",
    symbol: "£",
    code: ISO_4217 {
        alphabetic: "GIP",
        numeric: "292",
    },
    minor: 2,
    fractions: 100,
};

/// Gambian dalasi (GMD)
pub const GMD: &'static Currency = &Currency {
    name: "Gambian Dalasi",
    symbol: "D",
    code: ISO_4217 {
        alphabetic: "GMD",
        numeric: "270",
    },
    minor: 2,
    fractions: 100,
};

/// Guinean franc (GNF)
pub const GNF: &'static Currency = &Currency {
    name: "Guinean Franc",
    symbol: "FG",
    code: ISO_4217 {
        alphabetic: "GNF",
        numeric: "324",
    },
    minor: 0,
    fractions: 1,
};

/// Guatemalan quetzal (GTQ)
pub const GTQ: &'static Currency = &Currency {
    name: "Guatemalan Quetzal",
    symbol: "Q",
    code: ISO_4217 {
        alphabetic: "GTQ",
        numeric: "320",
    },
    minor: 2,
    fractions: 100,
};

/// Guyanese dollar (GYD)
pub const GYD: &'static Currency = &Currency {
    name: "Guyanese Dollar",
    symbol: "GY$",
    code: ISO_4217 {
        alphabetic: "GYD",
        numeric: "328",
    },
    minor: 2,
    fractions: 100,
};

/// Hong Kong dollar (HKD)
pub const HKD: &'static Currency = &Currency {
    name: "Hong Kong Dollar",
    symbol: "HK$",
    code: ISO_4217 {
        alphabetic: "HKD",
        numeric: "344",
    },
    minor: 2,
    fractions: 100,
};

/// Honduran lempira (HNL)
pub const HNL: &'static Currency = &Currency {
    name: "Honduran Lempira",
    symbol: "L",
    code: ISO_4217 {
        alphabetic: "HNL",
        numeric: "340",
    },
    minor: 2,
    fractions: 100,
};

/// Croatian kuna (HRK)
pub const HRK: &'static Currency = &Currency {
    name: "Croatian Kuna",
    symbol: "kn",
    code: ISO_4217 {
        alphabetic: "HRK",
        numeric: "191",
    },
    minor: 2,
    fractions: 100,
};

/// Haitian gourde (HTG)
pub const HTG: &'static Currency = &Currency {
    name: "Haitian Gourde",
    symbol: "G",
    code: ISO_4217 {
        alphabetic: "HTG",
        numeric: "332",
    },
    minor: 2,
    fractions: 100,
};

/// Hungarian forint (HUF)
pub const HUF: &'static Currency = &Currency {
    name: "Hungarian Forint",
    symbol: "Ft",
    code: ISO_4217 {
        alphabetic: "HUF",
        numeric: "348",
    },
    minor: 2,
    fractions: 100,
};

/// Indonesian rupiah (IDR)
pub const IDR: &'static Currency = &Currency {
    name: "Indonesian Rupiah",
    symbol: "Rp",
    code: ISO_4217 {
        alphabetic: "IDR",
        numeric: "360",
    },
    minor: 2,
    fractions: 100,
};

/// Israeli new shekel (ILS)
pub const ILS: &'static Currency = &Currency {
    name: "Israeli New Shekel",
    symbol: "₪",
    code: ISO_4217 {
        alphabetic: "ILS",
        numeric: "376",
    },
    minor: 2,
    fractions: 100,
};

/// Indian rupee (INR)
pub const INR: &'static Currency = &Currency {
    name: "Indian Rupee",
    symbol: "₹",
    code: ISO_4217 {
        alphabetic: "INR",
        numeric: "356",
    },
    minor: 2,
    fractions: 100,
};

/// Iraqi dinar (IQD)
pub const IQD: &'static Currency = &Currency {
    name: "Iraqi Dinar",
    symbol: "ع.د",
    code: ISO_4217 {
        alphabetic: "IQD",
        numeric: "368",
    },
    minor: 3,
    fractions: 1000,
};

/// Iranian rial (IRR)
pub const IRR: &'static Currency = &Currency {
    name: "Iranian Rial",
    symbol: "﷼",
    code: ISO_4217 {
        alphabetic: "IRR",
        numeric: "364",
    },
    minor: 2,
    fractions: 100,
};

/// Icelandic króna (ISK)
pub const ISK: &'static Currency = &Currency {
    name: "Icelandic Króna",
    symbol: "kr",
    code: ISO_4217 {
        alphabetic: "ISK",
        numeric: "352",
    },
    minor: 0,
    fractions: 1,
};

/// Jamaican dollar (JMD)
pub const JMD: &'static Currency = &Currency {
    name: "Jamaican Dollar",
    symbol: "J$",
    code: ISO_4217 {
        alphabetic: "JMD",
        numeric: "388",
    },
    minor: 2,
    fractions: 100,
};

/// Jordanian dinar (JOD)
pub const JOD: &'static Currency = &Currency {
    name: "Jordanian Dinar",
    symbol: "JD",
    code: ISO_4217 {
        alphabetic: "JOD",
        numeric: "400",
    },
    minor: 3,
    fractions: 1000,
};

/// Japanese yen (JPY)
pub const JPY: &'static Currency = &Currency {
    name: "Japanese Yen",
    symbol: "¥",
    code: ISO_4217 {
        alphabetic: "JPY",
        numeric: "392",
    },
    minor: 0,
    fractions: 1,
};

/// Kenyan shilling (KES)
pub const KES: &'static Currency = &Currency {
    name: "Kenyan Shilling",
    symbol: "KSh",
    code: ISO_4217 {
        alphabetic: "KES",
        numeric: "404",
    },
    minor: 2,
    fractions: 100,
};

/// Kyrgyzstani som (KGS)
pub const KGS: &'static Currency = &Currency {
    name: "Kyrgyzstani Som",
    symbol: "лв",
    code: ISO_4217 {
        alphabetic: "KGS",
        numeric: "417",
    },
    minor: 2,
    fractions: 100,
};

/// Cambodian riel (KHR)
pub const KHR: &'static Currency = &Currency {
    name: "Cambodian Riel",
    symbol: "៛",
    code: ISO_4217 {
        alphabetic: "KHR",
        numeric: "116",
    },
    minor: 2,
    fractions: 100,
};

/// Comoro franc (KMF)
pub const KMF: &'static Currency = &Currency {
    name: "Comoro Franc",
    symbol: "CF",
    code: ISO_4217 {
        alphabetic: "KMF",
        numeric: "174",
    },
    minor: 0,
    fractions: 1,
};

/// North Korean won (KPW)
pub const KPW: &'static Currency = &Currency {
    name: "North Korean Won",
    symbol: "₩",
    code: ISO_4217 {
        alphabetic: "KPW",
        numeric: "408",
    },
    minor: 2,
    fractions: 100,
};

/// South Korean won (KRW)
pub const KRW: &'static Currency = &Currency {
    name: "South Korean Won",
    symbol: "₩",
    code: ISO_4217 {
        alphabetic: "KRW",
        numeric: "410",
    },
    minor: 0,
    fractions: 1,
};

/// Kuwaiti dinar (KWD)
pub const KWD: &'static Currency = &Currency {
    name: "Kuwaiti Dinar",
    symbol: "KD",
    code: ISO_4217 {
        alphabetic: "KWD",
        numeric: "414",
    },
    minor: 3,
    fractions: 1000,
};

/// Cayman Islands dollar (KYD)
pub const KYD: &'static Currency = &Currency {
    name: "Cayman Islands Dollar",
    symbol: "KY$",
    code: ISO_4217 {
        alphabetic: "KYD",
        numeric: "136",
    },
    minor: 2,
    fractions: 100,
};

/// Kazakhstani tenge (KZT)
pub const KZT: &'static Currency = &Currency {
    name: "Kazakhstani Tenge",
    symbol: "₸",
    code: ISO_4217 {
        alphabetic: "KZT",
        numeric: "398",
    },
    minor: 2,
    fractions: 100,
};

/// Lao kip (LAK)
pub const LAK: &'static Currency = &Currency {
    name: "Lao Kip",
    symbol: "₭",
    code: ISO_4217 {
        alphabetic: "LAK",
        numeric: "418",
    },
    minor: 2,
    fractions: 100,
};

/// Lebanese pound (LBP)
pub const LBP: &'static Currency = &Currency {
    name: "Lebanese Pound",
    symbol: "L£",
    code: ISO_4217 {
        alphabetic: "LBP",
        numeric: "422",
    },
    minor: 2,
    fractions: 100,
};

/// Sri Lankan rupee (LKR)
pub const LKR: &'static Currency = &Currency {
    name: "Sri Lankan Rupee",
    symbol: "Rs",
    code: ISO_4217 {
        alphabetic: "LKR",
        numeric: "144",
    },
    minor: 2,
    fractions: 100,
};

/// Liberian dollar (LRD)
pub const LRD: &'static Currency = &Currency {
    name: "Liberian Dollar",
    symbol: "L$",
    code: ISO_4217 {
        alphabetic: "LRD",
        numeric: "430",
    },
    minor: 2,
    fractions: 100,
};

/// Lesotho loti (LSL)
pub const LSL: &'static Currency = &Currency {
    name: "Lesotho Loti",
    symbol: "M",
    code: ISO_4217 {
        alphabetic: "LSL",
        numeric: "426",
    },
    minor: 2,
    fractions: 100,
};

/// Libyan dinar (LYD)
pub const LYD: &'static Currency = &Currency {
    name: "Libyan Dinar",
    symbol: "LD",
    code: ISO_4217 {
        alphabetic: "LYD",
        numeric: "434",
    },
    minor: 3,
    fractions: 1000,
};

/// Moroccan dirham (MAD)
pub const MAD: &'static Currency = &Currency {
    name: "Moroccan Dirham",
    symbol: "MAD",
    code: ISO_4217 {
        alphabetic: "MAD",
        numeric: "504",
    },
    minor: 2,
    fractions: 100,
};

/// Moldovan leu (MDL)
pub const MDL: &'static Currency = &Currency {
    name: "Moldovan Leu",
    symbol: "MDL",
    code: ISO_4217 {
        alphabetic: "MDL",
        numeric: "498",
    },
    minor: 2,
    fractions: 100,
};

/// Malagasy ariary (MGA)
pub const MGA: &'static Currency = &Currency {
    name: "Malagasy Ariary",
    symbol: "Ar",
    code: ISO_4217 {
        alphabetic: "MGA",
        numeric: "969",
    },
    minor: 2,
    fractions: 100,
};

/// Macedonian denar (MKD)
pub const MKD: &'static Currency = &Currency {
    name: "Macedonian Denar",
    symbol: "ден",
    code: ISO_4217 {
        alphabetic: "MKD",
        numeric: "807",
    },
    minor: 2,
    fractions: 100,
};

/// Myanmar kyat (MMK)
pub const MMK: &'static Currency = &Currency {
    name: "Myanmar Kyat",
    symbol: "K",
    code: ISO_4217 {
        alphabetic: "MMK",
        numeric: "104",
    },
    minor: 2,
    fractions: 100,
};

/// Mongolian tögrög (MNT)
pub const MNT: &'static Currency = &Currency {
    name: "Mongolian Tögrög",
    symbol: "₮",
    code: ISO_4217 {
        alphabetic: "MNT",
        numeric: "496",
    },
    minor: 2,
    fractions: 100,
};

/// Macanese pataca (MOP)
pub const MOP: &'static Currency = &Currency {
    name: "Macanese Pataca",
    symbol: "MOP$",
    code: ISO_4217 {
        alphabetic: "MOP",
        numeric: "446",
    },
    minor: 2,
    fractions: 100,
};

/// Mauritanian ouguiya (MRO)
pub const MRO: &'static Currency = &Currency {
    name: "Mauritanian Ouguiya",
    symbol: "UM",
    code: ISO_4217 {
        alphabetic: "MRO",
        numeric: "478",
    },
    minor: 2,
    fractions: 100,
};

/// Mauritian rupee (MUR)
pub const MUR: &'static Currency = &Currency {
    name: "Mauritian Rupee",
    symbol: "Rs",
    code: ISO_4217 {
        alphabetic: "MUR",
        numeric: "480",
    },
    minor: 2,
    fractions: 100,
};

/// Maldivian rufiyaa (MVR)
pub const MVR: &'static Currency = &Currency {
    name: "Maldivian Rufiyaa",
    symbol: "Rf",
    code: ISO_4217 {
        alphabetic: "MVR",
        numeric: "462",
    },
    minor: 2,
    fractions: 100,
};

/// Malawian kwacha (MWK)
pub const MWK: &'static Currency = &Currency {
    name: "Malawian Kwacha",
    symbol: "MK",
    code: ISO_4217 {
        alphabetic: "MWK",
        numeric: "454",
    },
    minor: 2,
    fractions: 100,
};

/// Mexican peso (MXN)
pub const MXN: &'static Currency = &Currency {
    name: "Mexican Peso",
    symbol: "MX$",
    code: ISO_4217 {
        alphabetic: "MXN",
        numeric: "484",
    },
    minor: 2,
    fractions: 100,
};

/// Malaysian ringgit (MYR)
pub const MYR: &'static Currency = &Currency {
    name: "Malaysian Ringgit",
    symbol: "RM",
    code: ISO_4217 {
        alphabetic: "MYR",
        numeric: "458",
    },
    minor: 2,
    fractions: 100,
};

// FINISH ALL THE CURRENCIES

/// Mozambican metical (MZN)
pub const MZN: &'static Currency = &Currency {
    name: "Mozambican Metical",
    symbol: "MT",
    code: ISO_4217 {
        alphabetic: "MZN",
        numeric: "943",
    },
    minor: 2,
    fractions: 100,
};

/// Namibian dollar (NAD)
pub const NAD: &'static Currency = &Currency {
    name: "Namibian Dollar",
    symbol: "N$",
    code: ISO_4217 {
        alphabetic: "NAD",
        numeric: "516",
    },
    minor: 2,
    fractions: 100,
};

/// Nigerian naira (NGN)
pub const NGN: &'static Currency = &Currency {
    name: "Nigerian Naira",
    symbol: "₦",
    code: ISO_4217 {
        alphabetic: "NGN",
        numeric: "566",
    },
    minor: 2,
    fractions: 100,
};

/// Nicaraguan córdoba (NIO)
pub const NIO: &'static Currency = &Currency {
    name: "Nicaraguan Córdoba",
    symbol: "C$",
    code: ISO_4217 {
        alphabetic: "NIO",
        numeric: "558",
    },
    minor: 2,
    fractions: 100,
};

/// Norwegian krone (NOK)
pub const NOK: &'static Currency = &Currency {
    name: "Norwegian Krone",
    symbol: "kr",
    code: ISO_4217 {
        alphabetic: "NOK",
        numeric: "578",
    },
    minor: 2,
    fractions: 100,
};

/// Nepalese rupee (NPR)
pub const NPR: &'static Currency = &Currency {
    name: "Nepalese Rupee",
    symbol: "Rs",
    code: ISO_4217 {
        alphabetic: "NPR",
        numeric: "524",
    },
    minor: 2,
    fractions: 100,
};

/// New Zealand dollar (NZD)
pub const NZD: &'static Currency = &Currency {
    name: "New Zealand Dollar",
    symbol: "NZ$",
    code: ISO_4217 {
        alphabetic: "NZD",
        numeric: "554",
    },
    minor: 2,
    fractions: 100,
};

/// Omani rial (OMR)
pub const OMR: &'static Currency = &Currency {
    name: "Omani Rial",
    symbol: "OMR",
    code: ISO_4217 {
        alphabetic: "OMR",
        numeric: "512",
    },
    minor: 3,
    fractions: 1000,
};

/// Panamanian balboa (PAB)
pub const PAB: &'static Currency = &Currency {
    name: "Panamanian Balboa",
    symbol: "B/.",
    code: ISO_4217 {
        alphabetic: "PAB",
        numeric: "590",
    },
    minor: 2,
    fractions: 100,
};

/// Peruvian sol (PEN)
pub const PEN: &'static Currency = &Currency {
    name: "Peruvian Sol",
    symbol: "S/.",
    code: ISO_4217 {
        alphabetic: "PEN",
        numeric: "604",
    },
    minor: 2,
    fractions: 100,
};

/// Papua New Guinean kina (PGK)
pub const PGK: &'static Currency = &Currency {
    name: "Papua New Guinean Kina",
    symbol: "K",
    code: ISO_4217 {
        alphabetic: "PGK",
        numeric: "598",
    },
    minor: 2,
    fractions: 100,
};

/// Philippine peso (PHP)
pub const PHP: &'static Currency = &Currency {
    name: "Philippine Peso",
    symbol: "₱",
    code: ISO_4217 {
        alphabetic: "PHP",
        numeric: "608",
    },
    minor: 2,
    fractions: 100,
};

/// Pakistani rupee (PKR)
pub const PKR: &'static Currency = &Currency {
    name: "Pakistani Rupee",
    symbol: "Rs",
    code: ISO_4217 {
        alphabetic: "PKR",
        numeric: "586",
    },
    minor: 2,
    fractions: 100,
};

/// Polish złoty (PLN)
pub const PLN: &'static Currency = &Currency {
    name: "Polish Złoty",
    symbol: "zł",
    code: ISO_4217 {
        alphabetic: "PLN",
        numeric: "985",
    },
    minor: 2,
    fractions: 100,
};

/// Paraguayan guaraní (PYG)
pub const PYG: &'static Currency = &Currency {
    name: "Paraguayan Guarani",
    symbol: "₲",
    code: ISO_4217 {
        alphabetic: "PYG",
        numeric: "600",
    },
    minor: 0,
    fractions: 1,
};

/// Qatari riyal (QAR)
pub const QAR: &'static Currency = &Currency {
    name: "Qatari Riyal",
    symbol: "QR",
    code: ISO_4217 {
        alphabetic: "QAR",
        numeric: "634",
    },
    minor: 2,
    fractions: 100,
};

/// Romanian leu (RON)
pub const RON: &'static Currency = &Currency {
    name: "Romanian Leu",
    symbol: "lei",
    code: ISO_4217 {
        alphabetic: "RON",
        numeric: "946",
    },
    minor: 2,
    fractions: 100,
};

/// Serbian dinar (RSD)
pub const RSD: &'static Currency = &Currency {
    name: "Serbian Dinar",
    symbol: "din",
    code: ISO_4217 {
        alphabetic: "RSD",
        numeric: "941",
    },
    minor: 2,
    fractions: 100,
};

/// Renminbi (Chinese) yuan (CNY)
pub const CNY: &'static Currency = &Currency {
    name: "Renminbi (Chinese) Yuan",
    symbol: "¥",
    code: ISO_4217 {
        alphabetic: "CNY",
        numeric: "156",
    },
    minor: 2,
    fractions: 100,
};

/// Russian ruble (RUB)
pub const RUB: &'static Currency = &Currency {
    name: "Russian Ruble",
    symbol: "₽",
    code: ISO_4217 {
        alphabetic: "RUB",
        numeric: "643",
    },
    minor: 2,
    fractions: 100,
};

/// Rwandan franc (RWF)
pub const RWF: &'static Currency = &Currency {
    name: "Rwandan Franc",
    symbol: "RF",
    code: ISO_4217 {
        alphabetic: "RWF",
        numeric: "646",
    },
    minor: 0,
    fractions: 1,
};

/// Saudi riyal (SAR)
pub const SAR: &'static Currency = &Currency {
    name: "Saudi Riyal",
    symbol: "SR",
    code: ISO_4217 {
        alphabetic: "SAR",
        numeric: "682",
    },
    minor: 2,
    fractions: 100,
};

/// Solomon Islands dollar (SBD)
pub const SBD: &'static Currency = &Currency {
    name: "Solomon Islands Dollar",
    symbol: "SI$",
    code: ISO_4217 {
        alphabetic: "SBD",
        numeric: "090",
    },
    minor: 2,
    fractions: 100,
};

/// Seychelles rupee (SCR)
pub const SCR: &'static Currency = &Currency {
    name: "Seychelles Rupee",
    symbol: "SR",
    code: ISO_4217 {
        alphabetic: "SCR",
        numeric: "690",
    },
    minor: 2,
    fractions: 100,
};

/// Sudanese pound (SDG)
pub const SDG: &'static Currency = &Currency {
    name: "Sudanese Pound",
    symbol: "SDG",
    code: ISO_4217 {
        alphabetic: "SDG",
        numeric: "938",
    },
    minor: 2,
    fractions: 100,
};

/// Swedish krona/kronor (SEK)
pub const SEK: &'static Currency = &Currency {
    name: "Swedish Krona/Kronor",
    symbol: "kr",
    code: ISO_4217 {
        alphabetic: "SEK",
        numeric: "752",
    },
    minor: 2,
    fractions: 100,
};

/// Singapore dollar (SGD)
pub const SGD: &'static Currency = &Currency {
    name: "Singapore Dollar",
    symbol: "S$",
    code: ISO_4217 {
        alphabetic: "SGD",
        numeric: "702",
    },
    minor: 2,
    fractions: 100,
};

/// Saint Helena pound (SHP)
pub const SHP: &'static Currency = &Currency {
    name: "Saint Helena Pound",
    symbol: "£",
    code: ISO_4217 {
        alphabetic: "SHP",
        numeric: "654",
    },
    minor: 2,
    fractions: 100,
};

/// Sierra Leonean (new) leone (SLE)
pub const SLE: &'static Currency = &Currency {
    name: "Sierra Leonean (new) Leone",
    symbol: "Le",
    code: ISO_4217 {
        alphabetic: "SLE",
        numeric: "925",
    },
    minor: 2,
    fractions: 100,
};

/// Sierra Leonean (old) leone (SLL)
pub const SLL: &'static Currency = &Currency {
    name: "Sierra Leonean (old) Leone",
    symbol: "Le",
    code: ISO_4217 {
        alphabetic: "SLL",
        numeric: "694",
    },
    minor: 2,
    fractions: 100,
};

/// Somali shilling (SOS)
pub const SOS: &'static Currency = &Currency {
    name: "Somali Shilling",
    symbol: "Sh",
    code: ISO_4217 {
        alphabetic: "SOS",
        numeric: "706",
    },
    minor: 2,
    fractions: 100,
};

/// Surinamese dollar (SRD)
pub const SRD: &'static Currency = &Currency {
    name: "Surinamese Dollar",
    symbol: "SR$",
    code: ISO_4217 {
        alphabetic: "SRD",
        numeric: "968",
    },
    minor: 2,
    fractions: 100,
};

/// South Sudanese pound (SSP)
pub const SSP: &'static Currency = &Currency {
    name: "South Sudanese Pound",
    symbol: "SSP",
    code: ISO_4217 {
        alphabetic: "SSP",
        numeric: "728",
    },
    minor: 2,
    fractions: 100,
};

/// São Tomé and Príncipe dobra (STN)
pub const STN: &'static Currency = &Currency {
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    code: ISO_4217 {
        alphabetic: "STN",
        numeric: "930",
    },
    minor: 2,
    fractions: 100,
};

/// Salvadoran colón (SVC)
pub const SVC: &'static Currency = &Currency {
    name: "Salvadoran Colón",
    symbol: "₡",
    code: ISO_4217 {
        alphabetic: "SVC",
        numeric: "222",
    },
    minor: 2,
    fractions: 100,
};

/// Syrian pound (SYP)
pub const SYP: &'static Currency = &Currency {
    name: "Syrian Pound",
    symbol: "LS",
    code: ISO_4217 {
        alphabetic: "SYP",
        numeric: "760",
    },
    minor: 2,
    fractions: 100,
};

/// Swazi lilangeni (SZL)
pub const SZL: &'static Currency = &Currency {
    name: "Swazi Lilangeni",
    symbol: "E",
    code: ISO_4217 {
        alphabetic: "SZL",
        numeric: "748",
    },
    minor: 2,
    fractions: 100,
};

/// Thai baht (THB)
pub const THB: &'static Currency = &Currency {
    name: "Thai Baht",
    symbol: "฿",
    code: ISO_4217 {
        alphabetic: "THB",
        numeric: "764",
    },
    minor: 2,
    fractions: 100,
};

/// Tajikistani somoni (TJS)
pub const TJS: &'static Currency = &Currency {
    name: "Tajikistani Somoni",
    symbol: "SM",
    code: ISO_4217 {
        alphabetic: "TJS",
        numeric: "972",
    },
    minor: 2,
    fractions: 100,
};

/// Turkmenistan manat (TMT)
pub const TMT: &'static Currency = &Currency {
    name: "Turkmenistan Manat",
    symbol: "T",
    code: ISO_4217 {
        alphabetic: "TMT",
        numeric: "934",
    },
    minor: 2,
    fractions: 100,
};

/// Tunisian dinar (TND)
pub const TND: &'static Currency = &Currency {
    name: "Tunisian Dinar",
    symbol: "DT",
    code: ISO_4217 {
        alphabetic: "TND",
        numeric: "788",
    },
    minor: 3,
    fractions: 1000,
};

/// Tongan paʻanga (TOP)
pub const TOP: &'static Currency = &Currency {
    name: "Tongan Paʻanga",
    symbol: "T$",
    code: ISO_4217 {
        alphabetic: "TOP",
        numeric: "776",
    },
    minor: 2,
    fractions: 100,
};

/// Turkish lira (TRY)
pub const TRY: &'static Currency = &Currency {
    name: "Turkish Lira",
    symbol: "₺",
    code: ISO_4217 {
        alphabetic: "TRY",
        numeric: "949",
    },
    minor: 2,
    fractions: 100,
};

/// Trinidad and Tobago dollar (TTD)
pub const TTD: &'static Currency = &Currency {
    name: "Trinidad and Tobago Dollar",
    symbol: "TT$",
    code: ISO_4217 {
        alphabetic: "TTD",
        numeric: "780",
    },
    minor: 2,
    fractions: 100,
};

/// New Taiwan dollar (TWD)
pub const TWD: &'static Currency = &Currency {
    name: "New Taiwan Dollar",
    symbol: "NT$",
    code: ISO_4217 {
        alphabetic: "TWD",
        numeric: "901",
    },
    minor: 2,
    fractions: 100,
};

/// Tanzanian shilling (TZS)
pub const TZS: &'static Currency = &Currency {
    name: "Tanzanian Shilling",
    symbol: "TSh",
    code: ISO_4217 {
        alphabetic: "TZS",
        numeric: "834",
    },
    minor: 2,
    fractions: 100,
};

/// Ukrainian hryvnia (UAH)
pub const UAH: &'static Currency = &Currency {
    name: "Ukrainian Hryvnia",
    symbol: "₴",
    code: ISO_4217 {
        alphabetic: "UAH",
        numeric: "980",
    },
    minor: 2,
    fractions: 100,
};

/// Ugandan shilling (UGX)
pub const UGX: &'static Currency = &Currency {
    name: "Ugandan Shilling",
    symbol: "USh",
    code: ISO_4217 {
        alphabetic: "UGX",
        numeric: "800",
    },
    minor: 0,
    fractions: 1,
};

/// United States dollar (USD)
pub const USD: &'static Currency = &Currency {
    name: "United States Dollar",
    symbol: "$",
    code: ISO_4217 {
        alphabetic: "USD",
        numeric: "840",
    },
    minor: 2,
    fractions: 100,
};

/// Uruguayan peso (UYU)
pub const UYU: &'static Currency = &Currency {
    name: "Uruguayan Peso",
    symbol: "$U",
    code: ISO_4217 {
        alphabetic: "UYU",
        numeric: "858",
    },
    minor: 2,
    fractions: 100,
};

/// Uzbekistan som (UZS)
pub const UZS: &'static Currency = &Currency {
    name: "Uzbekistan Som",
    symbol: "лв",
    code: ISO_4217 {
        alphabetic: "UZS",
        numeric: "860",
    },
    minor: 2,
    fractions: 100,
};

/// Venezuelan bolívar soberano (VES)
pub const VES: &'static Currency = &Currency {
    name: "Venezuelan Bolívar Soberano",
    symbol: "Bs",
    code: ISO_4217 {
        alphabetic: "VES",
        numeric: "928",
    },
    minor: 2,
    fractions: 100,
};

/// Vietnamese đồng (VND)
pub const VND: &'static Currency = &Currency {
    name: "Vietnamese Đồng",
    symbol: "₫",
    code: ISO_4217 {
        alphabetic: "VND",
        numeric: "704",
    },
    minor: 0,
    fractions: 1,
};

/// Vanuatu vatu (VUV)
pub const VUV: &'static Currency = &Currency {
    name: "Vanuatu Vatu",
    symbol: "VT",
    code: ISO_4217 {
        alphabetic: "VUV",
        numeric: "548",
    },
    minor: 0,
    fractions: 1,
};

/// Samoan tālā (WST)
pub const WST: &'static Currency = &Currency {
    name: "Samoan Tālā",
    symbol: "WS$",
    code: ISO_4217 {
        alphabetic: "WST",
        numeric: "882",
    },
    minor: 2,
    fractions: 100,
};

/// CFA franc BEAC (XAF)
pub const XAF: &'static Currency = &Currency {
    name: "CFA Franc BEAC",
    symbol: "FCFA",
    code: ISO_4217 {
        alphabetic: "XAF",
        numeric: "950",
    },
    minor: 0,
    fractions: 1,
};

/// East Caribbean dollar (XCD)
pub const XCD: &'static Currency = &Currency {
    name: "East Caribbean Dollar",
    symbol: "EC$",
    code: ISO_4217 {
        alphabetic: "XCD",
        numeric: "951",
    },
    minor: 2,
    fractions: 100,
};

/// CFA franc BCEAO (XOF)
pub const XOF: &'static Currency = &Currency {
    name: "CFA Franc BCEAO",
    symbol: "CFA",
    code: ISO_4217 {
        alphabetic: "XOF",
        numeric: "952",
    },
    minor: 0,
    fractions: 1,
};

/// CFP franc (XPF)
pub const XPF: &'static Currency = &Currency {
    name: "CFP Franc",
    symbol: "₣",
    code: ISO_4217 {
        alphabetic: "XPF",
        numeric: "953",
    },
    minor: 0,
    fractions: 1,
};

/// Yemeni rial (YER)
pub const YER: &'static Currency = &Currency {
    name: "Yemeni Rial",
    symbol: "YR",
    code: ISO_4217 {
        alphabetic: "YER",
        numeric: "886",
    },
    minor: 2,
    fractions: 100,
};

/// South African rand (ZAR)
pub const ZAR: &'static Currency = &Currency {
    name: "South African Rand",
    symbol: "R",
    code: ISO_4217 {
        alphabetic: "ZAR",
        numeric: "710",
    },
    minor: 2,
    fractions: 100,
};

/// Zambian kwacha (ZMW)
pub const ZMW: &'static Currency = &Currency {
    name: "Zambian Kwacha",
    symbol: "ZK",
    code: ISO_4217 {
        alphabetic: "ZMW",
        numeric: "967",
    },
    minor: 2,
    fractions: 100,
};

/// Zimbabwean dollar (ZWL)
pub const ZWL: &'static Currency = &Currency {
    name: "Zimbabwean Dollar",
    symbol: "Z$",
    code: ISO_4217 {
        alphabetic: "ZWL",
        numeric: "932",
    },
    minor: 2,
    fractions: 100,
};
