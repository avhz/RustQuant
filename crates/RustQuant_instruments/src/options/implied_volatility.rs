// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
//
// Copyright © 2013-2014 Peter Jäckel.
// Permission to use, copy, modify, and distribute this software is freely granted,
// provided that this notice is preserved.
//
// WARRANTY DISCLAIMER
// The Software is provided "as is" without warranty of any kind, either express or implied,
// including without limitation any implied warranties of condition, uninterrupted use,
// merchantability, fitness for a particular purpose, or non-infringement.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Let's Be Rational rust implementation based on
//! [py_lets_be_rational](https://github.com/vollib/py_lets_be_rational)
//! and paper [Let's Be Rational](http://www.jaeckel.org/LetsBeRational.pdf)
//! by Peter Jaeckel  with some modifications.
//! If price is below intrinsic value, it returns -INF.
//! If price is above intrinsic value, it returns INF.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::TypeFlag;
use errorfunctions::RealErrorFunctions;
use RustQuant_math::distributions::{gaussian::Gaussian, Distribution};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// CONSTANTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) const SIXTEENTH_ROOT_DBL_EPSILON: f64 = 0.105_112_051_906_714_33;
pub(crate) const FOURTH_ROOT_DBL_EPSILON: f64 = 0.000_122_070_312_5;
pub(crate) const SQRT_ONE_OVER_THREE: f64 = 0.577_350_269_189_625_7;
pub(crate) const SQRT_THREE: f64 = 1.732_050_807_568_877_2;
pub(crate) const TWO_PI_OVER_SQRT_TWENTY_SEVEN: f64 = 1.209_199_576_156_145_2;
pub(crate) const SQRT_PI_OVER_TWO: f64 = 1.253_314_137_315_500_3;
pub(crate) const ONE_OVER_SQRT_TWO_PI: f64 = 0.398_942_280_401_432_7;
pub(crate) const SQRT_TWO_PI: f64 = 2.506_628_274_631_000_7;
pub(crate) const SQRT_DBL_MIN: f64 = 1.491_668_146_240_041_3e-154;
pub(crate) const SQRT_DBL_MAX: f64 = 1.340_780_792_994_259_6e154;

// Set this to 0 if you want positive results for (positive) denormalized inputs, else to DBL_MIN.
// Note that you cannot achieve full machine accuracy from denormalized inputs!
pub(crate) const DENORMALIZATION_CUTOFF: f64 = 0.0;
pub(crate) const IMPLIED_VOLATILITY_MAXIMUM_ITERATIONS: usize = 2;
pub(crate) const ASYMPTOTIC_EXPANSION_ACCURACY_THRESHOLD: f64 = -10.0;
pub(crate) const SMALL_T_EXPANSION_OF_NORMALIZED_BLACK_THRESHOLD: f64 =
    2.0 * SIXTEENTH_ROOT_DBL_EPSILON;

//-(1.0 - f64::EPSILON.sqrt());
// cannot call non-const fn `std::f64::<impl f64>::sqrt` in constants
pub(crate) const MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE: f64 = 1.490_116_119_384_765_6e-8;
pub(crate) const MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE: f64 =
    2.0 / (f64::EPSILON * f64::EPSILON);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// PUBLIC API
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

