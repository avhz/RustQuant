// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Trait for generating sequences of numbers.
pub trait Seq<T> {
    /// Generate a sequence of numbers from `start` to `end` with a step size of `step`.
    fn seq(start: T, end: T, step: T) -> Vec<T>;
}

impl Seq<f64> for f64 {
    fn seq(start: f64, end: f64, step: f64) -> Vec<f64> {
        let mut seq = Vec::with_capacity(((end - start) / step).abs() as usize);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
    }
}

impl Seq<f32> for f32 {
    fn seq(start: f32, end: f32, step: f32) -> Vec<f32> {
        let mut seq = Vec::with_capacity(((end - start) / step).abs() as usize);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
    }
}

impl Seq<i32> for i32 {
    fn seq(start: i32, end: i32, step: i32) -> Vec<i32> {
        let mut seq = Vec::with_capacity(((end - start) / step).unsigned_abs() as usize);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
    }
}

impl Seq<i64> for i64 {
    fn seq(start: i64, end: i64, step: i64) -> Vec<i64> {
        let mut seq = Vec::with_capacity(((end - start) / step).unsigned_abs() as usize);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
    }
}

impl Seq<u32> for u32 {
    fn seq(start: u32, end: u32, step: u32) -> Vec<u32> {
        let mut seq = Vec::with_capacity(((end - start) / step) as usize);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
    }
}

impl Seq<u64> for u64 {
    fn seq(start: u64, end: u64, step: u64) -> Vec<u64> {
        let mut seq = Vec::with_capacity(((end - start) / step) as usize);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
    }
}

impl Seq<usize> for usize {
    fn seq(start: usize, end: usize, step: usize) -> Vec<usize> {
        let mut seq = Vec::with_capacity((end - start) / step);
        let mut x = start;
        while x <= end {
            seq.push(x);
            x += step;
        }
        seq
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
}
