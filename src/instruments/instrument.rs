// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::Date;

/// Instrument trait
/// The trait provides a common interface for all instruments.
/// All instruments can be queried for their net present value (NPV) and
/// error (if available).
/// The valuation date is the date at which the instrument's NPV is
/// being calculated; for most instruments it is the trade date, for
/// some exotic products it might be the exercise date.
pub trait Instrument {
    /// Returns the price (net present value) of the instrument.
    fn price(&self) -> f64;

    /// Returns the error on the NPV in case the pricing engine can
    /// provide it (e.g. Monte Carlo pricing engine).
    fn error(&self) -> Option<f64>;

    /// Returns the date at which the NPV is calculated.
    fn valuation_date(&self) -> Date;

    /// Instrument type.
    fn instrument_type(&self) -> &'static str;
}

/// Price structure.
pub struct Price {
    /// Price of the instrument.
    pub price: f64,

    /// Error on the price of the instrument.
    pub error: Option<f64>,
}

/// Pricing engine for instruments.
pub enum PricingEngine {
    /// Analytic pricing method (e.g. closed-form solution).
    Analytic,

    /// Simulation pricing method (e.g. Monte Carlo).
    Simulation,

    /// Numerical method (e.g. PDE, lattice, finite differences).
    Numerical,
}

/// Path independent payoff trait.
pub trait PathIndependentPayoff {
    /// Base method for path independent option payoffs.
    fn payoff(&self, underlying: f64) -> f64;
}

/// Path dependent payoff trait.
pub trait PathDependentPayoff {
    /// Base method for path dependent option payoffs.
    fn payoff(&self, path: &[f64]) -> f64;
}

// trait Payoff {
//     fn path_dependent(&self, path: &[f64]) -> f64;
//     fn path_independent(&self, path: &[f64]) -> f64;
// }

// struct MonteCarloPricer<PAYOFF, MODEL>
// where
//     PAYOFF: crate::instruments::PathDependentPayoff,
//     MODEL: crate::stochastics::StochasticProcess,
// {
//     payoff: PAYOFF,
//     model: MODEL,
// }

// impl PathDependentPayoff for EuropeanOption {
//     fn payoff(&self, path: &[f64]) -> f64 {
//         let spot = path.last().unwrap();
//         match self.option_type {
//             OptionType::Call => (spot - self.strike_price).max(0.0),
//             OptionType::Put => (self.strike_price - spot).max(0.0),
//         }
//     }
// }
