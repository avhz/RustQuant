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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl RidgeRegressionInput<f64> {
    /// Create a new `RidgeRegressionInput` struct.
    #[must_use]
    pub fn new(
        x: DMatrix<f64>,
        y: DVector<f64>,
        lambda: f64,
        fit_intercept: bool,
        max_iter: usize,
        tolerance: f64,
    ) -> Self {
        Self { x, y, lambda, fit_intercept, max_iter, tolerance }
    }

    /// Fits a ridge regression to the input data.
    /// Returns the intercept and coefficients.
    /// The intercept is the first value of the coefficients.
    pub fn fit(&self) -> Result<RidgeRegressionOutput<f64>, RustQuantError> {

        let features_matrix = if self.fit_intercept {
            self.x.clone().insert_column(0, 1.)
        } else {
            self.x.clone()
        };

        let n_col: usize = features_matrix.ncols();
        let features_matrix_transpose = features_matrix.transpose();
        let mut regularization_matrix = DMatrix::<f64>::identity(n_col, n_col);
        
        if self.fit_intercept { regularization_matrix[(0,0)] = 0.0; }
    
        let ridge_matrix = (&features_matrix_transpose * features_matrix) + self.lambda * regularization_matrix;

        let ridge_matrix_inv = ridge_matrix
            .try_inverse()
            .ok_or(RustQuantError::MatrixInversionFailed)?;

        let mut coefficients = ridge_matrix_inv * &features_matrix_transpose * &self.y;
        let intercept: f64 =  if self.fit_intercept {
            coefficients[0]
        } else {
            coefficients = coefficients.insert_row(0, 0.0); 
            0.0
        };

        Ok(RidgeRegressionOutput {
            intercept,
            coefficients,
        })
    }
}

impl RidgeRegressionOutput<f64> {
    /// Predicts the output for the given input data.
    pub fn predict(&self, input: DMatrix<f64>) -> Result<DVector<f64>, RustQuantError> {
        let intercept = DVector::from_element(input.nrows(), self.intercept);
        let coefficients = self.coefficients.clone().remove_row(0);
        let predictions = input * coefficients + intercept;
        Ok(predictions)
    }
}