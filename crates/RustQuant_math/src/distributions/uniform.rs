// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::DistributionClass;
use crate::distributions::Distribution;
use num::Complex;
use RustQuant_error::RustQuantError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Uniform distribution: X ~ Uni(a, b)
pub struct Uniform {
    /// Lower bound.
    a: f64,
    /// Upper bound.
    b: f64,
    /// Continuous or discrete ?
    class: DistributionClass,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Uniform {
    /// New instance of a Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.mean(), 0.5, 1e-7);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `a` is greater than `b`.
    #[must_use]
    pub fn new(a: f64, b: f64, class: DistributionClass) -> Self {
        assert!(a <= b);

        match class {
            DistributionClass::Discrete => Self {
                a: a.round(),
                b: b.round(),
                class,
            },
            DistributionClass::Continuous => Self { a, b, class },
        }
    }
}

impl Distribution for Uniform {
    /// Characteristic function of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    /// let cf = dist.cf(1.0);
    ///
    /// assert_approx_equal!(cf.re, 0.8414710, 1e-7);
    /// assert_approx_equal!(cf.im, 0.4596977, 1e-7);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();

        match self.class {
            DistributionClass::Discrete => {
                ((i * t * self.a).exp() - (i * t * (self.b + 1.0)).exp())
                    / ((1.0 - (i * t).exp()) * (self.b - self.a + 1.0))
            }
            DistributionClass::Continuous => {
                ((i * t * self.b).exp() - (i * t * self.a).exp()) / (i * t * (self.b - self.a))
            }
        }
    }

