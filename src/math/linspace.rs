// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Linspace helper function.
/// Generates a sequence from `a` to `b` with `n` elements.
pub fn linspace(a: f64, b: f64, n: usize) -> Vec<f64> {
    assert!(a < b && n > 0, "Invalid parameters: a < b and n > 0");

    let step = (b - a) / (n - 1) as f64;
    let mut v: Vec<f64> = Vec::with_capacity(n);

    for i in 0..n {
        v.push(a + i as f64 * step);
    }
    v
}

#[cfg(test)]
mod tests_linspace {
    use super::*;

    #[test]
    fn test_linspace_positive_step() {
        let v = linspace(1.0, 5.0, 5);
        assert_eq!(v, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_linspace_negative_step() {
        let v = linspace(-5.0, -1.0, 5);
        assert_eq!(v, vec![-5.0, -4.0, -3.0, -2.0, -1.0]);
    }

    #[test]
    fn test_linspace_n_is_one() {
        let v = linspace(1.0, 5.0, 1);
        assert!(v[0].is_nan());
    }

    #[test]
    #[should_panic(expected = "Invalid parameters: a < b and n > 0")]
    fn test_linspace_a_equals_b() {
        linspace(1.0, 1.0, 5);
    }

    #[test]
    #[should_panic(expected = "Invalid parameters: a < b and n > 0")]
    fn test_linspace_n_is_zero() {
        linspace(1.0, 5.0, 0);
    }

    #[test]
    #[should_panic(expected = "Invalid parameters: a < b and n > 0")]
    fn test_linspace_a_greater_than_b() {
        linspace(5.0, 1.0, 5);
    }
}
