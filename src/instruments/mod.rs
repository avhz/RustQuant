// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// INSTRUMENTS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Financial instrument types and modules (bonds, options, etc).
//! ### :chart_with_downwards_trend: Bonds <a name="bonds"></a>
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
//! ### :money_with_wings: Option Pricing <a name="options"></a>
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
//! ```no_run
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
pub mod bonds {
    pub use crate::instruments::bonds::{bond::*, cox_ingersoll_ross::*, vasicek::*};

    /// Base bond traits.
    pub mod bond;
    /// Cox-Ingersoll-Ross bond pricing model.
    pub mod cox_ingersoll_ross;
    /// One-factor Hull-White bond pricing model.
    pub mod hull_white;
    /// Vasicek bond pricing model.
    pub mod vasicek;
}
pub use bonds::*;

/// Option pricers and sensitivity functions.
pub mod options {
    pub use crate::instruments::options::{
        asian::*, bachelier::*, barrier::*, binary::*, binomial::*, black_scholes_merton::*,
        forward_start::*, heston::*, implied_volatility::*, lookback::*, merton_jump_diffusion::*,
        option::*, power::*,
    };

    /// Asian option pricers.
    pub mod asian;

    /// Bachelier option pricer.
    pub mod bachelier;

    /// Barrier option pricers.
    pub mod barrier;

    /// Binary option pricers.
    pub mod binary;

    /// Binomial option pricers.
    pub mod binomial;

    /// Generalised Black-Scholes-Merton option pricer.
    pub mod black_scholes_merton;

    /// Forward start options pricers.
    pub mod forward_start;

    /// Heston model option pricer.
    pub mod heston;

    /// Implied volatility functions.
    pub mod implied_volatility;

    /// Lookback option pricers.
    pub mod lookback;

    /// Merton (1976) jump diffusion model.
    pub mod merton_jump_diffusion;

    /// Base option traits.
    pub mod option;

    /// Power option pricers.
    pub mod power;
}
pub use options::*;
