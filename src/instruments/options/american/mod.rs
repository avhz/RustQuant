// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-24 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct AmericanOption {
    /// The underlying asset price.
    underlying: f64,

    /// The strike price.
    strike: f64,

    /// The risk-free interest rate.
    rate: f64,

    /// The volatility of the underlying asset.
    volatility: f64,

    /// The time to expiry.
    time_to_expiry: f64,
}
