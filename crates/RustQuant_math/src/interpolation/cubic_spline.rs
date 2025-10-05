// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module containing functionality for interpolation.


use crate::interpolation::{InterpolationIndex, InterpolationValue, Interpolator, IntoValue};
use RustQuant_error::RustQuantError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS & ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Cubic Spline Interpolator.
pub struct CubicSplineInterpolator<IndexType, ValueType>
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

    /// The second derivative of the spline at each point.
    pub second_derivatives: Vec<ValueType>,

    /// The time steps between each point.
    pub time_steps: Vec<IndexType::Delta>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, FUNCTIONS, AND MACROS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<IndexType, ValueType> CubicSplineInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
{
    /// Create a new CubicSplineInterpolator.
    ///
    /// # Errors
    /// - `RustQuantError::UnequalLength` if ```xs.length() != ys.length()```.
    ///
    /// # Panics
    /// Panics if NaN is in the index.
    pub fn new(
        xs: Vec<IndexType>,
        ys: Vec<ValueType>,
    ) -> Result<CubicSplineInterpolator<IndexType, ValueType>, RustQuantError> {
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
            second_derivatives: vec![],
            time_steps: vec![],
        })
    }

    fn cholesky_decomposition(&self) -> Vec<Vec<ValueType>> {
        let n: usize = self.time_steps.len();
        let mut lower_tri_matrix: Vec<Vec<ValueType>> = vec![vec![]; n - 1];

        let mut prev_diag: ValueType = (ValueType::from_f64(2.0).unwrap() * (self.time_steps[0] + self.time_steps[1]).into_value()).square_root();
        lower_tri_matrix[0].push(prev_diag);
        let mut prev_sub_diag: ValueType;

        for (i, time) in self.time_steps[1..(self.time_steps.len() - 1)].iter().enumerate() {
            prev_sub_diag = time.into_value() / prev_diag;
            lower_tri_matrix[i + 1].push(prev_sub_diag);

            prev_diag = (ValueType::from_f64(2.0).unwrap() * (*time + self.time_steps[i + 1]).into_value() - (prev_sub_diag * prev_sub_diag)).square_root();
            lower_tri_matrix[i + 1].push(prev_diag)
        }

        lower_tri_matrix
    }

    fn backward_substitution(&self, matrix: &[Vec<ValueType>], rhs: &[ValueType]) -> Vec<ValueType> {
    
        let matrix_len: usize = matrix.len();
        let mut prev_value: ValueType = rhs[matrix_len - 1] / matrix[matrix_len - 1][1];
        let mut result: Vec<ValueType> = vec![prev_value];

        for i in (1..(matrix_len - 1)).rev() {
            prev_value = (rhs[i] - matrix[i + 1][0] * prev_value) / matrix[i][1];
            result.insert(0, prev_value)
        }

        result.insert(0, (rhs[0] - matrix[1][0] * prev_value) / matrix[0][0]);

        result
    } 

    fn forward_substitution(&self, matrix: &[Vec<ValueType>], rhs: &[ValueType]) -> Vec<ValueType> {
    
        let mut prev_value: ValueType = rhs[0] / matrix[0][0];
        let mut result: Vec<ValueType> = vec![prev_value];

        for i in 1..matrix.len() {
            prev_value = (rhs[i] - matrix[i][0] * prev_value) / matrix[i][1];
            result.push(prev_value)
        }

        result
    }
    
    // Compute the spline value at a given point.
    #[allow(clippy::too_many_arguments)]
    fn spline(
        &self,
        point: IndexType,
        time_step: IndexType::Delta,
        x_coordinate_lower: IndexType,
        x_coordinate_upper: IndexType,
        y_coordinate_lower: ValueType,
        y_coordinate_upper: ValueType,
        second_derivative_lower: ValueType,
        second_derivative_upper: ValueType
    ) -> ValueType {
        let term_1: ValueType = (
                second_derivative_lower / (ValueType::from_f64(6.0).unwrap() 
                * time_step.into_value()))
            * (x_coordinate_upper - point).into_value() 
            * (x_coordinate_upper - point).into_value() 
            * (x_coordinate_upper - point).into_value();

        let term_2: ValueType = (
                second_derivative_upper / (ValueType::from_f64(6.0).unwrap() 
                * time_step.into_value())) 
            * (point - x_coordinate_lower).into_value() 
            * (point - x_coordinate_lower).into_value() 
            * (point - x_coordinate_lower).into_value();

        let term_3: ValueType = (
                (y_coordinate_upper / time_step.into_value()) - (time_step.into_value() 
                    * second_derivative_upper / ValueType::from_f64(6.0).unwrap())
            ) * (point - x_coordinate_lower).into_value();

        let term_4: ValueType = (
            (y_coordinate_lower / time_step.into_value()) - (time_step.into_value() 
                * second_derivative_lower / ValueType::from_f64(6.0).unwrap())
            ) * (x_coordinate_upper - point).into_value();
        
        term_1 + term_2 + term_3 + term_4
    }
}

impl<IndexType, ValueType> Interpolator<IndexType, ValueType>
    for CubicSplineInterpolator<IndexType, ValueType>
