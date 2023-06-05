// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use {
    // Needed for the characteristic function.
    num_complex::Complex,
    // Needed for the CDF.
    statrs::function::gamma::*,
};

/// Poisson distribution: X ~ Pois(lambda)
pub struct Poisson {
    /// Rate parameter.
    lambda: f64,
}

impl Poisson {
    /// New instance of a Poisson distribution.
    pub fn new(lambda: f64) -> Poisson {
        assert!(lambda > 0.0);

        Poisson { lambda }
    }

    /// Poisson characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        (self.lambda * ((i * t).exp() - 1.0)).exp()
    }

    /// Poisson mass function.
    pub fn pmf(&self, k: usize) -> f64 {
        (self.lambda).powi(k as i32) * (-(self.lambda)).exp() / ((1..=k).product::<usize>() as f64)
    }

    /// Poisson distribution function.
    pub fn cdf(&self, k: usize) -> f64 {
        1.0 - gamma_li((k + 1) as f64, self.lambda) / gamma_ui((k + 1) as f64, self.lambda)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_poisson_distribution() {
        let dist: Poisson = Poisson::new(1.0);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.42079361743, 1e-10);
        assert_approx_equal!(cf.im, 0.47084264330, 1e-10);

        // Probability mass function
        let pmf = dist.pmf(1);
        assert_approx_equal!(pmf, 0.367879441171, 1e-10);

        // Distribution function
        let cdf = dist.cdf(1);
        assert_approx_equal!(cdf, 0.640859085770, 1e-10);
    }
}
