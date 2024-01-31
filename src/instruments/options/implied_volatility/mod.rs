// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Copyright © 2013-2014 Peter Jäckel.
// Permission to use, copy, modify, and distribute this software is freely granted,
// provided that this notice is preserved.

// WARRANTY DISCLAIMER
// The Software is provided "as is" without warranty of any kind, either express or implied,
// including without limitation any implied warranties of condition, uninterrupted use,
// merchantability, fitness for a particular purpose, or non-infringement.

// Documentation
//! Let's Be Rational rust implementation based on
//! [py_lets_be_rational](https://github.com/vollib/py_lets_be_rational)
//! and paper [Let's Be Rational](http://www.jaeckel.org/LetsBeRational.pdf)
//! by Peter Jaeckel  with some modifications.
//! If price is below intrinsic value, it returns -INF.
//! If price is above intrinsic value, it returns INF.

pub(crate) mod constants;
pub(crate) mod functions;

use super::{functions::*, TypeFlag};

/// Implied volatility function to calculate the implied volatility
/// of an option given its market price.
/// The method is based on lets be rational paper
/// [Let's Be Rational](http://www.jaeckel.org/LetsBeRational.pdf)
/// by Peter Jaeckel with some modifications.
/// If price is below intrinsic value, it returns -INF,
/// if price is above intrinsic value, it returns INF.
/// ```
/// use RustQuant::instruments::options::implied_volatility::*;
/// use RustQuant::instruments::options::TypeFlag;
/// use RustQuant::assert_approx_equal;
///
/// let price = 12.3;
/// let S = 100.0;
/// let K = 110.0;
/// let T = 0.89;
/// let r = 0.03;
///
/// let option_type = TypeFlag::Call;
///
/// let iv = implied_volatility(price, S, K, T, r, option_type);
///
/// assert_approx_equal!(iv, 0.40269973285787297, 1e-15);
/// ```
#[must_use]
pub fn implied_volatility(price: f64, S: f64, K: f64, T: f64, r: f64, flag: TypeFlag) -> f64 {
    let rate = (r * T).exp();

    let undiscounted_option_price = price * rate;

    let F = S * rate;

    let q = match flag {
        TypeFlag::Call => 1.0,
        TypeFlag::Put => -1.0,
    };

    implied_volatility_from_a_transformed_rational_guess_with_limited_iterations(
        undiscounted_option_price,
        F,
        K,
        T,
        q,
    )
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_lets_be_rational {
    use super::*;
    use crate::instruments::implied_volatility::constants::*;
    use crate::instruments::implied_volatility::functions::*;
    use crate::{assert_approx_equal, instruments::BlackScholesMerton};
    use std::panic;
    use time::{Duration, OffsetDateTime};

    // For normal prices, the precision is even higher, but for extrame cases, we need to be more relaxed
    const PRECISION: f64 = 1e-13;

    // This function generates beta for each region described in the paper
    // plus it checks behaviour outside of the bounds to return INF or -INF.
    // Moreover, it checks implied volatility  == 0.0 if price == intrinsic
    fn test_iv_region(
        underlying_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        days: i64,
        option_type: TypeFlag,
        slider: f64,
    ) {
        assert!(slider >= 1.0);

        let mut bs = BlackScholesMerton::new(
            risk_free_rate,
            underlying_price,
            strike_price,
            0.0,
            risk_free_rate,
            None,
            OffsetDateTime::now_utc() + Duration::days(days),
            option_type,
        );
        let T = bs.year_fraction();
        let F = bs.underlying_price * (bs.risk_free_rate * T).exp();
        let q = match option_type {
            TypeFlag::Call => 1.0,
            TypeFlag::Put => -1.0,
        };
        let x = (F / bs.strike_price).ln();
        // for
        // normalised_black_call, normalised_vega
        // we need to use CALL type, so we pass x*q

        let s_c = (2.0 * x * q).abs().sqrt();
        let b_c = normalised_black_call(x * q, s_c);
        let v_c = normalised_vega(x * q, s_c);

        let s_l = s_c - b_c / v_c;
        let b_l = normalised_black_call(x * q, s_l);
        let b_max = (0.5 * x * q).exp();
        let mut s_u = s_c;
        if v_c > f64::EPSILON {
            s_u = s_c + (b_max - b_c) / v_c;
        }
        let b_u = normalised_black_call(x * q, s_u);

        let normalized_intrinsic = normalised_intrinsic(x, q);
        // try beta = -0.1,0, b_l/2.0, b_l + (b_u - b_l)/2, b_u + (b_max - b_u)/2, b_max*1.1
        let betas = vec![
            -0.1,
            0.0,
            b_l / slider,
            b_l + (b_c - b_l) / slider,
            b_c + (b_u - b_c) / slider,
            b_u + (b_max - b_u) / slider,
            b_max * 1.1,
        ];
        for beta in betas {
            let s = unchecked_normalised_implied_volatility_from_a_transformed_rational_guess_with_limited_iterations(
                beta,
                x,
                q,
                IMPLIED_VOLATILITY_MAXIMUM_ITERATIONS
            )/ T.sqrt();
            // beta to price
            let beta_price =
                beta * (-bs.risk_free_rate * T).exp() * (F.sqrt() * bs.strike_price.sqrt());
            bs.volatility = s;
            match beta {
                temp if (normalized_intrinsic..b_max).contains(&temp) => {
                    assert_approx_equal!(beta_price, bs.price(), PRECISION)
                }
                temp if temp < normalized_intrinsic => assert_eq!(s, f64::NEG_INFINITY),
                temp if temp >= b_max => assert_eq!(s, f64::INFINITY),
                _ => panic!("Unexpected beta value"),
            }
        }
        // beta to price
    }

    #[test]
    fn test_OTM_calls() {
        test_iv_region(100.0, 110.0, 0.05, 30, TypeFlag::Call, 1.0001);
        test_iv_region(100.0, 120.0, 0.05, 745, TypeFlag::Call, 2.0);
        test_iv_region(100.0, 120.0, 0.05, 180, TypeFlag::Call, 1.0 / f64::EPSILON);
    }

    #[test]
    fn test_OTM_puts() {
        test_iv_region(100.0, 90.0, 0.05, 30, TypeFlag::Put, 1.0001);
        test_iv_region(100.0, 80.0, 0.05, 745, TypeFlag::Put, 10.0);
        test_iv_region(100.0, 80.0, 0.05, 180, TypeFlag::Put, 1.0 / f64::EPSILON);
    }

    #[test]
    fn test_ITM_calls() {
        test_iv_region(100.0, 90.0, 0.05, 30, TypeFlag::Call, 1.0001);
        test_iv_region(100.0, 80.0, 0.05, 745, TypeFlag::Call, 10.0);
        test_iv_region(100.0, 80.0, 0.05, 180, TypeFlag::Call, 1.0 / f64::EPSILON);
    }

    #[test]
    fn test_ITM_puts() {
        test_iv_region(100.0, 110.0, 0.05, 30, TypeFlag::Put, 1.0001);
        test_iv_region(100.0, 120.0, 0.05, 745, TypeFlag::Put, 10.0);
        test_iv_region(100.0, 120.0, 0.05, 180, TypeFlag::Put, 1.0 / f64::EPSILON);
    }

    #[test]
    fn test_implied_volatility() {
        // test OTM cases
        // these are unrealistic
        let bs = BlackScholesMerton::new(
            0.05,
            100.0,
            150.0,
            0.04,
            0.05,
            None,
            OffsetDateTime::now_utc() + Duration::days(365),
            TypeFlag::Call,
        );
        let s = implied_volatility(
            bs.price(), // the price is 1.431485617100085e-19
            bs.underlying_price,
            bs.strike_price,
            bs.year_fraction(),
            bs.risk_free_rate,
            bs.option_type,
        );
        assert_approx_equal!(s, 0.04, 1e-15);
    }
    #[test]
    fn test_linear_interpolation() {
        let x = -4.920_739_400_840_902;
        let beta = 0.005_550_954_806_846_956;
        // this values forces r == MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        let iv = unchecked_normalised_implied_volatility_from_a_transformed_rational_guess_with_limited_iterations(beta, x, 1.0,2);
        assert_approx_equal!(iv, 2.176_983_187_656_187, std::f64::EPSILON);
    }

    #[test]
    fn test_is_zero() {
        assert!(is_zero(0.0));
        assert!(is_zero(f64::MIN_POSITIVE / 2.0));
        assert!(!is_zero(f64::MIN_POSITIVE));
        assert!(!is_zero(f64::MIN_POSITIVE * 2.0));
        assert!(!is_zero(-0.1));
        assert!(!is_zero(0.1));
    }
}
