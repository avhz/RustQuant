// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use rayon::prelude::*;
use crate::math::integration::constants::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Integrates a function from `a` to `b`.
/// Uses the Tanh-Sinh quadrature over [-1, +1]
/// and then transforms to an integral over [a, b].
pub fn integrate<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    // Apply a linear change of variables:
    //
    // x = c * t + d
    //
    // where:
    //      c = 0.5 * (b - a)
    //      d = 0.5 * (a + b)

    let c = 0.5 * (b - a);
    let d = 0.5 * (a + b);

    c * tanhsinh(|x| {
        let out = f(c * x + d);
        if out.is_finite() {
            out
        } else {
            0.0
        }
    })
}

// This method integrates the function f(c * x + d).
fn tanhsinh<F>(f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut integral = 0.0;

    for i in 0..100 {
        integral += WEIGHTS[i] * f(ABSCISSAE[i])
    }

    integral
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_quadrature() {
        fn f(x: f64) -> f64 {
            (x.sin()).exp()
        }

        let integral = integrate(f, 0.0, 5.0);

        assert_approx_equal!(integral, 7.189119253631, 1e-8);
    }
}
