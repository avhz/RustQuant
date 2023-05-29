// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::currencies::currency::*;

/// Antarctic dollar
/// The ISO three-letter code is AAD.
/// It is divided into 100 cents.
/// It is pegged to the USD.
pub struct AADCurrency {
    /// The currency data.
    pub data: Currency,
}

impl AADCurrency {
    /// Create a new instance of `AADCurrency`.
    pub fn new(amount: f64) -> Self {
        Self {
            data: Currency {
                name: "Antarctic Dollar".to_string(),
                symbol: "AA$".to_string(),
                code: ISO_4217::AAD,
                minor: 2,
                fractions: 100,
                amount,
            },
        }
    }
}
