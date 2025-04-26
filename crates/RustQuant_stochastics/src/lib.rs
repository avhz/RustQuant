// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONTE CARLO SIMULATION AND STOCHASTIC PROCESSES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Monte Carlo engines to simulate stochastic processes.
//!
//! The following is a list of stochastic processes that can be generated.
//!
//! - Brownian Motions:
//!   - Standard Brownian Motion
//!     - $dX(t) = dW(t)$
//!   - Arithmetic Brownian Motion
//!     - $dX(t) = \mu dt + \sigma dW(t)$
//!   - Geometric Brownian Motion
//!     - $dX(t) = \mu X(t) dt + \sigma X(t) dW(t)$
//!   - Fractional Brownian Motion
//! - Cox-Ingersoll-Ross (1985)
//!   - $dX(t) = \left[ \theta - \alpha X(t) \right] dt + \sigma \sqrt{r_t} dW(t)$
//! - Ornstein-Uhlenbeck process
//!   - $dX(t) = \theta \left[ \mu - X(t) \right] dt + \sigma dW(t)$
//! - Ho-Lee (1986)
//!   - $dX(t) = \theta(t) dt + \sigma dW(t)$
//! - Hull-White (1990)
//!   - $dX(t) = \left[ \theta(t) - \alpha X(t) \right]dt + \sigma dW(t)$
//! - Extended Vasicek (1990)
//!   - $dX(t) = \left[ \theta(t) - \alpha(t) X(t) \right] dt + \sigma dW(t)$
//! - Black-Derman-Toy (1990)
//!   - $d\ln[X(t)] = \left[ \theta(t) + \frac{\sigma'(t)}{\sigma(t)}\ln[X(t)] \right]dt + \sigma_t dW(t)$
//!
//! ```rust
//! use RustQuant::stochastics::*;
//!
//! // Create new GBM with mu and sigma.
//! let gbm = GeometricBrownianMotion::new(0.05, 0.9);
//!
//! // Generate path using Euler-Maruyama scheme.
//! // Parameters: x_0, t_0, t_n, n, sims, parallel.
//! let config = StochasticProcessConfig::new(10., 0., 0.5, 10, StochasticScheme::EulerMaruyama, 1, false, None);
//! let output = (&gbm).generate(&config);
//!
//! println!("GBM = {:?}", output.paths);
//! ```

/// Arithmetic Brownian Motion.
pub mod arithmetic_brownian_motion;
pub use arithmetic_brownian_motion::*;

/// Black-Derman-Toy short rate model.
pub mod black_derman_toy;
pub use black_derman_toy::*;

/// Standard Brownian Motion.
pub mod brownian_motion;
pub use brownian_motion::*;

/// Constant Elasticity of Variance process.
pub mod constant_elasticity_of_variance;
pub use constant_elasticity_of_variance::*;

/// Cox-Ingersoll-Ross process.
pub mod cox_ingersoll_ross;
pub use cox_ingersoll_ross::*;

/// Extended Vasicek process.
pub mod extended_vasicek;
pub use extended_vasicek::*;

/// Fractional Brownian Motion.
pub mod fractional_brownian_motion;
pub use fractional_brownian_motion::*;

/// Fractional Cox-Ingersoll-Ross process.
pub mod fractional_cox_ingersoll_ross;
pub use fractional_cox_ingersoll_ross::*;

/// Fractional Ornstein-Uhlenbeck process.
pub mod fractional_ornstein_uhlenbeck;
pub use fractional_ornstein_uhlenbeck::*;

/// Geometric brownian bridge process.
pub mod geometric_brownian_bridge;
pub use geometric_brownian_bridge::*;

/// Geometric Brownian Motion.
pub mod geometric_brownian_motion;
pub use geometric_brownian_motion::*;

/// Heston model process.
pub mod heston;
pub use heston::*;

/// Ho-Lee process.
pub mod ho_lee;
pub use ho_lee::*;

/// Hull-White model process.
pub mod hull_white;
pub use hull_white::*;

/// Merton jump diffusion process.
pub mod merton_jump_diffusion;
pub use merton_jump_diffusion::*;

/// Ornstein-Uhlenbeck process.
pub mod ornstein_uhlenbeck;
pub use ornstein_uhlenbeck::*;

/// SABR model process.
pub mod sabr;
pub use sabr::*;

/// Defines `Trajectories` and `StochasticProcess`.
pub mod process;
pub use process::*;

/// Model parameter struct.
pub mod model_parameter;
pub use model_parameter::*;

/// Nelson-Siegel-Svensson model process.
pub mod nelson_siegel_svensson;
pub use nelson_siegel_svensson::*;

/// Curve model trait.
pub mod curve_model;
pub use curve_model::*;

/// Fractional Brownian Motion.
pub mod fractional_process;
pub use fractional_process::*;

/// Private stochastic process simulation module.
mod simulation;