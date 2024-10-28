// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use num::Float;

/// Statistics trait for vectors of floating point numbers.
pub trait Statistic<T: Float> {
    // MEAN FUNCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Calculate the mean of a vector.
    /// Simply a wrapper for `arithmetic_mean`.
    fn mean(&self) -> T;
    /// Calculate the arithmetic mean of a vector.
    fn arithmetic_mean(&self) -> T;
    /// Calculate the geometric mean of a vector.
    fn geometric_mean(&self) -> T;
    /// Calculate the harmonic mean of a vector.
    fn harmonic_mean(&self) -> T;

    // VARIANCE FUNCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Calculate the variance of a vector.
    /// Simply a wrapper for `sample_variance`.
    fn variance(&self) -> T;
    /// Calculate the sample variance of a vector.
    fn sample_variance(&self) -> f64;
    /// Calculate the population variance of a vector.
    fn population_variance(&self) -> f64;

    // STANDARD DEVIATION FUNCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Calculate the standard deviation of a vector.
    /// Simply a wrapper for `sample_standard_deviation`.
    fn standard_deviation(&self) -> T;
    /// Calculate the sample standard deviation of a vector.
    fn sample_standard_deviation(&self) -> f64;
    /// Calculate the population standard deviation of a vector.
    fn population_standard_deviation(&self) -> f64;

    /// Calculate the covariance of two vectors.
    fn covariance(&self, other: &Self) -> T;

    /// Calculate the correlation of two vectors.
    fn correlation(&self, other: &Self) -> T;

    /// Calculate the skewness of a vector.
    fn skewness(&self) -> T;

    /// Calculate the kurtosis of a vector.
    fn kurtosis(&self) -> T;

    /// Calculate the minimum value of a vector.
    fn min(&self) -> T;

    /// Calculate the maximum value of a vector.
    fn max(&self) -> T;

    /// Calculate the median of a vector.
    fn median(&self) -> T;

    /// Calculate the percentile of a vector.
    fn percentile(&self, percentile: f64) -> T;

    /// Calculate the quantile of a vector.
    fn quantile(&self, quantile: f64) -> T;

    /// Calculate the interquartile range of a vector.
    fn interquartile_range(&self) -> T;

    /// Calculate the range of a vector.
    fn range(&self) -> T;
}

impl Statistic<f64> for Vec<f64> {
    fn mean(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.arithmetic_mean()
    }

    fn arithmetic_mean(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.iter().sum::<f64>() / self.len() as f64
    }

    fn geometric_mean(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.iter().product::<f64>().powf(1.0 / self.len() as f64)
    }

    fn harmonic_mean(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.len() as f64 / self.iter().map(|x| 1.0 / x).sum::<f64>()
    }

    fn variance(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.sample_variance()
    }

    fn sample_variance(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        let mu = self.mean();
        self.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (self.len() - 1) as f64
    }

    fn population_variance(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        let mu = self.mean();
        self.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / self.len() as f64
    }

    fn standard_deviation(&self) -> f64 {
        assert!(self.len() > 1, "Vector must have at least two elements.");

        self.sample_standard_deviation()
    }

    fn sample_standard_deviation(&self) -> f64 {
        assert!(self.len() > 1, "Vector must have at least two elements.");

        self.variance().sqrt()
    }

    fn population_standard_deviation(&self) -> f64 {
        assert!(self.len() > 1, "Vector must have at least two elements.");

        self.population_variance().sqrt()
    }

