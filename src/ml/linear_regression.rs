// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for linear regression algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use nalgebra::{DMatrix, DVector};
use thiserror::Error;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Error, Debug)]

///For better error handling
#[allow(clippy::module_name_repetitions)]
pub enum LinearRegressionError {
    /// failed to invert matrix
    #[error("Matrix inversion failed")]
    MatrixInversionFailed,

    /// failed to perform SVD decomposition
    #[error("SVD decomposition failed: v_t is likely wrong type")]
    SvdDecompositionFailed,

    /// failed to compute u
    #[error("SVD decomposition failed: u is likely wrong type")]
    SvdDecompositionFailedOnU,
}

/// Struct to hold the input data for a linear regression.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct LinearRegressionInput<T> {
    /// The input data matrix, also known as the design matrix.
    /// You do not need to add a column of ones to the design matrix,
    /// as this is done automatically.
    pub x: DMatrix<T>,
    /// The output data vector, also known as the response vector.
    pub y: DVector<T>,
}

/// Struct to hold the output data for a linear regression.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct LinearRegressionOutput<T> {
    /// The intercept of the linear regression,
    /// often denoted as b0 or alpha.
    pub intercept: T,
    /// The coefficients of the linear regression,
    /// often denoted as b1, b2, ..., bn.
    pub coefficients: DVector<T>,
}

