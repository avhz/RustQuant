// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::math::{distributions::Distribution, DistributionError};
use num::Complex;

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
    ///
    /// # Panics
    ///
    /// Panics if lambda is greater than 0.0
    #[must_use]
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0);

        Self { lambda }
    }
}

impl Distribution for Exponential {
    /// ```
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_eq!(exp.cf(0.0).re, 1.0);
    /// assert_eq!(exp.cf(0.0).im, 0.0);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        1.0 / (1.0 - i * t / self.lambda)
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.pdf(1.0), 0.3678794, 1e-7);
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        assert!(x >= 0.0);

        self.lambda * (-self.lambda * x).exp()
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.pdf(1.0), 0.3678794, 1e-7);
    /// ```
    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.cdf(1.0), 0.6321206, 1e-7);
    /// ```
    fn cdf(&self, x: f64) -> f64 {
        assert!(x >= 0.0);

        1.0 - (-self.lambda * x).exp()
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.inv_cdf(0.5), 0.6931472, 1e-7);
    /// ```
    fn inv_cdf(&self, p: f64) -> f64 {
        -(1. - p).ln() / self.lambda
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.mean(), 1.0, 1e-10);
    /// ```
    fn mean(&self) -> f64 {
        self.lambda.recip()
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.median(), 0.6931472, 1e-7);
    /// ```
    fn median(&self) -> f64 {
        2_f64.ln() / self.lambda
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.mode(), 0.0, 1e-10);
    /// ```
    fn mode(&self) -> f64 {
        0.0
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.variance(), 1.0, 1e-10);
    /// ```
    fn variance(&self) -> f64 {
        self.lambda.recip().powi(2)
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.skewness(), 2.0, 1e-10);
    /// ```
    fn skewness(&self) -> f64 {
        2.0
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.kurtosis(), 6.0, 1e-10);
    /// ```
    fn kurtosis(&self) -> f64 {
        6.0
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.entropy(), 1.0, 1e-10);
    /// ```
    fn entropy(&self) -> f64 {
        1.0 - self.lambda.ln()
    }

    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::math::distributions::*;
    ///
    /// let exp = Exponential::new(1.0);
    ///
    /// assert_approx_equal!(exp.mgf(0.5), 2.0, 1e-10);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        assert!(t < self.lambda);

        self.lambda * (self.lambda - t).recip()
    }

    fn sample(&self, n: usize) -> Result<Vec<f64>, DistributionError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Exp};

        assert!(n > 0);

        let mut rng = thread_rng();
        let dist = Exp::new(self.lambda)?;
        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(dist.sample(&mut rng));
        }

        Ok(variates)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    use std::f64::EPSILON as EPS;

    #[test]
    fn test_exponential_characteristic_function() {
        let dist: Exponential = Exponential::new(1.0);

        // // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.5, EPS);
        assert_approx_equal!(cf.im, 0.5, EPS);
    }

    #[test]
    fn test_exponential_density_function() {
        let dist: Exponential = Exponential::new(1.0);

        // Values computed using R
        assert_approx_equal!(dist.pdf(0.0), 1.000_000_00, EPS);
        assert_approx_equal!(dist.pdf(1.0), 0.367_879_441_171_442_33, EPS);
        assert_approx_equal!(dist.pdf(2.0), 0.135_335_283_236_612_7, EPS);
        assert_approx_equal!(dist.pdf(3.0), 0.049_787_068_367_863_944, EPS);
        assert_approx_equal!(dist.pdf(4.0), 0.018_315_638_888_734_18, EPS);
    }

    #[test]
    fn test_exponential_distribution_function() {
        let dist: Exponential = Exponential::new(1.0);

        // Values computed using R
        assert_approx_equal!(dist.cdf(0.0), 0.000_000_0, EPS);
        assert_approx_equal!(dist.cdf(1.0), 0.632_120_558_828_557_7, EPS);
        assert_approx_equal!(dist.cdf(2.0), 0.864_664_716_763_387_3, EPS);
        assert_approx_equal!(dist.cdf(3.0), 0.950_212_931_632_136, EPS);
        assert_approx_equal!(dist.cdf(4.0), 0.981_684_361_111_265_8, EPS);
    }
}
