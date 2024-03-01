// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::math::DistributionError;
use errorfunctions::RealErrorFunctions;
use {
    super::Distribution,
    num::Complex,
    statrs::function::erf,
    std::f64::consts::{PI, SQRT_2},
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Gaussian (normal) distribution: X ~ N(mu, sigma^2)
/// <https://en.wikipedia.org/wiki/Normal_distribution>
pub struct Gaussian {
    /// Mean (location).
    mean: f64,
    /// Variance (squared scale).
    variance: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Default for Gaussian {
    fn default() -> Self {
        Self {
            mean: 0.0,
            variance: 1.0,
        }
    }
}

impl Gaussian {
    /// New instance of a Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.cf(5.0).re, 3.7266532e-6, 1e-7);
    /// assert_eq!(gaussian.cf(5.0).im, 0.0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if variance is not positive.
    #[must_use]
    pub fn new(mean: f64, variance: f64) -> Self {
        assert!(variance > 0.0);

        Self { mean, variance }
    }
}

impl Distribution for Gaussian {
    /// Characteristic function of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.cf(5.0).re, 3.7266532e-6, 1e-7);
    /// assert_eq!(gaussian.cf(5.0).im, 0.0);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!(self.variance > 0.0);

        let i: Complex<f64> = Complex::i();

        (i * self.mean * t - 0.5 * self.variance * (t).powi(2)).exp()
    }

    /// Probability density function of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.pdf(1.0), 0.2419707, 1e-7);
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        assert!(self.variance > 0.0);

        (-0.5 * ((x - self.mean).powi(2) / self.variance).exp()) / (2.0 * PI * self.variance).sqrt()
    }

    /// Probability mass function for the Gaussian distribution (continuous) is not defined.
    /// Using this method will call `self.pdf()` instead.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.pmf(1.0), gaussian.pdf(1.0), 1e-7);
    /// ```
    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }

    /// Cumulative distribution function of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.cdf(1.0), 0.8413447, 1e-7);
    /// assert_approx_equal!(gaussian.cdf(-1.0), 0.1586553, 1e-7);
    /// ```
    fn cdf(&self, x: f64) -> f64 {
        assert!(self.variance > 0.0);
        // `erfc` (complementary error function) is used instead of `erf` to avoid
        // subtractive cancellation that leads to inaccuracy in the tails.
        0.5 * (-(x - self.mean) / (SQRT_2 * self.variance.sqrt())).erfc()
    }

    /// Inverse distribution (quantile) function of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    /// # use std::f64::INFINITY;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.inv_cdf(0.5), 0.0);
    /// assert_approx_equal!(gaussian.inv_cdf(0.001), -3.090232306167813, f64::EPSILON);
    /// assert_approx_equal!(gaussian.inv_cdf(0.997), 2.747781385444993, f64::EPSILON);
    /// ```
    fn inv_cdf(&self, p: f64) -> f64 {
        assert!(self.variance > 0.0);

        self.mean + SQRT_2 * self.variance.sqrt() * erf::erf_inv(2.0 * p - 1.0)
    }

    /// Returns the mean of the Gaussian distribution.
    /// The mean of the Gaussian distribution is equal to its median.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.mean(), 0.0);
    /// ```
    fn mean(&self) -> f64 {
        self.mean
    }

    /// Returns the median of the Gaussian distribution.
    /// The median of the Gaussian distribution is equal to its mean.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.median(), gaussian.mean());
    /// ```
    fn median(&self) -> f64 {
        self.mean
    }

    /// Returns the mode of the Gaussian distribution.
    /// The mode of the Gaussian distribution is equal to its mean.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.mode(), gaussian.mean());
    /// ```
    fn mode(&self) -> f64 {
        self.mean
    }

    /// Returns the variance of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.variance(), 1.0);
    /// ```
    fn variance(&self) -> f64 {
        self.variance
    }

    /// Returns the skewness of the Gaussian distribution.
    /// The skewness of the Gaussian distribution is equal to 0.
    /// # Examples
    /// ```
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.skewness(), 0.0);
    /// ```
    fn skewness(&self) -> f64 {
        0.0
    }

    /// Returns the kurtosis of the Gaussian distribution.
    /// The kurtosis of the Gaussian distribution is equal to 0.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_eq!(gaussian.kurtosis(), 0.0);
    /// ```
    fn kurtosis(&self) -> f64 {
        0.0
    }

    /// Returns the entropy of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.entropy(), 1.4189385, 1e-7);
    /// ```
    fn entropy(&self) -> f64 {
        0.5 * (1.0 + (2.0 * PI * self.variance).ln())
    }

    /// Returns the moment generating function of the Gaussian distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// assert_approx_equal!(gaussian.mgf(1.0), 1.6487213, 1e-7);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        assert!(self.variance > 0.0);
        // M(t) = E(e^tX)
        (self.mean * t + self.variance * t * t / 2.0).exp()
    }

    /// Generates a random sample from the Gaussian distribution using the
    /// Standard Normal Random Variates Generator.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::statistics::distributions::*;
    ///
    /// let gaussian = Gaussian::new(0.0, 1.0);
    ///
    /// let sample = gaussian.sample(1000).expect("Gaussian sampled");
    /// let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    ///
    /// assert_approx_equal!(mean, gaussian.mean(), 0.1);
    /// ```
    ///
    fn sample(&self, n: usize) -> Result<Vec<f64>, DistributionError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Normal};

        assert!(n > 0);

        let mut rng = thread_rng();
        let normal = Normal::new(self.mean, self.variance.sqrt())?;
        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(normal.sample(&mut rng));
        }

        Ok(variates)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_gaussian {
    use super::*;
    use crate::assert_approx_equal;

    use std::f64::EPSILON as EPS;

    #[test]
    fn test_gaussian_characteristic_function() {
        // Standard normal distribution
        // mean = 0, variance = 1
        let dist: Gaussian = Gaussian::default();

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 1.0 / (1_f64.exp()).sqrt(), EPS);
        assert_approx_equal!(cf.im, 0.0, EPS);
    }

    #[test]
    fn test_gaussian_density_function() {
        // Standard normal distribution
        // mean = 0, variance = 1
        let dist: Gaussian = Gaussian::default();

        // Values from WolframAlpha
        assert_approx_equal!(dist.pdf(-4.0), 0.000_133_830_225_764_885_37, EPS);
        assert_approx_equal!(dist.pdf(-3.0), 0.004_431_848_411_938_007_5, EPS);
        assert_approx_equal!(dist.pdf(-2.0), 0.053_990_966_513_188_06, EPS);
        assert_approx_equal!(dist.pdf(-1.0), 0.241_970_724_519_143_37, EPS);
        assert_approx_equal!(dist.pdf(0.0), 0.398_942_280_401_432_7, EPS);
        assert_approx_equal!(dist.pdf(1.0), 0.241_970_724_519_143_37, EPS);
        assert_approx_equal!(dist.pdf(2.0), 0.053_990_966_513_188_06, EPS);
        assert_approx_equal!(dist.pdf(3.0), 0.004_431_848_411_938_007_5, EPS);
        assert_approx_equal!(dist.pdf(4.0), 0.000_133_830_225_764_885_37, EPS);
    }

    #[test]
    fn test_gaussian_distribution_function() {
        // Standard normal distribution
        // mean = 0, variance = 1
        let dist: Gaussian = Gaussian::default();

        // Values from WolframAlpha
        assert_approx_equal!(dist.cdf(-5.0), 2.866_515_718_791_939e-7, EPS);
        assert_approx_equal!(dist.cdf(-4.0), 0.000_031_671_241_833_119_924, EPS);
        assert_approx_equal!(dist.cdf(-3.0), 0.001_349_898_031_630_094_6, EPS);
        assert_approx_equal!(dist.cdf(-2.0), 0.022_750_131_948_179_21, EPS);
        assert_approx_equal!(dist.cdf(-1.0), 0.158_655_253_931_457_05, EPS);
        assert_approx_equal!(dist.cdf(0.0), 0.5, EPS);
        assert_approx_equal!(dist.cdf(1.0), 0.841_344_746_068_542_9, EPS);
        assert_approx_equal!(dist.cdf(2.0), 0.977_249_868_051_820_8, EPS);
        assert_approx_equal!(dist.cdf(3.0), 0.998_650_101_968_369_9, EPS);
        assert_approx_equal!(dist.cdf(4.0), 0.999_968_328_758_166_9, EPS);
        assert_approx_equal!(dist.cdf(5.0), 0.999_999_713_348_428_1, EPS);
    }

    #[test]
    fn test_gaussian_variate_generator() -> Result<(), DistributionError> {
        let normal = Gaussian::default();

        let v = normal.sample(1000)?;

        let mean = (v.iter().sum::<f64>()) / (v.len() as f64);

        assert_approx_equal!(mean, 0.0, 0.1);

        Ok(())
    }

    #[test]
    fn test_gaussian_moments() {
        let normal = Gaussian::default();

        assert_approx_equal!(normal.mean(), 0.0, EPS);
        assert_approx_equal!(normal.median(), 0.0, EPS);
        assert_approx_equal!(normal.mode(), 0.0, EPS);
        assert_approx_equal!(normal.variance(), 1.0, EPS);
        assert_approx_equal!(normal.skewness(), 0.0, EPS);
        assert_approx_equal!(normal.kurtosis(), 0.0, EPS);
        assert_approx_equal!(normal.entropy(), 1.418_938_533_204_672_7, EPS);
    }

    #[test]
    fn test_gaussian_mgf() {
        let normal = Gaussian::default();

        assert_approx_equal!(normal.mgf(0.0), 1.0, EPS);
        assert_approx_equal!(normal.mgf(1.0), 1.0_f64.exp().sqrt(), EPS);
        assert_approx_equal!(normal.mgf(2.0), 7.389_056_098_930_65, EPS);
    }

    #[test]
    fn test_gaussian_entropy() {
        let normal = Gaussian::default();

        assert_approx_equal!(normal.entropy(), 1.418_938_533_204_672_7, EPS);
    }
}
