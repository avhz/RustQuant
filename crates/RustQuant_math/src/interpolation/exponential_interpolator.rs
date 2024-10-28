// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing functionality for interpolation.

use super::Interpolator;
use crate::interpolation::{InterpolationIndex, InterpolationValue};
use num::Float;
use std::cmp::Ordering;
use RustQuant_error::RustQuantError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS & ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Exponential Interpolator.
pub struct ExponentialInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
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
    /// - `RustQuantError::UnequalLength` if ```xs.length() != ys.length()```.
    ///
    /// # Panics
    /// Panics if NaN is in the index.
    pub fn new(
        xs: Vec<IndexType>,
        ys: Vec<ValueType>,
    ) -> Result<ExponentialInterpolator<IndexType, ValueType>, RustQuantError> {
        if xs.len() != ys.len() {
            return Err(RustQuantError::UnequalLength);
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
    for ExponentialInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue + Float,
{
    fn fit(&mut self) -> Result<(), RustQuantError> {
        self.fitted = true;
        Ok(())
    }

    fn range(&self) -> (IndexType, IndexType) {
        (*self.xs.first().unwrap(), *self.xs.last().unwrap())
    }

    fn add_point(&mut self, point: (IndexType, ValueType)) {
        let idx = self.xs.partition_point(|&x| x < point.0);

        self.xs.insert(idx, point.0);
        self.ys.insert(idx, point.1);
    }

    fn interpolate(&self, point: IndexType) -> Result<ValueType, RustQuantError> {
        let range = self.range();

        let check1 = point.partial_cmp(&range.0).unwrap() == Ordering::Less;
        let check2 = point.partial_cmp(&range.1).unwrap() == Ordering::Greater;

        if check1 || check2 {
            return Err(RustQuantError::OutsideOfRange);
        }

        if let Ok(idx) = self
            .xs
            .binary_search_by(|p| p.partial_cmp(&point).expect("Cannot compare values."))
        {
            return Ok(self.ys[idx]);
        }

        //         let idx_r = self.xs.partition_point(|&x| x < point);
        //         let idx_l = idx_r - 1;

        // lambda = (x2 - X) / (x2 - x1)
        // exponent_1 = lambda * (X / x1)
        // exponent_2 = X / x2 - lambda * (X / x2) = (X / x2) * (1 - lambda)
        // term_1 = y1^exponent_1
        // term_2 = y2^exponent_2
        // result = y1^(lambda * X / x1) * y2^((X / x2) * (1 - lambda))

        //         let lambda = (self.xs[idx_r] - point) / (self.xs[idx_r] - self.xs[idx_l]);

        //         let exponent_1 = lambda * (point / self.xs[idx_l]);
        //         let exponent_2 = point / self.xs[idx_r] - lambda * (point / self.xs[idx_r]);

        //         let term_1 = self.ys[idx_l].powf(exponent_1);
        //         let term_2 = self.ys[idx_r].powf(exponent_2);

        //         let result = term_1 * term_2;

        //         Ok(result)

        // let idx_r = self.xs.partition_point(|&x| x < point);
        // let idx_l = idx_r - 1;

        // let x_l = self.xs[idx_l];
        // let y_l = self.ys[idx_l];

        // let x_r = self.xs[idx_r];
        // let y_r = self.ys[idx_r];

        // let term1 = y_r.ln() - y_l.ln();
        // let term2 = (point - x_l) / (x_r - x_l);
        // let term3 = y_l.ln();

        // let result = (term1 * term2 + term3).exp();

        let idx_r = self.xs.partition_point(|&x| x < point);

        let x_l = self.xs[idx_r - 1];
        let y_l = self.ys[idx_r - 1];

        let x_r = self.xs[idx_r];
        let y_r = self.ys[idx_r];

        let exp1 = (point - x_l) / (x_r - x_l);
        let exp2 = (x_r - point) / (x_r - x_l);

        let result = y_r.powf(exp1) * y_l.powf(exp2);

        Ok(result)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_exponential_interpolation {
    use super::*;
    use time::macros::date;
    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON};

    #[test]
    fn test_exponential_interpolation_numbers() {
        let xs = vec![1.0, 2.0, 3.0, 5.0];
        let ys = vec![5.0, 25.0, 125.0, 3125.0];

        let interpolator = ExponentialInterpolator::new(xs, ys).unwrap();
        assert_approx_equal!(
            625.0,
            interpolator.interpolate(4.0).unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_exponential_interpolation_dates() {
        let d_1m = date!(1990 - 06 - 16);
        let d_2m = date!(1990 - 07 - 17);

        let r_1m = 0.9870;
        let r_2m = 0.9753;

        let dates = vec![d_1m, d_2m];
        let rates = vec![r_1m, r_2m];

        let interpolator = ExponentialInterpolator::new(dates, rates).unwrap();

        assert_approx_equal!(
            0.9854824711068088,
            interpolator.interpolate(date!(1990 - 06 - 20)).unwrap(),
            RUSTQUANT_EPSILON
        );
    }
}
