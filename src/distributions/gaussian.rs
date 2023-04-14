use {
    num_complex::Complex,
    rand::thread_rng,
    rand_distr::{Distribution, Normal},
    statrs::function::erf,
    std::f64::consts::{PI, SQRT_2},
};

/// Gaussian (normal) distribution: X ~ N(mu, sigma^2)
pub struct Gaussian {
    /// Mean (location).
    mean: f64,
    /// Variance (squared scale).
    variance: f64,
}

impl Default for Gaussian {
    fn default() -> Self {
        Self {
            mean: 0.0,
            variance: 1.0,
        }
    }
}

impl Gaussian {
    /// New instance of a Gaussian distribution.
    pub fn new(mean: f64, variance: f64) -> Gaussian {
        assert!(variance > 0.0);

        Gaussian { mean, variance }
    }

    /// Gaussian characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        assert!(self.variance > 0.0);

        let i: Complex<f64> = Complex::i();

        (i * self.mean * t - 0.5 * (self.variance).powi(2) * (t).powi(2)).exp()
    }

    /// Gaussian density function.
    pub fn pdf(&self, x: f64) -> f64 {
        assert!(self.variance > 0.0);

        (-0.5 * ((x - self.mean) / self.variance).powi(2)).exp() / (2.0 * PI * self.variance).sqrt()
    }

    /// Gaussian distribution function.
    /// I used `erfc` (complementary error function) instead of `erf` to avoid
    /// subtractive cancellation that leads to inaccuracy in the tails.
    pub fn cdf(&self, x: f64) -> f64 {
        assert!(self.variance > 0.0);

        0.5 * erf::erfc(-(x - self.mean) / (SQRT_2 * self.variance.sqrt()))
    }

    /// Standard Normal Random Variates Generator
    pub fn variates(&self, n: usize) -> Vec<f64> {
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(normal.sample(&mut rng));
        }

        variates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_gaussian_characteristic_function() {
        // Standard normal distribution
        // mean = 0, variance = 1
        let dist: Gaussian = Gaussian::default();

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 1.0 / (1_f64.exp()).sqrt(), 1e-10);
        assert_approx_equal!(cf.im, 0.0, 1e-10);
    }

    #[test]
    fn test_gaussian_density_function() {
        // Standard normal distribution
        // mean = 0, variance = 1
        let dist: Gaussian = Gaussian::default();

        // Values from WolframAlpha
        assert_approx_equal!(dist.pdf(-4.0), 0.00013383, 1e-8);
        assert_approx_equal!(dist.pdf(-3.0), 0.00443185, 1e-8);
        assert_approx_equal!(dist.pdf(-2.0), 0.05399097, 1e-8);
        assert_approx_equal!(dist.pdf(-1.0), 0.24197072, 1e-8);
        assert_approx_equal!(dist.pdf(0.0), 0.39894228, 1e-8);
        assert_approx_equal!(dist.pdf(1.0), 0.24197072, 1e-8);
        assert_approx_equal!(dist.pdf(2.0), 0.05399097, 1e-8);
        assert_approx_equal!(dist.pdf(3.0), 0.00443185, 1e-8);
        assert_approx_equal!(dist.pdf(4.0), 0.00013383, 1e-8);
    }

    #[test]
    fn test_gaussian_distribution_function() {
        // Standard normal distribution
        // mean = 0, variance = 1
        let dist: Gaussian = Gaussian::default();

        // Values from WolframAlpha
        assert_approx_equal!(dist.cdf(-4.0), 0.00003167, 1e-8);
        assert_approx_equal!(dist.cdf(-3.0), 0.00134990, 1e-8);
        assert_approx_equal!(dist.cdf(-2.0), 0.02275013, 1e-8);
        assert_approx_equal!(dist.cdf(-1.0), 0.15865525, 1e-8);
        assert_approx_equal!(dist.cdf(0.0), 0.5, 1e-8);
        assert_approx_equal!(dist.cdf(1.0), 0.84134475, 1e-8);
        assert_approx_equal!(dist.cdf(2.0), 0.97724987, 1e-8);
        assert_approx_equal!(dist.cdf(3.0), 0.99865010, 1e-8);
        assert_approx_equal!(dist.cdf(4.0), 0.99996833, 1e-8);
    }

    #[test]
    fn test_gaussian_variate_generator() {
        let normal = Gaussian::default();

        let v = normal.variates(1000);

        let mean = (v.iter().sum::<f64>()) / (v.len() as f64);

        assert_approx_equal!(mean, 0.0, 0.1);
    }
}