/// A convenience function to calculate the implied volatility.
/// It is a wrapper around `implied_volatility` function.
pub fn iv(price: f64, S: f64, K: f64, T: f64, r: f64, flag: TypeFlag) -> f64 {
    implied_volatility(price, S, K, T, r, flag)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// INLINED FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[inline]
pub(crate) fn normalised_intrinsic_call(x: f64) -> f64 {
    normalised_intrinsic(x, 1.0)
}

#[inline]
pub(crate) fn is_below_horizon(x: f64) -> bool {
    x.abs() < DENORMALIZATION_CUTOFF
}

#[inline]
pub(crate) fn is_zero(x: f64) -> bool {
    x.abs() < f64::MIN_POSITIVE
}

#[inline]
pub(crate) fn inverse_f_upper_map(f: f64) -> f64 {
    -2.0 * Gaussian::default().inv_cdf(f)
}

#[inline]
pub(crate) fn householder_factor(newton: f64, halley: f64, hh3: f64) -> f64 {
    (1.0 + 0.5 * halley * newton) / (1.0 + newton * (halley + hh3 * newton / 6.0))
}

#[inline]
#[rustfmt::skip]
fn small_t_expansion_sum(t: f64, a: f64, w: f64, h2: f64) -> f64 {
    2.0 * t * 
        (a + 
            w * ((-1.0+3.0*a+a*h2)/6.0+
            w * ((-7.0+15.0*a+h2*(-1.0+10.0*a+a*h2))/120.0+
            w * ((-57.0+105.0*a+h2*(-18.0+105.0*a+h2*(-1.0+21.0*a+a*h2)))/5040.0+
            w * ((-561.0+945.0*a+h2*(-285.0+1260.0*a+h2*(-33.0+378.0*a+h2*(-1.0+36.0*a+a*h2))))/362_880.0+
            w * ((-6555.0+10395.0*a+
                h2*(-4680.0+17325.0*a+
                    h2*(-840.0+6930.0*a+
                        h2*(-52.0+990.0*a+
                            h2*(-1.0+55.0*a+a*
                                h2))))) / 39_916_800.0+((-89055.0+135_135.0*a+
                                    h2*(-82845.0+270_270.0*a+
                                        h2*(-20370.0+135_135.0*a+
                                            h2*(-1926.0+25740.0*a+
                                                h2*(-75.0+2145.0*a+
                                                    h2*(-1.0+78.0*a+a*
                                                        h2)))))) 
            * w) / 6_227_020_800.0 ))))))
}

#[inline]
#[rustfmt::skip]
fn asymptotic_expansion_sum(e: f64, q: f64) -> f64 {
    2.0 +
        q * (-6.0e0-2.0 * e + 3.0 *
        q * (1.0e1+e*(2.0e1+2.0*e) + 5.0 * 
        q * (-1.4e1+ e * (-7.0e1+ e * (-4.2e1-2.0 * e))+ 7.0 * 
        q * (1.8e1+e*(1.68e2+e*(2.52e2+e*(7.2e1+2.0*e)))+9.0 * 
        q * (-2.2e1+e*(-3.3e2+e*(-9.24e2+e*(-6.6e2+e*(-1.1e2-2.0*e))))+1.1e1*
        q * (2.6e1+e*(5.72e2+e*(2.574e3+e*(3.432e3+e*(1.43e3+e*(1.56e2+2.0*e)))))+1.3e1*
        q * (-3.0e1+e*(-9.1e2+e*(-6.006e3+e*(-1.287e4+e*(-1.001e4+e*(-2.73e3+e*(-2.1e2-2.0*e))))))+1.5e1*
        q * (3.4e1+e*(1.36e3+e*(1.2376e4+e*(3.8896e4+e*(4.862e4+e*(2.4752e4+e*(4.76e3+e*(2.72e2+2.0*e)))))))+1.7e1*
        q * (-3.8e1+e*(-1.938e3+e*(-2.3256e4+e*(-1.00776e5+e*(-1.84756e5+e*(-1.51164e5+e*(-5.4264e4+e*(-7.752e3+e*(-3.42e2-2.0*e))))))))+1.9e1*
        q * (4.2e1+e*(2.66e3+e*(4.0698e4+e*(2.3256e5+e*(5.8786e5+e*(7.05432e5+e*(4.0698e5+e*(1.08528e5+e*(1.197e4+e*(4.2e2+2.0*e)))))))))+2.1e1*
        q * (-4.6e1+e*(-3.542e3+e*(-6.7298e4+e*(-4.90314e5+e*(-1.63438e6+e*(-2.704_156e6+e*(-2.288_132e6+e*(-9.80628e5+e*(-2.01894e5+e*(-1.771e4+e*(-5.06e2-2.0*e))))))))))+2.3e1*
        q * (5.0e1+e*(4.6e3+e*(1.0626e5+e*(9.614e5+e*(4.08595e6+e*(8.9148e6+e*(1.04006e7+e*(6.53752e6+e*(2.16315e6+e*(3.542e5+e*(2.53e4+e*(6.0e2+2.0*e)))))))))))+2.5e1*
        q * (-5.4e1+e*(-5.85e3+e*(-1.6146e5+e*(-1.77606e6+e*(-9.37365e6+e*(-2.607_579e7+e*(-4.01166e7+e*(-3.476_772e7+e*(-1.687_257e7+e*(-4.44015e6+e*(-5.9202e5+e*(-3.51e4+e*(-7.02e2-2.0*e))))))))))))+2.7e1*
        q * (5.8e1+e*(7.308e3+e*(2.3751e5+e*(3.12156e6+e*(2.003_001e7+e*(6.919_458e7+e*(1.357_278_3e8+e*(1.551_175_2e8+e*(1.037_918_7e8+e*(4.006_002e7+e*(8.58429e6+e*(9.5004e5+e*(4.7502e4+e*(8.12e2+2.0*e)))))))))))))+2.9e1*
        q * (-6.2e1+e*(-8.99e3+e*(-3.39822e5+e*(-5.25915e6+e*(-4.032_015e7+e*(-1.693_446_3e8+e*(-4.125_061_5e8+e*(-6.010_803_9e8+e*(-5.303_650_5e8+e*(-2.822_410_5e8+e*(-8.870_433e7+e*(-1.577_745e7+e*(-1.472_562e6+e*(-6.293e4+e*(-9.3e2-2.0*e))))))))))))))+3.1e1*
        q * (6.6e1+e*(1.0912e4+e*(4.74672e5+e*(8.544_096e6+e*(7.71342e7+e*(3.870_734_4e8+e*(1.146_332_88e9+e*(2.074_316_64e9+e*(2.333_606_22e9+e*(1.637_618_4e9+e*(7.096_346_4e8+e*(1.851_220_8e8+e*(2.776_831_2e7+e*(2.215_136e6+e*(8.184e4+e*(1.056e3+2.0*e
        )))))))))))))))
        + 3.3e1*(
            -7.0e1+e*(
                -1.309e4+e*(
                    -6.49264e5+e*(
                        -1.344_904e7+e*(
                            -1.412_149_2e8+e*(
                                -8.344_518e8+e*(
                                    -2.952_675_6e9+e*(
                                        -6.495_886_32e9+e*(
                                            -9.075_135_3e9+e*(
                                                -8.119_857_9e9+e*(
                                                    -4.639_918_8e9+e*(
                                                        -1.668_903_6e9+e*(
                                                            -3.671_587_92e8+e*(
                                                                -4.707_164e7+e*(
                                                                    -3.24632e6+e*(
                                                                        -1.0472e5+e*(
                                                                            -1.19e3-2.0*e
        ))))))))))))))))) * q ))))))))))))))))
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) fn rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_l: f64,
) -> f64 {
    let h = x_r - x_l;
    let numerator = 0.5 * h * second_derivative_l + (d_r - d_l);
    let denominator = (y_r - y_l) / h - d_l;

    if is_zero(numerator) {
        return 0.0;
    }

    if is_zero(denominator) {
        return if numerator > 0.0 {
            MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        } else {
            MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE
        };
    }

    numerator / denominator
}

