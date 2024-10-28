// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{currency::Currency, Ticker};
use RustQuant_iso::isin::ISIN;

/// Equity instrument.
pub struct Equity {
    /// The ticker symbol.
    pub ticker: Ticker,

    /// The ISIN (International Securities Identification Number).
    pub isin: ISIN,

    /// The currency of the equity.
    pub currency: Currency,
}
