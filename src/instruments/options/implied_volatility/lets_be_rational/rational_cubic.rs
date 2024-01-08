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
//
//! “Shape preserving piecewise rational interpolation”, R. Delbourgo, J.A. Gregory - SIAM journal on scientific and
//! statistical computing, 1985 - SIAM. http://dspace.brunel.ac.uk/bitstream/2438/2200/1/TR_10_83.pdf  [caveat emptor:
//! there are some typographical errors in that draft version]

//-(1.0 - f64::EPSILON.sqrt());
// cannot call non-const fn `std::f64::<impl f64>::sqrt` in constants
const MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE: f64 = 1.490_116_119_384_765_6e-8; 
const MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE: f64 = 2.0/(f64::EPSILON*f64::EPSILON);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[inline(always)]
fn is_zero(x: f64) -> bool {
    x.abs() < f64::MIN_POSITIVE
}


fn rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
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
    if is_zero(numerator) {
        return 0.0;
    }
    let denominator =  (y_r - y_l) / h - d_l;
    if is_zero(denominator) {
        if numerator > 0.0 {
            return MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
        }
        return MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }
    numerator / denominator
    
}

fn minimum_rational_cubic_control_parameter(
    d_l: f64,
    d_r: f64,
    s: f64,
    preferShapePreservationOverSmoothness: bool,
) -> f64 {
    let monotonic = d_l * s >= 0.0 && d_r * s >= 0.0;
    let convex = d_l <= s  && s <= d_r;
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
            r1 = (d_r + d_l)/s;
        }
        else {
            // If division by zero would occur, and shape preservation is preferred, set value to enforce linear interpolation.
            if preferShapePreservationOverSmoothness {
                // This value enforces linear interpolation.
                r1 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
            }
        }
    }
    if convex || concave {
        // (3.18), avoiding division by zero
        if !(is_zero(s_m_d_l) || is_zero(d_r_m_s)) {
            r2 = (d_r_m_d_l / d_r_m_s).abs().max((d_r_m_d_l / s_m_d_l).abs());
        } else if preferShapePreservationOverSmoothness {
            // This value enforces linear interpolation.
            r2 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
        }
    } else if monotonic && preferShapePreservationOverSmoothness{
        // This enforces linear interpolation along segments that are inconsistent with the slopes on the boundaries, e.g., a perfectly horizontal segment that has negative slopes on either edge.
        r2 = MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }
    MINIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE.max(r1.max(r2))
}


fn rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_r: f64
) -> f64 {
    let h = x_r - x_l;
    let numerator = 0.5 * h * second_derivative_r + (d_r - d_l);
    if is_zero(numerator) {
        return 0.0;
    }
    let denominator = d_r - (y_r - y_l) / h;
    if is_zero(denominator) {
        return MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE;
    }
    numerator / denominator
}


pub fn convex_rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_r: f64,
    preferShapePreservationOverSmoothness: bool,
) -> f64 {
    let r = rational_cubic_control_parameter_to_fit_second_derivative_at_right_side(
        x_l,
        x_r,
        y_l,
        y_r,
        d_l,
        d_r,
        second_derivative_r
    );
    let r_min = minimum_rational_cubic_control_parameter(
        d_l,
        d_r,
        (y_r - y_l) / (x_r - x_l),
        preferShapePreservationOverSmoothness
    );
    r.max(r_min)
}


pub fn rational_cubic_interpolation(
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
    if is_zero(h) {
        return 0.5 * (y_l + y_r);
    } 
    // r should be greater than -1. We do not use  assert(r > -1)  here in order to allow values such as NaN to be propagated as they should.
    let t = (x - x_l) / h;
    if r < MAXIMUM_RATIONAL_CUBIC_CONTROL_PARAMETER_VALUE {
        let omt = 1.0 - t;
        let t2 = t*t;
        let omt2 = omt * omt;
        // Formula (2.4) divided by formula (2.5)
        return (y_r * t2 * t + (r * y_r - h * d_r) * t2 * omt + (r * y_l + h * d_l) * t * omt2 + y_l * omt2 * omt) / (1.0 + (r - 3.0) * t * omt);
    }
    // Linear interpolation without over-or underflow.
    y_r * t + y_l * (1.0 - t)
}

pub fn convex_rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
    x_l: f64,
    x_r: f64,
    y_l: f64,
    y_r: f64,
    d_l: f64,
    d_r: f64,
    second_derivative_l: f64,
    preferShapePreservationOverSmoothness: bool,
) -> f64 {
    let r = rational_cubic_control_parameter_to_fit_second_derivative_at_left_side(
        x_l,
        x_r,
        y_l,
        y_r,
        d_l,
        d_r,
        second_derivative_l
    );
    let r_min = minimum_rational_cubic_control_parameter(
        d_l,
        d_r,
        (y_r - y_l) / (x_r - x_l),
        preferShapePreservationOverSmoothness
    );
    r.max(r_min)

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[cfg(test)]
mod test_rational_cubic {
    use super::*;
    #[test]
    fn test_is_zero() {
        assert!(is_zero(0.0));
        assert!(is_zero(f64::MIN_POSITIVE/2.0));

        assert!(!is_zero(f64::MIN_POSITIVE));
        assert!(!is_zero(f64::MIN_POSITIVE*2.0));
        assert!(!is_zero(-0.1));
        assert!(!is_zero(0.1));
    }
}
