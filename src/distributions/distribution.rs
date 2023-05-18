// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num_complex::Complex;

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
    fn mean(&self) -> f64;

    /// Returns the median of the distribution.
    /// Median is the value that splits the distribution into two equal parts.
    fn median(&self) -> f64;

    /// Returns the mode of the distribution.
    /// Mode is the value that maximizes the probability density function.
    fn mode(&self) -> f64;

    /// Returns the variance of the distribution.
    /// Variance is a measure of the spread of the distribution.
    fn variance(&self) -> f64;

    /// Returns the skewness of the distribution.
    /// Skewness is a measure of the asymmetry of the distribution.
    fn skewness(&self) -> f64;

    /// Returns the kurtosis of the distribution.
    /// Kurtosis is a measure of the "tailedness" of the distribution.
    fn kurtosis(&self) -> f64;

    /// Returns the entropy of the distribution.
    /// Entropy is a measure of the uncertainty of the distribution.
    fn entropy(&self) -> f64;

    /// Moment generating function of the distribution.
    /// M = E[e^{tX}]
    fn mgf(&self, t: f64) -> f64;
}
