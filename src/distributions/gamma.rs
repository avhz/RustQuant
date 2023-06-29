// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::Distribution;
use num_complex::Complex;
use statrs::function::gamma::{gamma, gamma_li};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Gamma distribution
///
/// There are two common parametrizations for the Gamma distribution.
///
/// 1. X ~ Gamma(alpha, beta) = Gamma(shape, rate)
/// 2. X ~ Gamma(k, theta) = Gamma(shape, scale)
///
/// This implementation uses the first parametrization (shape, rate).
///
/// Note that scale = 1 / rate <=> rate = 1 / scale.
pub struct Gamma {
    /// Alpha: the shape parameter.
    alpha: f64,
    /// Beta: the rate parameter (inverse scale).
    beta: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Gamma {
    /// New instance of a Gamma distribution.
    pub fn new(alpha: f64, beta: f64) -> Self {
        assert!(alpha > 0.0 && beta > 0.0);

        Self { alpha, beta }
    }
}

impl Distribution for Gamma {
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        let alpha = self.alpha;
        let beta = self.beta;

        (1.0 - i * t / beta).powf(-alpha)
    }

    fn pdf(&self, x: f64) -> f64 {
        assert!(x > 0.0);

        let alpha = self.alpha;
        let beta = self.beta;

        beta.powf(alpha) * x.powf(alpha - 1.0) * (-beta * x).exp() / gamma(alpha)
    }

    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }

    fn cdf(&self, x: f64) -> f64 {
        assert!(x > 0.0);

        let alpha = self.alpha;
        let beta = self.beta;

        gamma_li(alpha, beta * x) / gamma(alpha)
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        unimplemented!()
    }

    fn mean(&self) -> f64 {
        self.alpha / self.beta
    }

    fn median(&self) -> f64 {
        unimplemented!()
    }

    fn mode(&self) -> f64 {
        if self.alpha >= 1.0 {
            (self.alpha - 1.0) / self.beta
        } else {
            0.0
        }
    }

    fn variance(&self) -> f64 {
        self.alpha / self.beta.powi(2)
    }

    fn skewness(&self) -> f64 {
        2. / self.alpha.sqrt()
    }

    fn kurtosis(&self) -> f64 {
        6. / self.alpha
    }

    fn entropy(&self) -> f64 {
        todo!()
    }

    fn mgf(&self, t: f64) -> f64 {
        assert!(t < self.beta);

        (1.0 - t / self.beta).powf(-self.alpha)
    }

    fn sample(&self, n: usize) -> Vec<f64> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Gamma};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = Gamma::new(self.alpha, self.beta.recip()).unwrap();

        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(dist.sample(&mut rng) as usize as f64);
        }

        variates
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
