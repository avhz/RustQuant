// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::{Div, Mul, Sub, AddAssign};
use RustQuant_error::RustQuantError;

pub mod linear_interpolator;
pub use linear_interpolator::*;

pub mod exponential_interpolator;
pub use exponential_interpolator::*;

pub mod b_splines;
pub use b_splines::*;

pub mod lagrange_interpolator;
pub use lagrange_interpolator::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Trait describing requirements to be interpolated.
pub trait InterpolationValue: num::Num + AddAssign + MulAssign + std::fmt::Debug + Copy + Clone + Sized {}

/// Trait describing requirements to be an index of interpolation.
pub trait InterpolationIndex:
    Sub<Self, Output = Self::Delta> + PartialOrd + Copy + Clone + Sized + std::fmt::Display
{
    /// Type of the difference of `Self` - `Self`
    type Delta: Div<Self::Delta, Output = Self::DeltaDiv>
        + Mul<Self::DeltaDiv, Output = Self::Delta>;

    /// Type of `Delta` / `Delta`
    type DeltaDiv: InterpolationValue;
}

/// Interpolator trait.
/// This trait is implemented by all interpolation models.
pub trait Interpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: InterpolationValue,
{
    /// Fit the interpolator to the data.
    ///
    /// # Errors
    /// - `RustQuantError::UnequalLength` when the length of `xs` != `ys`.
    fn fit(&mut self) -> Result<(), RustQuantError>;

    /// Interpolate at value `point`.
    ///
    /// # Errors
    /// - `RustQuantError::Unfitted` when the interpolator has not been fitted.
    fn interpolate(&self, point: IndexType) -> Result<ValueType, RustQuantError>;

    /// Return range of interpolation.
    fn range(&self) -> (IndexType, IndexType);

    /// Add a point to the interpolator.
    fn add_point(&mut self, point: (IndexType, ValueType));
}

impl<T> InterpolationValue for T where T: num::Num + AddAssign + std::fmt::Debug + Copy + Clone + Sized {}

macro_rules! impl_interpolation_index {
    ($a:ty, $b:ty, $c:ty) => {
        impl InterpolationIndex for $a {
            type Delta = $b;
            type DeltaDiv = $c;
        }
    };
}

// Implement InterpolationIndex for all signed integer types.
impl_interpolation_index!(i8, i8, i8);
impl_interpolation_index!(i16, i16, i16);
impl_interpolation_index!(i32, i32, i32);
impl_interpolation_index!(i64, i64, i64);
impl_interpolation_index!(i128, i128, i128);
impl_interpolation_index!(isize, isize, isize);

// Implement InterpolationIndex for all unsigned integer types.
impl_interpolation_index!(u8, u8, u8);
impl_interpolation_index!(u16, u16, u16);
impl_interpolation_index!(u32, u32, u32);
impl_interpolation_index!(u64, u64, u64);
impl_interpolation_index!(u128, u128, u128);
impl_interpolation_index!(usize, usize, usize);

// Implement InterpolationIndex for all floating point types.
impl_interpolation_index!(f32, f32, f32);
impl_interpolation_index!(f64, f64, f64);

// Implement InterpolationIndex for date/time types.
impl_interpolation_index!(time::Date, time::Duration, f64);
impl_interpolation_index!(time::Time, time::Duration, f64);
impl_interpolation_index!(time::OffsetDateTime, time::Duration, f64);
impl_interpolation_index!(time::PrimitiveDateTime, time::Duration, f64);

// Implement InterpolationIndex for Decimal type.
impl_interpolation_index!(
    rust_decimal::Decimal,
    rust_decimal::Decimal,
    rust_decimal::Decimal
);
