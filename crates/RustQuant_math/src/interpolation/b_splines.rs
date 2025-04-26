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

impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
    for BSplineInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
{
    fn fit(&mut self) -> Result<(), RustQuantError> {

        self.fitted = true;
        Ok(())
    }

    fn range(&self) -> (IndexType, IndexType) {
        (*self.knots.first().unwrap(), *self.knots.last().unwrap())
    }

    fn add_point(&mut self, point: (IndexType, ValueType)) {
        let idx = self.knots.partition_point(|&x| x < point.0);
        self.knots.insert(idx, point.0);
        self.control_points.insert(self.control_points.len(), point.1);
    }


    fn interpolate(&self, point: IndexType) -> Result<ValueType, RustQuantError> {
        if !(point.ge(&self.knots[self.degree]) && point.le(&self.knots[self.knots.len() - self.degree - 1])) {
            
            let error_message: String = format!(
                "Point {} is outside of the interpolation range [{}, {}]",
                point,
                self.knots[self.degree],
                self.knots[self.knots.len() - self.degree - 1]
            );
            return Err(RustQuantError::BSplineOutsideOfRange(error_message));
        }

        let mut value = ValueType::zero();
        for (index, control_point) in self.control_points.iter().enumerate() {
            value += self.cox_de_boor(point, index, self.degree) * (*control_point);
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests_b_splines {
    use super::*;
    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON};

    #[test]
    fn test_b_spline_uniform_knots() {
        let knots = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let control_points = vec![-1.0, 2.0, 0.0, -1.0];
        
        let mut interpolator = BSplineInterpolator::new(knots, control_points, 2).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(
            1.375,
            interpolator.interpolate(2.5).unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_b_spline_non_uniform_knots() {
        let knots = vec![0.0, 1.0, 3.0, 4.0, 6.0, 7.0, 8.0, 10.0, 11.0];
        let control_points = vec![2.0, -1.0, 1.0, 0.0, 1.0];
        
        let mut interpolator = BSplineInterpolator::new(knots, control_points, 3).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(
            0.058333333333333,
            interpolator.interpolate(5.0).unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_b_spline_dates() {
        let now = time::OffsetDateTime::now_utc();
        let knots: Vec<time::OffsetDateTime> = vec![
            now,
            now + time::Duration::days(1),
            now + time::Duration::days(2),
            now + time::Duration::days(3),
            now + time::Duration::days(4),
            now + time::Duration::days(5),
            now + time::Duration::days(6),
        ];
        let control_points = vec![-1.0, 2.0, 0.0, -1.0];
        
        let mut interpolator = BSplineInterpolator::new(
            knots.clone(), control_points, 2
        ).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(
            1.375,
            interpolator
                .interpolate(knots[2] + time::Duration::hours(12))
                .unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_b_spline_inconsistent_parameters() {
        let knots = vec![0.0, 1.0, 2.0, 3.0, 4.0,];
        let control_points = vec![-1.0, 2.0, 0.0, -1.0];

        match BSplineInterpolator::new(knots.clone(), control_points.clone(), 2) {
            Ok(_) => panic!("Constructor did not throw an error!"),
            Err(e) => assert_eq!(
                e.to_string(),
                "For 4 control points and degree 2, we need 4 + 2 + 1 (7) knots."
            )
        }
    }

    #[test]
    fn test_b_spline_out_of_range() {
        let knots = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let control_points = vec![-1.0, 2.0, 0.0, -1.0];
        let mut interpolator = BSplineInterpolator::new(knots, control_points, 2).unwrap();
        let _ = interpolator.fit();

        match interpolator.interpolate(5.5) {
            Ok(_) => panic!("Interpolation should have failed!"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Point 5.5 is outside of the interpolation range [2, 4]"
            )
        }
    }
}
