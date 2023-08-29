// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! RustQuant: A Rust library for quantitative finance.
//!
//! Contact: rustquantcontact@gmail.com
//!
//! This library is a work in progress.
//! Any contributions are greatly appreciated.

// Strictly enforce documentation.
#![forbid(missing_docs)]
// Allow snake case.
// This is because much of this library is based on mathematics, so I
// want to adhere to the standard mathematical notation.
#![allow(non_snake_case)]
// Strictly enforce SAFETY comments.
// There is no unsafe code currently, but for anyone to add any, it must be
// documented with a SAFETY comment.
#![forbid(clippy::undocumented_unsafe_blocks)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RUSTQUANT MISCELLANEOUS MODULES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use {error::*, macros::*};

/// Submodule of `utilities`: implements useful macros, such as `assert_approx_equal`.
#[macro_use]
pub mod macros;

/// Module containing the RustQuant `Error` type.
pub mod error;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// AUTOMATIC DIFFERENTIATION MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: automatic differentation modules.
pub mod autodiff {
    pub use crate::autodiff::{
        accumulate::*,
        gradient::*,
        graph::*,
        graphviz::*,
        overloading::*,
        overloading::{
            add::*, div::*, f64::*, iter::*, log::*, minmax::*, mul::*, pow::*, statrs::*, sub::*,
        },
        variables::{nalgebra::*, ndarray::*, variable::*},
        vertex::*,
    };

    /// Submodule of `autodiff`: `Accumulate` trait.
    pub mod accumulate;
    /// Submodule of `autodiff`: implements the gradient computation.
    pub mod gradient;
    /// Submodule of `autodiff`: implements the Graph (aka. tape or Wengert List).
    pub mod graph;
    /// Submodule of `autodiff`: visualisation of the `Graph`.
    pub mod graphviz;
    /// Submodule of `autodiff`: implements `Vertex` for `autodiff`.
    pub mod vertex;

    /// Submodule of `autodiff`: implements operator/function overloading.
    /// This module contains the overloaded operators and primitive functions.
    /// Operations such as `+` and `*` are redefined, along with primitive
    /// functions such as `sin`, `exp`, and `log`.
    /// Each overload has an associated test to ensure functionality.
    pub mod overloading {
        /// Overload the standard addition operator (`+`).
        pub mod add;
        /// Overload the standard division operator (`/`).
        pub mod div;
        /// Overload the standard f64 type methods.
        pub mod f64;
        /// Overload the iterator traits.
        pub mod iter;
        /// Overload the standard logarithm function (`log`).
        pub mod log;
        /// Overload the standard min/max functions (`min` and `max`).
        pub mod minmax;
        /// Overload the standard multiplication operator (`*`).
        pub mod mul;
        /// Overload the power functions.
        pub mod pow;
        /// Overloading functions from `statrs`.
        pub mod statrs;
        /// Overload the standard subtraction operator (`-`).
        pub mod sub;
    }

    /// Submodule of `autodiff`: implements `Variable`s for `autodiff`.
    pub mod variables {
        /// Implements `Variable`s for `nalgebra`.
        pub mod nalgebra;
        /// Implements `Variable`s for `ndarray`.
        pub mod ndarray;
        /// Submodule of `autodiff`: base trait for all `Variable`s.
        pub mod variable;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// INSTRUMENTS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module defining base traits for financial instruments.
/// Also contains all instrument modules.
pub mod instruments {
    pub use crate::instruments::instrument::*;

    /// Submodule of `instruments`: base trait for all instruments.
    pub mod instrument;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // BONDS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Parent module containing: bond pricing models.
    pub mod bonds {
        pub use crate::instruments::bonds::{bond::*, cox_ingersoll_ross::*, vasicek::*};

        /// Submodule of `bonds`: contains the generic bond traits.
        pub mod bond;
        /// Submodule of `bonds`: implements Cox-Ingersoll-Ross bond pricing model.
        pub mod cox_ingersoll_ross;
        /// Submodule of `bonds`: implements one-factor Hull-White bond pricing model.
        pub mod hull_white;
        /// Submodule of `bonds`: implements Vasicek bond pricing model.
        pub mod vasicek;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // OPTION PRICING MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Parent module containing: option pricers and sensitivity functions.
    pub mod options {
        pub use crate::instruments::options::{
            american::*, asian::*, barrier::*, binary::*, binomial::*, european::*,
            forward_start::*, greeks::*, heston::*, lookback::*, option::*,
        };

