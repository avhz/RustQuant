// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! A Rust library for quantitative finance.
//!
//! Contact: <RustQuantContact@gmail.com>
//!
//! Any contributions are greatly appreciated. Make a PR or open an issue !

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

pub use {error::*, macros::*, portfolio::*};

/// Macros module.
#[macro_use]
pub mod macros;

/// RustQuant error module.
pub mod error;

/// Portfolio module.
pub mod portfolio;

/// Curves module.
/// Curves (in the financial sense) are functions that map
/// a time to a value, such as a yield curve or a swap curve.
/// They may also be known as term structures.
pub mod curves {
    pub use crate::curves::curve::*;

    /// Base curve trait.
    pub mod curve;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// AUTOMATIC DIFFERENTIATION MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Reverse mode automatic differentation.
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

    /// [`Accumulate`] trait.
    pub mod accumulate;
    /// Implements the gradient computation.
    pub mod gradient;
    /// The Graph (aka. tape or Wengert List).
    pub mod graph;
    /// Visualisation of the [`Graph`].
    pub mod graphviz;
    /// Implements [`Vertex`] (nodes) for the `Graph`.
    pub mod vertex;

    /// Operator/function overloading.
    /// This module contains the overloaded operators and primitive functions.
    /// In Griewank and Walther - Evaluating Derivatives, they refer to this
    /// as the "elemental library".
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

    /// `Variable`s for `autodiff`.
    pub mod variables {
        /// Implements `Variable`s for `nalgebra`.
        pub mod nalgebra;
        /// Implements `Variable`s for `ndarray`.
        pub mod ndarray;
        /// Base trait for all `Variable`s.
        pub mod variable;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// INSTRUMENTS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Financial instrument types and modules (bonds, options, etc).
pub mod instruments {
    pub use crate::instruments::instrument::*;

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
            black_scholes_merton::*, european::*, forward_start::*, greeks::*, heston::*,
            lookback::*, option::*, power::*,
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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONEY RELATED ITEMS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Module containing all money related items.
/// This includes currencies, cashflows, exchange rates, and money types,
/// among other things.
pub mod money {
    pub use crate::money::{cashflows::*, currency::*, exchange::*, iso_currencies::*, quotes::*};

    /// Cashflow definitions.
    pub mod cashflows;
    /// Currency data struct.
    pub mod currency;
    /// Currency exchange rate helpers.
    pub mod exchange;
    /// Global currencies defined by ISO 4217.
    pub mod iso_currencies;
    /// Legs (sequence of cashflows).
    pub mod legs;
    /// Quotes (price, yield, etc).
    pub mod quotes;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STATISTICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Statistics related items.
pub mod statistics {
    pub use crate::statistics::{copulas::*, distributions::*, statistic::*};

    /// Base trait for statistics of a collection of data.
    pub mod statistic;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // COPULAS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Copula implementations.
    pub mod copulas {}

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // DISTRIBUTIONS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Random variable distributions (PDFs, CDFs, CFs, etc).
    pub mod distributions {
        pub use crate::statistics::distributions::{
            bernoulli::*, binomial::*, chi_squared::*, distribution::*, exponential::*, gamma::*,
            gaussian::*, poisson::*, uniform::*,
        };

        /// Bernoulli distribution.
        pub mod bernoulli;
        /// Binomial distribution.
        pub mod binomial;
        /// Chi-Squared distribution.
        pub mod chi_squared;
        /// Base trait for all distributions.
        pub mod distribution;
        /// Exponential distribution.
        pub mod exponential;
        /// Gamma distribution.
        pub mod gamma;
        /// Gaussian (normal) distribution.
        pub mod gaussian;
        /// Poisson distribution.
        pub mod poisson;
        /// Uniform distribution.
        pub mod uniform;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// DATA MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Data reading and writing utilities.
/// Disabled by default, due to Polars increasing compile times.
#[cfg(feature = "data")]
pub mod data {
    pub use crate::data::{io::*, yahoo::*};

    /// File reading and writing.
    pub mod io;
    /// Yahoo! Finance data reader.
    pub mod yahoo;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MATHEMATICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Mathematics related items.
pub mod math {
    pub use crate::math::{
        fft::*, integration::constants::*, integration::midpoint::*, integration::simpsons::*,
        integration::tanhsinh::*, integration::trapezoid::*, interpolation::*,
        optimization::gradient_descent::*, optimization::newton_raphson::*, risk_reward::*,
        sequences::*,
    };

