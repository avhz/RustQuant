// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for logistic regression algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::math::optimization::gradient_descent::GradientDescent;

use nalgebra::{DMatrix, DVector};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to hold the input data for a logistic regression.
#[derive(Clone, Debug)]
pub struct LogisticRegressionInput<T> {
    /// The input data matrix, also known as the design matrix.
    pub x: DMatrix<T>,
    /// The output data vector, also known as the response vector.
    /// The values of the response vector should be either 0 or 1.
    pub y: DVector<T>,
}

/// Struct to hold the output data for a logistic regression.
#[derive(Clone, Debug)]
pub struct LogisticRegressionOutput<T> {
    /// The intercept of the logistic regression,
    /// often denoted as b0 or alpha.
    pub intercept: T,
    /// The coefficients of the logistic regression,
    /// often denoted as b1, b2, ..., bn.
    pub coefficients: DVector<T>,
}

/// Algorithm to use for logistic regression.
pub enum LogisticRegressionAlgorithm {
    /// Maximum Likelihood Estimation using Algorithmic Adjoint Differentiation
    /// See: https://en.wikipedia.org/wiki/Logistic_regression#Maximum_likelihood_estimation_(MLE)
    MLE,
    /// Iterative Reweighted Least Squares
    /// From Wikipedia (https://en.wikipedia.org/wiki/Logistic_regression#Iteratively_reweighted_least_squares_(IRLS)):
    /// """
    /// Binary logistic regression can, be calculated using
    /// iteratively reweighted least squares (IRLS), which is equivalent to
    /// maximizing the log-likelihood of a Bernoulli
    /// distributed process using Newton's method.
    /// """
    IRLS,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl LogisticRegressionInput<f64> {
    /// Create a new `LogisticRegressionInput` struct.
    pub fn new(x: DMatrix<f64>, y: DVector<f64>) -> Self {
        Self { x, y }
    }

    /// Fit a logistic regression model to the input data.
    pub fn fit(&self, method: LogisticRegressionAlgorithm) -> LogisticRegressionOutput<f64> {
        // let x = &self.x;
        let x = &self.x.clone().insert_column(0, 1.);
        let y = &self.y;

        let (n_rows, n_cols) = x.clone().shape();

        match method {
            // MAXIMUM LIKELIHOOD ESTIMATION
            LogisticRegressionAlgorithm::MLE => unimplemented!(),
            // ITERATIVE REWEIGHTED LEAST SQUARES
            // From Murphy, Kevin P. Machine Learning: A Probabilistic Perspective.
            LogisticRegressionAlgorithm::IRLS => {
                // Initialize the coefficients to 0.0
                // let mut beta = DVector::from_element(n_cols, 0.0);
                let mut beta = DVector::from_element(n_cols + 1, 0.0);

                // Initialize the convergence criterion
                let mut delta = 1.0;

                // Maximum number of iterations
                let max_iter = 100;

                // Tolerance
                let tol = std::f64::EPSILON.sqrt();

                // Iteration counter
                let mut iter = 0;

                // Iterate until convergence
                while delta > tol && iter < max_iter {
                    // Calculate the predicted values
                    let yhat = logistic_vec(&(x.clone() * &beta));

                    // Calculate the weights
                    // let w = &yhat * (1.0 - &yhat);
                    let w = &yhat * (DVector::from_element(n_rows, 1.0) - &yhat);

                    // Calculate the diagonal matrix of weights
                    let w_diag = DMatrix::from_diagonal(&w);

                    // Calculate the Hessian matrix
                    let hessian = x.transpose() * w_diag * x;

                    // Calculate the gradient vector
                    let gradient = x.transpose() * (yhat - y);

                    // Calculate the Newton step
                    let newton_step = match hessian.lu().solve(&gradient) {
                        Some(step) => step,
                        None => panic!(
                            "Cannot solve linear system, check the condition of Hessian matrix"
                        ),
                    };

                    // Update the coefficients
                    beta -= newton_step.clone();

                    // Update the convergence criterion
                    delta = newton_step.clone().norm();

                    // Update the iteration counter
                    iter += 1;
                }

                if iter == max_iter {
                    println!("Warning: Maximum number of iterations reached.");
                }

                // The intercept and other coefficients are all elements in the beta vector
                LogisticRegressionOutput {
                    intercept: beta[0],
                    coefficients: beta.remove_row(0),
                }
            }
        }
    }
}

fn logistic(x: f64) -> f64 {
    (1. + (-x).exp()).recip()
}

fn logistic_vec(x: &DVector<f64>) -> DVector<f64> {
    x.map(|v| logistic(v))
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_logistic_regression {
    use std::time::Instant;

    // use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_logistic_regression() {
        // TEST DATA GENERATED FROM THE FOLLOWING R CODE:
        //
        // set.seed(1234)
        //
        // features    <- c("x1", "x2", "x3")
        //
        // (x_train  <- data.frame(matrix(rnorm(12), 4, 3))); colnames(x_train) <- features
        // (x_test   <- data.frame(matrix(rnorm(12), 4, 3))); colnames(x_test)  <- features
        //
        // (response <- sample(c(0,1), 4, replace = TRUE))
        //
        // (model <- glm(response ~ ., data = x_train, family = binomial))
        // (preds <- predict(model, newdata = x_test, type = "response"))

        #[rustfmt::skip]
        let x_train = DMatrix::from_row_slice(
            4, // rows
            3, // columns
            &[-1.2070657,  0.4291247, -0.5644520,
               0.2774292,  0.5060559, -0.8900378,
               1.0844412, -0.5747400, -0.4771927,
              -2.3456977, -0.5466319, -0.9983864],
        );

        #[rustfmt::skip]
        let _x_test = DMatrix::from_row_slice(
            4, // rows
            3, // columns
            &[-0.77625389, -0.5110095,  0.1340882,
               0.06445882, -0.9111954, -0.4906859,
               0.95949406, -0.8371717, -0.4405479,
              -0.11028549,  2.4158352,  0.4595894],
        );

        let response = DVector::from_row_slice(&[0., 1., 1., 1.]);

        // Fit the model to the training data.
        let input = LogisticRegressionInput {
            x: x_train,
            y: response,
        };

        let start_none = Instant::now();
        let output = input.fit(LogisticRegressionAlgorithm::IRLS);
        let elapsed_none = start_none.elapsed();

        println!(
            "None: time {:?}, Coefs: {:?}\n",
            elapsed_none, output.coefficients
        );

        // // Predict the response for the test data.
        // let preds = output.predict(x_test);

        // // Check intercept.
        // assert_approx_equal!(output.intercept, 0.45326734, 1e-6);

        // // Check coefficients.
        // for (i, coefficient) in output.coefficients.iter().enumerate() {
        //     assert_approx_equal!(
        //         coefficient,
        //         &[0.45326734, 1.05986612, -0.16909348, 2.29605328][i],
        //         1e-6
        //     );
        // }

        // // Check predictions.
        // for (i, pred) in preds.iter().enumerate() {
        //     assert_approx_equal!(
        //         pred,
        //         &[0.0030197504, 0.4041016953, 2.4605541127, 1.6571889522][i],
        //         1e-3
        //     );
        // }
    }
}
