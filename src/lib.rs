#![deny(missing_docs)]
#![allow(non_snake_case)]

//! RustQuant: A Rust library for quantitative finance tools.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BONDS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: bond pricing models.
pub mod bonds {
    pub use crate::bonds::{bond::*, cox_ingersoll_ross::*, vasicek::*};

    /// Submodule of `bonds`: contains the generic bond traits.
    pub mod bond;
    /// Submodule of `bonds`: implements Cox-Ingersoll-Ross bond pricing model.
    pub mod cox_ingersoll_ross;
    /// Submodule of `bonds`: implements Vasicek bond pricing model.
    pub mod vasicek;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// HELPERS AND UTILITIES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: helper functions used throughout the library.
#[macro_use]
pub mod helpers {
    pub use crate::helpers::{cumsum::*, linspace::*, macros::*, mean::*, minmax::*, plot::*};

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
// MATHEMATICS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: mathematical and statistical tools.
pub mod math {
    pub use crate::math::{
        integration::midpoint::*, integration::simpsons::*, integration::tanhsinh::*,
        integration::trapezoid::*, interpolation::*, newton_raphson::*, normal_distribution::*,
        risk_reward::*,
    };

    /// Submodule of `math`: implements numerical integration prodecures.
    /// The primary integrator is the Tanh-Sinh implementation.
    /// The midpoint, trapezoid, and Simpson integrators are innacurate.
    pub mod integration {
        /// Composite Midpoint rule.
        pub mod midpoint;
        /// Composite Simpson's 3/8 rule.
        pub mod simpsons;
        /// Tanh-Sinh (double exponential) quadrature.
        pub mod tanhsinh;
        /// Composite Trapezoidal rule.
        pub mod trapezoid;
    }
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
// RANDOM
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: random related stuff (random variables, PDFs, CDFs, CFs, etc).
pub mod distributions {
    pub use crate::distributions::{
        bernoulli::*, binomial::*, characteristic_functions::*, density_functions::*, poisson::*,
        uniform::*,
    };

    /// Submodule of `random`: characteristic functions (CFs) of common distributions.
    pub mod characteristic_functions;
    /// Submodule of `random`: density and mass functions (PDFs & PMFs) of common distributions.
    pub mod density_functions;

    /// Submodule of `distributions`: the Bernoulli distribution.
    pub mod bernoulli;
    /// Submodule of `distributions`: the Binomial distribution.
    pub mod binomial;
    /// Submodule of `distributions`: the Poisson distribution.
    pub mod poisson;
    /// Submodule of `distributions`: the Uniform distribution.
    pub mod uniform;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONTE CARLO SIMULATION AND STOCHASTIC PROCESSES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: Monte Carlo engines to simulate stochastic processes.
pub mod stochastics {
    pub use crate::stochastics::{
        brownian_motion::*, cox_ingersoll_ross::*, geometric_brownian_motion::*,
        ornstein_uhlenbeck::*, process::*,
    };

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
// AUTOMATIC DIFFERENTIATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: automatic differentation modules.
pub mod autodiff {
    pub use crate::autodiff::{gradient::*, overload::*, tape::*, variable::*};

    /// Submodule of `autodiff`: implements the gradient computation.
    pub mod gradient;
    /// Submodule of `autodiff`: implements operator/function overloading.
    pub mod overload;
    /// Submodule of `autodiff`: implements the Tape (Wengert List).
    pub mod tape;
    /// Submodule of `autodiff`: implements `Variable`s for `autodiff`.
    pub mod variable;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OPTION PRICING
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Parent module containing: option pricers and sensitivity functions.
pub mod options {
    pub use crate::options::{
        american::*, asian::*, barrier::*, binomial::*, european::*, greeks::*, heston::*,
        lookback::*, option::*,
    };

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
    /// Submodule of `options`: implements the Heston model.
    pub mod heston;
    /// Submodule of `options`: implements Lookback options.
    pub mod lookback;
    /// Submodule of `options`: base option traits.
    pub mod option;
}
