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

/// Trait definition of an interpolator.
pub mod interpolator;
pub use interpolator::*;

pub mod linear_interpolator;
pub use linear_interpolator::*;

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // STRUCTS, ENUMS, AND TRAITS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// // /// Linear interpolation between two points.
// // pub trait Lerp<T: Num + PartialOrd + Copy + FromPrimitive + ToPrimitive> {
// //     fn lerp(&self, other: &Self, amount: &Self) -> Self;
// // }

// // impl Lerp for f64 {
// //     fn lerp(&self, other: &Self, amount: &Self) -> Self {
// //         self + (other - self) * amount
// //     }
// // }

// /// Linear interpolation between two points.
// /// Low-level inlined function for use in other functions.
// #[inline]
// pub fn lerp<T>(a: T, b: T, t: T) -> T
// where
//     T: Num + PartialOrd + Copy + FromPrimitive + ToPrimitive,
// {
//     a + (b - a) * t
// }
