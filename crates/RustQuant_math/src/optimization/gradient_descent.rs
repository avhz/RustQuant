// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! ## Gradient Descent Primer  
//!
//! We want to implement an algorithm for solving uncostrained optimisation problems of the form:
//!
//! $$
//! \min_{x \in \mathbb{R}^n} f(x) \qquad f(x) \in \mathcal{C}^1
//! $$
//!
//! when the objective function $f(x)$ and its gradient, $\nabla f(x)$, are known.
//!
//! We start with an initial guess, $x_0$, and perform the iteration:
//!
//! $$
//! x_{k+1} = x_k + \alpha_k d_k = x_k - \alpha_k \nabla f(x_k)
//! $$
//!
//! Where:
//!
//! $$
//! d_k = -\nabla f(x_k) \qquad \text{is the descent direction}
//! $$
//!
//! and
//!
//! $$
//! \alpha_k \qquad \text{is the step size in iteration $k$}
//! $$
//!
//! This iteration gives us a monotonic sequence which converges to a local minimum, $f(x^*)$, if it exists:
//!
//! $$
//! f(x_0) \geq f(x_1) \geq f(x_2) \geq \cdots \geq f(x^*)
//! $$
//!
//! The algorithm is repeated until the stationarity condition is fulfilled:
//!
//! $$
//! \nabla f(x) = 0
//! $$
//!
//! Numerically, this condition is fulfilled if:
//!
//! $$
//! \| \nabla f(x_{k+1}) \| \leq \epsilon
//! $$
//!
//! Where $\|\cdot\|$ denotes the Euclidean norm:
//!
//! $$
//! \|x\| = \sqrt{\langle x,x \rangle}
//! $$
//!
//! Or in Rust, something like:
//!
//! ```ignore
//! gradient.iter().map(|&x| x * x).sum::<f64>().sqrt() < std::f64::EPSILON.sqrt()
//! ```
//!
//! See [this example](https://github.com/avhz/RustQuant/blob/main/examples/gradient_descent.rs)
//! for a demonstration using Himmelblau's function:
//!
//! $$
//! f(x,y) = (x^2 + y - 11)^2 + (x + y^2 - 7)^2
//! $$

use std::time::{Duration, Instant};
use RustQuant_autodiff::{variables::variable::Variable, Accumulate, Gradient, Graph};

// use ::log::{info, max_level, warn, Level};

/// Gradient descent optimizer.
/// NOTE: Only for functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$ for now.
/// The gradient descent optimizer is an iterative algorithm that
/// finds the local minimum of a function.
/// The algorithm starts with an initial guess for the local minimum
/// and moves iteratively in the direction of the negative gradient
/// until the gradient is close to zero.
#[derive(Default, Debug, Clone)]
pub struct GradientDescent {
    /// Learning rate (aka. alpha or eta).
    pub learning_rate: f64,

    /// Maximum number of iterations.
    pub max_iterations: usize,

    /// Tolerance for the gradient.
    pub tolerance: Option<f64>,
}

/// Result of the gradient descent optimization.
#[allow(clippy::module_name_repetitions)]
pub struct GradientDescentResult {
    /// Minimizer of the function.
    pub minimizer: Vec<f64>,

    /// Value of the function at the minimum.
    pub minimum: f64,

    /// Number of iterations.
    pub iterations: usize,

    /// Time elapsed during optimization.
    pub elapsed: Duration,
}

impl GradientDescent {
    /// Returns a new instance of the gradient descent optimizer.
    ///
    /// # Panics
    ///
    /// Panics if tolerance is not positive.
    #[must_use]
    pub fn new(learning_rate: f64, max_iterations: usize, tolerance: Option<f64>) -> Self {
        if tolerance.is_some() {
            assert!(tolerance.unwrap() > 0.0);
        }

        Self {
            learning_rate,
            max_iterations,
            tolerance,
        }
    }

    /// Checks if the gradient is equal to zero.
    /// This is a necessary condition for a local minimum.
    #[inline]
    fn is_stationary(gradient: &[f64], tol: f64) -> bool {
        gradient.iter().map(|&x| x * x).sum::<f64>().sqrt() < tol
    }