    /// Probability density function of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.pdf(0.5), 1.0, 1e-7);
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        match self.class {
            DistributionClass::Discrete => {
                if x >= self.a && x <= self.b {
                    (self.b - self.a + 1.0).recip()
                } else {
                    0.0
                }
            }
            DistributionClass::Continuous => {
                if x >= self.a && x <= self.b {
                    (self.b - self.a).recip()
                } else {
                    0.0
                }
            }
        }
    }

    /// Probability mass function of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Discrete);
    ///
    /// assert_approx_equal!(dist.pmf(0.5), 0.5, 1e-7);
    /// ```
    fn pmf(&self, x: f64) -> f64 {
        match self.class {
            DistributionClass::Discrete => {
                if x >= self.a && x <= self.b {
                    (self.b - self.a + 1.0).recip()
                } else {
                    0.0
                }
            }
            DistributionClass::Continuous => {
                if x >= self.a && x <= self.b {
                    (self.b - self.a).recip()
                } else {
                    0.0
                }
            }
        }
    }

    /// Cumulative distribution function of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.cdf(0.5), 0.5, 1e-7);
    /// ```
    fn cdf(&self, x: f64) -> f64 {
        match self.class {
            DistributionClass::Discrete => {
                if x < self.a {
                    0.0
                } else if x >= self.a && x <= self.b {
                    (x.floor() - self.a + 1.0) / (self.b - self.a + 1.0)
                } else {
                    1.0
                }
            }
            DistributionClass::Continuous => {
                if x < self.a {
                    0.0
                } else if x >= self.a && x <= self.b {
                    (x - self.a) / (self.b - self.a)
                } else {
                    1.0
                }
            }
        }
    }

    /// Inverse distribution (quantile) function of the Uniform distribution.
    /// Note: Only implemented for the continuous distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.inv_cdf(0.5), 0.5, 1e-7);
    /// ```
    fn inv_cdf(&self, p: f64) -> f64 {
        assert!((0.0..=1.0).contains(&p));
        match self.class {
            DistributionClass::Discrete => todo!(),
            DistributionClass::Continuous => self.a + p * (self.b - self.a),
        }
    }

    /// Mean of the Uniform distribution.
    /// The mean of the Uniform distribution is equal to its median.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.mean(), 0.5, 1e-7);
    /// ```    
    fn mean(&self) -> f64 {
        0.5 * (self.a + self.b)
    }

    /// Median of the Uniform distribution.
    /// The mean of the Uniform distribution is equal to its median.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.median(), 0.5, 1e-7);
    /// ```
    fn median(&self) -> f64 {
        0.5 * (self.a + self.b)
    }

    /// Mode of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.mode(), 0.5, 1e-7);
    /// ```
    fn mode(&self) -> f64 {
        match self.class {
            DistributionClass::Discrete => todo!(),
            DistributionClass::Continuous => (self.a + self.b) * 0.5,
        }
    }

    /// Variance of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.variance(), 0.0833333, 1e-7);
    /// ```
    fn variance(&self) -> f64 {
        match self.class {
            DistributionClass::Discrete => (self.b - self.a + 1.0).powi(2) / 12.0,
            DistributionClass::Continuous => (self.b - self.a).powi(2) / 12.0,
        }
    }

    /// Skewness of the Uniform distribution.
    /// The skewness of the Uniform distribution is equal to 0.
    /// # Examples
    /// ```
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_eq!(dist.skewness(), 0.0);
    /// ```
    fn skewness(&self) -> f64 {
        0.0
    }

    /// Kurtosis of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.kurtosis(), -6.0/5.0, 1e-7);
    /// ```
    fn kurtosis(&self) -> f64 {
        let n = self.b - self.a + 1.0;

        match self.class {
            DistributionClass::Discrete => -(6. * (n * n + 1.)) / (5. * (n * n - 1.)),
            DistributionClass::Continuous => -6.0 / 5.0,
        }
    }

    /// Entropy of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.5, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.entropy(), -0.6931472, 1e-7);
    /// ```
    fn entropy(&self) -> f64 {
        match self.class {
            DistributionClass::Discrete => (self.b - self.a + 1.0).ln(),
            DistributionClass::Continuous => (self.b - self.a).ln(),
        }
    }

    /// Moment generating function of the Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    ///
    /// assert_approx_equal!(dist.mgf(1.0), 1.7182818, 1e-7);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        let n = self.b - self.a + 1.0;

        match self.class {
            DistributionClass::Discrete => {
                ((t * self.a).exp() - (t * (self.b + 1.0)).exp()) / (n * (1.0 - (t).exp()))
            }
            DistributionClass::Continuous => {
                ((t * self.b).exp() - (t * self.a).exp()) / (t * (self.b - self.a))
            }
        }
    }

    /// Generates a random sample from a Uniform distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::utils::assert_approx_equal;
    /// # use RustQuant::math::*;
    ///
    /// let dist = Uniform::new(0.0, 1.0, DistributionClass::Continuous);
    /// let sample = dist.sample(1000).unwrap();
    /// let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    ///
    /// assert_approx_equal!(mean, dist.mean(), 0.1);
    /// ```
    fn sample(&self, n: usize) -> Result<Vec<f64>, RustQuantError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Uniform};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = Uniform::new(self.a, self.b);

        let mut variates: Vec<f64> = Vec::with_capacity(n);
        match self.class {
            DistributionClass::Discrete => {
                for _ in 0..variates.capacity() {
                    variates.push(dist.sample(&mut rng) as usize as f64);
                }
            }
            DistributionClass::Continuous => {
                for _ in 0..variates.capacity() {
                    variates.push(dist.sample(&mut rng));
                }
            }
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
    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON as EPS};

    #[test]
    fn test_uniform_distribution_continuous() {
        let dist: Uniform = Uniform::new(0.0, 1.0, DistributionClass::Continuous);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.841_470_984_807_896_5, EPS);
        assert_approx_equal!(cf.im, 0.459_697_694_131_860_23, EPS);

        // Probability mass function
        let pmf = dist.pmf(0.5);
        assert_approx_equal!(pmf, 1.0, EPS);

        // Distribution function
        let cdf = dist.cdf(0.5);
        assert_approx_equal!(cdf, 0.5, EPS);
    }

    #[test]
    fn test_uniform_distribution_discrete() {
        let dist: Uniform = Uniform::new(0.0, 1.0, DistributionClass::Discrete);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.770_151_152_934_069_9, EPS);
        assert_approx_equal!(cf.im, 0.420_735_492_403_948_36, EPS);

        // Probability mass function
        let pmf = dist.pmf(0.5);
        assert_approx_equal!(pmf, 0.5, EPS);

        // Distribution function
        let cdf = dist.cdf(0.5);
        assert_approx_equal!(cdf, 0.5, EPS);
    }
}
