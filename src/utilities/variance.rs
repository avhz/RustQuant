// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::utilities::{mean, MeanType};

/// Variance type (population or sample).
pub enum VarianceType {
    /// Population variance: `sum((x_i - mu)^2) / n`
    Population,
    /// Sample variance: `sum((x_i - mu)^2) / (n - 1)`
    Sample,
}

/// Variance of a vector.
pub fn variance(v: &Vec<f64>, variance_type: VarianceType) -> f64 {
    assert!(!v.is_empty(), "Vector must have at least one element.");

    match variance_type {
        VarianceType::Population => {
            let mu = mean(v, MeanType::Arithmetic);
            v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / v.len() as f64
        }
        VarianceType::Sample => {
            let mu = mean(v, MeanType::Arithmetic);
            v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() - 1) as f64
        }
    }
}

#[cfg(test)]
mod tests_variance {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_variance() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        // let mu = mean(&v, MeanType::Arithmetic);

        // Population variance.
        let var_pop = variance(&v, VarianceType::Population);
        assert_approx_equal!(var_pop, 2.0, 0.0001);

        // Sample variance.
        let var_samp = variance(&v, VarianceType::Sample);
        assert_approx_equal!(var_samp, 2.5, 0.0001);
    }
}
