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
