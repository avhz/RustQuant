// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for Lasso algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use nalgebra::{DMatrix, DVector};
use RustQuant_error::RustQuantError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to hold the input data for a Lasso regression.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct LassoInput<T> {
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

/// Struct to hold the output data for lasso.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct LassoOutput<T> {
    /// The intercept of the lasso regression,
    pub intercept: T,
    /// The coefficients of the lasso regression,
    pub coefficients: DVector<T>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl LassoInput<f64> {
    /// Create a new `LassoInput` struct.
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

    /// Fits a Lasso regression to the input data.
    /// Returns the intercept and coefficients.
    /// The intercept is the first value of the coefficients.
    pub fn fit(&self) -> Result<LassoOutput<f64>, RustQuantError> {
        let n_cols = self.x.ncols();
        let n_rows = self.x.nrows() as f64;
        let mut features_matrix = self.x.clone();
        let mut response_vec = self.y.clone();
        let feature_means = DVector::from_iterator(
                self.x.ncols(),
                (0..self.x.ncols()).map(|j| self.x.column(j).mean())
            );

        if self.fit_intercept {
            
            features_matrix = self.x.clone();
            for j in 0..self.x.ncols() {
                let mean = feature_means[j];
                for i in 0..self.x.nrows() {
                    features_matrix[(i, j)] -= mean;
                }
            }
            response_vec = &self.y - DVector::from_element(self.x.nrows(), self.y.mean());
        }
            
        let mut residual = response_vec;
        let mut coefficients = DVector::<f64>::zeros(n_cols);

        for _ in 0..self.max_iter {
            let mut max_delta: f64 = 0.0;
            for j in 0..n_cols {
                
                let feature_vals_col_j = features_matrix.column(j);
                let col_norm: f64 = feature_vals_col_j.dot(&feature_vals_col_j);
                let rho: f64 = (residual.dot(&feature_vals_col_j) + coefficients[j] * col_norm) / n_rows;
                
                let new_coefficient_j: f64 = if rho < -self.lambda {
                    (rho + self.lambda) / (col_norm / n_rows)
                } else if rho > self.lambda {
                    (rho - self.lambda) / (col_norm / n_rows)
                } else {
                    0.0
                };

                let delta: f64 = new_coefficient_j - coefficients[j];
                if delta.abs() > 0.0 {
                    residual -= &feature_vals_col_j * delta;
                }
                coefficients[j] = new_coefficient_j;
                max_delta = max_delta.max(delta.abs());
            }

            if max_delta < self.tolerance {
                break;
            }
        }
        
        let intercept: f64 = if self.fit_intercept {
            self.y.mean() - feature_means.dot(&coefficients)
        } else {
            0.0
        };
        coefficients = coefficients.insert_row(0, intercept);

        Ok(LassoOutput {
            intercept,
            coefficients,
        })
    }
}

impl LassoOutput<f64> {
    /// Predicts the output for the given input data.
    pub fn predict(&self, input: DMatrix<f64>) -> Result<DVector<f64>, RustQuantError> {
        let intercept = DVector::from_element(
            input.nrows(),
            self.intercept
        );
        let coefficients = self.coefficients.clone().remove_row(0);
        let predictions = input * coefficients + intercept;
        Ok(predictions)
    }
}
