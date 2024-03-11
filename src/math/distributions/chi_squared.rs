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
use statrs::function::gamma::{digamma, gamma, gamma_li};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Chi-Squared distribution: X ~ ChiSq(k)
pub struct ChiSquared {
    /// k: degrees of freedom.
    k: usize,
}

impl Default for ChiSquared {
    // Degrees of freedom k = 1 is equivalent to the exponential distribution
    fn default() -> Self {
        Self { k: 1 }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ChiSquared {
    /// New instance of a Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let dist = ChiSquared::new(1);
    ///
    /// assert_eq!(dist.mean(), 1.0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `k` is not positive.
    #[must_use]
    pub fn new(k: usize) -> Self {
        assert!(k > 0);

        Self { k }
    }
}

impl Distribution for ChiSquared {
    /// Characteristic function of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use num::Complex;
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    /// let cf = chi.cf(1.0);
    ///
    /// assert_approx_equal!(cf.re, 0.5688645, 1e-7);
    /// assert_approx_equal!(cf.im, 0.3515776, 1e-7);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        let k = self.k;

        (1.0 - 2.0 * i * t).powf(-(k as f64 / 2.0))
    }

    /// Probability density function of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.pdf(1.0), 0.2419707, 1e-7);
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        assert!(if self.k == 1 { x > 0.0 } else { x >= 0.0 });

        let k = self.k;

        x.powf((k as f64 / 2.0) - 1.0) * (-x / 2.0).exp()
            / (2_f64.powf(k as f64 / 2.0) * gamma(k as f64 / 2.0))
    }

    /// Probability mass function of the Chi-Squared distribution.
    /// Using this method will call `self.pdf(x)` instead
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.pmf(1.0), 0.2419707, 1e-7);
    /// assert_approx_equal!(chi.pmf(1.0), chi.pdf(1.0), 1e-7);
    /// ```
    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }

    /// Cumulative distribution function of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.cdf(1.0), 0.6826895, 1e-7);
    ///
    fn cdf(&self, x: f64) -> f64 {
        assert!(if self.k == 1 { x > 0.0 } else { x >= 0.0 });

        let k = self.k;

        gamma_li(k as f64 / 2.0, x / 2.0) / gamma(k as f64 / 2.0)
    }

    /// Inverse (quantile) distribution function of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.inv_cdf(0.6826895), 1.0, 1e-7);
    /// ```
    fn inv_cdf(&self, p: f64) -> f64 {
        assert!((0.0..=1.0).contains(&p));

        let k = self.k as f64;
        let mut x = 0.5 * k;
        let mut delta = 0.5 * k;

        while delta > 1e-10 {
            let cdf = self.cdf(x);
            if cdf < p {
                x += delta;
            } else {
                x -= delta;
            }
            delta *= 0.5;
        }
        x
    }

    /// Mean of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_eq!(chi.mean(), 1.0);
    /// ```
    fn mean(&self) -> f64 {
        self.k as f64
    }

    /// Median of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.median(), 0.4705075, 1e-7);
    /// ```
    fn median(&self) -> f64 {
        self.k as f64 * (1.0 - (2.0 / (9.0 * self.k as f64))).powf(3.0)
    }

    /// Mode of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_eq!(chi.mode(), 0.0);
    /// ```
    fn mode(&self) -> f64 {
        0_f64.max(self.k as f64 - 2.0)
    }

    /// Variance of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_eq!(chi.variance(), 2.0);
    /// ```
    fn variance(&self) -> f64 {
        2.0 * self.k as f64
    }

    /// Skewness of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.skewness(), 2.8284271, 1e-7);
    /// ```
    fn skewness(&self) -> f64 {
        (8.0 / self.k as f64).sqrt()
    }

    /// Kurtosis of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_eq!(chi.kurtosis(), 12.0);
    fn kurtosis(&self) -> f64 {
        12.0 / self.k as f64
    }

    /// Entropy of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.entropy(), 0.7837571, 1e-7);
    /// ```
    fn entropy(&self) -> f64 {
        let k = self.k as f64;

        k / 2.0 + (2.0 * gamma(k / 2.0)).ln() + (1.0 - k / 2.0) * digamma(k / 2.0)
    }

    /// Moment-generating function of the Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(1);
    ///
    /// assert_approx_equal!(chi.mgf(0.25), 1.4142135, 1e-7);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        assert!(t < 0.5);

        (1.0 - 2.0 * t).powf(-(self.k as f64) / 2.0)
    }

    /// Generates a random sample from a Chi-Squared distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let chi = ChiSquared::new(100);
    ///
    /// let sample = chi.sample(100).expect("Error generating Chi-Squared sample.");
    /// let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    ///
    /// assert_approx_equal!(mean, chi.mean(), 5.0);
    /// ```
    fn sample(&self, n: usize) -> Result<Vec<f64>, DistributionError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{ChiSquared, Distribution};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = ChiSquared::new(self.k as f64)?;

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
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_chi_squared_characteristic_function() {
        let dist: ChiSquared = ChiSquared::new(1);

        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.568_864_5, 1e-7);
        assert_approx_equal!(cf.im, 0.351_577_6, 1e-7);
    }

    #[test]
    fn test_chi_squared_density_function() {
        let dist: ChiSquared = ChiSquared::new(1);

        // Values computed using R
        assert_approx_equal!(dist.pdf(1.0), 0.241_970_72, 1e-8);
        assert_approx_equal!(dist.pdf(2.0), 0.103_776_87, 1e-8);
        assert_approx_equal!(dist.pdf(3.0), 0.051_393_44, 1e-8);
        assert_approx_equal!(dist.pdf(4.0), 0.026_995_48, 1e-8);
        assert_approx_equal!(dist.pdf(5.0), 0.014_644_98, 1e-8);
    }

    #[test]
    fn test_chi_squared_distribution_function() {
        let dist: ChiSquared = ChiSquared::new(1);

        // Values computed using R
        assert_approx_equal!(dist.cdf(1.0), 0.682_689_5, 1e-7);
        assert_approx_equal!(dist.cdf(2.0), 0.842_700_8, 1e-7);
        assert_approx_equal!(dist.cdf(3.0), 0.916_735_5, 1e-7);
        assert_approx_equal!(dist.cdf(4.0), 0.954_499_7, 1e-7);
        assert_approx_equal!(dist.cdf(5.0), 0.974_652_7, 1e-7);
    }
}
