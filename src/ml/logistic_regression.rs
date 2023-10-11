// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module for logistic regression (classification) algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::ml::ActivationFunction;
use nalgebra::{DMatrix, DVector};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to hold the input data for a logistic regression.
#[derive(Clone, Debug)]
pub struct LogisticRegressionInput<T> {
    /// The input data matrix, also known as the design matrix.
    /// You do not need to add a column of ones to the design matrix,
    /// as this is done automatically.
    pub x: DMatrix<T>,
    /// The output data vector, also known as the response vector.
    /// The values of the response vector should be either 0 or 1.
    pub y: DVector<T>,
}

/// Struct to hold the output data for a logistic regression.
#[derive(Clone, Debug)]
pub struct LogisticRegressionOutput<T> {
    /// The coefficients of the logistic regression,
    /// often denoted as b0, b1, b2, ..., bn.
    /// The first coefficient is the intercept (aka. b0 or alpha).
    pub coefficients: DVector<T>,
    /// Number of iterations required to converge.
    pub iterations: usize,
}

/// Algorithm to use for logistic regression.
pub enum LogisticRegressionAlgorithm {
    /// Maximum Likelihood Estimation using Algorithmic Adjoint Differentiation
    /// See: <https://en.wikipedia.org/wiki/Logistic_regression#Maximum_likelihood_estimation_(MLE)>
    MLE,
    /// Iterative Reweighted Least Squares
    /// From Wikipedia (<https://en.wikipedia.org/wiki/Logistic_regression#Iteratively_reweighted_least_squares_(IRLS)>):
    /// """
    /// Binary logistic regression can, be calculated using
    /// iteratively reweighted least squares (IRLS), which is equivalent to
    /// maximizing the log-likelihood of a Bernoulli
    /// distributed process using Newton's method.
    /// """
    ///
    /// References:
    ///     - Elements of Statistical Learning (Hastie, Tibshirani, Friedman 2009)
    ///     - Machine Learning: A Probabilistic Perspective (Murphy, Kevin P. 2012)
    IRLS,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
type PrepareInputResult = Result<(DMatrix<f64>, DMatrix<f64>, DVector<f64>), &'static str>;

impl LogisticRegressionInput<f64> {
    /// Create a new `LogisticRegressionInput` struct.
    pub fn new(x: DMatrix<f64>, y: DVector<f64>) -> Self {
        assert!(x.nrows() == y.len());

        Self { x, y }
    }

    /// Function to validate and prepare the input data.
    fn prepare_input(&self) -> PrepareInputResult {
        // Check that the response vector is either 0 or 1.
        if self.y.iter().any(|&x| x != 0. && x != 1.) {
            return Err("The elements of the response vector should be either 0 or 1.");
        }

        // Check dimensions match.
        let (n_rows, _) = self.x.shape();

        if n_rows != self.y.len() {
            return Err("The number of rows in the design matrix should match the length of the response vector.");
        }

        // Check the input data is finite.
        if self.x.iter().any(|&x| !x.is_finite()) || self.y.iter().any(|&x| !x.is_finite()) {
            return Err("The input data should be finite.");
        }

        // Add a column of ones to the design matrix.
        let x = self.x.clone().insert_column(0, 1.0);

        // Also return the transpose of the design matrix.
        Ok((x.clone(), x.transpose(), self.y.clone()))
    }

    /// Function to validate and prepare the output data.
    fn prepare_output(&self) -> Result<LogisticRegressionOutput<f64>, &'static str> {
        // Initial guess for the coefficients.
        // hyperplane orthogonal to line between  means of class 0 and 1; plane goes through the location of (weighted) mean of both clusters

        let (_n_sample, n_feat) = self.x.shape();
        let ones = DVector::from_element(n_feat, 1.).transpose();

        // Calculate mean of features of samples of class 0
        let mask0 = &self.y * &ones;
        let x0_mean = self.x.component_mul(&mask0).row_mean();

        // Calculate mean of features of samples of class 1
        let mask1 = (-&mask0).add_scalar(1.);
        let x1_mean = self.x.component_mul(&mask1).row_mean();

        //vector from x0_mean to x1_mean
        let delta = &x1_mean - &x0_mean;

        //fraction of samples in class 1 , used as weight
        let y_mean = self.y.mean();
        let mid = x1_mean * y_mean + x0_mean * (1. - y_mean);

        //compute projection of weighted mean of class on direction delta
        let scaler = mid.dot(&delta) / delta.magnitude_squared();
        let dir = delta * scaler;

        // <dir,x>=|dir|^2 is the plane orthogonal to dir with distance |dir| from the origin
        let bias = -dir.magnitude_squared();
        let coef = dir.insert_column(0, bias);

        // Return the output struct, with the initial guess for the coefficients.
        Ok(LogisticRegressionOutput {
            coefficients: coef.transpose(),
            iterations: 0,
        })
    }

