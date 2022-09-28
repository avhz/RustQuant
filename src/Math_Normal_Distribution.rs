#![allow(non_snake_case)]

use statrs::function::erf;
use std::f64::consts::SQRT_2; // PI

/// Standard Normal Distribution Function
///
/// We use `erfc` (complementary error function) instead of `erf` to avoid
/// subtractive cancellation that leads to inaccuracy in the tails.
pub fn pnorm(x: f64) -> f64 {
    0.5 * erf::erfc(-x / SQRT_2)
}
// pub fn pnorm(x: f64) -> f64 {
//     0.5 + 0.5 * erf::erf(x / SQRT_2)
// }