pub(crate) fn minimum_rational_cubic_control_parameter(
    d_l: f64,
    d_r: f64,
    s: f64,
    prefer_shape_preservation_over_smoothness: bool,
) -> f64 {
    let monotonic = d_l * s >= 0.0 && d_r * s >= 0.0;
    let convex = d_l <= s && s <= d_r;
    let concave = d_l >= s && s >= d_r;
    if !monotonic && !convex && !concave {
        //If 3==r_non_shape_preserving_target, this means revert to standard cubic.
        return MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }
    let d_r_m_d_l = d_r - d_l;
    let d_r_m_s = d_r - s;
    let s_m_d_l = s - d_l;
    let mut r1 = f64::MIN;
    let mut r2 = r1;
    // If monotonicity on this interval is possible, set r1 to satisfy the monotonicity condition (3.8).
    if monotonic {
        // (3.8), avoiding division by zero.
        if !is_zero(s) {
            // # (3.8)
            r1 = (d_r + d_l) / s;
        } else if prefer_shape_preservation_over_smoothness {
            // If division by zero would occur, and shape preservation is preferred, set value to enforce linear interpolation.
            // This value enforces linear interpolation.
            r1 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
        }
    }
    if convex || concave {
        // (3.18), avoiding division by zero
        if !is_zero(s_m_d_l) || is_zero(d_r_m_s) {
            r2 = (d_r_m_d_l / d_r_m_s).abs().max((d_r_m_d_l / s_m_d_l).abs());
        } else if prefer_shape_preservation_over_smoothness {
            // This value enforces linear interpolation.
            r2 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
        }
    } else if monotonic && prefer_shape_preservation_over_smoothness {
        // This enforces linear interpolation along segments that are inconsistent with the slopes on the boundaries, e.g., a perfectly horizontal segment that has negative slopes on either edge.
        r2 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }

    MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE.max(r1.max(r2))
}

pub(crate) fn rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_r: f64,
) -> f64 {
    let h = x_r - x_l;
    let numerator = 0.5 * h * second_derivative_r + (d_r - d_l);
    let denominator = d_r - (y_r - y_l) / h;

    if is_zero(numerator) {
        return 0.0;
    }

    if is_zero(denominator) {
        return MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }

    numerator / denominator
}

#[allow(clippy::too_many_arguments)]
fn interpolation(
    x: f64,
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    r: f64,
) -> f64 {
    let h = x_r - x_l;
    if h.abs() <= 0.0 {
        return 0.5 * (y_l + y_r);
    }
    // r should be greater than -1. We do not use  assert(r > -1)  here in order to allow values such as NaN to be propagated as they should.
    let t = (x - x_l) / h;
    if r < MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE {
        let omt = 1.0 - t;
        let t2 = t * t;
        let omt2 = omt * omt;
        // Formula (2.4) divided by formula (2.5)
        return (y_r * t2 * t
            + (r * y_r - h * d_r) * t2 * omt
            + (r * y_l + h * d_l) * t * omt2
            + y_l * omt2 * omt)
            / (1.0 + (r - 3.0) * t * omt);
    }

    // Linear interpolation without over-or underflow.
    y_r * t + y_l * (1.0 - t)
}

#[allow(clippy::too_many_arguments)]
fn convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_l: f64,
    prefer_shape_preservation_over_smoothness: bool,
) -> f64 {
    let r = rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
        x_l,
        x_r,
        y_l,
        y_r,
        d_l,
        d_r,
        second_derivative_l,
    );

    let r_min = minimum_rational_cubic_control_parameter(
        d_l,
        d_r,
        (y_r - y_l) / (x_r - x_l),
        prefer_shape_preservation_over_smoothness,
    );

    f64::max(r, r_min)
}

#[allow(clippy::too_many_arguments)]
fn convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_r: f64,
    prefer_shape_preservation_over_smoothness: bool,
) -> f64 {
    let r = rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
        x_l,
        x_r,
        y_l,
        y_r,
        d_l,
        d_r,
        second_derivative_r,
    );

    let r_min = minimum_rational_cubic_control_parameter(
        d_l,
        d_r,
        (y_r - y_l) / (x_r - x_l),
        prefer_shape_preservation_over_smoothness,
    );

    f64::max(r, r_min)
}

pub(crate) fn compute_f_lower_map_and_first_two_derivatives(x: f64, s: f64) -> (f64, f64, f64) {
    let ax = x.abs();
    let z = SQRT_ONE_OVER_THREE * ax / s;
    let y = z * z;
    let s2 = s * s;
    let N = Gaussian::default();
    let PHI = N.cdf(-z);
    let phi = N.pdf(z);

    let fpp = std::f64::consts::FRAC_PI_6 * y / (s2 * s)
        * PHI
        * (8.0 * SQRT_THREE * s * ax + (3.0 * s2 * (s2 - 8.0) - 8.0 * x * x) * PHI / phi)
        * (2.0 * y + 0.25 * s2).exp();

    let mut fp = 1.0;

    let mut f = 0.0;

    if !is_below_horizon(s) {
        let PHI2 = PHI * PHI;
        fp = std::f64::consts::TAU * y * PHI2 * (y + 0.125 * s * s).exp();
        if !is_below_horizon(x) {
            f = TWO_PI_OVER_SQRT_TWENTY_SEVEN * ax * (PHI2 * PHI);
        }
    }

    (f, fp, fpp)
}