/// Enum for type of matrix decomposition used.
#[derive(Copy, Clone)]
pub enum Decomposition {
    /// No decomposition to be used.
    /// Naive implementation of linear regression.
    None,
    /// QR decomposition.
    /// X = Q * R
    /// where
    ///     - Q is an orthogonal matrix, and
    ///     - R is an upper triangular matrix.
    QR,
    /// SVD decomposition.
    /// X = U * S * V^T
    /// where
    ///     - U is an orthogonal matrix, and
    ///     - S is a diagonal matrix, and
    ///     - V is an orthogonal matrix.
    SVD,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl LinearRegressionInput<f64> {
    /// Create a new `LinearRegressionInput` struct.
    #[must_use]
    pub fn new(x: DMatrix<f64>, y: DVector<f64>) -> Self {
        Self { x, y }
    }

    /// Fits a linear regression to the input data.
    /// Returns the intercept and coefficients.
    /// The intercept is the first value of the coefficients.
    /// A `method` can be specified to use a matrix decomposition.
    /// Possible decommpositions are:
    ///     - `None`: No decomposition to be used.
    ///     - `QR` decomposition (generally fastest).
    ///     - `SVD` decomposition (generally most stable).
    /// Both QR and SVD are usually faster than the naive implementation.
    pub fn fit(
        &self,
        method: Decomposition,
    ) -> Result<LinearRegressionOutput<f64>, LinearRegressionError> {
        // Insert a column of 1s to the input data matrix,
        // to account for the intercept.
        let x = self.x.clone().insert_column(0, 1.);
        let y = self.y.clone();

        match method {
            Decomposition::None => {
                let x_t = x.transpose();
                let x_t_x = x_t.clone() * x;
                let x_t_x_inv = x_t_x
                    .try_inverse()
                    .ok_or(LinearRegressionError::MatrixInversionFailed)?;
                let x_t_y = x_t * y;

                let coefficients = x_t_x_inv * x_t_y;
                let intercept = coefficients[0];

                Ok(LinearRegressionOutput {
                    intercept,
                    coefficients,
                })
            }
            Decomposition::QR => {
                let qr = x.qr();
                let q = qr.q();
                let r = qr.r();

                let coefficients = r
                    .try_inverse()
                    .ok_or(LinearRegressionError::MatrixInversionFailed)?
                    * q.transpose()
                    * y;
                let intercept = coefficients[0];

                Ok(LinearRegressionOutput {
                    intercept,
                    coefficients,
                })
            }
            Decomposition::SVD => {
                let svd = x.svd(true, true);
                let v = svd
                    .v_t
                    .ok_or(LinearRegressionError::SvdDecompositionFailed)?
                    .transpose();
                let s_inv = DMatrix::from_diagonal(&svd.singular_values.map(|x| 1.0 / x));
                let u = svd
                    .u
                    .ok_or(LinearRegressionError::SvdDecompositionFailedOnU)?;

                let pseudo_inverse = v * s_inv * u.transpose();
                let coefficients = &pseudo_inverse * y;

                // The first value of the coefficients is not always the intercept
                // Depends on how the input data is structured
                let intercept = coefficients[0];

                Ok(LinearRegressionOutput {
                    intercept,
                    coefficients,
                })
            }
        }
    }
}

impl LinearRegressionOutput<f64> {
    /// Predicts the output for the given input data.
    pub fn predict(&self, input: DMatrix<f64>) -> Result<DVector<f64>, LinearRegressionError> {
        let intercept = DVector::from_element(input.nrows(), self.intercept);
        let coefficients = self.coefficients.clone().remove_row(0);

        // Y = B * X + A
        let predictions = input * coefficients + intercept;

        Ok(predictions)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_linear_regression {
    use super::*;
    use crate::assert_approx_equal;
    use std::time::Instant;

    use std::f64::EPSILON as EPS;

    #[test]
    fn test_linear_regression() -> Result<(), LinearRegressionError> {
        // TEST DATA GENERATED FROM THE FOLLOWING R CODE:
        //
        // set.seed(2023)
        //
        // features    <- c("x1", "x2", "x3")
        //
        // x_train     <- data.frame(matrix(rnorm(12), 4, 3)); colnames(x_train) <- features
        // x_test      <- data.frame(matrix(rnorm(12), 4, 3)); colnames(x_test)  <- features
        //
        // response    <- rnorm(4)
        //
        // (model <- lm(response ~ ., data = x_train))
        // (preds <- predict(model, newdata = x_test))

        #[rustfmt::skip]
        let x_train = DMatrix::from_row_slice(
            4, // rows
            3, // columns
            &[-0.083_784_355, -0.633_485_70, -0.399_266_60,
              -0.982_943_745,  1.090_797_46, -0.468_123_05,
              -1.875_067_321, -0.913_727_27,  0.326_962_08,
              -0.186_144_661,  1.001_639_71, -0.412_746_90],
        );

        #[rustfmt::skip]
        let x_test = DMatrix::from_row_slice(
            4, // rows
            3, // columns
            &[0.562_036_47, 0.595_846_45, -0.411_653_01,
              0.663_358_26, 0.452_091_83, -0.294_327_15,
             -0.602_897_28, 0.896_743_96, 1.218_573_96,
              0.698_377_69, 0.572_216_51, 0.244_111_43],
        );

        let response =
            DVector::from_row_slice(&[-0.445_151_96, -1.847_803_64, -0.628_825_31, -0.861_080_69]);

        // Fit the model to the training data.
        let input = LinearRegressionInput {
            x: x_train,
            y: response,
        };

        let start_none = Instant::now();
        let output = input.fit(Decomposition::None)?;

        let elapsed_none = start_none.elapsed();
        let coefficients = output.coefficients.clone();

        let start_qr = Instant::now();
        let output_qr = input.fit(Decomposition::QR)?;
        let coefficients_qr = output_qr.coefficients.clone();
        let elapsed_qr = start_qr.elapsed();

        let start_svd = Instant::now();
        let output_svd = input.fit(Decomposition::SVD)?;
        let coefficients_svd = output_svd.coefficients.clone();
        let elapsed_svd = start_svd.elapsed();

        println!("None: time {:?}, Coefs: {:?}\n", elapsed_none, coefficients);
        println!("QR: time {:?}, Coefs: {:?}\n", elapsed_qr, coefficients_qr);
        println!(
            "SVD: time {:?}, Coefs: {:?}\n",
            elapsed_svd, coefficients_svd
        );

        // Predict the response for the test data.

        let preds = output.predict(x_test)?;

        // Check intercept.
        assert_approx_equal!(output.intercept, 0.453_267_356_085_818_9, EPS);

        // Check coefficients.
        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[
                    0.453_267_356_085_818_9,
                    1.059_866_132_317_468_5,
                    -0.169_093_464_601_759_45,
                    2.296_053_332_765_449_5
                ][i],
                EPS
            );
        }

        // Check predictions.
        for (i, pred) in preds.iter().enumerate() {
            assert_approx_equal!(
                pred,
                &[
                    0.003_019_769_611_493_972,
                    0.404_101_701_919_158_5,
                    2.460_554_206_769_587_4,
                    1.657_189_007_522_339_4
                ][i],
                EPS
            );
        }
        Ok(())
    }
}
