// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Probability Density and Mass Functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Bernoulli: Bern(p)
pub fn pmf_bernoulli(k: i32, p: f64) -> f64 {
    assert!((0_f64..=1_f64).contains(&p));
    assert!(k == 0 || k == 1);

    p.powi(k) * (1_f64 - p).powi(1 - k)
}

/// Binomial: B(n, p)
pub fn pmf_binomial(n: u32, k: u32, p: f64) -> f64 {
    assert!((0..=n).contains(&k));
    assert!((0_f64..=1_f64).contains(&p));

    let n_C_k = |n: u32, k: u32| -> u32 {
        (1..=n).product::<u32>() / ((1..=k).product::<u32>() * (1..=(n - k)).product::<u32>())
    };

    n_C_k(n, k) as f64 * p.powi(k as i32) * (1_f64 - p).powi((n - k) as i32)
}

/// Poisson: Pois(lambda)
pub fn pmf_poisson(lambda: f64, k: u32) -> f64 {
    assert!(lambda > 0_f64);

    lambda.powi(k as i32) * (-lambda).exp() / ((1..=k).product::<u32>() as f64)
}

// /// Uniform (continuous): U(a, b)
// pub fn pdf_uniform_continuous(t: f64, a: f64, b: f64) -> f64 {}

// /// Uniform (discrete): DU(a, b)
// pub fn pmf_uniform_discrete(t: f64, a: f64, b: f64) -> f64 {}

// /// Normal (univariate): N(mu,sigma^2)
// pub fn pdf_gaussian(x: f64, mu: f64, sigma_sq: f64) -> f64 {}

// /// Chi-Squared: X^2(k)
// pub fn pdf_chi_squared(x: f64, k: isize) ->f64 {}

// /// Gamma: Gamma(k, theta)
// pub fn pdf_gamma(x: f64, k: f64, theta: f64) -> f64 {}

// /// Exponential: Exp(lambda)
// pub fn pdf_exponential(x: f64, lambda: f64) -> Complex<f64> {}

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // TESTS: Characteristic Functions
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::assert_approx_equal;

//     #[test]
//     fn test_cf_bernoulli() {
//         let cf = cf_bernoulli(1.0, 1.0);
//         assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
//         assert_approx_equal!(cf.im, 0.84147098480, 1e-10);
//     }

//     #[test]
//     fn test_cf_binomial() {
//         let cf = cf_binomial(1.0, 1.0, 1);
//         assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
//         assert_approx_equal!(cf.im, 0.84147098480, 1e-10);
//     }

//     #[test]
//     fn test_cf_poisson() {
//         let cf = cf_poisson(1.0, 1.0);
//         assert_approx_equal!(cf.re, 0.42079361743, 1e-10);
//         assert_approx_equal!(cf.im, 0.47084264330, 1e-10);
//     }

//     #[test]
//     fn test_cf_uniform_continuous() {
//         let cf = cf_uniform_continuous(1.0, 0.0, 1.0);
//         assert_approx_equal!(cf.re, 0.84147098480, 1e-10);
//         assert_approx_equal!(cf.im, 0.45969769413, 1e-10);
//     }

//     #[test]
//     fn test_cf_uniform_discrete() {
//         let cf = cf_uniform_discrete(1.0, 0.0, 1.0);
//         assert_approx_equal!(cf.re, 0.77015115293, 1e-10);
//         assert_approx_equal!(cf.im, 0.42073549240, 1e-10);
//     }

//     #[test]
//     fn test_cf_gaussian() {
//         let cf = cf_gaussian(1.0, 1.0, 1.0);
//         assert_approx_equal!(cf.re, 0.32770991402, 1e-10);
//         assert_approx_equal!(cf.im, 0.51037795154, 1e-10);
//     }

//     #[test]
//     fn test_cf_chi_square() {
//         let cf = cf_chi_squared(1.0, 1);
//         assert_approx_equal!(cf.re, 0.56886448100, 1e-10);
//         assert_approx_equal!(cf.im, 0.35157758425, 1e-10);
//     }

//     #[test]
//     fn test_cf_gamma() {
//         let cf = cf_gamma(1.0, 1.0, 1.0);
//         assert_approx_equal!(cf.re, 0.5, 1e-10);
//         assert_approx_equal!(cf.im, 0.5, 1e-10);
//     }

//     #[test]
//     fn test_cf_exponential() {
//         let cf = cf_exponential(1.0, 1.0);
//         assert_approx_equal!(cf.re, 0.5, 1e-10);
//         assert_approx_equal!(cf.im, 0.5, 1e-10);
//     }
// }
