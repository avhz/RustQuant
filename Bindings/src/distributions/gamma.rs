// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num_complex::Complex;
use statrs::function::gamma::{gamma, gamma_li};

/// Gamma distribution: X ~ Gamma(alpha, beta) = Gamma(alpha, 1/beta)
pub struct Gamma {
    /// Alpha: the shape parameter.
    alpha: f64,
    /// Beta: the rate parameter (inverse scale).
    beta: f64,
}

impl Gamma {
    /// New instance of a Gamma distribution.
    pub fn new(alpha: f64, beta: f64) -> Self {
        assert!(alpha > 0.0 && beta > 0.0);

        Self { alpha, beta }
    }

    /// Gamma characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        let alpha = self.alpha;
        let beta = self.beta;

        (1.0 - i * t / beta).powf(-alpha)
    }

    /// Gamma probability density function.
    pub fn pdf(&self, x: f64) -> f64 {
        assert!(x > 0.0);

        let alpha = self.alpha;
        let beta = self.beta;

        beta.powf(alpha) * x.powf(alpha - 1.0) * (-beta * x).exp() / gamma(alpha)
    }

    /// Gamma distribution function.
    pub fn cdf(&self, x: f64) -> f64 {
        assert!(x > 0.0);

        let alpha = self.alpha;
        let beta = self.beta;

        gamma_li(alpha, beta * x) / gamma(alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_gamma_characteristic_function() {
        let dist: Gamma = Gamma::new(1.0, 1.0);

        // // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.5, 1e-10);
        assert_approx_equal!(cf.im, 0.5, 1e-10);
    }

    #[test]
    fn test_gamma_density_function() {
        // Gamma(1,1) is equivalent to Exp(1)
        let dist: Gamma = Gamma::new(1.0, 1.0);

        // Values computed using R
        // assert_approx_equal!(dist.pdf(0.0), 1.00000000, 1e-8);
        assert_approx_equal!(dist.pdf(1.0), 0.36787944, 1e-8);
        assert_approx_equal!(dist.pdf(2.0), 0.13533528, 1e-8);
        assert_approx_equal!(dist.pdf(3.0), 0.04978707, 1e-8);
        assert_approx_equal!(dist.pdf(4.0), 0.01831564, 1e-8);
    }

    #[test]
    fn test_gamma_distribution_function() {
        let dist: Gamma = Gamma::new(1.0, 1.0);

        // Values computed using R
        // assert_approx_equal!(dist.cdf(0.0), 0.0000000, 1e-7);
        assert_approx_equal!(dist.cdf(1.0), 0.6321206, 1e-7);
        assert_approx_equal!(dist.cdf(2.0), 0.8646647, 1e-7);
        assert_approx_equal!(dist.cdf(3.0), 0.9502129, 1e-7);
        assert_approx_equal!(dist.cdf(4.0), 0.9816844, 1e-7);
    }
}
