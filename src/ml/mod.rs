// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Machine learning algorithms. This module relies on the [`nalgebra`] crate.
//!
//! ### Regression
//!
//! - [x] Linear (using QR or SVD decomposition)
//! - [x] Logistic (via IRLS, adding MLE in the future).
//!
//! ### Classification
//!
//! - [x] K-Nearest Neighbours

/// Thin wrapper for ML data to be used
/// in `ml` algorithms
pub mod ml_data;
pub use ml_data::*;

/// Submodule of `ml`: activation functions.
pub mod activations;
pub use activations::*;

/// K Nearest Neighbor classifier
pub mod k_nearest_neighbors;
pub use k_nearest_neighbors::*;

/// Linear regression.
pub mod linear_regression;
pub use linear_regression::*;

/// Logistic regression.
pub mod logistic_regression;
pub use logistic_regression::*;
