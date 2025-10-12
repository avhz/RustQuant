// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for ridge regression algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use nalgebra::{DMatrix, DVector};

use RustQuant_error::RustQuantError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to hold the input data for a ridge regression.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct RidgeRegressionInput<T> {
    /// The features matrix.
    pub x: DMatrix<T>,
    /// The output data vector, also known as the response vector.
    pub y: DVector<T>,
    /// The regularization parameter.
    pub lambda: T,
    /// Include the intercept.
    pub fit_intercept: bool,
    /// The maximum number of iterations for training.
    pub max_iter: usize,
    /// The tolerance for the convergence.
    pub tolerance: T,
}

/// Struct to hold the output data for a ridge regression.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct RidgeRegressionOutput<T> {
    /// The intercept of the ridge regression,
    pub intercept: T,
    /// The coefficients of the ridge regression,
    pub coefficients: DVector<T>,
}
