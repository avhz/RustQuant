// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::statistics::distributions::Distribution;
use num_complex::Complex;
use statrs::function::gamma::{gamma, gamma_li};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Chi-Squared distribution: X ~ ChiSq(k)
pub struct ChiSquared {
    /// k: degrees of freedom.
    k: usize,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ChiSquared {
    /// New instance of a Chi-Squared distribution.
    pub fn new(k: usize) -> Self {
        assert!(k > 0);

        Self { k }
    }
}

impl Distribution for ChiSquared {
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        let k = self.k;

        (1.0 - 2.0 * i * t).powf(-(k as f64 / 2.0))
    }

    fn pdf(&self, x: f64) -> f64 {
        assert!(if self.k == 1 { x > 0.0 } else { x >= 0.0 });

        let k = self.k;

        x.powf((k as f64 / 2.0) - 1.0) * (-x / 2.0).exp()
            / (2_f64.powf(k as f64 / 2.0) * gamma(k as f64 / 2.0))
    }

    fn cdf(&self, x: f64) -> f64 {
        assert!(if self.k == 1 { x > 0.0 } else { x >= 0.0 });

        let k = self.k;

        gamma_li(k as f64 / 2.0, x / 2.0) / gamma(k as f64 / 2.0)
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        todo!()
    }

    fn pmf(&self, x: f64) -> f64 {
        self.pdf(x)
    }

    fn mean(&self) -> f64 {
        self.k as f64
    }

    fn median(&self) -> f64 {
        self.k as f64 * (1.0 - (2.0 / (9.0 * self.k as f64))).powf(3.0)
    }

    fn mode(&self) -> f64 {
        0_f64.max(self.k as f64 - 2.0)
    }

    fn variance(&self) -> f64 {
        2.0 * self.k as f64
    }

    fn skewness(&self) -> f64 {
        (8.0 / self.k as f64).sqrt()
    }

    fn kurtosis(&self) -> f64 {
        12.0 / self.k as f64
    }

    fn entropy(&self) -> f64 {
        todo!()
    }

    fn mgf(&self, t: f64) -> f64 {
        assert!(t < 0.5);

        (1.0 - 2.0 * t).powf(-(self.k as f64) / 2.0)
    }

    fn sample(&self, n: usize) -> Vec<f64> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{ChiSquared, Distribution};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = ChiSquared::new(self.k as f64).unwrap();

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
    fn test_chi_squared_characteristic_function() {
        let dist: ChiSquared = ChiSquared::new(1);

        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.5688645, 1e-7);
        assert_approx_equal!(cf.im, 0.3515776, 1e-7);
    }

    #[test]
    fn test_chi_squared_density_function() {
        let dist: ChiSquared = ChiSquared::new(1);

        // Values computed using R
        assert_approx_equal!(dist.pdf(1.0), 0.24197072, 1e-8);
        assert_approx_equal!(dist.pdf(2.0), 0.10377687, 1e-8);
        assert_approx_equal!(dist.pdf(3.0), 0.05139344, 1e-8);
        assert_approx_equal!(dist.pdf(4.0), 0.02699548, 1e-8);
        assert_approx_equal!(dist.pdf(5.0), 0.01464498, 1e-8);
    }

    #[test]
    fn test_chi_squared_distribution_function() {
        let dist: ChiSquared = ChiSquared::new(1);

        // Values computed using R
        assert_approx_equal!(dist.cdf(1.0), 0.6826895, 1e-7);
        assert_approx_equal!(dist.cdf(2.0), 0.8427008, 1e-7);
        assert_approx_equal!(dist.cdf(3.0), 0.9167355, 1e-7);
        assert_approx_equal!(dist.cdf(4.0), 0.9544997, 1e-7);
        assert_approx_equal!(dist.cdf(5.0), 0.9746527, 1e-7);
    }
}
