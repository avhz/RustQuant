#![deny(missing_docs)]

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
pub fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
    let mut result = guess;

    let iteration =
        |f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64| -> f64 { guess - f(guess) / fd(guess) };

    for _ in 0..iterations {
        result = iteration(f, fd, result);
    }
    result
}

#[cfg(test)]
mod tests {
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
        println!("ROOT = {}", root);
        assert_eq!(1, 2)
    }
}
