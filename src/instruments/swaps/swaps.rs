// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Swap type enum.
pub enum SwapType {
    /// Credit default swap.
    CreditDefaultSwap,
    /// Interest rate swap.
    InterestRateSwap,
    /// Currency swap.
    CurrencySwap,
    /// Commodity swap.
    CommoditySwap,
    /// Equity swap.
    EquitySwap,
    /// Total return swap.
    TotalReturnSwap,
    /// Variance swap.
    VarianceSwap,
    /// Volatility swap.
    VolatilitySwap,
    /// Inflation swap.
    InflationSwap,
}
