// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::DistributionClass;
use crate::distributions::Distribution;
use num_complex::Complex;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Uniform distribution: X ~ Uni(a, b)
pub struct Uniform {
    /// Lower bound.
    a: f64,
    /// Upper bound.
    b: f64,
    /// Continuous or discrete ?
    class: DistributionClass,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Uniform {
    /// New instance of a Uniform distribution.
    pub fn new(a: f64, b: f64, class: DistributionClass) -> Self {
        assert!(a <= b);

        match class {
            DistributionClass::Discrete => Self {
                a: a.round(),
                b: b.round(),
                class,
            },
            DistributionClass::Continuous => Self { a, b, class },
        }
    }
}

impl Distribution for Uniform {
    fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();

        match self.class {
            DistributionClass::Discrete => {
                ((i * t * self.a).exp() - (i * t * (self.b + 1.0)).exp())
                    / ((1.0 - (i * t).exp()) * (self.b - self.a + 1.0))
            }
            DistributionClass::Continuous => {
                ((i * t * self.b).exp() - (i * t * self.a).exp()) / (i * t * (self.b - self.a))
            }
        }
    }

    fn pdf(&self, _x: f64) -> f64 {
        todo!()
    }

    fn pmf(&self, x: f64) -> f64 {
        match self.class {
            DistributionClass::Discrete => {
                if x >= self.a && x <= self.b {
                    (self.b - self.a + 1.0).recip()
                } else {
                    0.0
                }
            }
            DistributionClass::Continuous => {
                if x >= self.a && x <= self.b {
                    (self.b - self.a).recip()
                } else {
                    0.0
                }
            }
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        match self.class {
            DistributionClass::Discrete => {
                if x < self.a {
                    0.0
                } else if x >= self.a && x <= self.b {
                    (x.floor() - self.a + 1.0) / (self.b - self.a + 1.0)
                } else {
                    1.0
                }
            }
            DistributionClass::Continuous => {
                if x < self.a {
                    0.0
                } else if x >= self.a && x <= self.b {
                    (x - self.a) / (self.b - self.a)
                } else {
                    1.0
                }
            }
        }
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        todo!()
    }

    fn mean(&self) -> f64 {
        todo!()
    }

    fn median(&self) -> f64 {
        todo!()
    }

    fn mode(&self) -> f64 {
        todo!()
    }

    fn variance(&self) -> f64 {
        todo!()
    }

    fn skewness(&self) -> f64 {
        todo!()
    }

    fn kurtosis(&self) -> f64 {
        todo!()
    }

    fn entropy(&self) -> f64 {
        todo!()
    }

    fn mgf(&self, _t: f64) -> f64 {
        todo!()
    }

    fn sample(&self, _n: usize) -> Vec<f64> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        todo!()
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
    fn test_uniform_distribution_continuous() {
        let dist: Uniform = Uniform::new(0.0, 1.0, DistributionClass::Continuous);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.84147098480, 1e-10);
        assert_approx_equal!(cf.im, 0.45969769413, 1e-10);

        // Probability mass function
        let pmf = dist.pmf(0.5);
        assert_approx_equal!(pmf, 1.0, 1e-10);

        // Distribution function
        let cdf = dist.cdf(0.5);
        assert_approx_equal!(cdf, 0.5, 1e-10);
    }

    #[test]
    fn test_uniform_distribution_discrete() {
        let dist: Uniform = Uniform::new(0.0, 1.0, DistributionClass::Discrete);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.77015115293, 1e-10);
        assert_approx_equal!(cf.im, 0.42073549240, 1e-10);

        // Probability mass function
        let pmf = dist.pmf(0.5);
        assert_approx_equal!(pmf, 0.5, 1e-10);

        // Distribution function
        let cdf = dist.cdf(0.5);
        assert_approx_equal!(cdf, 0.5, 1e-10);
    }
}