    fn covariance(&self, other: &Self) -> f64 {
        assert_eq!(
            self.len(),
            other.len(),
            "Vectors must have the same length."
        );
        assert!(self.len() > 1, "Vectors must have at least two elements.");

        let n = self.len() as f64;
        let mean_x = self.iter().sum::<f64>() / n;
        let mean_y = other.iter().sum::<f64>() / n;

        let cov = self
            .iter()
            .zip(other.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<f64>();

        cov / (n - 1.0)
    }

    fn correlation(&self, other: &Self) -> f64 {
        let cov = self.covariance(other);
        let std_dev_x = self.standard_deviation();
        let std_dev_y = other.standard_deviation();

        assert!(
            !(std_dev_x == 0.0 || std_dev_y == 0.0),
            "Standard deviation should not be zero."
        );

        cov / (std_dev_x * std_dev_y)
    }

    fn skewness(&self) -> f64 {
        assert!(self.len() > 2, "Vector must have at least three elements.");

        let n = self.len() as f64;
        let mean = self.mean();
        let std_dev = self.standard_deviation();

        let skew = self
            .iter()
            .map(|x| (x - mean) / std_dev)
            .map(|x| x.powi(3))
            .sum::<f64>();

        skew * n / ((n - 1.0) * (n - 2.0))
    }

    fn kurtosis(&self) -> f64 {
        assert!(self.len() > 3, "Vector must have at least four elements.");

        let n = self.len() as f64;
        let mean = self.mean();
        let std_dev = self.standard_deviation();

        let kurt = self
            .iter()
            .map(|x| (x - mean) / std_dev)
            .map(|x| x.powi(4))
            .sum::<f64>();

        kurt * n * (n + 1.0) / ((n - 1.0) * (n - 2.0) * (n - 3.0))
            - 3.0 * (n - 1.0).powi(2) / ((n - 2.0) * (n - 3.0))
    }

    fn min(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    fn max(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    fn median(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        let mut sorted = self.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    fn percentile(&self, percentile: f64) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");
        assert!(
            (0.0..=1.0).contains(&percentile),
            "Percentile must be between 0 and 1."
        );

        let mut sorted = self.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = percentile * (sorted.len() - 1) as f64;
        let lower = sorted[index.floor() as usize];
        let upper = sorted[index.ceil() as usize];

        lower + (upper - lower) * (index - index.floor())
    }

    fn quantile(&self, quantile: f64) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");
        assert!(
            (0.0..=1.0).contains(&quantile),
            "Quantile must be between 0 and 1."
        );

        let mut sorted = self.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = quantile * (sorted.len() - 1) as f64;
        let lower = sorted[index.floor() as usize];
        let upper = sorted[index.ceil() as usize];

        lower + (upper - lower) * (index - index.floor())
    }

    fn interquartile_range(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        let mut sorted = self.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let q1 = sorted[sorted.len() / 4];
        let q3 = sorted[sorted.len() * 3 / 4];

        q3 - q1
    }

    fn range(&self) -> f64 {
        assert!(!self.is_empty(), "Vector must have at least one element.");

        self.max() - self.min()
    }
}

#[cfg(test)]
mod tests_statistics {

    use super::*;
    use crate::assert_approx_equal;
    use std::f64::EPSILON as EPS;

    #[test]
    fn test_mean_arithmetic() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let m = v.arithmetic_mean();
        assert_approx_equal!(m, 3.0, EPS);
    }

    #[test]
    fn test_mean_geometric() {
        let v = vec![1.0, 2.0, 4.0, 8.0, 16.0];
        let m = v.geometric_mean();
        assert_approx_equal!(m, 4.0, EPS);
    }

    #[test]
    fn test_mean_harmonic() {
        let v = vec![1.0, 2.0, 4.0];
        let m = v.harmonic_mean();
        assert_approx_equal!(m, 1.714_285_714_285_714_2, EPS);
    }

    #[test]
    #[should_panic(expected = "Vector must have at least one element.")]
    fn test_mean_empty_vector() {
        let v: Vec<f64> = vec![];
        v.mean();
    }

    #[test]
    fn test_standard_deviation() {
        let v = vec![1.0, 2.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let stddev = v.standard_deviation();
        assert_approx_equal!(stddev, 2.559_994_419_636_775, EPS);
    }

    #[test]
    #[should_panic(expected = "Vector must have at least two elements.")]
    fn test_standard_deviation_single_element() {
        let v = vec![1.0];
        v.standard_deviation();
    }

    #[test]
    fn test_correlation() {
        let x = vec![2.1, 2.5, 4.0, 3.6];
        let y = vec![8.0, 10.0, 12.0, 14.0];
        let corr = x.correlation(&y);
        assert_approx_equal!(corr, 0.864_226_802_873_516_2, EPS);
    }

    #[test]
    #[should_panic(expected = "Standard deviation should not be zero.")]
    fn test_correlation_std_dev_zero() {
        let x = vec![1.0, 1.0];
        let y = vec![2.0, 2.0];
        x.correlation(&y);
    }

    #[test]
    #[should_panic(expected = "Vectors must have the same length.")]
    fn test_correlation_diff_length() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0];
        x.correlation(&y);
    }

