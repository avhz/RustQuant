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

use std::cmp::Ordering;

use num::{FromPrimitive, Num, ToPrimitive};
use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Interpolation method, either linear, quadratic, or cubic.
pub enum InterpolationMethod {
    /// Linear interpolation.
    Linear,

    /// Quadratic Bezier curve interpolation.
    Quadratic,

    /// Cubic Bezier curve interpolation.
    Cubic,
}

/// Interpolatable trait.
/// This trait is implemented by all types that can be interpolated.
///
/// There must be a difference between the two points, and the two points must
/// be ordered.
trait Interpolatable {
    fn difference(&self, other: &Self) -> f64;
    fn compare(&self, other: &Self) -> Ordering;
}

// Macro to implement Interpolatable for all types that implement Num::Float.
macro_rules! impl_interpolatable_for_float {
    ($($t:ty)*) => ($(
        impl Interpolatable for $t {
            #[inline]
            fn difference(&self, other: &Self) -> f64 {
                (other - self) as f64
            }

            #[inline]
            fn compare(&self, other: &Self) -> Ordering {
                match self.partial_cmp(other) {
                    Some(ordering) => ordering,
                    None => panic!("Cannot compare {:?} and {:?}", self, other),
                }
            }
        }
    )*)
}

// Macro to implement Interpolatable for all types that implement Num::Integer.
macro_rules! impl_interpolatable_for_integer {
    ($($t:ty)*) => ($(
        impl Interpolatable for $t {
            #[inline]
            fn difference(&self, other: &Self) -> f64 {
                (*other - *self) as f64
            }

            #[inline]
            fn compare(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }
    )*)
}

impl_interpolatable_for_float! { f32 f64 }
impl_interpolatable_for_integer! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

impl Interpolatable for OffsetDateTime {
    #[inline]
    fn difference(&self, other: &Self) -> f64 {
        (*other - *self).as_seconds_f64()
    }

    #[inline]
    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

trait Interpolator {
    fn interpolate(&self, other: &Self, amount: &Self, method: InterpolationMethod) -> Self;
}

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
