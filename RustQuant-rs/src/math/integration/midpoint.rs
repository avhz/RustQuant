// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::linspace;

/// Midpoint rule for numerical integration.
pub fn midpoint<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let y = linspace(a, b, n + 1);

    let mut integral = 0.0;

    for i in 0..n {
        integral += (y[i + 1] - y[i]) * f(0.5 * (y[i] + y[i + 1]))
    }

    integral
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_midpoint() {
        fn f(x: f64) -> f64 {
            x.sin()
        }

        let integral = midpoint(f, 0.0, 1.0, 10000);

        assert_approx_equal!(integral, 1.0 - 1_f64.cos(), 1e-4);
    }
}