        /// Submodule of `options`: implements American option pricers.
        pub mod american;
        /// Submodule of `options`: implements Asian option pricers.
        pub mod asian;
        /// Submodule of `options`: implements Barrier option pricers.
        pub mod barrier;
        /// Submodule of `options`: implements Binary option pricers.
        pub mod binary;
        /// Submodule of `options`: implements Binomial option pricers.
        pub mod binomial;
        /// Submodule of `options`: implements European option pricers.
        pub mod european;
        /// Submodule of `options`: forward start options.
        pub mod forward_start;
        /// Submodule of `options`: implements option Greeks/sensitivities.
        pub mod greeks;
        /// Submodule of `options`: implements the Heston model.
        pub mod heston;
        /// Submodule of `options`: implements Lookback options.
        pub mod lookback;
        /// Submodule of `options`: base option traits.
        pub mod option;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONEY RELATED ITEMS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing all money related items.
/// This includes currencies, cashflows, exchange rates, and money types,
/// among other things.
pub mod money {
    pub use crate::money::{cashflows::*, currency::*, exchange::*, iso_currencies::*, quotes::*};

    /// Submodule of `money`: cashflow definitions.
    pub mod cashflows;
    /// Submodule of `currencies`: currency data struct.
    pub mod currency;
    /// Submodule of `currencies`: currency exchange rates.
    pub mod exchange;
    /// Submodule of `currencies`: global currencies defined by ISO 4217.
    pub mod iso_currencies;
    /// Submodule of `money`: legs definitions.
    pub mod legs;
    /// Submodule of `money`: quotes definitions.
    pub mod quotes;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STATISTICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: all statistics related items.
pub mod statistics {
    pub use crate::statistics::{copulas::*, distributions::*, statistic::*};

    /// Submodule of `statistics`: base trait for statistics.
    pub mod statistic;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // COPULAS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Parent module containing: copula implementations.
    pub mod copulas {}

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // DISTRIBUTIONS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Parent module containing: random variable distributions (PDFs, CDFs, CFs, etc).
    pub mod distributions {
        pub use crate::statistics::distributions::{
            bernoulli::*, binomial::*, chi_squared::*, distribution::*, exponential::*, gamma::*,
            gaussian::*, poisson::*, uniform::*,
        };

        /// Submodule of `distributions`: the Bernoulli distribution.
        pub mod bernoulli;
        /// Submodule of `distributions`: the Binomial distribution.
        pub mod binomial;
        /// Submodule of `distributions`: the Chi-Squared distribution.
        pub mod chi_squared;
        /// Submodule of `distributions`: base trait for all distributions.
        pub mod distribution;
        /// Submodule of `distributions`: the Exponential distribution.
        pub mod exponential;
        /// Submodule of `distributions`: the Gamma distribution.
        pub mod gamma;
        /// Submodule of `distributions`: the Gaussian (normal) distribution.
        pub mod gaussian;
        /// Submodule of `distributions`: the Poisson distribution.
        pub mod poisson;
        /// Submodule of `distributions`: the Uniform distribution.
        pub mod uniform;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// DATA MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: data reading and writing utilities.
/// Disabled by default, due to Polars increasing compile times.
#[cfg(feature = "data")]
pub mod data {
    pub use crate::data::{io::*, yahoo::*};

    /// Submodule of `data`: file reading and writing.
    pub mod io;
    /// Submodule of `data`: Yahoo! Finance data reader.
    pub mod yahoo;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MATHEMATICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: mathematical and statistical tools.
pub mod math {
    pub use crate::math::{
        cumsum::*, fft::*, integration::constants::*, integration::midpoint::*,
        integration::simpsons::*, integration::tanhsinh::*, integration::trapezoid::*,
        interpolation::*, linspace::*, newton_raphson::*, risk_reward::*, sequence::*,
    };

    /// Submodule of `math`: implements numerical integration prodecures.
    /// The primary integrator is the Tanh-Sinh implementation.
    /// The midpoint, trapezoid, and Simpson integrators are innacurate.
    pub mod integration {
        /// Constants used in numerical integration.
        pub mod constants;
        /// Composite Midpoint rule.
        pub mod midpoint;
        /// Composite Simpson's 3/8 rule.
        pub mod simpsons;
        /// Tanh-Sinh (double exponential) quadrature.
        pub mod tanhsinh;
        /// Composite Trapezoidal rule.
        pub mod trapezoid;
    }
    /// Submodule of `math`: implements numerical optimization procedures.
    pub mod optimization {
        /// Gradient descent optimization.
        pub mod gradient_descent;
        // pub mod bisection;
        // pub mod brent;
        // pub mod golden_section;
        // pub mod newton;
        // pub mod newton_raphson;
        // pub mod secant;
    }
    /// Submodule of `utilities`: implements the cumulative sum of a vector.
    pub mod cumsum;
    /// Submodule of `math`: implements a fast fourier transform function
    pub mod fft;
    /// Submodule of `math`: implements interpolation solvers.
    pub mod interpolation;
    /// Submodule of `utilities`: implements generating a linearly spaced sequence.
    pub mod linspace;
    /// Submodule of `math`: implements Newton-Raphson method.
    pub mod newton_raphson;
    /// Submodule of `math`: implements simple risk/reward functions.
    pub mod risk_reward;
    /// Submodule of `utilities`: sequences of numbers.
    pub mod sequence;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONTE CARLO SIMULATION AND STOCHASTIC PROCESSES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: Monte Carlo engines to simulate stochastic processes.
pub mod stochastics {
    pub use crate::stochastics::{
        arithmetic_brownian_motion::*, black_derman_toy::*, brownian_motion::*,
        cox_ingersoll_ross::*, extended_vasicek::*, fractional_brownian_motion::*,
        geometric_brownian_motion::*, ho_lee::*, hull_white::*, ornstein_uhlenbeck::*, process::*,
    };

