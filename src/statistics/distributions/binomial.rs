// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::statistics::{distributions::Distribution, DistributionError};
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
// IMPLMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Default for Binomial {
    fn default() -> Self {
        Self::new(1, 0.5)
    }
}

impl Binomial {
    /// New instance of a Binomial distribution.
    pub fn new(trials: usize, probability: f64) -> Self {
        assert!((0.0..=1.0).contains(&probability));

        Self {
            n: trials,
            p: probability,
        }
    }
}

impl Distribution for Binomial {
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0.0..=1.0).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        (1.0 - self.p + self.p * (i * t).exp()).powi(self.n as i32)
    }

    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
    }

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

    fn cdf(&self, k: f64) -> f64 {
        statrs::function::beta::beta_reg((self.n - k as usize) as f64, 1. + k, 1. - self.p)
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        todo!()
    }

    fn mean(&self) -> f64 {
        self.n as f64 * self.p
    }

    fn median(&self) -> f64 {
        self.mean().floor()
    }

    fn mode(&self) -> f64 {
        ((self.n as f64 + 1.) * self.p).floor()
    }

    fn variance(&self) -> f64 {
        self.n as f64 * self.p * (1. - self.p)
    }

    fn skewness(&self) -> f64 {
        (1. - 2. * self.p) / self.variance().sqrt()
    }

    fn kurtosis(&self) -> f64 {
        (1. - 6. * self.p * (1. - self.p)) / self.variance()
    }

    fn entropy(&self) -> f64 {
        0.5 * (2. * PI * E * self.variance()).ln()
    }

    fn mgf(&self, t: f64) -> f64 {
        ((1. - self.p) + self.p * t.exp()).powi(self.n as i32)
    }

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

    #[test]
    fn test_binomial_distribution() {
        // n = 2 trials, p = 0.5 probability
        let dist: Binomial = Binomial::new(2, 0.5);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.41611444379, 1e-10);
        assert_approx_equal!(cf.im, 0.64805984911, 1e-10);

        // Probability mass function
        // k = 1 successes.
        let pmf = dist.pmf(1.);
        assert_approx_equal!(pmf, 0.5, 1e-10);

        // Distribution function
        // k = 1 successes.
        let cdf = dist.cdf(1.);
        assert_approx_equal!(cdf, 0.75, 1e-10);
    }

    #[test]
    fn test_binomial_functions() {
        let binomial = Binomial::new(5, 0.4);

        // Characteristic function
        let cf = binomial.cf(1.0);
        assert_approx_equal!(cf.re, -0.2014034, 1e-6);
        assert_approx_equal!(cf.im, 0.4969347, 1e-6);

        // Probability mass function
        let pmf = binomial.pmf(3.0);
        assert_approx_equal!(pmf, 0.2304, 1e-4);

        // Distribution function
        let cdf = binomial.cdf(3.0);
        assert_approx_equal!(cdf, 0.91296, 1e-5);

        assert_eq!(binomial.mean(), 2.0);
        assert_eq!(binomial.median(), 2.0);
        assert_eq!(binomial.mode(), 2.0);
        assert_approx_equal!(binomial.variance(), 1.2, 1e-6);
        assert_approx_equal!(binomial.skewness(), 0.182574, 1e-6);
        assert_approx_equal!(binomial.kurtosis(), -0.366667, 1e-6);
        assert_approx_equal!(binomial.entropy(), 1.510099, 1e-6);
        assert_approx_equal!(binomial.mgf(1.0), 13.67659, 1e-5);
    }
}
