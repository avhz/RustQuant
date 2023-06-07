// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num_complex::Complex;
use statrs::function::gamma::{gamma, gamma_li};

/// Chi-Squared distribution: X ~ ChiSq(k)
pub struct ChiSquared {
    /// k: degrees of freedom.
    k: usize,
}

impl ChiSquared {
    /// New instance of a Chi-Squared distribution.
    pub fn new(k: usize) -> Self {
        assert!(k > 0);

        Self { k }
    }

    /// Chi-Squared characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        let k = self.k;

        (1.0 - 2.0 * i * t).powf(-(k as f64 / 2.0))
    }

    /// Chi-Squared probability density function.
    pub fn pdf(&self, x: f64) -> f64 {
        assert!(if self.k == 1 { x > 0.0 } else { x >= 0.0 });

        let k = self.k;

        x.powf((k as f64 / 2.0) - 1.0) * (-x / 2.0).exp()
            / (2_f64.powf(k as f64 / 2.0) * gamma(k as f64 / 2.0))
    }

    /// Chi-Squared distribution function.
    pub fn cdf(&self, x: f64) -> f64 {
        assert!(if self.k == 1 { x > 0.0 } else { x >= 0.0 });

        let k = self.k;

        gamma_li(k as f64 / 2.0, x / 2.0) / gamma(k as f64 / 2.0)
    }
}

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
