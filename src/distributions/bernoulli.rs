use super::Distribution;
use core::panic;
use num_complex::Complex;

/// Bernoulli distribution: X ~ Bern(p)
pub struct Bernoulli {
    /// Probability of k = 1 (q = 1 - p: probability of k = 0).
    p: f64,
}

impl Bernoulli {
    /// New instance of a Bernoulli distribution.
    pub fn new(probability: f64) -> Bernoulli {
        assert!((0_f64..=1_f64).contains(&probability));

        Bernoulli { p: probability }
    }
}

impl Distribution for Bernoulli {
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0_f64..=1_f64).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        1_f64 - self.p + self.p * (i * t).exp()
    }

    fn pdf(&self, _x: f64) -> f64 {
        panic!("Bernoulli distribution is discrete. Use pmf() instead.");
    }

    fn pmf(&self, k: f64) -> f64 {
        assert!((0_f64..=1_f64).contains(&self.p));
        assert!(k == 0.0 || k == 1.0);

        (self.p).powi(k as i32) * (1_f64 - self.p).powi(1 - k as i32)
    }

    fn cdf(&self, k: f64) -> f64 {
        assert!((0_f64..=1_f64).contains(&self.p));

        if (k as i32) < 0 {
            0_f64
        } else if (0..1).contains(&(k as i32)) {
            1_f64 - self.p
        } else {
            1_f64
        }
    }

    fn inv_cdf(&self, _p: f64) -> f64 {
        panic!("Generalised inverse CDF not implemented for Bernoulli distribution.");
    }

    fn mean(&self) -> f64 {
        self.p
    }

    /// Returns the median of the Bernoulli distribution.
    fn median(&self) -> f64 {
        panic!("Median not implemented for Bernoulli distribution.");
    }

    fn mode(&self) -> f64 {
        panic!("Mode not implemented for Bernoulli distribution.");
    }

    fn variance(&self) -> f64 {
        self.p * (1_f64 - self.p)
    }

    fn skewness(&self) -> f64 {
        let p = self.p;
        ((1.0 - p) - p) / (p * (1.0 - p)).sqrt()
    }

    fn kurtosis(&self) -> f64 {
        let p = self.p;
        (1.0 - 6.0 * p * (1.0 - p)) / (p * (1.0 - p))
    }

    fn entropy(&self) -> f64 {
        (self.p - 1.0) * (1.0 - self.p).ln() - self.p * (self.p).ln()
    }

    fn mgf(&self, t: f64) -> f64 {
        1.0 - self.p + self.p * f64::exp(t)
    }
}

#[cfg(test)]
mod bernoulli_tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_bernoulli_distribution() {
        let dist: Bernoulli = Bernoulli::new(1.0);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
        assert_approx_equal!(cf.im, 0.84147098480, 1e-10);

        // Probability mass function
        let pmf = dist.pmf(1.0);
        assert_approx_equal!(pmf, 1.0, 1e-10);

        // Distribution function
        let cdf = dist.cdf(1.0);
        assert_approx_equal!(cdf, 1.0, 1e-10);
    }
}
