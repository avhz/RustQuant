//! # RustQuant Models
//!
//! This crate provides the low-level model structs and implementations for various financial models.

/// Asay 1982 model.
pub mod asay82;
pub use asay82::*;

/// Bachelier (1901) normal model for option pricing.
pub mod bachelier;
pub use bachelier::*;

/// Black 1976 model for option pricing.
pub mod black76;
pub use black76::*;

/// Black-Scholes (1973) model for option pricing.
pub mod blackscholes73;
pub use blackscholes73::*;

/// Garman-Kohlhagen (1983) model for foreign exchange options.
pub mod garmankohlhagen83;
pub use garmankohlhagen83::*;

/// Heston (1993) model for option pricing with stochastic volatility.
pub mod heston93;
pub use heston93::*;

/// SABR (2002) model for option pricing with stochastic volatility.
pub mod sabr;
pub use sabr::*;

/// Merton (1973) model for option pricing.
pub mod merton73;
pub use merton73::*;

/// Generalised Black-Scholes-Merton option pricing model and it's Greeks.
pub trait GeneralisedBlackScholesMerton {
    /// Price a European option.
    fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Delta of a European option.
    fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Gamma of a European option.
    fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Theta of a European option.
    fn theta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Vega of a European option.
    fn vega(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Rho of a European option.
    fn rho(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Vanna of a European option.
    fn vanna(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Charm of a European option.
    fn charm(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Lambda of a European option.
    fn lambda(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Zomma of a European option.
    fn zomma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Speed of a European option.
    fn speed(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Color of a European option.
    fn color(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Vomma of a European option.
    fn vomma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Ultima of a European option.
    fn ultima(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Calculate d1.
    fn d1(&self, k: f64, t: f64) -> f64;

    /// Calculate d2.
    fn d2(&self, k: f64, t: f64) -> f64;
}
