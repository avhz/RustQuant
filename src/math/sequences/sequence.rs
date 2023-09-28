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
}

#[cfg(test)]
mod tests_sequences {
    use super::*;
    use crate::assert_approx_equal;

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
}
