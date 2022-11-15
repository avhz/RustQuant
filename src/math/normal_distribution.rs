#![allow(non_snake_case)]
#![deny(missing_docs)]

use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use statrs::function::erf;
use std::f64::consts::{PI, SQRT_2};

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Standard Normal Density Function
pub fn dnorm(x: f64) -> f64 {
    (-x * x / 2.0).exp() / (2.0 * PI).sqrt()
}

/// Standard Normal Distribution Function
///
/// I used `erfc` (complementary error function) instead of `erf` to avoid
/// subtractive cancellation that leads to inaccuracy in the tails.
pub fn pnorm(x: f64) -> f64 {
    0.5 * erf::erfc(-x / SQRT_2)
}

/// Standard Normal Random Variates Generator
pub fn rnorm(n: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();

    // let mut variates: Vec<f64> = vec![0.0; n];
    let mut variates: Vec<f64> = Vec::with_capacity(n);

    for _ in 0..variates.capacity() {
        // variates[i] = normal.sample(&mut rng);
        variates.push(normal.sample(&mut rng));
    }

    // for item in &mut variates {
    //     item = normal.sample(&mut rng);
    // }

    variates
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::assert_approx_equal;

    #[test]
    fn TEST_pnorm() {
        // Values from WolframAlpha
        assert_approx_equal(pnorm(-4.0), 0.00003167, 1e-8);
        assert_approx_equal(pnorm(-3.0), 0.00134990, 1e-8);
        assert_approx_equal(pnorm(-2.0), 0.02275013, 1e-8);
        assert_approx_equal(pnorm(-1.0), 0.15865525, 1e-8);
        assert_approx_equal(pnorm(0.0), 0.5, 1e-8);
        assert_approx_equal(pnorm(1.0), 0.84134475, 1e-8);
        assert_approx_equal(pnorm(2.0), 0.97724987, 1e-8);
        assert_approx_equal(pnorm(3.0), 0.99865010, 1e-8);
        assert_approx_equal(pnorm(4.0), 0.99996833, 1e-8);
    }

    #[test]
    fn TEST_dnorm() {
        // Values from WolframAlpha
        assert_approx_equal(dnorm(-4.0), 0.00013383, 1e-8);
        assert_approx_equal(dnorm(-3.0), 0.00443185, 1e-8);
        assert_approx_equal(dnorm(-2.0), 0.05399097, 1e-8);
        assert_approx_equal(dnorm(-1.0), 0.24197072, 1e-8);
        assert_approx_equal(dnorm(0.0), 0.39894228, 1e-8);
        assert_approx_equal(dnorm(1.0), 0.24197072, 1e-8);
        assert_approx_equal(dnorm(2.0), 0.05399097, 1e-8);
        assert_approx_equal(dnorm(3.0), 0.00443185, 1e-8);
        assert_approx_equal(dnorm(4.0), 0.00013383, 1e-8);
    }

    #[test]
    fn TEST_rnorm() {
        let v = rnorm(1000);
        println!("{:?}", v);
        let mean = (v.iter().sum::<f64>()) / (v.len() as f64);
        println!("MEAN = {}", mean);
        // assert!(5 == 6);
    }
}