    /// Numerical integration routines.
    /// The primary (useful) integrator is the Tanh-Sinh (double exponential) implementation.
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

    /// Numerical optimization and root-finding routines.
    pub mod optimization {
        /// Gradient descent optimization.
        pub mod gradient_descent;
        /// Newton-Raphson method.
        pub mod newton_raphson;

        // pub mod bisection;
        // pub mod brent;
        // pub mod golden_section;
        // pub mod newton;
        // pub mod newton_raphson;
        // pub mod secant;
    }

    /// Sequences of numbers and associated functions.
    pub mod sequences {
        pub use crate::math::sequences::{cumsum::*, linspace::*, sequence::*};

        /// Cumulative sum of a vector.
        pub mod cumsum;
        /// Generate a linearly spaced sequence.
        pub mod linspace;
        /// Sequences of numbers.
        pub mod sequence;
    }

    /// Fast fourier transform.
    pub mod fft;
    /// Interpolation routines.
    pub mod interpolation;
    /// Simple risk/reward measures.
    pub mod risk_reward;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONTE CARLO SIMULATION AND STOCHASTIC PROCESSES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Monte Carlo engines to simulate stochastic processes.
pub mod stochastics {
    pub use crate::stochastics::{
        arithmetic_brownian_motion::*, black_derman_toy::*, brownian_motion::*,
        cox_ingersoll_ross::*, extended_vasicek::*, fractional_brownian_motion::*,
        geometric_brownian_motion::*, ho_lee::*, hull_white::*, ornstein_uhlenbeck::*, process::*,
    };

    /// Arithmetic Brownian Motion.
    pub mod arithmetic_brownian_motion;
    /// Black-Derman-Toy short rate model.
    pub mod black_derman_toy;
    /// Standard Brownian Motion.
    pub mod brownian_motion;
    /// Cox-Ingersoll-Ross process.
    pub mod cox_ingersoll_ross;
    /// Extended Vasicek process.
    pub mod extended_vasicek;
    /// Fractional Brownian Motion.
    pub mod fractional_brownian_motion;
    /// Geometric Brownian Motion.
    pub mod geometric_brownian_motion;
    /// Ho-Lee process.
    pub mod ho_lee;
    /// Hull-White model process.
    pub mod hull_white;
    /// Ornstein-Uhlenbeck process.
    pub mod ornstein_uhlenbeck;
    /// Defines `Trajectories` and `StochasticProcess`.
    pub mod process;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TRADING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Trading related items.
pub mod trading {
    /// Contains limit order book implementation
    pub mod limit_order_book;
    /// Order definition.
    pub mod order;
    /// Contains a limit orderbook (LOB) implementation.
    pub mod order_book;
    /// Order lifespan definitions.
    pub mod order_lifespan;
    /// Order side definitions.
    pub mod order_side;
    /// Order types definitions.
    pub mod order_type;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TIME AND DATE MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Time and date functionality.
pub mod time {
    pub use crate::time::{
        calendar::*,
        calendars::{australia::*, austria::*, canada::*, united_kingdom::*, united_states::*},
        constants::*,
        conventions::*,
        daycount::*,
        schedule::*,
    };

    /// Calendar definitions.
    pub mod calendar;
    /// Date/time constants
    pub mod constants;
    /// Day count and business day conventions.
    pub mod conventions;
    /// Daycount definitions.
    pub mod daycount;
    /// Scheduling definitions.
    pub mod schedule;

    /// Calendar definitions for settlement purposes.
    pub mod calendars {
        /// Australian settlement calendar.
        pub mod australia;
        /// Austrian settlement calendar.
        pub mod austria;
        /// Canadian settlement calendar.
        pub mod canada;
        /// UK settlement calendar.
        pub mod united_kingdom;
        /// USA settlement calendar.
        pub mod united_states;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MACHINE LEARNING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Machine learning algorithms. This module relies on the [`nalgebra`] crate.
pub mod ml {
    pub use crate::ml::activations::*;
    pub use crate::ml::regression::{linear::*, logistic::*};

    /// Regression algorithms.
    pub mod regression {
        pub mod linear;
        pub mod logistic;
    }
    /// Submodule of `ml`: activation functions.
    pub mod activations;
}