where
    IndexType: InterpolationIndex<DeltaDiv = ValueType>,
    ValueType: InterpolationValue,
{
    // The first step in fitting the cubic spline interpolator
    // is to compute the 2nd derivative at each point of the spline.
    // This is done by solving a system of linear equations Ax = b
    // where A is a tridiagonal matrix, b is a vector of values and
    // x is the vector of second derivatives at each point.
    //
    // The system is solved by decomposing A into L * D * L^T
    // and subsequently computing the inverse of A.
    //
    // Once all the second derivatives are computed, we can then
    // compute the spline value at a given point.
    fn fit(&mut self) -> Result<(), RustQuantError> {

        self.time_steps = self.xs.windows(2)
            .map(|x| x[1] - x[0])
            .collect();

        let mut rhs: Vec<ValueType> = self.xs.iter().zip(self.ys.iter())
            .collect::<Vec<(&IndexType, &ValueType)>>()
            .windows(3)
            .map(|window| {
                let x0: &IndexType = window[0].0;
                let x1: &IndexType = window[1].0;
                let x2: &IndexType = window[2].0;

                let y0: &ValueType = window[0].1;
                let y1: &ValueType = window[1].1;
                let y2: &ValueType = window[2].1;
                
                ValueType::from_f64(6.0).unwrap() * (
                    (ValueType::one() / (*x2 - *x1).into_value()) * (*y2)
                        - (
                            (ValueType::one() / (*x2 - *x1).into_value())
                                + (ValueType::one() / (*x1 - *x0).into_value())
                        ) * (*y1)
                        + (ValueType::one() / (*x1 - *x0).into_value()) * (*y0)
                )
            }).collect();

        let lower_tri_matrix: Vec<Vec<ValueType>> = self.cholesky_decomposition();
        rhs = self.forward_substitution(&lower_tri_matrix, &rhs);
        self.second_derivatives = self.backward_substitution(&lower_tri_matrix, &rhs);
        self.second_derivatives.insert(0, ValueType::zero());
        self.second_derivatives.push(ValueType::zero());

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
        self.fitted = false;
    }

    fn interpolate(&self, point: IndexType) -> Result<ValueType, RustQuantError> {
        if !self.fitted {
            return Err(RustQuantError::Unfitted);
        }
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

        let mut result: Option<ValueType> = None;
        for k in 0..(self.xs.len() - 1) {
            if self.xs[k] <= point && point < self.xs[k + 1] {
                result = Some(
                        self.spline(
                        point,
                        self.time_steps[k],
                        self.xs[k],
                        self.xs[k + 1],
                        self.ys[k],
                        self.ys[k + 1],
                        self.second_derivatives[k],
                        self.second_derivatives[k + 1],
                    )
                );
                break;
            }
        }

        match result {
            Some(value) => Ok(value),
            None => Err(RustQuantError::OutsideOfRange),
        }
    }
}

#[cfg(test)]
mod tests_cubic_spline_interpolation {
    use super::*;
    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON};

    #[test]
    fn test_natural_cubic_interpolation() {

        let xs: Vec<f64> = vec![0., 1., 2., 3., 4.];
        let ys: Vec<f64> = vec![0., 1., 16., 81., 256.];

        let mut interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(
            36.660714285714285,
            interpolator.interpolate(2.5).unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_cubic_interpolation_out_of_range() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let mut interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let _ = interpolator.fit();

        assert!(interpolator.interpolate(6.).is_err());
    }

    #[test]
    fn test_cubic_interpolation_add_range_and_refit() {
        let xs: Vec<f64> = vec![0., 1., 2., 3., 4.];
        let ys: Vec<f64> = vec![0., 1., 16., 81., 256.];

        let mut interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let _ = interpolator.fit();

        interpolator.add_point((5.0, 625.0));
        let _ = interpolator.fit();

        assert_approx_equal!(
            39.97368421052632,
            interpolator.interpolate(2.5).unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_cubic_interpolation_add_range_and_no_refit() {
        let xs: Vec<f64> = vec![0., 1., 2., 3., 4.];
        let ys: Vec<f64> = vec![0., 1., 16., 81., 256.];

        let mut interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let _ = interpolator.fit();

        interpolator.add_point((5.0, 625.0));

        assert!(
            matches!(
                interpolator.interpolate(2.5),
                Err(RustQuantError::Unfitted),
            )
        );
    }

    #[test]
    fn test_cubic_spline_unfitted() {

        let xs: Vec<f64> = vec![0., 1., 2., 3., 4.];
        let ys: Vec<f64> = vec![0., 1., 16., 81., 256.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();

        assert!(
            matches!(
                interpolator.interpolate(2.5),
                Err(RustQuantError::Unfitted),
            )
        );
    }
}

#[cfg(test)]
mod tests_cubic_spline_helper_functions {
    use super::*;

    #[test]
    fn test_cholesky_decomposition() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let mut interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        interpolator.time_steps = interpolator.xs.windows(2)
            .map(|x| (x[1] - x[0]))
            .collect();
        let result: Vec<Vec<f64>> = interpolator.cholesky_decomposition();
        let expected: &[&[f64]] = &[
            &[2.0],
            &[0.5, 1.9364916731037085],
            &[0.5163977794943222, 1.9321835661585918],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_forward_substitution() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let result: Vec<f64> = interpolator.forward_substitution(
            &[
                [2.0].to_vec(),
                [1.0, 2.0].to_vec(),
                [1.0, 2.0].to_vec(),
            ],
            &[1.0, 1.0, 1.0]
        );
        let expected: &[f64] = &[0.5, 0.25, 0.375];
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_backward_substitution() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let result: Vec<f64> = interpolator.backward_substitution(
            &[
                [2.0].to_vec(),
                [1.0, 2.0].to_vec(),
                [1.0, 2.0].to_vec(),
            ],
            &[1.0, 1.0, 1.0]
        );
        let expected: &[f64] = &[0.375, 0.25, 0.5];
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spline() {
        let xs: Vec<f64> = vec![0., 1., 2., 3., 4.];
        let ys: Vec<f64> = vec![0., 1., 16., 81., 256.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let spline_result: f64 = interpolator.spline(
            2.5, 
            1.0, 
            2.0, 
            3.0, 
            16.0, 
            81.0, 
            32.75733333333333, 
            156.85714285714286
        );

        assert_eq!(spline_result, 36.64909523809524);
    }
}
