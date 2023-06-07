// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::currencies::currency::*;

/// Australian dollar
/// The ISO three-letter code is AUD; the numeric code is 36.
/// It is divided into 100 cents.
pub struct AUDCurrency {
    /// The currency data.
    pub data: Currency,
}

/// New Zealand dollar
/// The ISO three-letter code is NZD; the numeric code is 554.
/// It is divided into 100 cents.
pub struct NZDCurrency {
    /// The currency data.
    pub data: Currency,
}

impl AUDCurrency {
    /// Create a new instance of `AUDCurrency`.
    pub fn new(amount: f64) -> Self {
        Self {
            data: Currency {
                name: "Australian Dollar".to_string(),
                symbol: "AU$".to_string(),
                code: ISO_4217::AUD,
                minor: 2,
                fractions: 100,
                amount,
            },
        }
    }
}

impl NZDCurrency {
    /// Create a new instance of `NZDCurrency`.
    pub fn new(amount: f64) -> Self {
        Self {
            data: Currency {
                name: "New Zealand Dollar".to_string(),
                symbol: "NZ$".to_string(),
                code: ISO_4217::NZD,
                minor: 2,
                fractions: 100,
                amount,
            },
        }
    }
}
