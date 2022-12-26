use num_complex::Complex;

/// Binomial distribution: X ~ Bin(n, p)
pub struct Binomial {
    /// Number of trials.
    n: usize,
    /// Probability of k = 1 (q = 1 - p: probability of k = 0).
    p: f64,
}

impl Binomial {
    /// New instance of a Binomial distribution.
    pub fn new(trials: usize, probability: f64) -> Binomial {
        assert!((0_f64..=1_f64).contains(&probability));

        Binomial {
            n: trials,
            p: probability,
        }
    }

    /// Binomial characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0_f64..=1_f64).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        (1_f64 - self.p + self.p * (i * t).exp()).powi(self.n as i32)
    }

    /// Binomial mass function.
    pub fn pmf(&self, k: usize) -> f64 {
        assert!(k <= self.n);
        assert!((0_f64..=1_f64).contains(&self.p));

        let n_C_k = |n: u32, k: u32| -> u32 {
            (1..=n).product::<u32>() / ((1..=k).product::<u32>() * (1..=(n - k)).product::<u32>())
        };

        n_C_k(self.n as u32, k as u32) as f64
            * self.p.powi(k as i32)
            * (1_f64 - self.p).powi((self.n - k) as i32)
    }

    /// Binomial distribution function.
    pub fn cdf(&self, k: i32) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_binomial_distribution() {
        let binomial: Binomial = Binomial::new(1, 1.0);

        // Characteristic function
        let cf = binomial.cf(1.0);
        assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
        assert_approx_equal!(cf.im, 0.84147098480, 1e-10);

        // Probability mass function
        let pmf = binomial.pmf(1);
        assert_approx_equal!(pmf, 1.0, 1e-10);

        // Distribution function
        // let cdf = binomial.cdf(1);
        // assert_approx_equal!(cdf, 1.0, 1e-10);
    }
}