    /// Fit a logistic regression model to the input data.
    pub fn fit(
        &self,
        method: LogisticRegressionAlgorithm,
        tolerance: f64,
    ) -> Result<LogisticRegressionOutput<f64>, &'static str> {
        // Validate and prepare the input and output data.
        let (X, X_T, y) = self.prepare_input()?;
        let mut output = self.prepare_output()?;

        // Number of rows and columns in the design matrix.
        let (n_rows, n_cols) = X.shape();

        // Vector of ones.
        let ones_samples = DVector::from_element(n_rows, 1.);
        let ones_features = DVector::from_element(n_cols, 1.);

        // Vector of coefficients that we update each iteration.
        let mut coefs = DVector::zeros(n_cols);

        match method {
            // MAXIMUM LIKELIHOOD ESTIMATION
            // Using Algorithmic Adjoint Differentiation (AAD)
            // from the `autodiff` module.
            LogisticRegressionAlgorithm::MLE => unimplemented!(),

            // ITERATIVELY RE-WEIGHTED LEAST SQUARES
            // References:
            //      - Elements of Statistical Learning (Hastie, Tibshirani, Friedman 2009)
            //      - Machine Learning: A Probabilistic Perspective (Murphy, Kevin P. 2012)
            LogisticRegressionAlgorithm::IRLS => {
                let mut eta: DVector<f64>;
                let mut mu: DVector<f64>;

                // While not converged.
                // Convergence is defined as the norm of the change in
                // the weights being less than the tolerance.
                while (&coefs - &output.coefficients).norm() >= tolerance {
                    eta = &X * &output.coefficients;
                    mu = ActivationFunction::logistic(&eta);
                    //multiplication of matrix  with diagonal matrix equals elementwise multiplication of each row / col  with diagonal entries
                    //can be realized by elementwise multiplication with ones * diag_entries.T
                    //
                    let diag_entries = &mu.component_mul(&(&ones_samples - &mu));

                    // Break if data turns out to be linearly separable.
                    if (&y - &mu).max() < tolerance {
                        break;
                    }

                    // For diag-matrix product as elementwise.
                    let diag_repeated = &ones_features * diag_entries.transpose();
                    let X_T_W = X_T.component_mul(&diag_repeated);
                    let hessian = &X_T_W * &X;
                    let z = &X_T * (&y - &mu);
                    let delta_coefs = hessian
                        .lu()
                        .solve(&z)
                        .unwrap_or_else(|| panic!("IRLS[{}]:", output.iterations));

                    coefs = &output.coefficients + delta_coefs;

                    output.iterations += 1;

                    std::mem::swap(&mut output.coefficients, &mut coefs);
                }
            }
        }

        Ok(output)
    }
}

impl LogisticRegressionOutput<f64> {
    /// Predicts the output for the given input data.
    pub fn predict(&self, input: &DMatrix<f64>) -> DVector<f64> {
        let probabilities = self.predict_proba(input);

        // Predictions (y_hat)
        probabilities.map(|p| if p > 0.5 { 1. } else { 0. })
    }

    /// Compute the probabilities Pr(output_i=1|input_i,coefficients) for the given input data.
    pub fn predict_proba(&self, input: &DMatrix<f64>) -> DVector<f64> {
        let coef = self.coefficients.clone();
        let bias = coef[0];
        let n = coef.remove_row(0);
        let eta = (input * n).add_scalar(bias);

        // Probabilities
        ActivationFunction::logistic(&eta)
    }

    /// Compute the misclassification rate for given y and y_hat.
    pub fn score_misclassification(&self, y: &DVector<f64>, y_hat: &DVector<f64>) -> f64 {
        assert_eq!(y.shape(), y_hat.shape());
        let (N_samples, _) = y.shape();
        (y - y_hat).abs().sum() / N_samples as f64
    }

