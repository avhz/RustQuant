// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::linspace;

/// Composite Simpson's 3/8 rule for numerical integration.
pub fn simpsons<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    assert!(n % 3 == 0, "'n' must be a multiple of 3.");

    let x = linspace(a, b, n + 1);
    let h = (b - a) / n as f64;

    let mut integral = 0.0;

    for i in 1..=(n / 3) {
        integral += f(x[3 * i - 3]) + 3.0 * (f(x[3 * i - 2]) + f(x[3 * i - 1])) + f(x[3 * i]);
    }

    (3.0 / 8.0) * h * integral
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_simpsons() {
        fn f(x: f64) -> f64 {
            x.sin()
        }

        let integral = simpsons(f, 0.0, 1.0, 3 * 3000);

        assert_approx_equal!(integral, 1.0 - 1_f64.cos(), 1e-4);
    }
}