    /// Compute Euclidean norm of a vector.
    #[inline]
    fn norm(x: &[f64]) -> f64 {
        x.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    // /// Compute the dot product of two vectors.
    // fn dot(x: &[f64], y: &[f64]) -> f64 {
    //     x.iter().zip(y.iter()).map(|(&x, &y)| x * y).sum()
    // }

    /// Performs gradient descent optimization.
    #[allow(clippy::assign_op_pattern)]
    pub fn optimize<F>(&self, f: F, x0: &[f64], verbose: bool) -> GradientDescentResult
    where
        F: for<'v> Fn(&[Variable<'v>]) -> Variable<'v>,
    {
        let start = Instant::now();

        let tolerance = self.tolerance.unwrap_or(f64::EPSILON.sqrt());

        let mut result = GradientDescentResult {
            minimum: 0.0,
            minimizer: x0.to_vec(),
            iterations: 0,
            elapsed: start.elapsed(),
        };

        for k in 0..self.max_iterations {
            let graph = Graph::new();

            result.iterations = k + 1;

            let location = graph.vars(&result.minimizer);
            let function = f(&location);
            let gradient = function.accumulate().wrt(&location);

            if Self::is_stationary(&gradient, tolerance) {
                break;
            }

            // for (xi, gi) in result.minimizer.iter_mut().zip(&gradient) {
            //     // Cannot use -= since it is not implemented for `Variable`.
            //     *xi = (*xi) - self.learning_rate * (*gi);
            // }

            result
                .minimizer
                .iter_mut()
                .zip(&gradient)
                .for_each(|(xi, gi)| *xi = *xi - self.learning_rate * gi);

            result.minimum = f(&location).value;

            if verbose {
                println!(
                    "Iter: {:?}, Norm: {}, Func: {:.4?}, X: {:.4?}",
                    k + 1,
                    Self::norm(&gradient),
                    function.value,
                    location.iter().map(|x| x.value).collect::<Vec<f64>>()
                );
            }

            // if max_level() == Level::Info {
            //     info!(
            //         "Iter: {:?}, Norm: {}, Func: {:.4?}, X: {:.4?}",
            //         k + 1,
            //         Self::norm(&gradient),
            //         function.value,
            //         location.iter().map(|x| x.value),
            //     );
            // }
        }

        result.elapsed = start.elapsed();
        result
    }
}

#[cfg(test)]
mod test_gradient_descent {
    use super::*;
    use RustQuant_autodiff::overload::Powf;
    use RustQuant_autodiff::variables::variable::Variable;

    // Test the creation of a new GradientDescent optimizer.
    #[test]
    fn test_gradient_descent_new() {
        let gd = GradientDescent::new(0.1, 1000, Some(0.0001));
        assert_eq!(gd.learning_rate, 0.1);
        assert_eq!(gd.max_iterations, 1000);
        assert_eq!(gd.tolerance, Some(0.0001));
    }

    // Test the is_stationary function.
    #[test]
    fn test_is_stationary() {
        assert!(GradientDescent::is_stationary(&[0.00001, 0.00001], 0.0001));
        assert!(!GradientDescent::is_stationary(&[0.01, 0.01], 0.0001));
    }

    // Test the norm function.
    #[test]
    fn test_norm() {
        // let graph = graph::new();
        // let vars = graph.vars(&vec![3.0, 4.0]);
        assert_eq!(GradientDescent::norm(&[3.0, 4.0]), 5.0);
    }

    // Test the optimize function on x^2.
    #[test]
    fn test_optimize_x_squared() {
        // Function: f(x) = x^2
        // Gradient: f'(x) = 2x
        // Minimum: f(0) = 0
        fn f<'v>(x: &[Variable<'v>]) -> Variable<'v> {
            x[0] * x[0]
        }

        // GradientDescent::new(learning_rate, max_iterations, tolerance)
        let gd = GradientDescent::new(0.1, 1000, Some(0.000_001));
        let result = gd.optimize(f, &[10.0], false);

        println!("Minimum: {:?}", result.minimum);
        println!("Minimizer: {:?}", result.minimizer);
        println!("Iterations: {:?}", result.iterations);
        println!("Elapsed: {:?}", result.elapsed);
    }

    // Test the optimize function on Booth function.
    // Function: f(x,y) = (x + 2y - 7)^2 + (2x + y - 5)^2
    // Gradient: f'(x,y) = [2(x + 2y - 7) + 4(2x + y - 5),
    //                      4(x + 2y - 7) + 2(2x + y - 5)]
    // Minimum: f(1, 3) = 0
    #[test]
    fn test_optimize_booth() {
        fn f<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
            let x = variables[0];
            let y = variables[1];

            (x + 2. * y - 7.).powf(2.0) + (2. * x + y - 5.).powf(2.0)
        }

        // GradientDescent::new(learning_rate, max_iterations, tolerance)
        let gd = GradientDescent::new(0.1, 1000, Some(0.000_001));
        let result = gd.optimize(f, &[5.0, 5.0], false);

        println!("Minimum: {:?}", result.minimum);
        println!("Minimizer: {:?}", result.minimizer);
        println!("Iterations: {:?}", result.iterations);
    }

    // Test the optimize function on Rosenbrock function (a = 1, b = 100, n = 2).
    // Function: f(x,y) = (1 - x)^2 + 100(y - x^2)^2
    // Gradient: f'(x,y) = [-2(1 - x) - 400x(y - x^2),
    //                      200(y - x^2)]
    // Minimum: f(1, 1) = 0
    #[test]
    fn test_optimize_rosenbrock() {
        fn f<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
            let x = variables[0];
            let y = variables[1];

            (1. - x).powf(2.0) + 100. * (y - x.powf(2.0)).powf(2.0)
        }

        // GradientDescent::new(learning_rate, max_iterations, tolerance)
        let gd = GradientDescent::new(0.001, 10000, Some(0.000_001));
        let result = gd.optimize(f, &[0.0, 5.0], false);

        println!("Minimum: {:?}", result.minimum);
        println!("Minimizer: {:?}", result.minimizer);
        println!("Iterations: {:?}", result.iterations);
    }

    // Test the optimize function on Himmelblau function.

    // Test the optimize function on Beale function.
}
