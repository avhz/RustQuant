// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing functionality for interpolation.

use crate::math::interpolation::{InterpolationError, InterpolationIndex, InterpolationValue};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS & ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Exponential Interpolator.
pub struct ExponentialInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex,
    ValueType: InterpolationValue,
{
    /// X-axis values for the interpolator.
    pub xs: Vec<IndexType>,

    /// Y-axis values for the interpolator.
    pub ys: Vec<ValueType>,

    /// Whether the interpolator has been fitted.
    pub fitted: bool,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, FUNCTIONS, AND MACROS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<IndexType, ValueType> ExponentialInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
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

// impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
//     for ExponentialInterpolator<IndexType, ValueType>
// where
//     IndexType: InterpolationIndex<DeltaDiv = ValueType>, //+ std::ops::Div<Output = ValueType>,
//     ValueType: InterpolationValue + num_traits::Float,
// {
//     fn fit(&mut self) -> Result<(), InterpolationError> {
//         self.fitted = true;
//         Ok(())
//     }

//     fn range(&self) -> (IndexType, IndexType) {
//         (*self.xs.first().unwrap(), *self.xs.last().unwrap())
//     }

//     fn add_point(&mut self, point: (IndexType, ValueType)) {
//         let idx = self.xs.partition_point(|&x| x < point.0);

//         self.xs.insert(idx, point.0);
//         self.ys.insert(idx, point.1);
//     }

//     fn interpolate(&self, point: IndexType) -> Result<ValueType, InterpolationError> {
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

//         let exponent_1 = lambda * (point / self.xs[idx_l]);
//         let exponent_2 = point / self.xs[idx_r] - lambda * (point / self.xs[idx_r]);

//         let term_1 = self.ys[idx_l].powf(exponent_1);
//         let term_2 = self.ys[idx_r].powf(exponent_2);

//         let result = term_1 * term_2;

//         Ok(result)
//     }
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_exponential_interpolation {
    use super::*;
    use crate::{assert_approx_equal, RUSTQUANT_EPSILON};
    use time::macros::date;

    // #[test]
    // fn test_exponential_interpolation_dates() {
    //     let d_1m = date!(1990 - 06 - 16);
    //     let d_2m = date!(1990 - 07 - 17);

    //     let r_1m = 0.9870;
    //     let r_2m = 0.9753;

    //     let dates = vec![d_1m, d_2m];
    //     let rates = vec![r_1m, r_2m];

    //     let mut interpolator = ExponentialInterpolator::new(dates, rates).unwrap();

    //     assert_approx_equal!(
    //         0.9854,
    //         interpolator.interpolate(date!(1990 - 06 - 20)).unwrap(),
    //         RUSTQUANT_EPSILON
    //     );
    // }
}
