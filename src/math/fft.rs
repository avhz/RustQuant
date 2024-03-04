// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num::Complex;
use std::f64::consts::PI;

// pub const i: Complex<f64> = Complex { re: 0.0, im: 1.0 };

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Real FFT inplace,
/// `x` length must be a power of 2
#[allow(clippy::module_name_repetitions)]
pub fn fft_real_inplace(x: &mut Vec<f64>) {
    check_vec_length(x);

    fft_real_calculation(x);
}

/// Real FFT and returns a new vector,
/// `x` length must be a power of 2
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn fft_real(x: &Vec<f64>) -> Vec<f64> {
    check_vec_length(x);

    let mut result = x.clone();

    fft_real_calculation(&mut result);

    result
}

/// Complex FFT inplace,
/// `x` length must be a power of 2
#[allow(clippy::module_name_repetitions)]
pub fn fft_complex_inplace(x: &mut Vec<Complex<f64>>) {
    check_vec_length(x);

    fft_complex_calculation(x);
}

/// Complex FFT and returns a new vector,
/// `x` length must be a power of 2
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn fft_complex(x: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    check_vec_length(x);

    let mut result = x.clone();

    fft_complex_calculation(&mut result);

    result
}

/// Helper function to check if a vector length is a power of 2
#[must_use]
pub fn is_valid_length<T>(x: &[T]) -> bool {
    ((x.len() as f64).log2() % 1.0).abs() < 1e-10
}

fn check_vec_length<T>(x: &Vec<T>) {
    assert!(
        is_valid_length(x),
        "FFT can only handle vectors which length is a power of 2."
    );
}

/// Real fourier transform in place
fn fft_real_calculation(x: &mut Vec<f64>) {
    let n = x.len();
    if n == 1 {
        return;
    }

    let (mut even, mut odd) = split_array(x);

    fft_real_calculation(&mut even);
    fft_real_calculation(&mut odd);

    for k in 0..(n / 2) {
        let t = Complex::new(0.0, -2.0 * PI * (k as f64) / (n as f64))
            .exp()
            .norm()
            * odd[k];
        x[k] = even[k] + t;
        x[n / 2 + k] = even[k] - t;
    }
}

/// Complex fourier transform of data in place
fn fft_complex_calculation(x: &mut Vec<Complex<f64>>) {
    let n = x.len();
    if n == 1 {
        return;
    }

    let (mut even, mut odd) = split_array(x);

    fft_complex_calculation(&mut even);
    fft_complex_calculation(&mut odd);

    for k in 0..(n / 2) {
        let t = Complex::new(0.0, -2.0 * PI * (k as f64) / (n as f64)).exp() * odd[k];
        x[k] = even[k] + t;
        x[n / 2 + k] = even[k] - t;
    }
}

fn split_array<T: Copy>(x: &[T]) -> (Vec<T>, Vec<T>) {
    let n = x.len();

    let mut even = Vec::with_capacity(n / 2);
    let mut odd = Vec::with_capacity(n / 2);

    for (i, x_value) in x.iter().enumerate() {
        if i % 2 == 0 {
            even.push(*x_value);
        } else {
            odd.push(*x_value);
        }
    }

    (even, odd)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test {
    use super::*;
    use num::Complex;

    const SQRT_20: f64 = 4.472_135_955;
    const REAL_TEST_SEQUENCE: [f64; 4] = [-1.0, 2.0, 3.0, 0.0];
    const REAL_TEST_RESULT: [f64; 4] = [4.0, SQRT_20, 0.0, SQRT_20];
    const COMPLEX_TEST_SEQUENCE: [Complex<f64>; 4] = [
        Complex::new(-1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
        Complex::new(0.0, 0.0),
    ];
    const COMPLEX_TEST_RESULT: [Complex<f64>; 4] = [
        Complex::new(4.0, 0.0),
        Complex::new(-4.0, -2.0),
        Complex::new(0.0, 0.0),
        Complex::new(-4.0, 2.0),
    ];

    fn assert_complex_vecs_almost_equal(x: &Vec<Complex<f64>>, y: &Vec<Complex<f64>>) {
        assert_eq!(x.len(), y.len());

        for (x_value, y_value) in x.iter().zip(y.iter()) {
            assert!((x_value - y_value).norm() <= 1e-10);
        }
    }

    fn assert_real_vecs_almost_equal(x: &Vec<f64>, y: &Vec<f64>) {
        assert_eq!(x.len(), y.len());

        for (x_value, y_value) in x.iter().zip(y.iter()) {
            assert!(x_value - y_value <= 1e-10);
        }
    }

    #[test]
    fn test_complex_inplace() {
        let mut test_vec = COMPLEX_TEST_SEQUENCE.to_vec();
        fft_complex_inplace(&mut test_vec);

        assert_complex_vecs_almost_equal(&test_vec, &COMPLEX_TEST_RESULT.to_vec());
    }

    #[test]
    fn test_complex_new_vec() {
        let test_vec = COMPLEX_TEST_SEQUENCE.to_vec();
        let result = fft_complex(&test_vec);

        assert_complex_vecs_almost_equal(&result, &COMPLEX_TEST_RESULT.to_vec());
        assert_complex_vecs_almost_equal(&test_vec, &COMPLEX_TEST_SEQUENCE.to_vec());
    }

    #[test]
    fn test_real_inplace() {
        let mut test_vec = REAL_TEST_SEQUENCE.to_vec();
        fft_real_inplace(&mut test_vec);

        assert_real_vecs_almost_equal(&test_vec, &REAL_TEST_RESULT.to_vec());
    }

    #[test]
    fn test_real_new_vec() {
        let test_vec = REAL_TEST_SEQUENCE.to_vec();
        let result = fft_real(&test_vec);

        assert_real_vecs_almost_equal(&result, &REAL_TEST_RESULT.to_vec());
        assert_real_vecs_almost_equal(&test_vec, &REAL_TEST_SEQUENCE.to_vec());
    }

    #[test]
    #[should_panic(expected = "FFT can only handle vectors which length is a power of 2.")]
    fn test_invalid_vec_length() {
        let test_vec = vec![0; 31];
        check_vec_length(&test_vec);
    }
}
