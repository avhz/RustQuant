// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num_complex::Complex;

/// Exponential distribution: X ~ Exp(lambda)
pub struct Exponential {
    /// Rate (inverse scale).
    lambda: f64,
}

impl Exponential {
    /// New instance of a Exponential distribution.
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0);

        Self { lambda }
    }

    /// Exponential characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        1.0 / (1.0 - i * t / self.lambda)
    }

    /// Exponential probability density function.
    pub fn pdf(&self, x: f64) -> f64 {
        assert!(x >= 0.0);

        self.lambda * (-self.lambda * x).exp()
    }

    /// Exponential distribution function.
    pub fn cdf(&self, x: f64) -> f64 {
        assert!(x >= 0.0);

        1.0 - (-self.lambda * x).exp()
    }
}

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
