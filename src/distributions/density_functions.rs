// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Probability Density and Mass Functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use statrs::function::gamma::gamma;
use std::f64::consts::PI;

// /// Bernoulli: Bern(p)
// pub fn pmf_bernoulli(k: i32, p: f64) -> f64 {
//     assert!((0_f64..=1_f64).contains(&p));
//     assert!(k == 0 || k == 1);

//     p.powi(k) * (1_f64 - p).powi(1 - k)
// }

// /// Binomial: B(n, p)
// pub fn pmf_binomial(n: u32, k: u32, p: f64) -> f64 {
//     assert!((0..=n).contains(&k));
//     assert!((0_f64..=1_f64).contains(&p));

//     let n_C_k = |n: u32, k: u32| -> u32 {
//         (1..=n).product::<u32>() / ((1..=k).product::<u32>() * (1..=(n - k)).product::<u32>())
//     };

//     n_C_k(n, k) as f64 * p.powi(k as i32) * (1_f64 - p).powi((n - k) as i32)
// }

// /// Poisson: Pois(lambda)
// pub fn pmf_poisson(lambda: f64, k: u32) -> f64 {
//     assert!(lambda > 0_f64);

//     lambda.powi(k as i32) * (-lambda).exp() / ((1..=k).product::<u32>() as f64)
// }

/// Uniform (continuous): U(a, b)
pub fn pdf_uniform_continuous(x: f64, a: f64, b: f64) -> f64 {
    if x >= a && x <= b {
        (b - a).recip()
    } else {
        0.0
    }
}

/// Uniform (discrete): DU(a, b)
pub fn pmf_uniform_discrete(a: isize, b: isize) -> f64 {
    assert!(a <= b);

    let n = b - a + 1;

    1_f64 / n as f64
}

/// Normal (univariate): N(mean, variance)
pub fn pdf_gaussian(x: f64, mean: f64, variance: f64) -> f64 {
    assert!(variance >= 0_f64);

    (2_f64 * variance * PI).sqrt().recip() * (-0.5 * (x - mean).powi(2) / variance).exp()
}

/// Chi-Squared: X^2(k)
pub fn pdf_chi_squared(x: f64, k: usize) -> f64 {
    if k == 1 {
        assert!(x > 0_f64);
    } else {
        assert!(x >= 0_f64);
    }

    (2_f64.powf(k as f64 / 2_f64) * gamma(k as f64 / 2_f64)).recip()
        * x.powf(-1_f64 + k as f64 / 2_f64)
        * (-0.5 * x).exp()
}

/// Gamma: Gamma(k, theta)
pub fn pdf_gamma(x: f64, k: f64, theta: f64) -> f64 {
    assert!(x > 0_f64);
    assert!(k > 0_f64);
    assert!(theta > 0_f64);

    (gamma(k) * theta.powf(k)).recip() * x.powf(k - 1_f64) * (-x / theta).exp()
}

/// Exponential: Exp(lambda)
pub fn pdf_exponential(x: f64, lambda: f64) -> f64 {
    assert!(lambda > 0_f64);
    assert!(x >= 0_f64);

    lambda * (-lambda * x).exp()
}

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // TESTS: Characteristic Functions
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    // #[test]
    // fn test_pmf_bernoulli() {
    //     let value = pmf_bernoulli(1, 0.5);
    //     assert_approx_equal!(value, 0.5, 1e-10);
    // }

    // #[test]
    // fn test_pmf_binomial() {
    //     let value = pmf_binomial(1, 1, 0.5);
    //     assert_approx_equal!(value, 0.5, 1e-10);
    // }

    // #[test]
    // fn test_pmf_poisson() {
    //     let value = pmf_poisson(1.0, 1);
    //     assert_approx_equal!(value, 0.367879441171, 1e-10);
    // }

    #[test]
    fn test_pdf_uniform_continuous() {
        let value = pdf_uniform_continuous(1.0, 0.0, 1.0);
        assert_approx_equal!(value, 1_f64, 1e-10);
    }

    #[test]
    fn test_pmf_uniform_discrete() {
        let value = pmf_uniform_discrete(0, 1);
        assert_approx_equal!(value, 0.5, 1e-10);
    }

    #[test]
    fn test_pdf_gaussian() {
        let value = pdf_gaussian(1.0, 0.0, 1.0);
        assert_approx_equal!(value, 0.241970724519, 1e-10);
    }

    #[test]
    fn test_pdf_chi_square() {
        let value = pdf_chi_squared(1.0, 1);
        assert_approx_equal!(value, 0.241970724519, 1e-10);
    }

    #[test]
    fn test_pdf_gamma() {
        let value = pdf_gamma(1.0, 1.0, 1.0);
        assert_approx_equal!(value, 0.367879441171, 1e-10);
    }

    #[test]
    fn test_cf_exponential() {
        let value = pdf_exponential(1.0, 1.0);
        assert_approx_equal!(value, 0.367879441171, 1e-10);
    }
}
