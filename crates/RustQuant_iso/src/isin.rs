// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// ISIN (International Securities Identification Number) data.
///
/// Apple example: US0378331005
/// - US: ISO 3166-1 alpha-2 country code.
/// - 037833100: NSIN (National Securities Identifying Number).
/// - 5: Check digit.
pub struct ISIN {
    /// The ISO 3166-1 alpha-2 country code.
    pub country_code: &'static str,

    /// The NSIN (National Securities Identifying Number).
    pub nsin: &'static str,

    /// The check digit.
    pub check_digit: &'static str,
}
