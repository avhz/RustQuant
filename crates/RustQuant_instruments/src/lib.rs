// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Financial instrument types and modules (bonds, options, etc).
//!
//! ### Options
//!
//! The following options and pricing methods are implemented:
//!
//! | Option | Analytic | Monte-Carlo | Finite Difference | Lattice | Greeks |
//! |--------|:--------:|:-----------:|:-----------------:|:-------:|:------:|
//! | Asian         |❌|✅|❌|❌|❌|
//! | Barrier       |❌|✅|❌|❌|❌|
//! | Basket        |❌|❌|❌|❌|❌|
//! | Binary        |❌|✅|❌|❌|❌|
//! | Chooser       |❌|❌|❌|❌|❌|
//! | Cliquet       |❌|❌|❌|❌|❌|
//! | Compound      |❌|❌|❌|❌|❌|
//! | Exchange      |❌|❌|❌|❌|❌|
//! | Forward Start |❌|❌|❌|❌|❌|
//! | Log           |❌|✅|❌|❌|❌|
//! | Lookback      |❌|✅|❌|❌|❌|
//! | Power         |❌|✅|❌|❌|❌|
//! | Quanto        |❌|❌|❌|❌|❌|
//! | Spread        |❌|❌|❌|❌|❌|
//! | Supershare    |❌|✅|❌|❌|❌|
//! | Vanilla       |✅|✅|✅|✅|✅|
//!
//! - Closed-form price solutions:
//!   - [x] Generalised Black-Scholes-Merton
//!   - [x] Bachelier and Modified Bachelier
//!   - [x] Heston Model
//!
//! - Lattice models:
//!   - [x] Binomial Tree (Cox-Ross-Rubinstein)
//!
//! ### Bonds
//!
//! ### FX
//!
//! ### Equities
//!
//! ### Commodities

/// Base trait for all instruments.
pub mod instrument;
pub use instrument::*;

/// Bond pricing models.
pub mod bonds;
// pub use bonds::*;

/// Option pricers and sensitivity functions.
pub mod options;
pub use options::*;

/// FX instruments.
pub mod fx;
pub use fx::*;

/// Equity instruments.
pub mod equities;
pub use equities::*;

/// Ticker symbol.
pub mod ticker;
pub use ticker::*;

/// Generic derivative payoff trait.
pub mod payoff;
pub use payoff::*;

/// Analytic option pricer.
pub mod analytic_option_pricer;
pub use analytic_option_pricer::*;

/// Monte-Carlo pricer.
pub mod monte_carlo_pricer;
pub use monte_carlo_pricer::*;
