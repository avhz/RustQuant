// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the function value and function root.
pub struct NewtonRaphson {
    /// Value of function evaluated at `x`.
    pub value: f64,
    /// Root of function.
    pub root: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl NewtonRaphson {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Find a root using the Newton-Raphson algorithm.
pub fn find_root(f: fn(f64) -> f64, df: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
    let mut result = guess;

    let iteration =
        |f: fn(f64) -> f64, df: fn(f64) -> f64, guess: f64| -> f64 { guess - f(guess) / df(guess) };

    for _ in 0..iterations {
        result = iteration(f, df, result);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_newton_raphson() {
        fn f(x: f64) -> f64 {
            x * x
        }
        fn df(x: f64) -> f64 {
            2.0 * x
        }
        let root = find_root(f, df, 10.0, 100);
        assert_approx_equal!(root, 0.0, 1e-10);
        println!("ROOT = {}", root);
    }
}
