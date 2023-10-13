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
//! ```rust
//! use RustQuant::options::*;
//!
//! fn main() {
//!     let VanillaOption = EuropeanOption {
//!         initial_price: 100.0,
//!         strike_price: 110.0,
//!         risk_free_rate: 0.05,
//!         volatility: 0.2,
//!         dividend_rate: 0.02,
//!         time_to_maturity: 0.5,
//!     };
//!
//!     let prices = VanillaOption.price();
//!
//!     println!("Call price = {}", prices.0);
//!     println!("Put price = {}", prices.1);
//! }
//! ```

pub use bonds::*;
pub use instrument::*;
pub use options::*;

/// Base trait for all instruments.
pub mod instrument;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BONDS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OPTION PRICING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Option pricers and sensitivity functions.
pub mod options {
    pub use crate::instruments::options::{
        american::*, asian::*, bachelier::*, barrier::*, binary::*, binomial::*,
        black_scholes_merton::*, european::*, forward_start::*, greeks::*, heston::*, lookback::*,
        option::*, power::*,
    };

    /// American option pricers.
    pub mod american;
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
    /// European option pricers.
    pub mod european;
    /// Forward start options pricers.
    pub mod forward_start;
    /// European option Greeks/sensitivities.
    pub mod greeks;
    /// Heston model option pricer.
    pub mod heston;
    /// Lookback option pricers.
    pub mod lookback;
    /// Base option traits.
    pub mod option;
    /// Power option pricers.
    pub mod power;
}
