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

    /// Bernoulli characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0_f64..=1_f64).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        1_f64 - self.p + self.p * (i * t).exp()
    }

    /// Bernoulli mass function.
    pub fn pmf(&self, k: i32) -> f64 {
        assert!((0_f64..=1_f64).contains(&self.p));
        assert!(k == 0 || k == 1);

        (self.p).powi(k) * (1_f64 - self.p).powi(1 - k)
    }

    /// Bernoulli distribution function.
    pub fn cdf(&self, k: i32) -> f64 {
        assert!((0_f64..=1_f64).contains(&self.p));

        if k < 0 {
            0_f64
        } else if k >= 0 && k < 1 {
            1_f64 - self.p
        } else {
            1_f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_bernoulli_distribution() {
        let Bernoulli: Bernoulli = Bernoulli::new(1.0);

        // Characteristic function
        let cf = Bernoulli.cf(1.0);
        assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
        assert_approx_equal!(cf.im, 0.84147098480, 1e-10);

        // Probability mass function
        let pmf = Bernoulli.pmf(1);
        assert_approx_equal!(pmf, 1.0, 1e-10);

        // Distribution function
        let cdf = Bernoulli.cdf(1);
        assert_approx_equal!(cdf, 1.0, 1e-10);
    }
}
