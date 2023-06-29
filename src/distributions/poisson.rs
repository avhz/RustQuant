// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::Distribution;
use num_complex::Complex;
use statrs::function::gamma::*;

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
    pub fn new(lambda: f64) -> Poisson {
        assert!(lambda > 0.0);

        Poisson { lambda }
    }
}

impl Distribution for Poisson {
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();
        (self.lambda * ((i * t).exp() - 1.0)).exp()
    }

    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
    }

    fn pmf(&self, x: f64) -> f64 {
        (self.lambda).powi(x as i32) * (-(self.lambda)).exp()
            / ((1..=x as usize).product::<usize>() as f64)
    }

    fn cdf(&self, x: f64) -> f64 {
        1.0 - gamma_li(x + 1., self.lambda) / gamma_ui(x + 1., self.lambda)
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        todo!()
    }

    fn mean(&self) -> f64 {
        self.lambda
    }

    fn median(&self) -> f64 {
        (self.lambda + 1.0 / 3.0 - 0.02 / self.lambda).floor()
    }

    fn mode(&self) -> f64 {
        self.lambda.floor()
    }

    fn variance(&self) -> f64 {
        self.lambda
    }

    fn skewness(&self) -> f64 {
        self.lambda.sqrt().recip()
    }

    fn kurtosis(&self) -> f64 {
        self.lambda.recip()
    }

    fn entropy(&self) -> f64 {
        todo!()
    }

    fn mgf(&self, t: f64) -> f64 {
        (self.lambda * (t.exp() - 1.0)).exp()
    }

    fn sample(&self, n: usize) -> Vec<f64> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Distribution, Poisson};

        let mut rng = thread_rng();

        let dist = Poisson::new(self.lambda).unwrap();

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
    fn test_poisson_distribution() {
        let dist: Poisson = Poisson::new(1.0);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.42079361743, 1e-10);
        assert_approx_equal!(cf.im, 0.47084264330, 1e-10);

        // Probability mass function
        let pmf = dist.pmf(1.);
        assert_approx_equal!(pmf, 0.367879441171, 1e-10);

        // Distribution function
        let cdf = dist.cdf(1.);
        assert_approx_equal!(cdf, 0.640859085770, 1e-10);
    }
}
