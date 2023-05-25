// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt::{self, Formatter};

/// Currency data struct.
pub struct Currency {
    /// Currency name. e.g. United States Dollar
    pub name: String,
    /// Currency symbol. e.g. $
    pub symbol: String,
    /// ISO 4217 currency code. e.g. USD = 840
    pub code: ISO_4217,
    /// Minor unit: digits after decimal separator. Usually D = 2.
    pub minor: u8,
    /// Fractions per unit. e.g. 100 cents = 1 dollar.
    pub fractions: u8,
    /// Amount of currency.
    pub amount: f64,
}

/// ISO 4217 codes enum.
/// Format:
///     - First two letters are the ISO 3166-1 alpha-2 country code. e.g. US = United States
///     - Third letter is the first letter of the currency name. e.g. USD = United States Dollar
///     - The number is the ISO numeric code. e.g. 840 = USD
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ISO_4217 {
    /// Antarctic dollar
    AAD,
    /// United Arab Emirates dirham
    AED = 784,
    /// Argentine peso
    ARS = 32,
    /// Australian dollar
    AUD = 36,
    /// Bulgarian lev
    BGN = 975,
    /// Brunei dollar
    BND = 96,
    /// Brazilian real
    BRL = 986,
    /// Canadian dollar
    CAD = 124,
    /// Swiss franc
    CHF = 756,
    /// Chilean peso
    CLP = 152,
    /// Chinese yuan
    CNY = 156,
    /// Colombian peso
    COP = 170,
    /// Czech koruna
    CZK = 203,
    /// Danish krone
    DKK = 208,
    /// Egyptian pound
    EGP = 818,
    /// Euro
    EUR = 978,
    /// Pound sterling
    GBP = 826,
    /// Ghanaian cedi
    GHS = 936,
    /// Hong Kong dollar
    HKD = 344,
    /// Croatian kuna
    HRK = 191,
    /// Hungarian forint
    HUF = 348,
    /// Indonesian rupiah
    IDR = 360,
    /// Israeli new shekel
    ILS = 376,
    /// Indian rupee
    INR = 356,
    /// Jamaican dollar
    JMD = 388,
    /// Japanese yen
    JPY = 392,
    /// Kenyan shilling
    KES = 404,
    /// South Korean won
    KRW = 410,
    /// Kuwaiti dinar
    KWD = 414,
    /// Sri Lankan rupee
    LKR = 144,
    /// Moroccan dirham
    MAD = 504,
    /// Mexican peso
    MXN = 484,
    /// Malaysian ringgit
    MYR = 458,
    /// Nigerian naira
    NGN = 566,
    /// Norwegian krone
    NOK = 578,
    /// New Zealand dollar
    NZD = 554,
    /// Peruvian sol
    PEN = 604,
    /// Philippine peso
    PHP = 608,
    /// Polish złoty
    PLN = 985,
    /// Romanian leu
    RON = 946,
    /// Russian ruble
    RUB = 643,
    /// Saudi riyal
    SAR = 682,
    /// Swedish krona/kronor
    SEK = 752,
    /// Singapore dollar
    SGD = 702,
    /// Thai baht
    THB = 764,
    /// Turkish lira
    TRY = 949,
    /// Tunisian dinar
    TND = 788,
    /// New Taiwan dollar
    TWD = 901,
    /// Ukrainian hryvnia
    UAH = 980,
    /// United States dollar
    USD = 840,
    /// Uruguayan peso
    UYU = 858,
    /// Venezuelan bolívar
    VEF = 937,
    /// Vietnamese đồng
    VND = 704,
    /// South African rand
    ZAR = 710,
    /// Zambian kwacha
    ZMW = 967,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Quantity:\t{}\nCurrency:\t{}\nISO Code:\t{}",
            self.amount, self.name, self.code as u16
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_currencies {
    use super::*;

    #[test]
    fn test_fmt() {
        let usd = Currency {
            name: "United States Dollar".to_string(),
            symbol: "$".to_string(),
            code: ISO_4217::USD,
            minor: 2,
            fractions: 100,
            amount: 0.0,
        };
        println!("{}", usd);
        // assert_eq!(format!("{}", usd), "Amount: \t {} \n Name: \t {} \n ISO: \t {}".to_string());
    }
}
