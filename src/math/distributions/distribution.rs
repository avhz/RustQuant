// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num::Complex;
use thiserror::Error;

/// Imaginary unit.
#[allow(non_upper_case_globals)]
pub const i: Complex<f64> = Complex { re: 0.0, im: 1.0 };

/// Distribution class/type.
#[allow(clippy::module_name_repetitions)]
pub enum DistributionClass {
    /// Discrete distribution.
    Discrete,
    /// Continuous distribution.
    Continuous,
}

/// Distribution error type.
#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum DistributionError {
    /// Error variant from constructing Bernoulli distribution.
    #[error("{0}")]
    Bernoulli(#[from] rand_distr::BernoulliError),

    /// Error variant from constructing Binomial distribution.
    #[error("{0}")]
    Binomial(#[from] rand_distr::BinomialError),

    /// Error variant from constructing ChiSquared distribution.
    #[error("{0}")]
    ChiSquared(#[from] rand_distr::ChiSquaredError),

    /// Error variant from constructing Exponential distribution.
    #[error("{0}")]
    Exponential(#[from] rand_distr::ExpError),

    /// Error variant from constructing Gamma distribution.
    #[error("{0}")]
    Gamma(#[from] rand_distr::GammaError),

    /// Error variant from constructing Gaussian distribution.
    #[error("{0}")]
    Gaussian(#[from] rand_distr::NormalError),

    /// Error variant from constructing Poisson distribution.
    #[error("{0}")]
    Poisson(#[from] rand_distr::PoissonError),
}

/// Base trait for all distributions.
/// Provides common methods for all distributions.
/// All distributions must implement this trait.
pub trait Distribution {
    /// Characteristic function of the distribution.
    /// Returns the value of the characteristic function at t.
    /// The characteristic function is defined as:  
    ///    cf(t) = E[e^{itX}]
    fn cf(&self, t: f64) -> Complex<f64>;

    /// Probability density function of the distribution.
    /// Returns the probability that a random variable is equal to x.
    /// NOTE: Panics if the distribution is discrete.
    fn pdf(&self, x: f64) -> f64;

    /// Probability mass function of the distribution.
    /// Returns the probability that a random variable is equal to x.
    /// NOTE: Panics if the distribution is continuous.
    fn pmf(&self, x: f64) -> f64;

    /// Distribution function of the distribution.
    /// Returns the probability that a random variable is less than or equal to x.
    fn cdf(&self, x: f64) -> f64;

    /// Inverse distribution function of the distribution.
    /// Returns the value of x such that cdf(x) = p.
    fn inv_cdf(&self, p: f64) -> f64;

    /// Returns the mean of the distribution.
    /// Mean is the average value of the distribution.
    /// <https://en.wikipedia.org/wiki/Mean>
    fn mean(&self) -> f64;

    /// Returns the median of the distribution.
    /// Median is the value that splits the distribution into two equal parts.
    /// <https://en.wikipedia.org/wiki/Median>
    fn median(&self) -> f64;

    /// Returns the mode of the distribution.
    /// Mode is the value that maximizes the probability density function.
    /// <https://en.wikipedia.org/wiki/Mode_(statistics)>
    fn mode(&self) -> f64;

    /// Returns the variance of the distribution.
    /// Variance is a measure of the spread of the distribution.
    /// <https://en.wikipedia.org/wiki/Variance>
    fn variance(&self) -> f64;

    /// Returns the skewness of the distribution.
    /// Skewness is a measure of the asymmetry of the distribution.
    /// <https://en.wikipedia.org/wiki/Skewness>
    fn skewness(&self) -> f64;

    /// Returns the kurtosis of the distribution.
    /// Kurtosis is a measure of the "tailedness" of the distribution.
    /// <https://en.wikipedia.org/wiki/Kurtosis>
    fn kurtosis(&self) -> f64;

    /// Returns the entropy of the distribution.
    /// Entropy is a measure of the uncertainty of the distribution.
    fn entropy(&self) -> f64;

    /// Moment generating function of the distribution.
    /// M = E[e^{tX}]
    /// <https://en.wikipedia.org/wiki/Moment-generating_function>
    fn mgf(&self, t: f64) -> f64;

    /// Generates a random sample from the distribution.
    fn sample(&self, n: usize) -> Result<Vec<f64>, DistributionError>;
}
