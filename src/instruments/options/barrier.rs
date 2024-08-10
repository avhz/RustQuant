// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Barrier option.
#[derive(Debug, Clone)]
pub struct BarrierOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Barrier type (up-and-out, down-and-out, up-and-in, down-and-in).
    pub barrier_type: BarrierType,

    /// Barrier level.
    pub barrier: f64,

    /// Strike price of the option.
    pub strike: f64,

    /// Rebate amount.
    pub rebate: Option<f64>,
}