pub(crate) fn compute_f_upper_map_and_first_two_derivatives(x: f64, s: f64) -> (f64, f64, f64) {
    let N = Gaussian::default();

    let f = N.cdf(-0.5 * s);
    let mut fp = -0.5;
    let mut fpp = 0.0;

    if !is_below_horizon(x) {
        let w = f64::powi(x / s, 2);
        fp = -0.5 * (0.5 * w).exp();
        fpp = SQRT_PI_OVER_TWO * (w + 0.125 * s * s).exp() * (w / s);
    }

    (f, fp, fpp)
}

pub(crate) fn inverse_f_lower_map(x: f64, f: f64) -> f64 {
    if is_below_horizon(f) {
        return 0.0;
    }

    let N = Gaussian::default();

    (x / (SQRT_THREE * N.inv_cdf((f / (TWO_PI_OVER_SQRT_TWENTY_SEVEN * x.abs())).powf(1.0 / 3.0))))
        .abs()
}

pub(crate) fn normalized_black_call_using_norm_cdf(x: f64, s: f64) -> f64 {
    // b(x,s)   =  Φ(x/s+s/2)·exp(x/2)  -   Φ(x/s-s/2)·exp(-x/2)
    //          =  Φ(h+t)·exp(x/2)      -   Φ(h-t)·exp(-x/2)
    // with
    // h  =  x/s
    // and
    // t  =  s/2

    let h = x / s;
    let t = 0.5 * s;
    let b_max = (0.5 * x).exp();
    let N = Gaussian::default();
    let b = N.cdf(h + t) * b_max - N.cdf(h - t) / b_max;

    f64::max(b, 0.0)
}

pub(crate) fn asymptotic_expansion_of_normalized_black_call(h: f64, t: f64) -> f64 {
    //     Asymptotic expansion of
    //
    //              b  =  Φ(h+t)·exp(x/2) - Φ(h-t)·exp(-x/2)
    // with
    //              h  =  x/s   and   t  =  s/2
    // which makes
    //              b  =  Φ(h+t)·exp(h·t) - Φ(h-t)·exp(-h·t)
    //
    //                    exp(-(h²+t²)/2)
    //                 =  ---------------  ·  [ Y(h+t) - Y(h-t) ]
    //                        √(2π)
    // with
    //           Y(z) := Φ(z)/φ(z)
    //
    // for large negative (t-|h|) by the aid of Abramowitz & Stegun (26.2.12) where Φ(z) = φ(z)/|z|·[1-1/z^2+...].
    // We define
    //                     r
    //         A(h,t) :=  --- · [ Y(h+t) - Y(h-t) ]
    //                     t
    //
    // with r := (h+t)·(h-t) and give an expansion for A(h,t) in q:=(h/r)² expressed in terms of e:=(t/h)² .

    let e = (t / h) * (t / h);
    let r = (h + t) * (h - t);
    let q = (h / r) * (h / r);

    let b = ONE_OVER_SQRT_TWO_PI
        * f64::exp(-0.5 * (h * h + t * t))
        * (t / r)
        * asymptotic_expansion_sum(e, q);

    b.max(0.0)
}

pub(crate) fn small_t_expansion_of_normalized_black_call(h: f64, t: f64) -> f64 {
    // Calculation of
    //
    //              b  =  Φ(h+t)·exp(h·t) - Φ(h-t)·exp(-h·t)
    //
    //                    exp(-(h²+t²)/2)
    //                 =  --------------- ·  [ Y(h+t) - Y(h-t) ]
    //                        √(2π)
    // with
    //           Y(z) := Φ(z)/φ(z)
    //
    // using an expansion of Y(h+t)-Y(h-t) for small t to twelvth order in t.
    // Theoretically accurate to (better than) precision  ε = 2.23E-16  when  h<=0  and  t < τ  with  τ := 2·ε^(1/16) ≈ 0.21.
    // The main bottleneck for precision is the coefficient a:=1+h·Y(h) when |h|>1 .
    //
    // Y(h) := Φ(h)/φ(h) = √(π/2)·erfcx(-h/√2)
    // a := 1+h·Y(h)  --- Note that due to h<0, and h·Y(h) -> -1 (from above) as h -> -∞, we also have that a>0 and a -> 0 as h -> -∞
    // w := t² , h2 := h²

    let a = 1.0 + h * (0.5 * SQRT_TWO_PI) * (-std::f64::consts::FRAC_1_SQRT_2 * h).erfcx();
    let w = t * t;
    let h2 = h * h;

    let expansion = small_t_expansion_sum(t, a, w, h2);
    let b = ONE_OVER_SQRT_TWO_PI * (-0.5 * (h * h + t * t)).exp() * expansion;
    b.max(0.0)
}

