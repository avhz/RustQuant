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
//! ### Bonds
//!
//! - Prices:
//!   - [x] The Vasicek Model
//!   - [x] The Cox, Ingersoll, and Ross Model
//!   - [x] The Hull–White (One-Factor) Model
//!   - [ ] The Rendleman and Bartter Model
//!   - [ ] The Ho–Lee Model
//!   - [ ] The Black–Derman–Toy Model
//!   - [ ] The Black–Karasinski Model
//! - [ ] Duration
//! - [ ] Convexity
//!
//! ### Options
//!
//! - Closed-form price solutions:
//!   - [x] Heston Model
//!   - [x] Barrier
//!   - [x] European
//!   - [x] Greeks/Sensitivities
//!   - [x] Lookback
//!   - [x] Asian: Continuous Geometric Average
//!   - [x] Forward Start
//!   - [x] Bachelier and Modified Bachelier
//!   - [x] Generalised Black-Scholes-Merton
//!   - [ ] Basket
//!   - [ ] Rainbow
//!   - [ ] American
//!
//! - Lattice models:
//!   - [x] Binomial Tree (Cox-Ross-Rubinstein)
//!
//! The stochastic process generators can be used to price path-dependent options via Monte-Carlo.
//!
//! - Monte Carlo pricing:
//!   - [x] Lookback
//!   - [ ] Asian
//!   - [ ] Chooser
//!   - [ ] Barrier
//!
//! ```ignore
//! use RustQuant::instruments::*;
//! use time::{Duration, OffsetDateTime};
//!
//! let VanillaOption = EuropeanOption {
//!     initial_price: 100.,
//!     strike_price: 110.,
//!     risk_free_rate: 0.05,
//!     volatility: 0.3,
//!     dividend_rate: 0.02,
//!     evaluation_date: None,
//!     expiration_date: OffsetDateTime::now_utc() + Duration::days(365),
//! };
//!
//! let prices = VanillaOption.price();
//!
//! println!("Call price = {}", prices.0);
//! println!("Put price = {}", prices.1);
//! ```

/// Base trait for all instruments.
pub mod instrument;
pub use instrument::*;

/// Bond pricing models.
pub mod bonds;
pub use bonds::*;

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
