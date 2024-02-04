// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MODELS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing all models (e.g. Black-Scholes, Heston, etc).
//! Also a `Model` trait is defined here for all models to implement.

/// Model trait.
pub mod model;
pub use model::*;

/// Arithmetic Brownian Motion.
pub mod arithmetic_brownian_motion;
pub use arithmetic_brownian_motion::*;

/// Black-Derman-Toy.
pub mod black_derman_toy;
pub use black_derman_toy::*;

/// Brownian Motion.
pub mod brownian_motion;
pub use brownian_motion::*;

/// Constant Elasticity of Variance.
pub mod constant_elasticity_of_variance;
pub use constant_elasticity_of_variance::*;

/// Cox-Ingersoll-Ross.
pub mod cox_ingersoll_ross;
pub use cox_ingersoll_ross::*;

/// Extended Vasicek.
pub mod extended_vasicek;
pub use extended_vasicek::*;

/// Fractional Brownian Motion.
pub mod fractional_brownian_motion;
pub use fractional_brownian_motion::*;

/// Fractional Cox-Ingersoll-Ross.
pub mod fractional_cox_ingersoll_ross;
pub use fractional_cox_ingersoll_ross::*;

/// Fractional Ornstein-Uhlenbeck.
pub mod fractional_ornstein_uhlenbeck;
pub use fractional_ornstein_uhlenbeck::*;

/// Geometric Brownian Bridge.
pub mod geometric_brownian_bridge;
pub use geometric_brownian_bridge::*;

/// Geometric Brownian Motion.
pub mod geometric_brownian_motion;
pub use geometric_brownian_motion::*;

/// Heston stochastic volatility model.
pub mod heston;
pub use heston::*;

/// Ho-Lee.
pub mod ho_lee;
pub use ho_lee::*;

/// Hull-White.
pub mod hull_white;
pub use hull_white::*;

/// Merton Jump Diffusion.
pub mod merton_jump_diffusion;
pub use merton_jump_diffusion::*;

/// Model parameters.
pub mod model_parameter;
pub use model_parameter::*;

/// Ornstein-Uhlenbeck.
pub mod ornstein_uhlenbeck;
pub use ornstein_uhlenbeck::*;

/// SABR: Stochastic Alpha, Beta, Rho.
pub mod sabr;
pub use sabr::*;
