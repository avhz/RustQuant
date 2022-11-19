//! RustQuant: A Rust library for quantitative finance tools.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Miscellaneous modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use helpers::*;

/// Parent module containing: helper functions used throughout the library.
#[macro_use]
pub mod helpers {
    /// Submodule of `helpers` that implements the cumulative sum of a vector.
    pub mod cumsum;
    /// Submodule of `helpers` that implements generating a linearly spaced sequence.
    pub mod linspace;
    /// Submodule of `helpers` that implements useful macros, such as `assert_approx_equal`.
    pub mod macros;
    /// Submodule of `helpers` that implements plotting/writing vectors to files.
    pub mod plot;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Mathematics and statistics modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use math::*;

/// Parent module containing: mathematical and statistical tools.
pub mod math {
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

pub use stochastics::*;

/// Parent module containing: Monte Carlo engines to simulate stochastic processes.
pub mod stochastics {
    /// Submodule of `stochastics` that implements Geometric Brownian Motion.
    pub mod geometric_brownian_motion;
    //  Submodule of `stochastics` that implements Ornstein-Uhlenbeck process.
    // pub mod ornstein_uhlenbeck;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Automatic Differentiation modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Option pricing modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use options::*;

/// Parent module containing: option pricers and sensitivity functions.
pub mod options {
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
