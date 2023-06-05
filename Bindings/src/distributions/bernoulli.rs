// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::Distribution;
use core::panic;
use num_complex::Complex;

/// Bernoulli distribution: X ~ Bern(p)
pub struct Bernoulli {
    /// Probability of k = 1 (q = 1 - p: probability of k = 0).
    p: f64,
}

impl Bernoulli {
    /// New instance of a Bernoulli distribution.
    pub fn new(probability: f64) -> Bernoulli {
        assert!((0.0..=1.0).contains(&probability));

        Bernoulli { p: probability }
    }
}

impl Distribution for Bernoulli {
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0.0..=1.0).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        1.0 - self.p + self.p * (i * t).exp()
    }

    fn pdf(&self, _x: f64) -> f64 {
        panic!("Bernoulli distribution is discrete. Use pmf() instead.");
    }

    fn pmf(&self, k: f64) -> f64 {
        assert!((0.0..=1.0).contains(&self.p));
        assert!(k == 0.0 || k == 1.0);

        (self.p).powi(k as i32) * (1.0 - self.p).powi(1 - k as i32)
    }

    fn cdf(&self, k: f64) -> f64 {
        assert!((0.0..=1.0).contains(&self.p));

        if (k as i32) < 0 {
            0.0
        } else if (0..1).contains(&(k as i32)) {
            1.0 - self.p
        } else {
            1.0
        }
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        panic!("Generalised inverse CDF not implemented for Bernoulli distribution.");
    }

    fn mean(&self) -> f64 {
        self.p
    }

    /// Returns the median of the Bernoulli distribution.
    fn median(&self) -> f64 {
        panic!("Median not implemented for Bernoulli distribution.");
    }

    fn mode(&self) -> f64 {
        panic!("Mode not implemented for Bernoulli distribution.");
    }

    fn variance(&self) -> f64 {
        self.p * (1.0 - self.p)
    }

    fn skewness(&self) -> f64 {
        let p = self.p;
        ((1.0 - p) - p) / (p * (1.0 - p)).sqrt()
    }

    fn kurtosis(&self) -> f64 {
        let p = self.p;
        (1.0 - 6.0 * p * (1.0 - p)) / (p * (1.0 - p))
    }

    fn entropy(&self) -> f64 {
        (self.p - 1.0) * (1.0 - self.p).ln() - self.p * (self.p).ln()
    }

    fn mgf(&self, t: f64) -> f64 {
        1.0 - self.p + self.p * f64::exp(t)
    }
}

#[cfg(test)]
mod tests_bernoulli {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_bernoulli_functions() {
        let dist: Bernoulli = Bernoulli::new(1.0);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
        assert_approx_equal!(cf.im, 0.84147098480, 1e-10);

        let bernoulli = Bernoulli::new(0.5);

        // Probability mass function
        let pmf = dist.pmf(1.0);
        assert_approx_equal!(pmf, 1.0, 1e-10);
        // Test pmf for k = 0.0 and 1.0
        let pmf_zero = bernoulli.pmf(0.0);
        let pmf_one = bernoulli.pmf(1.0);
        assert_eq!(pmf_zero, 0.5);
        assert_eq!(pmf_one, 0.5);

        // Distribution function
        let cdf = dist.cdf(1.0);
        assert_approx_equal!(cdf, 1.0, 1e-10);
        // Test cdf for k = -1.0, 0.0, 0.5, 1.0 and 2.0
        let cdf_neg = bernoulli.cdf(-1.0);
        let cdf_zero = bernoulli.cdf(0.0);
        let cdf_half = bernoulli.cdf(0.5);
        let cdf_one = bernoulli.cdf(1.0);
        let cdf_two = bernoulli.cdf(2.0);
        assert_eq!(cdf_neg, 0.0);
        assert_eq!(cdf_zero, 0.5);
        assert_eq!(cdf_half, 0.5);
        assert_eq!(cdf_one, 1.0);
        assert_eq!(cdf_two, 1.0);

        // Test moment generating function for t = 1.0
        let mgf = bernoulli.mgf(1.0);
        assert_eq!(mgf, 1.0 - 0.5 + 0.5 * 1_f64.exp());

        // Test characteristic function for t = 1.0
        let cf = bernoulli.cf(1.0);
        assert_eq!(
            cf,
            Complex::new(1.0 - 0.5 + 0.5 * 1_f64.cos(), 0.5 * 1_f64.sin())
        );
    }

    #[test]
    fn test_bernoulli_moments() {
        let bernoulli = Bernoulli::new(0.5);

        // Test mean and variance
        assert_eq!(bernoulli.mean(), 0.5);
        assert_eq!(bernoulli.variance(), 0.25);

        // Test skewness and kurtosis
        assert_eq!(bernoulli.skewness(), 0.0);
        assert_eq!(bernoulli.kurtosis(), -2.0);
    }

    #[test]
    fn test_bernoulli_entropy() {
        let bernoulli = Bernoulli::new(0.5);

        // Test entropy
        assert_eq!(
            bernoulli.entropy(),
            -(0.5f64.ln() * 0.5 + (1.0 - 0.5 as f64).ln() * (1.0 - 0.5))
        );
    }
}