    #[test]
    #[should_panic(expected = "Vectors must have at least two elements.")]
    fn test_correlation_one_element() {
        let x = vec![1.0];
        let y = vec![1.0];
        x.correlation(&y);
    }

    #[test]
    fn test_covariance() {
        let x = vec![2.1, 2.5, 4.0, 3.6];
        let y = vec![8.0, 10.0, 12.0, 14.0];
        let cov = x.covariance(&y);
        assert_approx_equal!(cov, 2.0, EPS);
    }

    #[test]
    #[should_panic(expected = "Vectors must have the same length.")]
    fn test_covariance_diff_length() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0];
        x.covariance(&y);
    }

    #[test]
    #[should_panic(expected = "Vectors must have at least two elements.")]
    fn test_covariance_one_element() {
        let x = vec![1.0];
        let y = vec![1.0];
        x.covariance(&y);
    }

    #[test]
    fn test_variance() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        // let mu = mean(&v, MeanType::Arithmetic);

        // Population variance.
        let var_pop = v.population_variance();
        assert_approx_equal!(var_pop, 2.0, EPS);

        // Sample variance.
        let var_samp = v.variance();
        assert_approx_equal!(var_samp, 2.5, EPS);
    }

    #[test]
    fn test_population_standard_deviation() {
        let v = vec![1.0, 2.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let pop_stddev = v.population_standard_deviation();
        assert_approx_equal!(pop_stddev, 2.394_655_507_583_502, EPS);
    }

    #[test]
    fn test_sample_standard_deviation() {
        let v = vec![1.0, 2.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let samp_stddev = v.sample_standard_deviation();
        assert_approx_equal!(samp_stddev, 2.559_994_419_636_775, EPS);
    }

    #[test]
    fn test_sample_standard_deviation_two_elements() {
        let v = vec![1.0, 2.0];
        let samp_stddev = v.sample_standard_deviation();
        assert_approx_equal!(samp_stddev, std::f64::consts::FRAC_1_SQRT_2, EPS);
    }

    #[test]
    #[should_panic(expected = "Vector must have at least two elements.")]
    fn test_population_standard_deviation_single_element() {
        let v = vec![1.0];
        v.population_standard_deviation();
    }

    #[test]
    fn test_population_standard_deviation_two_elements() {
        let v = vec![1.0, 2.0];
        let pop_stddev = v.population_standard_deviation();
        assert_approx_equal!(pop_stddev, 0.5, EPS);
    }

    #[test]
    #[should_panic(expected = "Vector must have at least one element.")]
    fn test_min_empty_vector() {
        let v: Vec<f64> = vec![];
        v.min();
    }

    #[test]
    #[should_panic(expected = "Vector must have at least one element.")]
    fn test_max_empty_vector() {
        let v: Vec<f64> = vec![];
        v.max();
    }

    #[test]
    fn test_min_single_element() {
        let v = vec![42.0];
        assert_approx_equal!(v.min(), 42.0, EPS);
    }

    #[test]
    fn test_max_single_element() {
        let v = vec![42.0];
        assert_approx_equal!(v.max(), 42.0, EPS);
    }

    // #[test]
    // fn test_skewness() {
    //     let v = vec![1.0, 2.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    //     let skew = v.skewness();
    //     assert_approx_equal!(skew, 0.2824751, 1e-6);
    // }

    // #[test]
    // fn test_kurtosis() {
    //     let v = vec![1.0, 2.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    //     let kurt = v.kurtosis();
    //     assert_approx_equal!(kurt, 2.351825, 1e-6);
    // }
}
