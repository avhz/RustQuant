// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::{AddAssign, Neg};

use num_traits::{NumOps, One, Zero};

/// A nonadditive zero trait
pub trait Initial: Sized {
    /// return zero
    fn zero() -> Self;
}

impl<T> Initial for T where T: Zero {
    fn zero() -> Self {
        Zero::zero()
    }
}

/// A variable which can be evaluated upon by elementary operations.
pub trait DiffOps:
    NumOps + One + Zero + AddAssign + Neg<Output = Self> + Sized + Clone + Copy + NumOps<f64>
{
    /// Compute the derivative asin of the variable.
    fn asin_diff(self) -> Self;
    /// Compute the error function of the variable.
    fn erf(self) -> Self;
    /// Compute the complementary error function of the variable.
    fn erfc(self) -> Self;
    /// Compute the absolute value of the variable.
    fn abs(self) -> Self;
    /// Compute the signum of the variable.
    fn signum(self) -> Self;
    /// Compute the reciprocal of the variable.
    fn recip(self) -> Self;
    /// Compute the square root of the variable.
    fn sqrt(self) -> Self;
    /// Compute the cube root of the variable.
    fn cbrt(self) -> Self;
    /// Compute the integer power of the variable.
    fn powi(self, n: i32) -> Self;
    /// Compute the real power of the variable.
    fn powf(self, other: Self) -> Self;
    /// Compute the exponential of the variable.
    fn exp(self) -> Self;
    /// Compute the natural logarithm of the variable.
    fn ln(self) -> Self;
    /// Compute the natural logarithm of 1 + the variable.
    fn ln_1p(self) -> Self;
    /// Compute the exponential of the variable minus 1.
    fn exp_m1(self) -> Self;
    /// Compute the base 2 exponential of the variable.
    fn exp2(self) -> Self;
    /// Compute the binary logarithm of the variable.
    fn log2(self) -> Self;
    /// Compute the decimal logarithm of the variable.
    fn log10(self) -> Self;
    /// Compute the sine of the variable.
    fn sin(self) -> Self;
    /// Compute the cosine of the variable.
    fn cos(self) -> Self;
    /// Compute the tangent of the variable.
    fn tan(self) -> Self;
    /// Compute the arcsine of the variable.
    fn asin(self) -> Self;
    /// Compute the acosine of the variable.
    fn acos(self) -> Self;
    /// Compute the arctangent of the variable.
    fn atan(self) -> Self;
    /// Compute the sinh of the variable.
    fn sinh(self) -> Self;
    /// Compute the cosh of the variable.
    fn cosh(self) -> Self;
    /// Compute the tanh of the variable.
    fn tanh(self) -> Self;
    /// Compute the arcsinh of the variable.
    fn asinh(self) -> Self;
    /// Compute the arccosh of the variable.
    fn acosh(self) -> Self;
    /// Compute the atanh of the variable.
    fn atanh(self) -> Self;
}