pub(crate) fn normalised_black_call_using_erfcx(h: f64, t: f64) -> f64 {
    // Given h = x/s and t = s/2, the normalised Black function can be written as
    //
    //     b(x,s)  =  Φ(x/s+s/2)·exp(x/2)  -   Φ(x/s-s/2)·exp(-x/2)
    //             =  Φ(h+t)·exp(h·t)      -   Φ(h-t)·exp(-h·t) .                     (*)
    //
    // It is mentioned in section 4 (and discussion of figures 2 and 3) of George Marsaglia's article "Evaluating the
    // Normal Distribution" (available at http:#www.jstatsoft.org/v11/a05/paper) that the error of any cumulative normal
    // function Φ(z) is dominated by the hardware (or compiler implementation) accuracy of exp(-z²/2) which is not
    // reliably more than 14 digits when z is large. The accuracy of Φ(z) typically starts coming down to 14 digits when
    // z is around -8. For the (normalised) Black function, as above in (*), this means that we are subtracting two terms
    // that are each products of terms with about 14 digits of accuracy. The net result, in each of the products, is even
    // less accuracy, and then we are taking the difference of these terms, resulting in even less accuracy. When we are
    // using the asymptotic expansion asymptotic_expansion_of_normalized_black_call() invoked in the second branch at the
    // beginning of this function, we are using only *one* exponential instead of 4, and this improves accuracy. It
    // actually improves it a bit more than you would expect from the above logic, namely, almost the full two missing
    // digits (in 64 bit IEEE floating point).  Unfortunately, going higher order in the asymptotic expansion will not
    // enable us to gain more accuracy (by extending the range in which we could use the expansion) since the asymptotic
    // expansion, being a divergent series, can never gain 16 digits of accuracy for z=-8 or just below. The best you can
    // get is about 15 digits (just), for about 35 terms in the series (26.2.12), which would result in an prohibitively
    // long expression in function asymptotic expansion asymptotic_expansion_of_normalized_black_call(). In this last branch,
    // here, we therefore take a different tack as follows.
    //     The "scaled complementary error function" is defined as erfcx(z) = exp(z²)·erfc(z). Cody's implementation of this
    // function as published in "Rational Chebyshev approximations for the error function", W. J. Cody, Math. Comp., 1969, pp.
    // 631-638, uses rational functions that theoretically approximates erfcx(x) to at least 18 significant decimal digits,
    // *without* the use of the exponential function when x>4, which translates to about z<-5.66 in Φ(z). To make use of it,
    // we write
    //             Φ(z) = exp(-z²/2)·erfcx(-z/√2)/2
    //
    // to transform the normalised black function to
    //
    //   b   =  ½ · exp(-½(h²+t²)) · [ erfcx(-(h+t)/√2) -  erfcx(-(h-t)/√2) ]
    //
    // which now involves only one exponential, instead of three, when |h|+|t| > 5.66 , and the difference inside the
    // square bracket is between the evaluation of two rational functions, which, typically, according to Marsaglia,
    // retains the full 16 digits of accuracy (or just a little less than that).

    let b = 0.5
        * (-0.5 * (h * h + t * t)).exp()
        * ((-std::f64::consts::FRAC_1_SQRT_2 * (h + t)).erfcx()
            - (-std::f64::consts::FRAC_1_SQRT_2 * (h - t)).erfcx());

    f64::max(b, 0.0)
}

pub(crate) fn normalised_intrinsic(x: f64, q: f64) -> f64 {
    if q * x <= 0.0 {
        return 0.0;
    }

    let x2 = x * x;

    // The factor 98 is computed from last coefficient: √√92897280 = 98.1749
    if x2 < 98.0 * FOURTH_ROOT_DBL_EPSILON {
        let mut ret = x
            * (1.0
                + x2 * ((1.0 / 24.0)
                    + x2 * ((1.0 / 1920.0)
                        + x2 * ((1.0 / 322_560.0) + (1.0 / 92_897_280.0) * x2))));
        if q < 0.0 {
            ret = -ret;
        }
        return ret.max(0.0);
    }

    let b_max = (0.5 * x).exp();
    let one_over_b_max = 1.0 / b_max;
    let mut ret = b_max - one_over_b_max;

    if q < 0.0 {
        ret = -ret;
    }

    f64::max(ret, 0.0)
}

pub(crate) fn normalised_black_call(x: f64, s: f64) -> f64 {
    if x > 0.0 {
        return normalised_intrinsic_call(x) + normalised_black_call(-x, s);
    }

    let ax = x.abs();

    if s <= ax * DENORMALIZATION_CUTOFF {
        return normalised_intrinsic_call(x);
    }

    // Denote h := x/s and t := s/2. We evaluate the condition |h|>|η|, i.e., h<η  &&  t < τ+|h|-|η|  avoiding any
    // divisions by s , where η = asymptotic_expansion_accuracy_threshold  and τ =
    // small_t_expansion_of_normalized_black_threshold .
    if x < s * ASYMPTOTIC_EXPANSION_ACCURACY_THRESHOLD
        && (0.5 * s * s + x)
            < s * (SMALL_T_EXPANSION_OF_NORMALIZED_BLACK_THRESHOLD
                + ASYMPTOTIC_EXPANSION_ACCURACY_THRESHOLD)
    {
        // Region 1.
        return asymptotic_expansion_of_normalized_black_call(x / s, 0.5 * s);
    }

    if 0.5 * s < SMALL_T_EXPANSION_OF_NORMALIZED_BLACK_THRESHOLD {
        // Region 2.
        return small_t_expansion_of_normalized_black_call(x / s, 0.5 * s);
    }

    // When b is more than, say, about 85% of b_max=exp(x/2), then b is dominated by the first of the two terms in the
    // Black formula, and we retain more accuracy by not attempting to combine the two terms in any way. We evaluate
    // the condition h+t>0.85  avoiding any divisions by s.
    if (x + 0.5 * s * s) > (s * 0.85) {
        // Region 3.
        return normalized_black_call_using_norm_cdf(x, s);
    }

    // Region 4.
    normalised_black_call_using_erfcx(x / s, 0.5 * s)
}

