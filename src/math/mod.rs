// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MATHEMATICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Mathematics related items.
//!
//! ### Optimization and Root Finding
//!
//! - [x] Gradient Descent
//! - [x] Newton-Raphson
//!
//! Note: the reason you need to specify the lifetimes and use the type `Variable` is because the gradient descent optimiser uses the `RustQuant::autodiff` module to compute the gradients. This is a slight inconvenience, but the speed-up is enormous when working with functions with many inputs (when compared with using finite-difference quotients).
//!
//! ```rust
//! use RustQuant::math::*;
//! use RustQuant::autodiff::*;
//!
//! // Define the objective function.
//! fn himmelblau<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
//!     let x = variables[0];
//!     let y = variables[1];
//!
//!     ((x.powf(2.0) + y - 11.0).powf(2.0) + (x + y.powf(2.0) - 7.0).powf(2.0))
//! }
//!
//! // Create a new GradientDescent object with:
//! //      - Step size: 0.005
//! //      - Iterations: 10000
//! //      - Tolerance: sqrt(machine epsilon)
//! let gd = GradientDescent::new(0.005, 10000, Some(std::f64::EPSILON.sqrt()));
//!
//! // Perform the optimisation with:
//! //      - Initial guess (10.0, 10.0),
//! //      - Verbose output.
//! let result = gd.optimize(&himmelblau, &vec![10.0, 10.0], true);
//!     
//! // Print the result.
//! println!("{:?}", result.minimizer);
//! ```
//!
//! ### Integration
//!
//! - Numerical Integration (needed for Heston model, for example):
//!   - [x] Tanh-Sinh (double exponential) quadrature
//!
//! ```rust
//! use RustQuant::math::*;
//!
//! // Define a function to integrate: e^(sin(x))
//! fn f(x: f64) -> f64 {
//!     (x.sin()).exp()
//! }
//!
//! // Integrate from 0 to 5.
//! let integral = integrate(f, 0.0, 5.0);
//!
//! // ~ 7.18911925
//! println!("Integral = {}", integral);
//! ```
//!
//! ### Risk-Reward Metrics
//!
//! - [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)

/// Numerical integration routines.
/// The primary (useful) integrator is the Tanh-Sinh (double exponential) implementation.
pub mod integration;
pub use integration::*;

/// Numerical optimization and root-finding routines.
pub mod optimization {
    /// Gradient descent optimization.
    pub mod gradient_descent;
    pub use gradient_descent::*;

    /// Newton-Raphson method.
    pub mod newton_raphson;
    pub use newton_raphson::*;
}
pub use optimization::*;

/// Fast fourier transform.
pub mod fft;
pub use fft::*;

/// Interpolation routines.
pub mod interpolation;
pub use interpolation::*;
// pub mod interpolation {
//     /// Linear interpolation.
//     pub mod linear_interpolator;
//     pub use linear_interpolator::*;
// }
// pub use interpolation::*;

/// Simple risk/reward measures.
pub mod risk_reward;
pub use risk_reward::*;

/// Sequences of numbers and associated functions.
pub mod sequences;
pub use sequences::*;
