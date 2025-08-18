// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing functionality for interpolation.

use crate::interpolation::{InterpolationIndex, InterpolationValue, Interpolator};
use RustQuant_error::RustQuantError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS & ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Linear Interpolator.
pub struct LagrangeInterpolator<IndexType, ValueType>
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

impl<IndexType, ValueType> LagrangeInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
{
    /// Create a new LagrangeInterpolator.
    ///
    /// # Errors
    /// - `RustQuantError::UnequalLength` if ```xs.length() != ys.length()```.
    ///
    /// # Panics
    /// Panics if NaN is in the index.
    pub fn new(
        xs: Vec<IndexType>,
        ys: Vec<ValueType>,
    ) -> Result<LagrangeInterpolator<IndexType, ValueType>, RustQuantError> {
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

    fn cardinal_function(&self, point: IndexType, pivot: IndexType, index: usize) -> ValueType {
        let mut lagrange_basis: ValueType = ValueType::one();
        for (i, x) in self.xs.iter().enumerate() {
            if i != index {
                lagrange_basis *= (point - *x) / (pivot - *x);
            }
        }
        lagrange_basis
    }

    fn lagrange_polynomial(&self, point: IndexType) -> ValueType {
        let mut polynomial: ValueType = ValueType::zero();
        for (i, (x, y)) in self.xs.iter().zip(&self.ys).enumerate() {
            polynomial += *y * self.cardinal_function(point, *x, i);
            
        }
        polynomial
    }
}

impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
    for LagrangeInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
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
        if point.partial_cmp(&range.0).unwrap() == std::cmp::Ordering::Less
            || point.partial_cmp(&range.1).unwrap() == std::cmp::Ordering::Greater
        {
            return Err(RustQuantError::OutsideOfRange);
        }
        if let Ok(idx) = self
            .xs
            .binary_search_by(|p| p.partial_cmp(&point).expect("Cannot compare values."))
        {
            return Ok(self.ys[idx]);
        }

        Ok(self.lagrange_polynomial(point))
    }
}