pub(crate) fn normalised_vega(x: f64, s: f64) -> f64 {
    let ax = x.abs();

    if ax <= 0.0 {
        return ONE_OVER_SQRT_TWO_PI * (-0.125 * s * s).exp();
    }

    if s <= 0.0 || s <= ax * SQRT_DBL_MIN {
        return 0.0;
    }

    ONE_OVER_SQRT_TWO_PI * f64::exp(-0.5 * (f64::powi(x / s, 2) + f64::powi(0.5 * s, 2)))
}

#[allow(clippy::too_many_lines)]
pub(crate) fn unchecked_normalised_implied_volatility_from_a_transformed_rational_guess_with_limited_iterations(
    mut beta: f64,
    mut x: f64,
    mut q: f64,
    N: usize,
) -> f64 {
    // See http://en.wikipedia.org/wiki/Householder%27s_method for a detailed explanation of the third order Householder iteration.
    //
    // Given the objective function g(s) whose root x such that 0 = g(s) we seek, iterate
    //
    //     s_n+1  =  s_n  -  (g/g') · [ 1 - (g''/g')·(g/g') ] / [ 1 - (g/g')·( (g''/g') - (g'''/g')·(g/g')/6 ) ]
    //
    // Denoting  newton:=-(g/g'), halley:=(g''/g'), and hh3:=(g'''/g'), this reads
    //
    //     s_n+1  =  s_n  +  newton · [ 1 + halley·newton/2 ] / [ 1 + newton·( halley + hh3·newton/6 ) ]
    //
    //
    // NOTE that this function returns 0 when beta<intrinsic without any safety checks.

    // Subtract intrinsic.
    if q * x > 0.0 {
        // we allow beta to be under the instrinisc value to then return -INF
        beta -= normalised_intrinsic(x, q);
        q = -q;
    }

    // Map puts to calls
    if q < 0.0 {
        x = -x;
        // after this, we do not use q anymore
    }

    // For negative prices we return -INF
    if beta < 0.0 {
        return f64::NEG_INFINITY;
    }

    // For positive or zero but denormalized (a.k.a. 'subnormal') prices, we return 0 since it would be impossible to converge to full machine accuracy anyway.
    if beta <= DENORMALIZATION_CUTOFF {
        return 0.0;
    }

    let b_max = (0.5 * x).exp();

    if beta >= b_max {
        return f64::INFINITY;
    }

    let mut iterations = 0;
    let mut direction_reversal_count = 0;
    let mut f = f64::MIN;
    let mut s = f64::MIN;
    let mut ds = s;
    let mut ds_previous = 0.0;
    let mut s_left = f64::MIN_POSITIVE;
    let mut s_right = f64::MAX;

    // The temptation is great to use the optimised form
    // b_c = exp(x/2)/2-exp(-x/2)·Phi(sqrt(-2·x))
    // but that would require implementing all of the above types of
    // round-off and over/underflow handling for this expression, too.
    let s_c = (2.0 * x).abs().sqrt();
    let b_c = normalised_black_call(x, s_c);
    let v_c = normalised_vega(x, s_c);

    // Four branches.
    if beta < b_c {
        let s_l = s_c - b_c / v_c;
        let b_l = normalised_black_call(x, s_l);

        if beta < b_l {
            let (f_lower_map_l, d_f_lower_map_l_d_beta, d2_f_lower_map_l_d_beta2) =
                compute_f_lower_map_and_first_two_derivatives(x, s_l);
            let r_ll =
                convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
                    0.,
                    b_l,
                    0.,
                    f_lower_map_l,
                    1.,
                    d_f_lower_map_l_d_beta,
                    d2_f_lower_map_l_d_beta2,
                    true,
                );
            f = interpolation(
                beta,
                0.,
                b_l,
                0.,
                f_lower_map_l,
                1.,
                d_f_lower_map_l_d_beta,
                r_ll,
            );

            // This can happen due to roundoff truncation for extreme values such as |x|>500.
            if f <= 0.0 {
                // We switch to quadratic interpolation using f(0)≡0, f(b_l), and f'(0)≡1 to specify the quadratic.
                let t = beta / b_l;
                f = (f_lower_map_l * t + b_l * (1.0 - t)) * t;
            }

            s = inverse_f_lower_map(x, f);
            s_right = s_l;

            // In this branch, which comprises the lowest segment, the objective function is
            //     g(s) = 1/ln(b(x,s)) - 1/ln(beta)
            //        ≡ 1/ln(b(s)) - 1/ln(beta)
            // This makes
            //              g'               =   -b'/(b·ln(b)²)
            //              newton = -g/g'   =   (ln(beta)-ln(b))·ln(b)/ln(beta)·b/b'
            //              halley = g''/g'  =   b''/b'  -  b'/b·(1+2/ln(b))
            //              hh3    = g'''/g' =   b'''/b' +  2(b'/b)²·(1+3/ln(b)·(1+1/ln(b)))  -  3(b''/b)·(1+2/ln(b))
            //
            // The Householder(3) iteration is
            //     s_n+1  =  s_n  +  newton · [ 1 + halley·newton/2 ] / [ 1 + newton·( halley + hh3·newton/6 ) ]

            while iterations < N && ds.abs() > f64::EPSILON * s {
                if ds * ds_previous < 0.0 {
                    direction_reversal_count += 1;
                }
                if iterations > 0 && (3 == direction_reversal_count || !(s > s_left && s < s_right))
                {
                    // If looping inefficently, or the forecast step takes us outside the bracket, or onto its edges, switch to binary nesting.
                    // NOTE that this can only really happen for very extreme values of |x|, such as |x| = |ln(F/K)| > 500.
                    s = 0.5 * (s_left + s_right);
                    if (s_right - s_left) <= f64::EPSILON * s {
                        break;
                    }
                    direction_reversal_count = 0;
                    ds = 0.0;
                }
                ds_previous = ds;
                let b = normalised_black_call(x, s);
                let bp = normalised_vega(x, s);
                if b > beta && s < s_right {
                    s_right = s;
                } else if b < beta && s > s_left {
                    s_left = s; // Tighten the bracket if applicable.
                }
                if b <= 0.0 || bp <= 0.0 {
                    //Numerical underflow. Switch to binary nesting for this iteration.
                    ds = 0.5 * (s_left + s_right) - s;
                } else {
                    let ln_b = b.ln();
                    let ln_beta = beta.ln();
                    let bpob = bp / b;
                    let h = x / s;
                    let b_halley = h * h / s - s / 4.0;
                    let newton = (ln_beta - ln_b) * ln_b / ln_beta / bpob;
                    let halley = b_halley - bpob * (1.0 + 2.0 / ln_b);
                    let b_hh3 = b_halley * b_halley - 3.0 * f64::powi(h / s, 2) - 0.25;
                    let hh3 = b_hh3
                        + 2.0 * f64::powi(bpob, 2) * (1.0 + 3.0 / ln_b * (1.0 + 1.0 / ln_b))
                        - 3.0 * b_halley * bpob * (1.0 + 2.0 / ln_b);
                    ds = newton * householder_factor(newton, halley, hh3);
                }
                ds = ds.max(-0.5 * s);
                s += ds;
                iterations += 1;
            }
            return s;
        }

        let v_l = normalised_vega(x, s_l);
        let r_lm = convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
            b_l,
            b_c,
            s_l,
            s_c,
            1.0 / v_l,
            1.0 / v_c,
            0.0,
            false,
        );
        s = interpolation(beta, b_l, b_c, s_l, s_c, 1.0 / v_l, 1.0 / v_c, r_lm);
        s_left = s_l;
        s_right = s_c;
    } else {
        let mut s_u = s_c;
        if v_c > f64::EPSILON {
            s_u = s_c + (b_max - b_c) / v_c;
        }
        let b_u = normalised_black_call(x, s_u);
        if beta <= b_u {
            let v_u = normalised_vega(x, s_u);
            let r_hm =
                convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
                    b_c,
                    b_u,
                    s_c,
                    s_u,
                    1.0 / v_c,
                    1.0 / v_u,
                    0.0,
                    false,
                );
            s = interpolation(beta, b_c, b_u, s_c, s_u, 1.0 / v_c, 1.0 / v_u, r_hm);
            s_left = s_c;
            s_right = s_u;
        } else {
            let (f_upper_map_h, d_f_upper_map_h_d_beta, d2_f_upper_map_h_d_beta2) =
                compute_f_upper_map_and_first_two_derivatives(x, s_u);
            if d2_f_upper_map_h_d_beta2 > -SQRT_DBL_MAX && d2_f_upper_map_h_d_beta2 < SQRT_DBL_MAX {
                let r_hh =
                    convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
                        b_u,
                        b_max,
                        f_upper_map_h,
                        0.,
                        d_f_upper_map_h_d_beta,
                        -0.5,
                        d2_f_upper_map_h_d_beta2,
                        true,
                    );
                f = interpolation(
                    beta,
                    b_u,
                    b_max,
                    f_upper_map_h,
                    0.,
                    d_f_upper_map_h_d_beta,
                    -0.5,
                    r_hh,
                );
            }
            if f <= 0.0 {
                let h = b_max - b_u;
                let t = (beta - b_u) / h;
                // We switch to quadratic interpolation using f(b_h), f(b_max)≡0, and f'(b_max)≡-1/2 to specify the quadratic.
                f = (f_upper_map_h * (1.0 - t) + 0.5 * h * t) * (1.0 - t);
            }
            s = inverse_f_upper_map(f);
            s_left = s_u;
            // Else we better drop through and let the objective function be g(s) = b(x,s)-beta.
            if beta > 0.5 * b_max {
                // In this branch, which comprises the upper segment, the objective function is
                //     g(s) = ln(b_max-beta)-ln(b_max-b(x,s))
                //          ≡ ln((b_max-beta)/(b_max-b(s)))
                // This makes
                //              g'               =   b'/(b_max-b)
                //              newton = -g/g'   =   ln((b_max-b)/(b_max-beta))·(b_max-b)/b'
                //              halley = g''/g'  =   b''/b'  +  b'/(b_max-b)
                //              hh3    = g'''/g' =   b'''/b' +  g'·(2g'+3b''/b')
                // and the iteration is
                //     s_n+1  =  s_n  +  newton · [ 1 + halley·newton/2 ] / [ 1 + newton·( halley + hh3·newton/6 ) ].

                while iterations < N && ds.abs() > f64::EPSILON * s {
                    if ds * ds_previous < 0.0 {
                        direction_reversal_count += 1;
                    }
                    if iterations > 0
                        && (3 == direction_reversal_count || !(s > s_left && s < s_right))
                    {
                        // If looping inefficently, or the forecast step takes us outside the bracket, or onto its edges, switch to binary nesting.
                        // NOTE that this can only really happen for very extreme values of |x|, such as |x| = |ln(F/K)| > 500.
                        s = 0.5 * (s_left + s_right);
                        if (s_right - s_left) <= f64::EPSILON * s {
                            break;
                        }
                        direction_reversal_count = 0;
                        ds = 0.0;
                    }
                    ds_previous = ds;
                    let b = normalised_black_call(x, s);
                    let bp = normalised_vega(x, s);
                    if b > beta && s < s_right {
                        s_right = s;
                    } else if b < beta && s > s_left {
                        s_left = s; // Tighten the bracket if applicable.
                    }
                    if b >= b_max || bp <= f64::EPSILON {
                        // Numerical underflow. Switch to binary nesting for this iteration.
                        ds = 0.5 * (s_left + s_right) - s;
                    } else {
                        let b_max_minus_b = b_max - b;
                        let g = ((b_max - beta) / b_max_minus_b).ln();
                        let gp = bp / b_max_minus_b;
                        let b_halley = f64::powi(x / s, 2) / s - s / 4.0;
                        let b_hh3 = b_halley * b_halley - 3.0 * f64::powi(x / (s * s), 2) - 0.25;
                        let newton = -g / gp;
                        let halley = b_halley + gp;
                        let hh3 = b_hh3 + gp * (2.0 * gp + 3.0 * b_halley);
                        ds = newton * householder_factor(newton, halley, hh3);
                    }
                    ds = ds.max(-0.5 * s);
                    s += ds;
                    iterations += 1;
                }
                return s;
            }
        }
    }
    // In this branch, whunchecked_normalised_implied_volatility_from_a_transformed_rational_guess_with_limited_iterationsich comprises the two middle segments, the objective function is g(s) = b(x,s)-beta, or g(s) = b(s) - beta, for short.
    // This makes
    //                 newton = -g/g'   =  -(b-beta)/b'
    //                 halley = g''/g'  =    b''/b'    =  x²/s³-s/4
    //                 hh3    = g'''/g' =    b'''/b'   =  halley² - 3·(x/s²)² - 1/4
    // and the iteration is
    //     s_n+1  =  s_n  +  newton · [ 1 + halley·newton/2 ] / [ 1 + newton·( halley + hh3·newton/6 ) ].
    while iterations < N && ds.abs() > f64::EPSILON * s {
        if ds * ds_previous < 0.0 {
            direction_reversal_count += 1;
        }
        if iterations > 0 && (3 == direction_reversal_count || !(s > s_left && s < s_right)) {
            // If looping inefficently, or the forecast step takes us outside the bracket, or onto its edges, switch to binary nesting.
            // NOTE that this can only really happen for very extreme values of |x|, such as |x| = |ln(F/K)| > 500.
            s = 0.5 * (s_left + s_right);
            if (s_right - s_left) <= f64::EPSILON * s {
                break;
            }
            direction_reversal_count = 0;
            ds = 0.0;
        }
        ds_previous = ds;
        let b = normalised_black_call(x, s);
        let bp = normalised_vega(x, s);
        if b > beta && s < s_right {
            s_right = s;
        } else if b < beta && s > s_left {
            s_left = s; // Tighten the bracket if applicable.
        }
        let newton = (beta - b) / bp;
        let halley = f64::powi(x / s, 2) - s / 4.0;
        let hh3 = halley * halley - 3.0 * f64::powi(x / (s * s), 2) - 0.25;

        ds = newton * householder_factor(newton, halley, hh3);
        ds = ds.max(-0.5 * s);
        s += ds;
        iterations += 1;
    }
    s
}

