// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Option contract data.
pub struct OptionContract {
    /// The option's type flag (call or put).
    pub type_flag: TypeFlag,

    /// The option's strike type (fixed or floating).
    pub strike_flag: StrikeFlag,

    /// The option's exercise type (European, American, Bermudan).
    pub exercise_flag: ExerciseFlag,
}

/// Option type enum.
#[derive(Debug, Clone, Copy)]
pub enum TypeFlag {
    /// Call option (right to BUY the underlying asset).
    Call = 1,

    /// Put option (right to SELL the underlying asset).
    Put = -1,
}

/// American/European option type enum.
#[derive(Debug, Clone, Copy)]
pub enum ExerciseFlag {
    /// European option (can only be exercised at expiry).
    European,

    /// American option (can be exercised at any time before expiry).
    American,

    /// Bermudan option (can be exercised at specific dates before expiry).
    Bermudan,
}

/// Option strike type enum.
#[derive(Debug, Clone, Copy)]
pub enum StrikeFlag {
    /// Strike is fixed.
    Fixed,

    /// Strike is floating (e.g. strike = S_max).
    Floating,
}

/// Generic option parameters struct.
/// Contains the common parameters (as in Black-Scholes).
/// Other option types may have additional parameters,
/// such as lookback options (S_min, S_max).
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct OptionParameters {
    /// `S` - Initial price of the underlying.
    pub S: Vec<f64>,
    /// `K` - Strike price.
    pub K: Vec<f64>,
    /// `T` - Time to expiry/maturity.
    pub T: Vec<f64>,
    /// `r` - Risk-free rate parameter.
    pub r: Vec<f64>,
    /// `v` - Volatility parameter.
    pub v: Vec<f64>,
    /// `q` - Dividend rate.
    pub q: Vec<f64>,
}

impl OptionParameters {
    /// New option parameters struct initialiser.
    #[must_use]
    pub const fn new(
        initial_price: Vec<f64>,
        strike_price: Vec<f64>,
        risk_free_rate: Vec<f64>,
        volatility: Vec<f64>,
        dividend_rate: Vec<f64>,
        time_to_maturity: Vec<f64>,
    ) -> Self {
        Self {
            S: initial_price,
            K: strike_price,
            T: time_to_maturity,
            r: risk_free_rate,
            v: volatility,
            q: dividend_rate,
        }
    }
}

// pub trait PathIndependentOption {
//     fn price(&self) -> f64;
// }

// /// Path-dependent option trait.
// pub trait PathDependentOption {
//     /// Base method for path-dependent call option payoff.
//     fn call_payoff(&self, path: &[f64]) -> f64;

//     /// Base method for path-dependent put option payoff.
//     fn put_payoff(&self, path: &[f64]) -> f64;

//     /// Base method for path-dependent option prices using closed-form solution (call and put).
//     fn closed_form_prices(&self) -> (f64, f64);

//     /// Base method for path-dependent option prices using Monte Carlo (call and put).
//     fn monte_carlo_prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64);
// }

// /// General option trait.
// /// All option types must implement this trait.
// /// All option contracts have:
// ///     - `Prices` struct to store the option prices (call, put).
// ///     - `Parameters` struct to store the option parameters (S, K, T, etc...).
// ///     - `TypeFlag` enum to store the option type (call, put).
// ///     - `Greeks` struct to store the option Greeks (sensitivities).
// ///     - `prices` method to compute the option prices (call, put).
// ///     - `set_parameters` method to set the option parameters.
// ///     - `option_type` method to set the option type.
// ///     - `greeks` method to compute the option Greeks (sensitivities).
// pub trait OptionContract {
//     /// Option prices struct.
//     type Prices;
//     /// Option parameters struct.
//     type Parameters;
//     /// Option type enum (call or put).
//     type Type;
//     /// Option Greeks struct.
//     type Greeks;

//     /// Base method for computing the options prices (call and put).
//     fn prices(&self) -> Self::Prices;
//     /// Base method for setting the option parameters.
//     fn set_parameters(&self) -> Self::Parameters;
//     /// Base method for setting the option type.
//     fn option_type(&self) -> Self::Type;
//     /// Base method for computing the Greeks (sensitivities).
//     fn greeks(&self) -> Self::Greeks;
// }
