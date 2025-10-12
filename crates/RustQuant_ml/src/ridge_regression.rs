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
        let mut regularisation_matrix = DMatrix::<f64>::identity(n_col, n_col);
        
        if self.fit_intercept { regularisation_matrix[(0,0)] = 0.0; }
    
        let ridge_matrix = (&features_matrix_transpose * features_matrix) + self.lambda * regularisation_matrix;

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_ridge_regression {
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
    fn test_ridge_regression_without_intercept() -> Result<(), RustQuantError> {

        let data: DataForTests = setup_test();

        let input: RidgeRegressionInput<f64> = RidgeRegressionInput {
            x: data.training_set,
            y: data.response,
            lambda: 1.0,
            fit_intercept: false,
            max_iter: 1000,
            tolerance: 1e-4,
        };

        let output = input.fit()?;

        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[
                    0.0,
                    0.620_453_495_948_496_1,
                    -0.420_204_780_485_896_43,
                    0.490_065_457_911_238_96
                ][i],
                f64::EPSILON
            );
        }

        let predictions = output.predict(data.testing_set)?;
        for (i, pred) in predictions.iter().enumerate() {
            assert_approx_equal!(
                pred,
                &[
                    -0.103_396_954_909_688_48,
                    0.077_372_233_758_234_32,
                    -0.153_704_818_231_581,
                    0.312_493_346_002_296_7
                ][i],
                f64::EPSILON
            );
        }
        Ok(())
    }

    #[test]
    fn test_ridge_regression_with_intercept() -> Result<(), RustQuantError> {

        let data: DataForTests = setup_test();

        let input: RidgeRegressionInput<f64> = RidgeRegressionInput {
            x: data.training_set,
            y: data.response,
            lambda: 1.0,
            fit_intercept: true,
            max_iter: 1000,
            tolerance: 1e-4,
        };

        let output = input.fit()?;

        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[
                    -0.701_404_539_262_792_8,
                    0.215_855_099_335_031_66,
                    -0.371_997_155_606_467_07,
                    0.104_115_015_026_450_71,
                ][i],
                f64::EPSILON
            );
        }

        let predictions = output.predict(data.testing_set)?;

        for (i, pred) in predictions.iter().enumerate() {
            assert_approx_equal!(
                pred,
                &[
                    -0.844_598_545_101_076_9,
                    -0.757_036_026_633_643_9,
                    -1.038_257_347_797_051_1,
                    -0.738_103_402_522_953_9,
                ][i],
                f64::EPSILON
            );
        }
        Ok(())
    }
}
