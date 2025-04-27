// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// AUTOMATIC DIFFERENTIATION MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Reverse mode automatic differentation.
//! Currently only gradients can be computed.
//! Suggestions on how to extend the functionality to Hessian matrices are
//! definitely welcome.
//!
//! Additionally, only functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$
//! (scalar output) are supported. However, you can manually apply the
//! differentiation to multiple functions that could represent a vector output.
//!
//! - [x] Reverse (Adjoint) Mode
//!   - Implementation via Operator and Function Overloading.
//!   - Useful when number of outputs is *smaller* than number of inputs.
//!     - i.e for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \ll n$
//! - [ ] Forward (Tangent) Mode
//!   - Implementation via Dual Numbers.
//!   - Useful when number of outputs is *larger* than number of inputs.
//!     - i.e. for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \gg n$
//!
//! ```
//! # use RustQuant_autodiff::*;
//!
//! // Create a new Graph to store the computations.
//! let g = Graph::new();
//!
//! // Assign variables.
//! let x = g.var(69.);
//! let y = g.var(420.);
//!
//! // Define a function.
//! let f = {
//!     let a = x.powi(2);
//!     let b = y.powi(2);
//!
//!     a + b + (x * y).exp()
//! };
//!
//! // Accumulate the gradient.
//! let gradient = f.accumulate();
//!
//! println!("Function = {}", f);
//! println!("Gradient = {:?}", gradient.wrt([x, y]));
//! ```
//!
//! You can also generate Graphviz (dot) code to visualize the computation graphs:
//!
//! ```ignore
//! println!("{}", graphviz(&graph, &variables));
//! ```  
//!
//! The computation graph from computing Black-Scholes Greeks is shown at the
//! following link:
//!
//! [Black-Scholes Greeks tape.](https://github.com/avhz/RustQuant/blob/main/images/black_scholes_tape.png)
//!
//! It is clearly a work in progress, but gives a general idea of how the
//! computation graph is structured.
//!
//! If you want to improve the visualization, please feel free to submit a PR!

/// [`Accumulate`] trait.
pub mod accumulate;
pub use accumulate::*;

/// Implements the gradient computation.
pub mod gradient;
pub use gradient::*;

/// The Graph (aka. tape or Wengert List).
pub mod graph;
pub use graph::*;

/// Visualisation of the [`Graph`].
pub mod graphviz;
pub use graphviz::*;

/// Implements [`Vertex`] (nodes) for the `Graph`.
pub mod vertex;
pub use vertex::*;

/// Operator/function overloading.
/// This module contains the overloaded operators and primitive functions.
/// In Griewank and Walther - Evaluating Derivatives, they refer to this
/// as the "elemental library".
/// Operations such as `+` and `*` are redefined, along with primitive
/// functions such as `sin`, `exp`, and `log`.
/// Each overload has an associated test to ensure functionality.
pub mod overload;
pub use overload::*;

/// `DiffOps`s for `autodiff`.
pub mod diff_traits;
pub use diff_traits::*;

/// `Symbol`s for `autodiff`.
pub mod symbol;
pub use symbol::*;

/// `Variable`s for `autodiff`.
pub mod variable;
pub use variable::*;
