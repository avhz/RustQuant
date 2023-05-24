// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Cumulative sum of a vector helper function.
pub fn cumsum(v1: &[f64]) -> Vec<f64> {
    let v2: Vec<f64> = v1
        .iter()
        .scan(0.0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    v2
}

#[cfg(test)]
mod tests_cumsum {
    use super::*;

    #[test]
    fn test_cumsum_empty() {
        let v: Vec<f64> = Vec::new();
        let result = cumsum(&v);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_cumsum_single() {
        let v: Vec<f64> = vec![5.0];
        let result = cumsum(&v);
        assert_eq!(result, vec![5.0]);
    }

    #[test]
    fn test_cumsum_multiple() {
        let v: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = cumsum(&v);
        assert_eq!(result, vec![1.0, 3.0, 6.0, 10.0, 15.0]);
    }

    #[test]
    fn test_cumsum_negative() {
        let v: Vec<f64> = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        let result = cumsum(&v);
        assert_eq!(result, vec![-1.0, -3.0, -6.0, -10.0, -15.0]);
    }

    #[test]
    fn test_cumsum_mixed() {
        let v: Vec<f64> = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let result = cumsum(&v);
        assert_eq!(result, vec![1.0, -1.0, 2.0, -2.0, 3.0]);
    }
}
