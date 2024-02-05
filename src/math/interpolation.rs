// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing functionality for interpolation.

use rust_decimal::Decimal;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS & ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Linear Interpolator.
pub struct LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: InterpolationValue,
{
    xs: Vec<IndexType>,
    ys: Vec<ValueType>,
    fitted: bool,
}

/// Exponential Interpolator.
pub struct ExponentialInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: InterpolationValue,
{
    xs: Vec<IndexType>,
    ys: Vec<ValueType>,
    fitted: bool,
}

/// Error for `interpolator`s.
#[derive(Debug, PartialEq)]
pub enum InterpolationError {
    /// The length of xs and ys are not equal.
    UnequalLength,

    /// The interpolator has not been fitted.
    Unfitted,

    /// Outside of interpolation range.
    OutsideOfRange,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Trait describing requirements to be interpolated.
pub trait InterpolationValue: num_traits::Num + Copy + Clone + Sized {}

/// Trait describing requirements to be an index of interpolation.
pub trait InterpolationIndex:
    std::ops::Sub<Self, Output = Self::Delta> + PartialOrd + Copy + Clone + Sized
{
    /// Type of the difference of `Self` - `Self`
    type Delta: std::ops::Div<Self::Delta, Output = Self::DeltaDiv>
        + std::ops::Mul<Self::DeltaDiv, Output = Self::Delta>;

    /// Type of `Delta` / `Delta`
    type DeltaDiv: InterpolationValue; // + num_traits::Num
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
    /// - `InterpolationError::UnequalLength` when the length of `xs` != `ys`.
    fn fit(&mut self) -> Result<(), InterpolationError>;

    /// Interpolate at value `point`.
    ///
    /// # Errors
    /// - `InterpolationError::Unfitted` when the interpolator has not been fitted.
    fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError>;

    /// Return range of interpolation.
    fn range(&self) -> (IndexType, IndexType);

    /// Add a point to the interpolator.
    fn add_point(&mut self, point: (IndexType, ValueType));
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, FUNCTIONS, AND MACROS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<T> InterpolationValue for T where T: num_traits::Num + Copy + Clone + Sized {}

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

// Implement InterpolationIndex for Decimal type.
impl_interpolation_index!(Decimal, Decimal, Decimal);

// Implement InterpolationIndex for date/time types.
impl_interpolation_index!(time::OffsetDateTime, time::Duration, f64);
impl_interpolation_index!(time::PrimitiveDateTime, time::Duration, f64);
impl_interpolation_index!(time::Time, time::Duration, f64);

impl<IndexType, ValueType> LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: InterpolationValue,
{
    /// Create a new LinearInterpolator.
    ///
    /// # Errors
    /// - `InterpolationError::UnequalLength` if ```xs.length() != ys.length()```.
    ///
    /// # Panics
    /// Panics if NaN is in the index.
    pub fn new(
        xs: Vec<IndexType>,
        ys: Vec<ValueType>,
    ) -> Result<LinearInterpolator<IndexType, ValueType>, InterpolationError> {
        if xs.len() != ys.len() {
            return Err(InterpolationError::UnequalLength);
        }

        let mut tmp: Vec<_> = xs.into_iter().zip(ys).collect();

        tmp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (xs, ys): (Vec<IndexType>, Vec<ValueType>) = tmp.into_iter().unzip();

        Ok(Self {
            xs,
            ys,
            fitted: false,
        })
    }
}

impl<IndexType, ValueType> ExponentialInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: InterpolationValue,
{
    /// Create a new ExponentialInterpolator.
    ///
    /// # Errors
    /// - `InterpolationError::UnequalLength` if ```xs.length() != ys.length()```.
    ///
    /// # Panics
    /// Panics if NaN is in the index.
    pub fn new(
        xs: Vec<IndexType>,
        ys: Vec<ValueType>,
    ) -> Result<ExponentialInterpolator<IndexType, ValueType>, InterpolationError> {
        if xs.len() != ys.len() {
            return Err(InterpolationError::UnequalLength);
        }

        let mut tmp: Vec<_> = xs.into_iter().zip(ys).collect();

        tmp.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (xs, ys): (Vec<IndexType>, Vec<ValueType>) = tmp.into_iter().unzip();

        Ok(Self {
            xs,
            ys,
            fitted: false,
        })
    }
}

impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
    for LinearInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType> + std::fmt::Debug,
    ValueType: InterpolationValue + std::fmt::Debug,
{
    fn fit(&mut self) -> Result<(), InterpolationError> {
        self.fitted = true;
        Ok(())
    }

    fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError>
where {
        let range = self.range();
        if point.partial_cmp(&range.0).unwrap() == std::cmp::Ordering::Less
            || point.partial_cmp(&range.1).unwrap() == std::cmp::Ordering::Greater
        {
            return Err(InterpolationError::OutsideOfRange);
        }
        if let Ok(idx) = self
            .xs
            .binary_search_by(|p| p.partial_cmp(&point).expect("Cannot compare values."))
        {
            return Ok(self.ys[idx]);
        }
        let idx_r = self.xs.partition_point(|&x| x < point);
        let idx_l = idx_r - 1;

        Ok(self.ys[idx_l]
            + (self.ys[idx_r] - self.ys[idx_l])
                * ((point - self.xs[idx_l]) / (self.xs[idx_r] - self.xs[idx_l])))
    }

    fn range(&self) -> (IndexType, IndexType) {
        (*self.xs.first().unwrap(), *self.xs.last().unwrap())
    }

    fn add_point(&mut self, point: (IndexType, ValueType)) {
        let idx = self.xs.partition_point(|&x| x < point.0);
        self.xs.insert(idx, point.0);
        self.ys.insert(idx, point.1);
    }
}

// impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
//     for ExponentialInterpolator<IndexType, ValueType>
// where
//     IndexType: InterpolationIndex<DeltaDiv = ValueType> + std::fmt::Debug,
//     ValueType: InterpolationValue + std::fmt::Debug,
// {
//     fn fit(&mut self) -> Result<(), InterpolationError> {
//         self.fitted = true;
//         Ok(())
//     }

//     fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError>
// where {
//         let range = self.range();

//         if point.partial_cmp(&range.0).unwrap() == std::cmp::Ordering::Less
//             || point.partial_cmp(&range.1).unwrap() == std::cmp::Ordering::Greater
//         {
//             return Err(InterpolationError::OutsideOfRange);
//         }

//         if let Ok(idx) = self
//             .xs
//             .binary_search_by(|p| p.partial_cmp(&point).expect("Cannot compare values."))
//         {
//             return Ok(self.ys[idx]);
//         }

//         let idx_r = self.xs.partition_point(|&x| x < point);
//         let idx_l = idx_r - 1;

//         let lambda = (self.xs[idx_r] - point) / (self.xs[idx_r] - self.xs[idx_l]);

//         // lambda = (x_r - x) / (x_r - x_l)
//         // term1  = lambda       * (x - x0) / (x_l - x0)
//         // term2  = (1 - lambda) * (x - x1) / (x_r - x0)
//         // out    = y_l^lambda * y_r^(1 - lambda)

//         Ok(self.ys[idx_l].powf(lambda * point / self.xs[idx_l])
//             * self.ys[idx_r].powf((1.0 - lambda) * point / self.xs[idx_r]))
//     }

//     fn range(&self) -> (IndexType, IndexType) {
//         (*self.xs.first().unwrap(), *self.xs.last().unwrap())
//     }

//     fn add_point(&mut self, point: (IndexType, ValueType)) {
//         let idx = self.xs.partition_point(|&x| x < point.0);
//         self.xs.insert(idx, point.0);
//         self.ys.insert(idx, point.1);
//     }
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_interpolation {

    use super::*;
    use crate::assert_approx_equal;
    use std::f64::EPSILON as EPS;

    #[test]
    fn test_linear_interpolation() {
        let xs = vec![1., 2., 3., 4., 5.];
        let ys = vec![1., 2., 3., 4., 5.];

        let mut interpolator = LinearInterpolator::new(xs, ys).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(2.5, interpolator.interpolate(2.5).unwrap(), EPS);
        assert_approx_equal!(3.5, interpolator.interpolate(3.5).unwrap(), EPS);
    }

    #[test]
    fn test_linear_interpolation_out_of_range() {
        let xs = vec![1., 2., 3., 4., 5.];
        let ys = vec![1., 2., 3., 4., 5.];

        let mut interpolator = LinearInterpolator::new(xs, ys).unwrap();
        let _ = interpolator.fit();

        assert!(InterpolationError::OutsideOfRange == interpolator.interpolate(6.).err().unwrap());
    }

    #[test]
    fn test_linear_interpolation_dates() {
        let now = time::OffsetDateTime::now_utc();

        let xs = vec![
            now,
            now + time::Duration::days(1),
            now + time::Duration::days(2),
            now + time::Duration::days(3),
            now + time::Duration::days(4),
        ];

        let ys = vec![1., 2., 3., 4., 5.];

        let mut interpolator = LinearInterpolator::new(xs.clone(), ys).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(
            2.5,
            interpolator
                .interpolate(xs[1] + time::Duration::hours(12))
                .unwrap(),
            EPS
        );
    }
}
