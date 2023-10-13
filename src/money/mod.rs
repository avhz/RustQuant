// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONEY RELATED ITEMS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing all money related items.
//! This includes currencies, cashflows, exchange rates, and money types,
//! among other things.

/// Cashflow definitions.
pub mod cashflows;
pub use cashflows::*;

/// Currency data struct.
pub mod currency;
pub use currency::*;

/// Currency exchange rate helpers.
pub mod exchange;
pub use exchange::*;

/// Global currencies defined by ISO 4217.
pub mod iso_currencies;
pub use iso_currencies::*;

/// Legs (sequence of cashflows).
pub mod legs;
pub use legs::*;

/// Quotes (price, yield, etc).
pub mod quotes;
pub use quotes::*;
