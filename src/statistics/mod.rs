// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STATISTICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Statistics related items.
//!
//! Probability density/mass functions, distribution functions, characteristic functions, etc.
//!
//! - [x] Gaussian
//! - [x] Bernoulli
//! - [x] Binomial
//! - [x] Poisson
//! - [x] Uniform (discrete & continuous)
//! - [x] Chi-Squared
//! - [x] Gamma
//! - [x] Exponential

/// Base trait for statistics of a collection of data.
pub mod statistic;
pub use statistic::*;

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
pub use distributions::*;

// /// Copula implementations.
// pub mod copulas;
// pub use copulas::*;
