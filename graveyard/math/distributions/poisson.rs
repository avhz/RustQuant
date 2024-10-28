// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{error::RustQuantError, math::distributions::Distribution};
use num::Complex;
use statrs::function::gamma::{gamma_li, gamma_ui};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Poisson distribution: X ~ Pois(lambda)
pub struct Poisson {
    /// Rate parameter.
    lambda: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Poisson {
    /// New instance of a Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_eq!(poisson.mean(), 1.0);
    /// assert_approx_equal!(poisson.cf(1.0).re, 0.4207936, 1e-7);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if lambda is not positive.
    #[must_use]
    pub fn new(lambda: f64) -> Poisson {
        assert!(lambda > 0.0);

        Poisson { lambda }
    }
}

impl Distribution for Poisson {
    /// Characteristic function of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_approx_equal!(poisson.cf(1.0).re, 0.4207936, 1e-7);
    /// assert_approx_equal!(poisson.cf(1.0).im, 0.4708426, 1e-7);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        (self.lambda * ((i * t).exp() - 1.0)).exp()
    }

    /// Probability density function of the Poisson distribution.
    /// Using this method will call `self.pmf()` instead.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_approx_equal!(poisson.pdf(1.0), 0.3678794, 1e-7);
    ///
    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
    }

    /// Probability mass function of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_approx_equal!(poisson.pmf(1.0), poisson.pdf(1.0), 1e-7);
    /// ```
    fn pmf(&self, x: f64) -> f64 {
        (self.lambda).powi(x as i32) * (-(self.lambda)).exp()
            / ((1..=x as usize).product::<usize>() as f64)
    }

    /// Cumulative distribution function of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_approx_equal!(poisson.cdf(1.0), 0.6408591, 1e-7);
    /// assert_approx_equal!(poisson.cdf(2.0), 0.9126873, 1e-7);
    /// ```
    fn cdf(&self, x: f64) -> f64 {
        1.0 - gamma_li(x + 1., self.lambda) / gamma_ui(x + 1., self.lambda)
    }

    /// Inverse cumulative distribution function of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    /// # use std::f64::INFINITY;
    /// # use std::f64::NAN;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_eq!(poisson.inv_cdf(1.0), INFINITY);
    /// assert_approx_equal!(poisson.inv_cdf(0.5), 1.0, 1e-7);
    /// assert_approx_equal!(poisson.inv_cdf(0.9), 2.0, 1e-7);
    /// ```
    fn inv_cdf(&self, p: f64) -> f64 {
        if !(0.0..=1.0).contains(&p) {
            return f64::NAN;
        }
        if (p - 1.0).abs() < f64::EPSILON {
            return f64::INFINITY;
        }
        let mut sum = 0.0;
        let mut k = 0;
        while sum < p {
            sum += self.pmf(f64::from(k));
            k += 1;
        }
        f64::from(k - 1)
    }

    /// Returns the mean of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_eq!(poisson.mean(), 1.0);
    /// ```
    fn mean(&self) -> f64 {
        self.lambda
    }

    /// Returns the median of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_eq!(poisson.median(), 1.0);
    /// ```
    fn median(&self) -> f64 {
        (self.lambda + 1.0 / 3.0 - 0.02 / self.lambda).floor()
    }

    /// Returns the mode of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_eq!(poisson.mode(), 1.0);
    /// ```
    fn mode(&self) -> f64 {
        self.lambda.floor()
    }

    /// Returns the variance of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_eq!(poisson.variance(), 1.0);
    /// ```
    fn variance(&self) -> f64 {
        self.lambda
    }

    /// Returns the skewness of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(2.0);
    ///
    /// assert_approx_equal!(poisson.skewness(), 0.7071068, 1e-7);
    fn skewness(&self) -> f64 {
        self.lambda.sqrt().recip()
    }

    /// Returns the kurtosis of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(2.0);
    ///
    /// assert_approx_equal!(poisson.kurtosis(), 0.5, 1e-7);
    /// ```
    fn kurtosis(&self) -> f64 {
        self.lambda.recip()
    }

    fn entropy(&self) -> f64 {
        todo!()
    }

    /// Returns the moment generating function of the Poisson distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// assert_approx_equal!(poisson.mgf(1.0), 5.5749415, 1e-7);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        (self.lambda * (t.exp() - 1.0)).exp()
    }

    /// Generates a random sample from a Poisson distribution using the
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let poisson = Poisson::new(1.0);
    ///
    /// let sample = poisson.sample(1000).expect("Poisson sampled");
    /// let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    ///
    /// assert_approx_equal!(mean, poisson.mean(), 0.1);
    /// ```
    fn sample(&self, n: usize) -> Result<Vec<f64>, RustQuantError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Poisson};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = Poisson::new(self.lambda)?;

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

    use std::f64::EPSILON as EPS;

    #[allow(clippy::similar_names)]
    #[test]
    fn test_poisson_distribution() -> Result<(), RustQuantError> {
        let dist: Poisson = Poisson::new(1.0);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.420_793_617_430_045_7, EPS);
        assert_approx_equal!(cf.im, 0.470_842_643_309_935_9, EPS);

        // Probability mass function
        let pmf = dist.pmf(1.);
        assert_approx_equal!(pmf, 0.367_879_441_171_442_33, EPS);
        // Probability density function is same as pmf
        assert_approx_equal!(dist.pdf(1.), pmf, EPS);

        // Distribution function
        let cdf = dist.cdf(1.);
        assert_approx_equal!(cdf, 0.640_859_085_770_477_5, EPS);

        // Inverse distribution function
        let icdf = dist.inv_cdf(0.5);
        assert_approx_equal!(icdf, 1.0, EPS);
        // p needs to be in [0, 1]
        assert!(dist.inv_cdf(1.1).is_nan());
        assert!(dist.inv_cdf(-0.1).is_nan());
        // p =1 => x = inf
        assert!(dist.inv_cdf(1.0).is_infinite() && dist.inv_cdf(1.0).is_sign_positive());

        // Mean
        assert_approx_equal!(dist.mean(), 1.0, EPS);

        // Median
        assert_approx_equal!(dist.median(), 1.0, EPS);

        // Mode
        assert_approx_equal!(dist.mode(), 1.0, EPS);

        // Variance
        assert_approx_equal!(dist.variance(), 1.0, EPS);

        // Skewness
        assert_approx_equal!(dist.skewness(), 1.0, EPS);

        // Kurtosis
        assert_approx_equal!(dist.kurtosis(), 1.0, EPS);

        // Moment generating function
        let mgf = dist.mgf(1.0);
        assert_approx_equal!(mgf, 5.574_941_5, 1e-7);

        // Sample
        let sample = dist.sample(1000)?;
        let mean = sample.iter().sum::<f64>() / sample.len() as f64;
        assert_approx_equal!(mean, dist.mean(), 0.1);

        Ok(())
    }
}
