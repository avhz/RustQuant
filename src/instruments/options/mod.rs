// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use crate::instruments::options::{
    black_scholes_merton::*, implied_volatility::*, merton_jump_diffusion::*, option_contract::*,
};

/// Asian option pricers.
pub mod asian;
pub use asian::*;

// /// Bachelier option pricer.
// pub mod bachelier;

/// Barrier option pricers.
pub mod barrier;
pub use barrier::*;

/// Binary option pricers.
pub mod binary;
pub use binary::*;

// /// Binomial option pricers.
// pub mod binomial;

/// Generalised Black-Scholes-Merton option pricer.
pub mod black_scholes_merton;

// /// Forward start options pricers.
// pub mod forward_start;

// /// Heston model option pricer.
// pub mod heston;

/// Implied volatility functions.
pub mod implied_volatility;

// /// Lookback option pricers.
// pub mod lookback;

/// Merton (1976) jump diffusion model.
pub mod merton_jump_diffusion;

/// Base option traits.
pub mod option_contract;
pub use option_contract::*;

/// Power options.
pub mod power;
pub use power::*;

/// Finite Difference Pricer
pub mod finite_difference_pricer;

/// Option flags.
pub mod option_flags;
pub use option_flags::*;

/// Vanilla option.
pub mod vanilla;
pub use vanilla::*;

/// Supershare options.
pub mod supershare;
pub use supershare::*;
