#![allow(non_snake_case)]

use statrs::function::erf;
use std::f64::consts::SQRT_2; // PI

pub fn pnorm(x: f64) -> f64 {
    0.5 + 0.5 * erf::erf(x / SQRT_2)
}