    /// Submodule of `stochastics`: implements Arithmetic Brownian Motion.
    pub mod arithmetic_brownian_motion;
    /// Submodule of `stochastics`: implements Black-Derman-Toy short rate model.
    pub mod black_derman_toy;
    /// Submodule of `stochastics`: implements Standard Brownian Motion.
    pub mod brownian_motion;
    /// Submodule of `stochastics`: implements the Cox-Ingersoll-Ross process.
    pub mod cox_ingersoll_ross;
    /// Submodule of `stochastics`: implements the extended Vasicek process.
    pub mod extended_vasicek;
    /// Submodule of `stochastics`: implements Fractional Brownian Motion.
    pub mod fractional_brownian_motion;
    /// Submodule of `stochastics`: implements Geometric Brownian Motion.
    pub mod geometric_brownian_motion;
    /// Submodule of `stochastics`: implements Ho-Lee process.
    pub mod ho_lee;
    /// Submodule of `stochastics`: implements the Hull-White model process.
    pub mod hull_white;
    /// Submodule of `stochastics`: implements the Ornstein-Uhlenbeck process.
    pub mod ornstein_uhlenbeck;
    /// Submodule of `stochastics`: defines `Trajectories` and `StochasticProcess`.
    pub mod process;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TRADING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: trading related items.
pub mod trading {
    // pub use crate::trading::order_book::*;

    /// Submodule of `trading`: contains limit order book implementation
    pub mod limit_order_book;
    /// Submodule of `trading`: order definition.
    pub mod order;
    /// Submodule of `trading`: contains a limit orderbook (LOB) implementation.
    pub mod order_book;
    /// Submodule of `trading`: order lifespan definitions.
    pub mod order_lifespan;
    /// Submodule of `trading`: order side definitions.
    pub mod order_side;
    /// Submodule of `trading`: order types definitions.
    pub mod order_type;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TIME AND DATE MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: time and date functionality.
pub mod time {
    pub use crate::time::{
        calendar::*,
        calendars::{australia::*, austria::*, canada::*, united_kingdom::*, united_states::*},
        constants::*,
        conventions::*,
        daycount::*,
        schedule::*,
    };

    /// Submodule of `time`: calendar definitions.
    pub mod calendar;
    /// Submodule of `time`: date/time constants
    pub mod constants;
    /// Submodule of `time`: day count and business day conventions.
    pub mod conventions;
    /// Submodule of `time`: daycount definitions.
    pub mod daycount;
    /// Submodule of `time`: scheduling definitions.
    pub mod schedule;

    /// Submodule of `time`: contains calendar definitions for settlement purposes.
    pub mod calendars {
        /// Submodule of `calendars`: contains Australia calendar.
        pub mod australia;
        /// Submodule of `calendars`: contains Austria calendar.
        pub mod austria;
        /// Submodule of `calendars`: contains Canadian settlement calendar.
        pub mod canada;
        /// Submodule of `calendars`: contains UK calendar.
        pub mod united_kingdom;
        /// Submodule of `calendars`: contains USA calendar.
        pub mod united_states;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MACHINE LEARNING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: machine learning functionality.
/// This module relies on the `nalgebra` crate.
pub mod ml {
    pub use crate::ml::activations::*;
    pub use crate::ml::regression::{linear::*, logistic::*};

    /// Submodule of `ml`: regression implentations.
    pub mod regression {
        pub mod linear;
        pub mod logistic;
    }
    /// Submodule of `ml`: activation functions.
    pub mod activations;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RUSTQUANT INTERACTIVE
// This is the parent module for the interactive TUI.
// All boilerplate currently taken from:
// https://www.monkeypatch.io/blog/2021-05-31-rust-tui
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: RustQuant interactive TUI.
#[cfg(feature = "interactive")]
pub mod interactive {}
