// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Interpolation methods.
//! This module will be used to construct term structures
//! like yield curves and volatility surfaces from market data.
//!
//! "Interpolatable"
//! - ADJECTIVE:
//!     - *Able to be interpolated, or suited to interpolation.*

use num::{FromPrimitive, Num, ToPrimitive};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// /// Linear interpolation between two points.
// pub trait Lerp<T: Num + PartialOrd + Copy + FromPrimitive + ToPrimitive> {
//     fn lerp(&self, other: &Self, amount: &Self) -> Self;
// }

// impl Lerp for f64 {
//     fn lerp(&self, other: &Self, amount: &Self) -> Self {
//         self + (other - self) * amount
//     }
// }

/// Linear interpolation between two points.
/// Low-level inlined function for use in other functions.
#[inline]
pub fn lerp<T>(a: T, b: T, t: T) -> T
where
    T: Num + PartialOrd + Copy + FromPrimitive + ToPrimitive,
{
    a + (b - a) * t
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_interpolation {
    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lerp(0.0, 1.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 1.0, 1.0), 1.0);
        assert_eq!(lerp(0.0, 1.0, 2.0), 2.0);
        assert_eq!(lerp(0.0, 1.0, -1.0), -1.0);
    }
}
