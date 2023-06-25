// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for logistic regression (classification) algorithms.
//!
//! BROKEN: This module is currently broken and does not work.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
    /// Number of iterations required to converge.
    pub iterations: usize,
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

/// Logistic function.
/// Also known as the sigmoid, logit, or squashing function.
///
/// sigmoid(x) = 1 / (1 + exp(-x)) = exp(x) / (exp(x) + 1)
///
/// Note:
///
/// mu(x) = E[Y | X] = P(Y = 1 | X) = sigmoid(w^T x)
pub trait LogisticFunction {
    /// Logistic function.
    fn logistic(&self) -> Self;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl LogisticFunction for f64 {
    #[inline]
    fn logistic(&self) -> Self {
        (1. + (-*self).exp()).recip()
    }
}

impl LogisticFunction for DVector<f64> {
    #[inline]
    fn logistic(&self) -> Self {
        self.map(|x| LogisticFunction::logistic(&x))
    }
}

impl LogisticRegressionInput<f64> {
    /// Create a new `LogisticRegressionInput` struct.
    pub fn new(x: DMatrix<f64>, y: DVector<f64>) -> Self {
        assert!(x.nrows() == y.len());

        Self { x, y }
    }

    /// Fit a logistic regression model to the input data.
    pub fn fit(
        &self,
        method: LogisticRegressionAlgorithm,
        tolerance: f64,
    ) -> Result<LogisticRegressionOutput<f64>, &'static str> {
        let mut out = LogisticRegressionOutput {
            intercept: 0.,
            coefficients: DVector::zeros(self.x.ncols()),
            iterations: 0,
        };

        let X = self.x.clone().insert_column(0, 1.0);
        let X_T = X.clone().transpose();
        let y = self.y.clone();

        let (n_rows, _) = X.shape();

        match method {
            // MAXIMUM LIKELIHOOD ESTIMATION
            LogisticRegressionAlgorithm::MLE => unimplemented!(),
            // ITERATIVELY RE-WEIGHTED LEAST SQUARES
            // From Murphy, Kevin P. Machine Learning: A Probabilistic Perspective.
            LogisticRegressionAlgorithm::IRLS => {
                // let mut S = DMatrix::zeros(n_rows, n_rows);
                // let mut z = DVector::zeros(n_rows);
                // let mut mu = DVector::zeros(n_rows);

                let guess = (y.mean() / (1. - y.mean())).ln();

                let mut w_curr = DVector::from_element(n_rows, guess);
                let mut w_next = DVector::zeros(n_rows);

                // While not converged.
                // Convergence is defined as the norm of the change in
                // the weights being less than the tolerance.
                while (&w_next - &w_curr).norm() >= tolerance {
                    std::mem::swap(&mut w_curr, &mut w_next);

                    let eta = &X * &w_curr;
                    let mu = LogisticFunction::logistic(&eta);
                    let S = DMatrix::from_diagonal(
                        &mu.component_mul(&(&DVector::from_element(n_rows, 1.0) - &mu)),
                    );

                    let z = match S.clone().try_inverse() {
                        Some(inv) => eta + inv * (&y - &mu),
                        None => return Err("S matrix is not invertible."),
                    };

                    let X_T_S = &X_T * &S;
                    let hessian = &X_T_S * &X;

                    w_next = match hessian.try_inverse() {
                        Some(inv) => {
                            println!("S = {:?}", S);

                            out.intercept = w_curr[0];
                            out.coefficients = w_curr.clone();

                            inv * (&X_T_S * z)
                        }
                        None => return Err("Hessian matrix (X^T S X) is not invertible."),
                    };

                    out.iterations += 1;
                    println!("w_curr = {:?}", w_curr);
                }
            }
        }

        Ok(out)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #[cfg(test)]
// mod tests_logistic_regression {
//     use super::*;
//     use std::time::Instant;

//     // use crate::assert_approx_equal;

//     #[test]
//     fn test_logistic_regression() {
//         // PROFILE THIS UNIT TEST WITH (on MacOS):
//         // sudo -E cargo flamegraph --release --freq 5000 --unit-test -- tests_logistic_regression::test_logistic_regression

//         // TEST DATA GENERATED FROM THE FOLLOWING R v4.3.0 CODE:
//         //
//         // set.seed(1234)
//         //
//         // features    <- c("x1", "x2", "x3")
//         //
//         // (x_train  <- data.frame(matrix(rnorm(12), 4, 3))); colnames(x_train) <- features
//         // (x_test   <- data.frame(matrix(rnorm(12), 4, 3))); colnames(x_test)  <- features
//         //
//         // (response <- sample(c(0,1), 4, replace = TRUE))
//         //
//         // (model <- glm(response ~ ., data = x_train, family = binomial))
//         // (preds <- predict(model, newdata = x_test, type = "response"))

//         #[rustfmt::skip]
//         let x_train = DMatrix::from_row_slice(
//             4, // rows
//             3, // columns
//             &[-1.2070657,  0.4291247, -0.5644520,
//                0.2774292,  0.5060559, -0.8900378,
//                1.0844412, -0.5747400, -0.4771927,
//               -2.3456977, -0.5466319, -0.9983864],
//         );

//         #[rustfmt::skip]
//         let _x_test = DMatrix::from_row_slice(
//             4, // rows
//             3, // columns
//             &[-0.77625389, -0.5110095,  0.1340882,
//                0.06445882, -0.9111954, -0.4906859,
//                0.95949406, -0.8371717, -0.4405479,
//               -0.11028549,  2.4158352,  0.4595894],
//         );

//         let response = DVector::from_row_slice(&[0., 1., 1., 1.]);

//         // Fit the model to the training data.
//         let input = LogisticRegressionInput {
//             x: x_train,
//             y: response,
//         };

//         let start_none = Instant::now();
//         let output = input.fit(LogisticRegressionAlgorithm::IRLS, f64::EPSILON.sqrt());
//         let elapsed_none = start_none.elapsed();

//         match output {
//             Ok(output) => {
//                 println!("Iterations: \t{}", output.iterations);
//                 println!("Time taken: \t{:?}", elapsed_none);
//                 println!("Intercept: \t{:?}", output.intercept);
//                 println!("Coefficients: \t{:?}", output.coefficients);
//             }
//             Err(err) => {
//                 panic!("Failed to fit logistic regression model: {}", err);
//             }
//         }

//         // // Predict the response for the test data.
//         // let preds = output.predict(x_test);

//         // // Check intercept.
//         // assert_approx_equal!(output.intercept, 0.45326734, 1e-6);

//         // // Check coefficients.
//         // for (i, coefficient) in output.coefficients.iter().enumerate() {
//         //     assert_approx_equal!(
//         //         coefficient,
//         //         &[0.45326734, 1.05986612, -0.16909348, 2.29605328][i],
//         //         1e-6
//         //     );
//         // }

//         // // Check predictions.
//         // for (i, pred) in preds.iter().enumerate() {
//         //     assert_approx_equal!(
//         //         pred,
//         //         &[0.0030197504, 0.4041016953, 2.4605541127, 1.6571889522][i],
//         //         1e-3
//         //     );
//         // }
//     }
// }
