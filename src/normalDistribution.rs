use statrs::function::erf;
use std::f64::consts::{PI, SQRT_2};

pub fn normalCDF(x: f64) -> f64 {
    0.5 + 0.5 * erf::erf(x / SQRT_2)
}
