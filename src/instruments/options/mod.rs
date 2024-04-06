// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

/// Finite Difference Pricer
pub mod finite_difference_pricer;