pub(crate) fn implied_volatility_from_a_transformed_rational_guess_with_limited_iterations(
    undiscounted_option_price: f64,
    F: f64,
    K: f64,
    T: f64,
    q: f64,
) -> f64 {
    let mut intrinsic = if q < 0.0 { K - F } else { F - K };

    intrinsic = intrinsic.max(0.0);

    if undiscounted_option_price < intrinsic {
        return f64::NEG_INFINITY;
    }

    let max_price = if q < 0.0 { K } else { F };

    if undiscounted_option_price >= max_price {
        return f64::INFINITY;
    }

    let x = (F / K).ln();
    let mut new_price = undiscounted_option_price;
    let mut new_q = q;

    if q * x > 0.0 {
        new_price = (undiscounted_option_price - intrinsic).max(0.0);
        new_q = -q;
    }

    f64::recip(T.sqrt()) * unchecked_normalised_implied_volatility_from_a_transformed_rational_guess_with_limited_iterations(
        new_price/(F.sqrt() * K.sqrt()),
        x,
        new_q,
        IMPLIED_VOLATILITY_MAXIMUM_ITERATIONS
    )
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_lets_be_rational {
    use super::*;
    use crate::BlackScholesMerton;
    use std::panic;
    use time::Duration;
    use RustQuant_time::today;
    use RustQuant_utils::assert_approx_equal;

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
            today() + Duration::days(days),
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
            today() + Duration::days(365),
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
        assert_approx_equal!(s, 0.04000000000000133, 1e-10);
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
