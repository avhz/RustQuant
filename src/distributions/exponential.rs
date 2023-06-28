// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::Distribution;
use num_complex::Complex;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Exponential distribution: X ~ Exp(lambda)
pub struct Exponential {
    /// Rate (inverse scale).
    lambda: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Exponential {
    /// New instance of a Exponential distribution.
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0);

        Self { lambda }
    }
}

impl Distribution for Exponential {
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        1.0 / (1.0 - i * t / self.lambda)
    }

    fn pdf(&self, x: f64) -> f64 {
        assert!(x >= 0.0);

        self.lambda * (-self.lambda * x).exp()
    }

    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }

    fn cdf(&self, x: f64) -> f64 {
        assert!(x >= 0.0);

        1.0 - (-self.lambda * x).exp()
    }

    fn inv_cdf(&self, p: f64) -> f64 {
        -(1. - p).ln() / self.lambda
    }

    fn mean(&self) -> f64 {
        self.lambda.recip()
    }

    fn median(&self) -> f64 {
        2_f64.ln() / self.lambda
    }

    fn mode(&self) -> f64 {
        0.0
    }

    fn variance(&self) -> f64 {
        self.lambda.recip().powi(2)
    }

    fn skewness(&self) -> f64 {
        2.0
    }

    fn kurtosis(&self) -> f64 {
        6.0
    }

    fn entropy(&self) -> f64 {
        1.0 - self.lambda.ln()
    }

    fn mgf(&self, t: f64) -> f64 {
        assert!(t < self.lambda);

        self.lambda * (self.lambda - t).recip()
    }

    fn sample(&self, n: usize) -> Vec<f64> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Exp};

        let mut rng = thread_rng();
        let dist = Exp::new(self.lambda).unwrap();
        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(dist.sample(&mut rng));
        }

        variates
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_exponential_characteristic_function() {
        let dist: Exponential = Exponential::new(1.0);

        // // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.5, 1e-10);
        assert_approx_equal!(cf.im, 0.5, 1e-10);
    }

    #[test]
    fn test_exponential_density_function() {
        let dist: Exponential = Exponential::new(1.0);

        // Values computed using R
        assert_approx_equal!(dist.pdf(0.0), 1.00000000, 1e-8);
        assert_approx_equal!(dist.pdf(1.0), 0.36787944, 1e-8);
        assert_approx_equal!(dist.pdf(2.0), 0.13533528, 1e-8);
        assert_approx_equal!(dist.pdf(3.0), 0.04978707, 1e-8);
        assert_approx_equal!(dist.pdf(4.0), 0.01831564, 1e-8);
    }

    #[test]
    fn test_exponential_distribution_function() {
        let dist: Exponential = Exponential::new(1.0);

        // Values computed using R
        assert_approx_equal!(dist.cdf(0.0), 0.0000000, 1e-7);
        assert_approx_equal!(dist.cdf(1.0), 0.6321206, 1e-7);
        assert_approx_equal!(dist.cdf(2.0), 0.8646647, 1e-7);
        assert_approx_equal!(dist.cdf(3.0), 0.9502129, 1e-7);
        assert_approx_equal!(dist.cdf(4.0), 0.9816844, 1e-7);
    }
}
