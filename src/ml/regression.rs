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

/// Struct for a linear regression.
pub struct LinearRegression {}

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

impl LinearRegression {
    /// Fits a linear regression to the input data.
    pub fn fit(&self, input: LinearRegressionInput<f64>) -> LinearRegressionOutput<f64> {
        // Insert a column of 1s to the input data matrix,
        // to account for the intercept.
        let x = input.x.insert_column(0, 1.);
        let y = input.y;

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_regression {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_linear_regression() {
        // Test data generated from the following R code:
        //
        // set.seed(2023)
        // x <- matrix(rnorm(12), 4, 3)
        // y <- rexp(4)
        // lm(y ~ x)

        #[rustfmt::skip]
        let x = DMatrix::from_row_slice(
            4, // rows
            3, // columns
            &[-0.08378436, -0.6334857, -0.3992666, 
              -0.98294375, 1.0907975, -0.4681231,
              -1.87506732, -0.9137273, 0.3269621, 
              -0.18614466, 1.0016397, -0.4127469],
        );

        let y = DVector::from_row_slice(&[0.4259088, 0.2617037, 0.4928989, 2.1477291]);

        let input = LinearRegressionInput { x, y };

        let linear_regression = LinearRegression {};
        let output = linear_regression.fit(input);

        // Call:
        // lm(formula = y ~ x)
        //
        // Coefficients:
        // (Intercept)           x1           x2           x3
        //       3.682        2.105        1.232        5.759

        assert_approx_equal!(output.intercept, 3.682226, 1e-6);

        for (i, coefficient) in output.coefficients.iter().enumerate() {
            assert_approx_equal!(
                coefficient,
                &[3.682226, 2.104646, 1.232251, 5.758977][i],
                1e-6
            );
        }

        println!("intercept: {:?}", output.intercept);
        println!("coefficients: {:?}", output.coefficients);
    }
}
