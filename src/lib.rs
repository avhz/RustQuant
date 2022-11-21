#![deny(missing_docs)]

//! RustQuant: A Rust library for quantitative finance tools.

/// Global import for user convenience.
pub mod prelude {
    // use crate::bonds;
    // use crate::gradients;
    pub use crate::helpers::*;
    pub use crate::math::*;
    pub use crate::options::*;
    pub use crate::stochastics::*;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Miscellaneous modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: helper functions used throughout the library.
#[macro_use]
pub mod helpers {
    /// Submodule of `helpers`: implements the cumulative sum of a vector.
    pub mod cumsum;
    /// Submodule of `helpers`: implements generating a linearly spaced sequence.
    pub mod linspace;
    /// Submodule of `helpers`: implements useful macros, such as `assert_approx_equal`.
    pub mod macros;
    /// Submodule of `helpers`: mean of a vector.
    pub mod mean;
    /// Submodule of `helpers`: min and max of a vector.
    pub mod minmax;
    /// Submodule of `helpers`: implements plotting/writing vectors to files.
    pub mod plot;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Mathematics and statistics modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// pub use math::*;

/// Parent module containing: mathematical and statistical tools.
pub mod math {
    /// Submodule of `math`: implements interpolation solvers.
    pub mod interpolation;
    /// Submodule of `math`: implements Newton-Raphson method.
    pub mod newton_raphson;
    /// Submodule of `math`: implements normal distribution functions.
    pub mod normal_distribution;
    /// Submodule of `math`: implements simple risk/reward functions.
    pub mod risk_reward;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Monte Carlo simulators/engines.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: Monte Carlo engines to simulate stochastic processes.
pub mod stochastics {
    /// Submodule of `stochastics`: implements Standard Brownian Motion.
    pub mod brownian_motion;
    /// Submodule of `stochastics`: implements the Cox-Ingersoll-Ross process.
    pub mod cox_ingersoll_ross;
    /// Submodule of `stochastics`: implements Geometric Brownian Motion.
    pub mod geometric_brownian_motion;
    /// Submodule of `stochastics`: implements the Ornstein-Uhlenbeck process.
    pub mod ornstein_uhlenbeck;
    /// Submodule of `stochastics`: defines `Trajectories` and `StochasticProcess`.
    pub mod process;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Automatic Differentiation modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Option pricing modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// pub use options::*;

/// Parent module containing: option pricers and sensitivity functions.
pub mod options {
    /// Submodule of `options`: implements American option pricers.
    pub mod american;
    /// Submodule of `options`: implements Asian option pricers.
    pub mod asian;
    /// Submodule of `options`: implements Barrier option pricers.
    pub mod barrier;
    /// Submodule of `options`: implements Binomial option pricers.
    pub mod binomial;
    /// Submodule of `options`: implements European option pricers.
    pub mod european;
    /// Submodule of `options`: implements option Greeks/sensitivities.
    pub mod greeks;
    /// Submodule of `options`: implements Lookback options.
    pub mod lookback;
    /// Submodule of `options`: base option traits.
    pub mod option;
}
