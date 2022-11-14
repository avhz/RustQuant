//! RustQuant: A Rust library for quantitative finance tools.

#![allow(non_snake_case)]
#![deny(missing_docs)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Mathematics and statistics modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use math::*;

/// Module for mathematical and statistical tools.
mod math {
    /// Submodule of `math` that implements interpolation solvers.
    pub mod interpolation;
    /// Submodule of `math` that implements Newton-Raphson method.
    pub mod newton_raphson;
    /// Submodule of `math` that implements normal distribution functions.
    pub mod normal_distribution;
    /// Submodule of `math` that implements simple risk/reward functions.
    pub mod risk_reward;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Monte Carlo simulators/engines.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use monte_carlo::*;

/// Module for Monte Carlo engines to generate time series.
mod monte_carlo {
    /// Submodule of `monte_carlo` to simulate Geometric Brownian Motion.
    pub mod geometric_brownian_motion;
    // /// Submodule of `monte_carlo` to simulate Ornstein-Uhlenbeck process.
    // pub mod ornstein_uhlenbeck;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Automatic Differentiation modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// pub use AAD::*;

// mod AAD {
//     pub mod chain;
//     pub mod dcc;
//     pub mod tape;
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Option pricing modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use options::*;

/// Module containing option pricers and sensitivity functions.
mod options {
    /// Submodule of `options` that implements American option pricers.
    pub mod american;
    /// Submodule of `options` that implements Asian option pricers.
    pub mod asian;
    /// Submodule of `options` that implements Barrier option pricers.
    pub mod barrier;
    /// Submodule of `options` that implements Binomial option pricers.
    pub mod binomial;
    /// Submodule of `options` that implements European option pricers.
    pub mod european;
    /// Submodule of `options` that implements option Greeks/sensitivities.
    pub mod greeks;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Miscellaneous modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod helpers;

pub use helpers::*;
