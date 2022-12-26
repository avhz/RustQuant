use num_complex::Complex;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Characteristic Functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// /// Bernoulli: Bern(p)
// pub fn cf_bernoulli(t: f64, p: f64) -> Complex<f64> {
//     assert!((0_f64..=1_f64).contains(&p));

//     let i: Complex<f64> = Complex::i();
//     1_f64 - p + p * (i * t).exp()
// }

/// Binomial: B(n, p)
pub fn cf_binomial(t: f64, p: f64, n: i32) -> Complex<f64> {
    assert!(n >= 0);
    assert!((0_f64..=1_f64).contains(&p));

    let i: Complex<f64> = Complex::i();
    (1_f64 - p + p * (i * t).exp()).powi(n)
}

/// Poisson: Pois(lambda)
pub fn cf_poisson(t: f64, lambda: f64) -> Complex<f64> {
    assert!(lambda > 0_f64);

    let i: Complex<f64> = Complex::i();
    (lambda * ((i * t).exp() - 1_f64)).exp()
}

/// Uniform (continuous): U(a, b)
pub fn cf_uniform_continuous(t: f64, a: f64, b: f64) -> Complex<f64> {
    let i: Complex<f64> = Complex::i();
    ((i * t * b).exp() - (i * t * a).exp()) / (i * t * (b - a))
}

/// Uniform (discrete): DU(a, b)
pub fn cf_uniform_discrete(t: f64, a: f64, b: f64) -> Complex<f64> {
    let i: Complex<f64> = Complex::i();
    ((i * t * a).exp() - (i * t * (b + 1_f64)).exp()) / ((1_f64 - (i * t).exp()) * (b - a + 1_f64))
}

/// Normal (univariate): N(mu,sigma^2)
pub fn cf_gaussian(t: f64, mu: f64, sigma_sq: f64) -> Complex<f64> {
    assert!(sigma_sq >= 0_f64);

    let i: Complex<f64> = Complex::i();
    (i * t * mu - 0.5 * (t * sigma_sq).powi(2)).exp()
}

/// Chi-Squared: X^2(k)
pub fn cf_chi_squared(t: f64, k: isize) -> Complex<f64> {
    assert!(k >= 0);

    let i: Complex<f64> = Complex::i();
    (1_f64 - 2_f64 * i * t).powf(-k as f64 / 2_f64)
}

/// Gamma: Gamma(k, theta)
pub fn cf_gamma(t: f64, k: f64, theta: f64) -> Complex<f64> {
    assert!(k > 0_f64);
    assert!(theta > 0_f64);

    let i: Complex<f64> = Complex::i();
    (1_f64 - i * t * theta).powf(-k)
}

/// Exponential: Exp(lambda)
pub fn cf_exponential(t: f64, lambda: f64) -> Complex<f64> {
    assert!(lambda > 0_f64);

    let i: Complex<f64> = Complex::i();
    1_f64 / (1_f64 - i * t / lambda)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS: Characteristic Functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    // #[test]
    // fn test_cf_bernoulli() {
    //     let cf = cf_bernoulli(1.0, 1.0);
    //     assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
    //     assert_approx_equal!(cf.im, 0.84147098480, 1e-10);
    // }

    #[test]
    fn test_cf_binomial() {
        let cf = cf_binomial(1.0, 1.0, 1);
        assert_approx_equal!(cf.re, 0.54030230586, 1e-10);
        assert_approx_equal!(cf.im, 0.84147098480, 1e-10);
    }

    #[test]
    fn test_cf_poisson() {
        let cf = cf_poisson(1.0, 1.0);
        assert_approx_equal!(cf.re, 0.42079361743, 1e-10);
        assert_approx_equal!(cf.im, 0.47084264330, 1e-10);
    }

    #[test]
    fn test_cf_uniform_continuous() {
        let cf = cf_uniform_continuous(1.0, 0.0, 1.0);
        assert_approx_equal!(cf.re, 0.84147098480, 1e-10);
        assert_approx_equal!(cf.im, 0.45969769413, 1e-10);
    }

    #[test]
    fn test_cf_uniform_discrete() {
        let cf = cf_uniform_discrete(1.0, 0.0, 1.0);
        assert_approx_equal!(cf.re, 0.77015115293, 1e-10);
        assert_approx_equal!(cf.im, 0.42073549240, 1e-10);
    }

    #[test]
    fn test_cf_gaussian() {
        let cf = cf_gaussian(1.0, 1.0, 1.0);
        assert_approx_equal!(cf.re, 0.32770991402, 1e-10);
        assert_approx_equal!(cf.im, 0.51037795154, 1e-10);
    }

    #[test]
    fn test_cf_chi_square() {
        let cf = cf_chi_squared(1.0, 1);
        assert_approx_equal!(cf.re, 0.56886448100, 1e-10);
        assert_approx_equal!(cf.im, 0.35157758425, 1e-10);
    }

    #[test]
    fn test_cf_gamma() {
        let cf = cf_gamma(1.0, 1.0, 1.0);
        assert_approx_equal!(cf.re, 0.5, 1e-10);
        assert_approx_equal!(cf.im, 0.5, 1e-10);
    }

    #[test]
    fn test_cf_exponential() {
        let cf = cf_exponential(1.0, 1.0);
        assert_approx_equal!(cf.re, 0.5, 1e-10);
        assert_approx_equal!(cf.im, 0.5, 1e-10);
    }
}
