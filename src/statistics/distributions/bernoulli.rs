// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::statistics::DistributionError;

use super::Distribution;
use num_complex::Complex;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Bernoulli distribution: X ~ Bern(p)
pub struct Bernoulli {
    /// Probability of k = 1 (q = 1 - p: probability of k = 0).
    p: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Default for Bernoulli {
    fn default() -> Self {
        Self::new(0.5)
    }
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

    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
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
        todo!()
    }

    fn mean(&self) -> f64 {
        self.p
    }

    fn median(&self) -> f64 {
        todo!()
    }

    fn mode(&self) -> f64 {
        todo!()
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

    fn sample(&self, n: usize) -> Result<Vec<f64>, DistributionError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Bernoulli, Distribution};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = Bernoulli::new(self.p)?;

        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(dist.sample(&mut rng) as usize as f64);
        }

        Ok(variates)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_bernoulli {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_bernoulli_functions() {
        let dist = Bernoulli::new(1.0);

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
            -(0.5f64.ln() * 0.5 + (1.0 - 0.5_f64).ln() * (1.0 - 0.5))
        );
    }

    #[test]
    fn test_default() {
        let bernoulli = Bernoulli::default();
        assert_eq!(bernoulli.p, 0.5);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_probability_low() {
        Bernoulli::new(-0.5);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_probability_high() {
        Bernoulli::new(1.5);
    }

    #[test]
    #[should_panic]
    fn test_pmf_invalid_input() {
        let bernoulli = Bernoulli::new(0.5);
        bernoulli.pmf(2.0);
    }

    #[test]
    fn test_cdf_negative_input() {
        let bernoulli = Bernoulli::new(0.5);
        let cdf_neg = bernoulli.cdf(-1.0);
        assert_eq!(cdf_neg, 0.0);
    }

    #[test]
    fn test_cdf_positive_input() {
        let bernoulli = Bernoulli::new(0.5);
        let cdf_one = bernoulli.cdf(1.0);
        let cdf_two = bernoulli.cdf(2.0);
        assert_eq!(cdf_one, 1.0);
        assert_eq!(cdf_two, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_inv_cdf_not_implemented() {
        let bernoulli = Bernoulli::new(0.5);
        bernoulli.inv_cdf(0.5);
    }

    #[test]
    #[should_panic]
    fn test_median_not_implemented() {
        let bernoulli = Bernoulli::new(0.5);
        bernoulli.median();
    }

    #[test]
    #[should_panic]
    fn test_mode_not_implemented() {
        let bernoulli = Bernoulli::new(0.5);
        bernoulli.mode();
    }

    #[test]
    #[should_panic]
    fn test_sample_zero_size() {
        let bernoulli = Bernoulli::new(0.5);
        _ = bernoulli.sample(0);
    }

    #[test]
    fn test_sample_positive_size() -> Result<(), DistributionError> {
        let bernoulli = Bernoulli::new(0.5);
        let sample = bernoulli.sample(100)?;
        assert_eq!(sample.len(), 100);
        for &value in sample.iter() {
            assert!(value == 0.0 || value == 1.0);
        }

        Ok(())
    }
}
