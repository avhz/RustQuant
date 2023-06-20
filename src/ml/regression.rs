// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for regression algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use nalgebra::{DMatrix, DVector};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to hold the input data for a linear regression.
pub struct LinearRegressionInput<T> {
    /// The input data matrix, also known as the design matrix.
    pub x: DMatrix<T>,
    /// The output data vector, also known as the response vector.
    pub y: DVector<T>,
}

/// Struct to hold the output data for a linear regression.
pub struct LinearRegressionOutput<T> {
    /// The intercept of the linear regression,
    /// often denoted as b0 or alpha.
    pub intercept: T,
    /// The coefficients of the linear regression,
    /// often denoted as b1, b2, ..., bn.
    pub coefficients: DVector<T>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl LinearRegressionInput<f64> {
    /// Fits a linear regression to the input data.
    pub fn fit(&self) -> LinearRegressionOutput<f64> {
        // Insert a column of 1s to the input data matrix,
        // to account for the intercept.
        let x = self.x.clone().insert_column(0, 1.);
        let y = self.y.clone();

        let x_t = x.transpose();
        let x_t_x = x_t.clone() * x;
        let x_t_x_inv = x_t_x.try_inverse().unwrap();
        let x_t_y = x_t * y;

        let coefficients = x_t_x_inv * x_t_y;
        let intercept = coefficients[0];

        LinearRegressionOutput {
            intercept,
            coefficients,
        }
    }
}

impl LinearRegressionOutput<f64> {
    /// Predicts the output for the given input data.
    pub fn predict(&self, input: DMatrix<f64>) -> DVector<f64> {
        let intercept = DVector::from_element(input.nrows(), self.intercept);
        let coefficients = self.coefficients.clone().remove_row(0);

        // Y = B * X + A
        input * coefficients + intercept
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_regression {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_linear_regression() {
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
            &[-0.083784355, -0.63348570, -0.39926660, 
              -0.982943745,  1.09079746, -0.46812305,
              -1.875067321, -0.91372727,  0.32696208,
              -0.186144661,  1.00163971, -0.41274690],
        );

        #[rustfmt::skip]
        let x_test = DMatrix::from_row_slice(
            4, // rows
            3, // columns
            &[0.56203647, 0.59584645, -0.41165301, 
              0.66335826, 0.45209183, -0.29432715,
             -0.60289728, 0.89674396, 1.21857396, 
              0.69837769, 0.57221651, 0.24411143],
        );

        let response =
            DVector::from_row_slice(&[-0.44515196, -1.84780364, -0.62882531, -0.86108069]);

        // Fit the model to the training data.
        let input = LinearRegressionInput {
            x: x_train,
            y: response,
        };
        let output = input.fit();

        // Predict the response for the test data.
        let preds = output.predict(x_test);

        // Check intercept.
        assert_approx_equal!(output.intercept, 0.45326734, 1e-6);

        // Check coefficients.
        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[0.45326734, 1.05986612, -0.16909348, 2.29605328][i],
                1e-6
            );
        }

        // Check predictions.
        for (i, pred) in preds.iter().enumerate() {
            assert_approx_equal!(
                pred,
                &[0.0030197504, 0.4041016953, 2.4605541127, 1.6571889522][i],
                1e-3
            );
        }
    }
}
