// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::Distribution as RQ_Distribution;
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
    pub fn new(trials: usize, probability: f64) -> Self {
        assert!((0.0..=1.0).contains(&probability));

        Self {
            n: trials,
            p: probability,
        }
    }
}

impl RQ_Distribution for Binomial {
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0.0..=1.0).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        (1.0 - self.p + self.p * (i * t).exp()).powi(self.n as i32)
    }

    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
    }

    fn pmf(&self, k: f64) -> f64 {
        assert!(k as usize <= self.n);
        assert!((0.0..=1.0).contains(&self.p));

        let n_C_k = |n: u32, k: u32| -> u32 {
            (1..=n).product::<u32>() / ((1..=k).product::<u32>() * (1..=(n - k)).product::<u32>())
        };

        n_C_k(self.n as u32, k as u32) as f64
            * self.p.powi(k as i32)
            * (1.0 - self.p).powi((self.n - k as usize) as i32)
    }

    fn cdf(&self, k: f64) -> f64 {
        statrs::function::beta::beta_reg((self.n - k as usize) as f64, (1. + k) as f64, 1. - self.p)
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

    fn mgf(&self, t: f64) -> f64 {
        todo!()
    }

    fn sample(&self, n: usize) -> Vec<f64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_binomial_distribution() {
        // n = 2 trials, p = 0.5 probability
        let dist: Binomial = Binomial::new(2, 0.5);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.41611444379, 1e-10);
        assert_approx_equal!(cf.im, 0.64805984911, 1e-10);

        // Probability mass function
        // k = 1 successes.
        let pmf = dist.pmf(1.);
        assert_approx_equal!(pmf, 0.5, 1e-10);

        // Distribution function
        // k = 1 successes.
        let cdf = dist.cdf(1.);
        assert_approx_equal!(cdf, 0.75, 1e-10);
    }
}
