// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Method of averaging (arithmetic, geometric, or harmonic).
pub enum MeanType {
    /// Arithmetic mean: `sum(x_i) / n`
    Arithmetic,
    /// Geometric mean: `prod(x_i)^(1/n)`
    Geometric,
    /// Harmonic mean: `n / sum(1/x_i)`
    Harmonic,
}

/// Mean of a vector.
pub fn mean(v: &Vec<f64>, mean_type: MeanType) -> f64 {
    assert!(!v.is_empty(), "Vector must have at least one element.");

    match mean_type {
        MeanType::Arithmetic => v.iter().sum::<f64>() / v.len() as f64,
        MeanType::Geometric => v.iter().product::<f64>().powf(1.0 / v.len() as f64),
        MeanType::Harmonic => v.len() as f64 / v.iter().map(|x| 1.0 / x).sum::<f64>(),
    }
}

#[cfg(test)]
mod tests_mean {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_mean_arithmetic() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let m = mean(&v, MeanType::Arithmetic);
        assert_eq!(m, 3.0);
    }

    #[test]
    fn test_mean_geometric() {
        let v = vec![1.0, 2.0, 4.0, 8.0, 16.0];
        let m = mean(&v, MeanType::Geometric);
        assert_approx_equal!(m, 4.0, 1e-10);
        // assert!((m - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_mean_harmonic() {
        let v = vec![1.0, 2.0, 4.0];
        let m = mean(&v, MeanType::Harmonic);
        assert_approx_equal!(m, 1.7142857, 1e-7);
        // assert!((m - 1.7142857).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic(expected = "Vector must have at least one element.")]
    fn test_mean_empty_vector() {
        let v: Vec<f64> = vec![];
        mean(&v, MeanType::Arithmetic);
    }
}