    /// Compute average cross-entropy for given y and p_hat.
    pub fn score_cross_entropy(&self, y: &DVector<f64>, p_hat: &DVector<f64>) -> f64 {
        //could be done with only one param input:&LogisticRegressionInput
        assert_eq!(y.shape(), p_hat.shape());

        let y_complement = (-y).add_scalar(1.);
        let p_complement = (-p_hat).add_scalar(1.);
        let log_p = p_hat.map(|x| x.ln());
        let log_p_complement = p_complement.map(|x| x.ln());

        // Cross-entropy
        (y.component_mul(&log_p) + y_complement.component_mul(&log_p_complement)).mean()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_logistic_regression {
    use crate::statistics::distributions::DistributionClass;

    use super::*;

    use std::time::Instant;
    // use crate::assert_approx_equal;

    #[test]
    fn test_logistic_regression() {
        // PROFILE THIS UNIT TEST WITH (on MacOS):
        // sudo -E cargo flamegraph --release --freq 5000 --unit-test -- tests_logistic_regression::test_logistic_regression

        // TEST DATA GENERATED FROM THE FOLLOWING R v4.3.0 CODE:
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
        let output = input.fit(LogisticRegressionAlgorithm::IRLS, f64::EPSILON.sqrt());
        let elapsed_none = start_none.elapsed();

        match output {
            Ok(output) => {
                println!("Iterations: \t{}", output.iterations);
                println!("Time taken: \t{:?}", elapsed_none);
                // println!("Intercept: \t{:?}", output.intercept);
                println!("Coefficients: \t{:?}", output.coefficients);
            }
            Err(err) => {
                panic!("Failed to fit logistic regression model: {}", err);
            }
        }

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

    #[test]
    fn test_logistic_regression_stochastic() {
        // cargo test --release   tests_logistic_regression::test_logistic_regression2 -- --nocapture

        // The test generates sample data in the following way:
        // - For each of the N samples (train/test) draw K feature values each from a uniform distribution over (-1.,1.) and arrange as design matrix "X".
        // - For the coefficients of the generating distribution draw K values from surface of the unit sphere S_(K-1)  and a bias from uniform(-0.7,0.7); arrange as DVector "coefs"
        // - compute vector of probabilities(target=1) as sigmoid(X_ext * coefs)
        // - compute target values:for each sample i draw from Bernoulli(prob_i)
        use crate::statistics::distributions::{Bernoulli, Distribution, Gaussian, Uniform};

        let N_train = 200; //Number of training samples
        let N_test = 40; //Number of test samples
        let K = 10; //Number of Features

        let distr_normal = Gaussian::default();
        let distr_uniform_bias = Uniform::new(-0.7, 0.7, DistributionClass::Continuous);
        let distr_uniform_steepness = Uniform::new(0.5, 5., DistributionClass::Continuous);

        //generate random coefficients which will be used to generate target values for the x_i (direction uniform from sphere, bias uniform between -0.5 and 0.5 ) scaled by steepness
        let bias = distr_uniform_bias.sample(1).unwrap()[0];
        let steepness = distr_uniform_steepness.sample(1).unwrap()[0];

        let coefs = DVector::from_vec(distr_normal.sample(K).unwrap())
            .normalize()
            .insert_row(0, bias)
            .scale(steepness);

        let logistic_regression = LogisticRegressionOutput {
            coefficients: coefs,
            iterations: 0,
        };

        //generate random design matrix for train/test
        let distr_uniform_features = Uniform::new(-0.5, 0.5, DistributionClass::Continuous);

        let x_train = DMatrix::<f64>::from_vec(
            N_train,
            K,
            distr_uniform_features.sample(N_train * K).unwrap(),
        );
        let x_test = DMatrix::from_vec(
            N_test,
            K,
            distr_uniform_features.sample(N_test * K).unwrap(),
        );

        //compute probabilities for each sample x_i
        let probs_train = logistic_regression.predict_proba(&x_train);
        let probs_test = logistic_regression.predict_proba(&x_test);

        // sample from Bernoulli distribution with p=p_i for each sample i
        let y_train = probs_train.map(|p| Bernoulli::new(p).sample(1).unwrap()[0]);
        let y_test = probs_test.map(|p| Bernoulli::new(p).sample(1).unwrap()[0]);

        // Fit the model to the training data.
        let input = LogisticRegressionInput {
            x: x_train,
            y: y_train,
        };

        let start_none = Instant::now();
        let output = input.fit(LogisticRegressionAlgorithm::IRLS, f64::EPSILON.sqrt());
        let elapsed_none = start_none.elapsed();

        match output {
            Ok(output) => {
                let y_hat = output.predict(&x_test);
                let misclassification_rate = output.score_misclassification(&y_test, &y_hat);
                let p_hat = output.predict_proba(&x_test);
                let crossentropy = output.score_cross_entropy(&y_test, &p_hat);
                println!(
                    "Number of samples N_train={}, N_test={}, number of Features K={}",
                    N_train, N_test, K
                );
                println!(
                    "Misclassification_rate(out of sample): \t{}",
                    misclassification_rate
                );
                println!("Avg crossentropy(out of sample): \t{}", crossentropy);
                println!("Iterations: \t{}", output.iterations);
                println!("Time taken: \t{:?}", elapsed_none);
                // println!("Intercept: \t{:?}", output.intercept);
                // print computed coeffs and original coeffs
                println!("Coefficients found by IRLS:\n{:?}", &output.coefficients);
                println!(
                    "Coefficients used for the generation of the training data:\n{:?}",
                    &logistic_regression.coefficients
                );
            }
            Err(err) => {
                panic!("Failed to fit logistic regression model: {}", err);
            }
        }
    }
}
