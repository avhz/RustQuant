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

/// B-Spline Interpolator.
pub struct BSplineInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
{
    /// Knots of the B-Spline.
    pub knots: Vec<IndexType>,

    /// Control points of the B-Spline.
    pub control_points: Vec<ValueType>,

    /// Degree of B-Spline.
    pub degree: usize,

    /// Whether the interpolator has been fitted.
    pub fitted: bool,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, FUNCTIONS, AND MACROS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<IndexType, ValueType> BSplineInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
{
    /// Create a new BSplineInterpolator.
    ///
    /// # Errors
    /// - `RustQuantError::UnequalLength` if ```xs.length() != ys.length()```.
    ///
    /// # Panics
    /// Panics if NaN is in the index.
    pub fn new(
        mut knots: Vec<IndexType>,
        control_points: Vec<ValueType>,
        degree: usize
    ) -> Result<BSplineInterpolator<IndexType, ValueType>, RustQuantError> {

        if knots.len() != control_points.len() + degree + 1 {
            return Err(RustQuantError::BSplineInvalidParameters(
                control_points.len(), degree, control_points.len() + degree + 1,
            ));
        }

        knots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // println!("Knots: {:?}", knots);
        Ok(Self {
            knots,
            control_points,
            degree,
            fitted: false,
        })
    }

    /// Cox de Boor algorithm to evalute the spline curves.
    fn cox_de_boor(&self, point: IndexType, index: usize, degree: usize) -> ValueType {
        if degree == 0 {
            return if point.ge(&self.knots[index]) && point.lt(&self.knots[index + 1]) {
                ValueType::one()
            } else {
                ValueType::zero()
            }
        }
    
        let mut left_term: ValueType = ValueType::zero();
        let mut right_term: ValueType = ValueType::zero();
    
        if self.knots[index + degree] != self.knots[index] {
            left_term = ((point - self.knots[index]) / (self.knots[index + degree] - self.knots[index]))
                * self.cox_de_boor(point, index, degree - 1);
        }
    
        if self.knots[index + degree + 1] != self.knots[index + 1] {
            right_term = ((self.knots[index + degree + 1] - point) / (self.knots[index + degree + 1] - self.knots[index + 1]))
                * self.cox_de_boor(point, index + 1, degree - 1);
        }
        left_term + right_term
    }
}
