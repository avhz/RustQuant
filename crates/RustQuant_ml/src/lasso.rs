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


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_lasso_regression {
    use super::*;
    use RustQuant_utils::assert_approx_equal;

    struct DataForTests {
        training_set: DMatrix<f64>,
        testing_set: DMatrix<f64>,
        response: DVector<f64>,
    }

    fn setup_test() -> DataForTests {
        DataForTests {
            training_set: DMatrix::from_row_slice(
            4,
            3,
        &[
                -0.083_784_355, -0.633_485_70, -0.399_266_60,
                -0.982_943_745,  1.090_797_46, -0.468_123_05,
                -1.875_067_321, -0.913_727_27,  0.326_962_08,
                -0.186_144_661,  1.001_639_71, -0.412_746_90],
            ),

            testing_set: DMatrix::from_row_slice(
            4,
            3,
            &[
                0.562_036_47, 0.595_846_45, -0.411_653_01,
                0.663_358_26, 0.452_091_83, -0.294_327_15,
                -0.602_897_28, 0.896_743_96, 1.218_573_96,
                0.698_377_69, 0.572_216_51, 0.244_111_43],
            ),

            response: DVector::from_row_slice(
                &[
                    -0.445_151_96,
                    -1.847_803_64,
                    -0.628_825_31,
                    -0.861_080_69
                ]
            ),
        }
    }

    #[test]
    fn test_lasso_without_intercept() -> Result<(), RustQuantError> {

        let data: DataForTests = setup_test();

        let input: LassoInput<f64> = LassoInput {
            x: data.training_set,
            y: data.response,
            lambda: 0.01,
            fit_intercept: false,
            max_iter: 1000,
            tolerance: 1e-4,
        };

        let output: LassoOutput<f64> = input.fit()?;
        let predictions = output.predict(data.testing_set)?;

        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[
                    0.0,
                    0.743_965_706_491_596_7,
                    -0.304_713_846_510_641_43,
                    1.355_162_653_724_116_22,
                ][i],
                f64::EPSILON
            );
        }

        for (i, pred) in predictions.iter().enumerate() {
            assert_approx_equal!(
                pred,
                &[
                    -0.321_283_589_676_737_6,
                    -0.04310400559445471,
                    0.9295807191488583,
                    0.6760174510230131
                ][i],
                f64::EPSILON
            );
        }
        Ok(())
    }

     #[test]
    fn test_lasso_with_intercept() -> Result<(), RustQuantError> {

        let data: DataForTests = setup_test();

        let input: LassoInput<f64> = LassoInput {
            x: data.training_set,
            y: data.response,
            lambda: 0.01,
            fit_intercept: true,
            max_iter: 1000,
            tolerance: 1e-4,
        };

        let output: LassoOutput<f64> = input.fit()?;
        let predictions = output.predict(data.testing_set)?;

        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[
                    0.009_633_706_736_496_328,
                    0.750_479_303_541_854_1,
                    -0.301_997_087_876_784_5,
                    1.373_605_833_196_545_3,
                ][i],
                f64::EPSILON
            );
        }

        for (i, pred) in predictions.iter().enumerate() {
            assert_approx_equal!(
                pred,
                &[
                    -0.313_962_423_203_417_3,
                    -0.033_349_554_520_968_38,
                    0.960_198_011_081_136_2,
                    0.696_256_873_679_798_4,
                ][i],
                f64::EPSILON
            );
        }
        Ok(())
    }
}
