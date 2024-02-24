// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::math::{distributions::Distribution, DistributionError};
use num_complex::Complex;
use std::f64::consts::{E, PI};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Binomial distribution: X ~ Bin(n, p)
pub struct Binomial {
    /// Number of trials.
    n: usize,
    /// Probability of k = 1 (q = 1 - p: probability of k = 0).
    p: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Default for Binomial {
    // Default is a Binomial distribution with n = 1 and p = 0.5.
    fn default() -> Self {
        Self { n: 1, p: 0.5 }
    }
}

impl Binomial {
    /// New instance of a Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_eq!(binomial.mean(), 2.0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if probability is not in $[0, 1]$.
    #[must_use]
    pub fn new(trials: usize, probability: f64) -> Self {
        assert!((0.0..=1.0).contains(&probability));

        Self {
            n: trials,
            p: probability,
        }
    }
}

impl Distribution for Binomial {
    /// Characteristic function of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    /// let cf = binomial.cf(1.0);
    ///
    /// assert_approx_equal!(cf.re, -0.2014034, 1e-7);
    /// assert_approx_equal!(cf.im, 0.4969347, 1e-7);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0.0..=1.0).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        (1.0 - self.p + self.p * (i * t).exp()).powi(self.n as i32)
    }

    /// Probability density function of the Binomial distribution.
    /// Note: Identical to the probability mass function.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    /// let pdf = binomial.pdf(3.0);
    /// let pmf = binomial.pmf(3.0);
    ///
    /// assert_approx_equal!(pdf, 0.2304000, 1e-7);
    /// assert_approx_equal!(pdf, pmf, 1e-7);
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
    }

    /// Probability mass function of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    /// let pmf = binomial.pmf(3.0);
    ///
    /// assert_approx_equal!(pmf, 0.2304000, 1e-7);
    /// ```
    fn pmf(&self, k: f64) -> f64 {
        assert!(k as usize <= self.n);
        assert!((0.0..=1.0).contains(&self.p));

        let n_C_k = |n: u32, k: u32| -> u32 {
            (1..=n).product::<u32>() / ((1..=k).product::<u32>() * (1..=(n - k)).product::<u32>())
        };

        n_C_k(self.n as u32, k as u32) as f64
            * self.p.powi(k as i32)
            * (1.0 - self.p).powi((self.n - k as usize) as i32)
    }

    /// Cumulative distribution function of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_approx_equal!(binomial.cdf(3.0), 0.9129600, 1e-7);
    /// ```
    fn cdf(&self, k: f64) -> f64 {
        statrs::function::beta::beta_reg((self.n - k as usize) as f64, 1_f64 + k, 1_f64 - self.p)
    }

    /// Inverse distribution (quantile) function of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_eq!(binomial.inv_cdf(0.5), 2.0);    
    fn inv_cdf(&self, p: f64) -> f64 {
        assert!((0.0..=1.0).contains(&p));

        let mut k = 0.0;
        let mut cdf = self.cdf(k);

        while cdf < p {
            k += 1.0;
            cdf = self.cdf(k);
        }
        k
    }

    /// Mean of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_eq!(binomial.mean(), 2.0);
    fn mean(&self) -> f64 {
        self.n as f64 * self.p
    }

    /// Median of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_eq!(binomial.median(), 2.0);
    ///
    fn median(&self) -> f64 {
        self.mean().floor()
    }

    /// Mode of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_eq!(binomial.mode(), 2.0);
    /// ```
    fn mode(&self) -> f64 {
        ((self.n as f64 + 1.) * self.p).floor()
    }

    /// Variance of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_eq!(binomial.variance(), 1.2);
    /// ```
    fn variance(&self) -> f64 {
        self.n as f64 * self.p * (1. - self.p)
    }

    /// Skewness of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_approx_equal!(binomial.skewness(), 0.1825742, 1e-7);
    /// ```
    fn skewness(&self) -> f64 {
        (1. - 2. * self.p) / self.variance().sqrt()
    }

    /// Kurtosis of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_approx_equal!(binomial.kurtosis(), -0.3666667, 1e-7);
    /// ```
    fn kurtosis(&self) -> f64 {
        (1. - 6. * self.p * (1. - self.p)) / self.variance()
    }

    /// Entropy of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_approx_equal!(binomial.entropy(), 1.510099, 1e-6);
    /// ```
    fn entropy(&self) -> f64 {
        0.5 * (2. * PI * E * self.variance()).ln()
    }

    /// Moment generating function of the Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(5, 0.4);
    ///
    /// assert_approx_equal!(binomial.mgf(1.0), 13.67659, 1e-5);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        ((1. - self.p) + self.p * t.exp()).powi(self.n as i32)
    }

    /// Generates a random sample from a Binomial distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::{Binomial, Distribution};
    ///
    /// let binomial = Binomial::new(100, 0.4);
    /// let sample = binomial.sample(100).unwrap();
    /// let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    ///
    /// assert_approx_equal!(mean, binomial.mean(), 1.0);
    /// ```
    fn sample(&self, n: usize) -> Result<Vec<f64>, DistributionError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Binomial, Distribution};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = Binomial::new(n as u64, self.p)?;

        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(dist.sample(&mut rng) as f64);
        }

        Ok(variates)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_binomial_distribution {
    use super::*;
    use crate::assert_approx_equal;

    use std::f64::EPSILON as EPS;

    #[test]
    fn test_binomial_distribution() {
        // n = 2 trials, p = 0.5 probability
        let dist: Binomial = Binomial::new(2, 0.5);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.416_114_443_797_284_35, EPS);
        assert_approx_equal!(cf.im, 0.648_059_849_110_368_7, EPS);

        // Probability mass function
        // k = 1 successes.
        let pmf = dist.pmf(1.);
        assert_approx_equal!(pmf, 0.5, EPS);

        // Distribution function
        // k = 1 successes.
        let cdf = dist.cdf(1.);
        assert_approx_equal!(cdf, 0.749_999_999_999_999_1, EPS);
    }

    #[test]
    fn test_binomial_functions() {
        let binomial = Binomial::new(5, 0.4);

        // Characteristic function
        let cf = binomial.cf(1.0);
        assert_approx_equal!(cf.re, -0.201_403_389_549_595_36, EPS);
        assert_approx_equal!(cf.im, 0.496_934_703_617_956_4, EPS);

        // Probability mass function
        let pmf = binomial.pmf(3.0);
        assert_approx_equal!(pmf, 0.2304, EPS);

        // Distribution function
        let cdf = binomial.cdf(3.0);
        assert_approx_equal!(cdf, 0.91296, EPS);

        assert_approx_equal!(binomial.mean(), 2.0, EPS);
        assert_approx_equal!(binomial.median(), 2.0, EPS);
        assert_approx_equal!(binomial.mode(), 2.0, EPS);
        assert_approx_equal!(binomial.variance(), 1.2, 1e-6);
        assert_approx_equal!(binomial.skewness(), 0.182_574_185_835_055_33, EPS);
        assert_approx_equal!(binomial.kurtosis(), -0.366_666_666_666_666_8, EPS);
        assert_approx_equal!(binomial.entropy(), 1.510_099_311_601_65, EPS);
        assert_approx_equal!(binomial.mgf(1.0), 13.676_592_816_585_314, EPS);
    }
}
