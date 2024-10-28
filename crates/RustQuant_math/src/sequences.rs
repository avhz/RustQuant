// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Sequences of numbers in the style of R's `seq` and `rep` functions.

use num::{FromPrimitive, Num, ToPrimitive};

/// Trait for generating sequences of numbers.
pub trait Sequence<T: Num + PartialOrd + Copy + FromPrimitive + ToPrimitive> {
    /// Generate a sequence of numbers from `start` to `end` with a step size of `step`.
    fn seq(start: T, end: T, step: T) -> Vec<T>;

    /// Repeat a number `x`, `n` times.
    fn rep(x: T, n: usize) -> Vec<T>;

    /// Generate a sequence of numbers from `start` to `end` with `n` elements (linearly spaced).
    fn linspace(start: T, end: T, n: usize) -> Vec<T>;

    /// Generate a sequence of numbers from `start` to `end` with `n` elements (logarithmically spaced).
    fn logspace(start: T, end: T, n: usize) -> Vec<T>;

    /// Compute the cumulative sum of a vector.
    fn cumsum(v: &[T]) -> Vec<T>;
}

impl<T> Sequence<T> for T
where
    T: Num + PartialOrd + Copy + FromPrimitive + ToPrimitive,
{
    fn seq(start: T, end: T, step: T) -> Vec<T> {
        let mut seq = Vec::with_capacity(((end - start) / step).to_usize().unwrap());
        let mut x = start;

        while x <= end {
            seq.push(x);
            x = x + step;
        }

        seq
    }

    fn rep(x: T, n: usize) -> Vec<T> {
        vec![x; n]
    }

    fn linspace(start: T, end: T, n: usize) -> Vec<T> {
        assert!(
            start < end && n > 0,
            "Invalid parameters: start < end and n > 0"
        );

        let step = (end - start) / T::from_usize(n - 1).unwrap();
        let mut v: Vec<T> = Vec::with_capacity(n);

        for i in 0..n {
            v.push(start + T::from_usize(i).unwrap() * step);
        }
        v
    }

    fn logspace(_start: T, _end: T, _n: usize) -> Vec<T> {
        todo!()
    }

    fn cumsum(v: &[T]) -> Vec<T> {
        let v2: Vec<T> = v
            .iter()
            .scan(T::zero(), |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect();

        v2
    }
}

#[cfg(test)]
mod tests_sequences {
    use super::*;
    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON as EPS};

    #[test]
    fn test_seq_f64() {
        let seq = f64::seq(0., 1., 0.1);
        let expected = [0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.];

        for (i, x) in seq.iter().enumerate() {
            assert_approx_equal!(x, &expected[i], 1e-6);
        }
    }

    #[test]
    fn test_seq_f32() {
        let seq = f32::seq(0., 1., 0.1);
        let expected = [0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];

        for (i, x) in seq.iter().enumerate() {
            assert_approx_equal!(x, &expected[i], 1e-6);
        }
    }

    #[test]
    fn test_seq_i32() {
        let seq = i32::seq(0, 10, 2);
        assert_eq!(seq, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_seq_i64() {
        let seq = i64::seq(0, 10, 2);
        assert_eq!(seq, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_seq_u32() {
        let seq = u32::seq(0, 10, 2);
        assert_eq!(seq, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_seq_u64() {
        let seq = u64::seq(0, 10, 2);
        assert_eq!(seq, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_seq_usize() {
        let seq = usize::seq(0, 10, 2);
        assert_eq!(seq, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_rep_f64() {
        let seq = f64::rep(1., 5);
        assert_eq!(seq, vec![1., 1., 1., 1., 1.]);
    }

    #[test]
    fn test_rep_f32() {
        let seq = f32::rep(1., 5);
        assert_eq!(seq, vec![1., 1., 1., 1., 1.]);
    }

    #[test]
    fn test_rep_i32() {
        let seq = i32::rep(1, 5);
        assert_eq!(seq, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_rep_i64() {
        let seq = i64::rep(1, 5);
        assert_eq!(seq, vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_cumsum_empty() {
        let v: Vec<f64> = Vec::new();
        let result = f64::cumsum(&v);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_cumsum_single() {
        let v: Vec<f64> = vec![5.0];
        let result = f64::cumsum(&v);
        assert_eq!(result, vec![5.0]);
    }

    #[test]
    fn test_cumsum_multiple() {
        let v: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = f64::cumsum(&v);
        assert_eq!(result, vec![1.0, 3.0, 6.0, 10.0, 15.0]);
    }

    #[test]
    fn test_cumsum_negative() {
        let v: Vec<f64> = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        let result = f64::cumsum(&v);
        assert_eq!(result, vec![-1.0, -3.0, -6.0, -10.0, -15.0]);
    }

    #[test]
    fn test_cumsum_mixed() {
        let v: Vec<f64> = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let result = f64::cumsum(&v);
        assert_eq!(result, vec![1.0, -1.0, 2.0, -2.0, 3.0]);
    }

    #[test]
    fn test_linspace_positive_step() {
        let v = f64::linspace(1.0, 5.0, 5);
        assert_eq!(v, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_linspace_negative_step() {
        let v = f64::linspace(-5.0, -1.0, 5);
        assert_eq!(v, vec![-5.0, -4.0, -3.0, -2.0, -1.0]);
    }

    #[test]
    fn test_linspace_n_is_one() {
        let v = f64::linspace(1.0, 5.0, 1);
        assert!(v[0].is_nan());
    }

    #[test]
    #[should_panic(expected = "Invalid parameters: start < end and n > 0")]
    fn test_linspace_a_equals_b() {
        f64::linspace(1.0, 1.0, 5);
    }

    #[test]
    #[should_panic(expected = "Invalid parameters: start < end and n > 0")]
    fn test_linspace_n_is_zero() {
        f64::linspace(1.0, 5.0, 0);
    }

    #[test]
    #[should_panic(expected = "Invalid parameters: start < end and n > 0")]
    fn test_linspace_a_greater_than_b() {
        f64::linspace(5.0, 1.0, 5);
    }
}
