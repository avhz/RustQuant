// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::*;

/// Gradient descent optimizer.
/// NOTE: Only for functions f: R^n -> R for now.
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
    pub tolerance: f64,
}

/// Result of the gradient descent optimization.
pub struct GradientDescentResult {
    /// Minimizer of the function.
    pub minimizer: Vec<f64>,
    /// Value of the function at the minimum.
    pub minimum: f64,
    /// Number of iterations.
    pub iterations: usize,
}

impl GradientDescent {
    /// Returns a new instance of the gradient descent optimizer.
    pub fn new(learning_rate: f64, max_iterations: usize, tolerance: f64) -> Self {
        assert!(tolerance > 0.0);

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
    pub fn optimize<F>(&self, f: F, x0: &[f64], verbose: bool) -> GradientDescentResult
    where
        F: for<'v> Fn(&[Variable<'v>]) -> Variable<'v>,
    {
        let mut result = GradientDescentResult {
            minimum: 0.0,
            minimizer: x0.to_vec(),
            iterations: 0,
        };

        for k in 0..self.max_iterations {
            let tape = Tape::new();

            result.iterations = k + 1;

            let location = tape.vars(&result.minimizer);
            let function = f(&location);
            let gradient = function.accumulate().wrt(&location);

            if Self::is_stationary(&gradient, self.tolerance) {
                break;
            }

            for (xi, gi) in result.minimizer.iter_mut().zip(&gradient) {
                // Cannot use -= since it is not implemented for `Variable`.
                *xi = (*xi) - self.learning_rate * (*gi);
            }

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
        }

        result
    }
}

#[cfg(test)]
mod test_gradient_descent {
    use super::*;
    use crate::autodiff::Variable;

    // Test the creation of a new GradientDescent optimizer.
    #[test]
    fn test_gradient_descent_new() {
        let gd = GradientDescent::new(0.1, 1000, 0.0001);
        assert_eq!(gd.learning_rate, 0.1);
        assert_eq!(gd.max_iterations, 1000);
        assert_eq!(gd.tolerance, 0.0001);
    }

    // Test the is_stationary function.
    #[test]
    fn test_is_stationary() {
        assert!(GradientDescent::is_stationary(
            &vec![0.00001, 0.00001],
            0.0001
        ));
        assert!(!GradientDescent::is_stationary(&vec![0.01, 0.01], 0.0001));
    }

    // Test the norm function.
    #[test]
    fn test_norm() {
        // let tape = Tape::new();
        // let vars = tape.vars(&vec![3.0, 4.0]);
        assert_eq!(GradientDescent::norm(&vec![3.0, 4.0]), 5.0);
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
        let gd = GradientDescent::new(0.1, 1000, 0.000001);
        let result = gd.optimize(f, &vec![10.0], false);

        println!("Minimum: {:?}", result.minimum);
        println!("Minimizer: {:?}", result.minimizer);
        println!("Iterations: {:?}", result.iterations);
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
        let gd = GradientDescent::new(0.1, 1000, 0.000001);
        let result = gd.optimize(f, &vec![5.0, 5.0], false);

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
        let gd = GradientDescent::new(0.001, 10000, 0.000001);
        let result = gd.optimize(f, &vec![0.0, 5.0], false);

        println!("Minimum: {:?}", result.minimum);
        println!("Minimizer: {:?}", result.minimizer);
        println!("Iterations: {:?}", result.iterations);
    }

    // Test the optimize function on Himmelblau function.

    // Test the optimize function on Beale function.
}
