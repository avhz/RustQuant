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

    // Compute L and D matrices in A = L * D * L^T
    // Computation of L optimized by tracking sub-diagonal entries
    // as diagonal entries of L are always 1
    // and all other entries outside the sub-diagonal are 0
    //
    // E.g. the matrix
    // [ 1 0 0 0 ]
    // [ a 1 0 0 ]
    // [ 0 b 1 0 ]
    // [ 0 0 c 1 ]
    // is represented as [a, b, c]
    fn compute_lower_tri_and_diagonal_matrices(
        &self, 
        time_steps: &[IndexType::Delta]
    ) -> (
        Vec<IndexType::Delta>,
        Vec<ValueType>
    ) {
        let mut diagonal: Vec<IndexType::Delta>  = time_steps
            .windows(2)
            .map(|t| {
                (t[0] + t[1]) * ValueType::from_f64(2.0).unwrap()
            }).collect();

        let mut lower_tri_matrix_sub_diag_entries: Vec<ValueType> = Vec::with_capacity(diagonal.len() - 1);

        for i in 1..diagonal.len() {
            let prev_diag: IndexType::Delta = diagonal[i - 1];
            let t_prev: IndexType::Delta = time_steps[i - 1];

            diagonal[i] = diagonal[i] - t_prev * (t_prev / prev_diag);
            lower_tri_matrix_sub_diag_entries.push(t_prev / prev_diag);
        }

        (diagonal, lower_tri_matrix_sub_diag_entries)
    }

    // L * D * L^T inverts to (L^T)^-1 * D^-1 * L^-1
    // This method computes L^-1
    //
    // The inverse of a lower triangular matrix
    // is a lower triangular matrix
    // represented as a vector of vectors
    // without the known 0 entries
    // i.e.
    //
    // [a 0 0 0]
    // [b c 0 0]
    // [d e f 0]
    // [g h i j]
    //
    // is represented as
    //
    // [[a], [b, c], [d, e, f], [g, h, i, j]]
    fn invert_lower_tri_matrix(
        &self,
        lower_tri_matrix_sub_diag_entries: &[ValueType]
    ) -> Vec<Vec<ValueType>> {
        
        let mut temp: ValueType;
        let mut inverse: Vec<Vec<ValueType>> = vec![vec![]; lower_tri_matrix_sub_diag_entries.len() + 1];
        
        inverse[0].push(ValueType::one());
        
        for i in 0..lower_tri_matrix_sub_diag_entries.len() {
            temp = ValueType::one();
            for j in i..lower_tri_matrix_sub_diag_entries.len() {
                temp *= - lower_tri_matrix_sub_diag_entries[j];
                inverse[j + 1].push(temp)
            }
            inverse[i + 1].push(ValueType::one());
        }

        inverse
    }

    // Transpose a matrix
    fn transpose(
        &self, matrix: Vec<Vec<ValueType>>
    ) -> Vec<Vec<ValueType>> {
        let mut transposed_matrix: Vec<Vec<ValueType>> = vec![];

        for i in 0..matrix.len() {
            let mut row: Vec<ValueType> = vec![];
            for j in i..matrix.len() {
                row.push(matrix[j][i]);
            }
            transposed_matrix.push(row);
        }
        transposed_matrix
    }
    
    // L * D * L^T inverts to (L^T)^-1 * D^-1 * L^-1
    // This method computes (L^T)^-1 * D^-1
    //
    // The inverse of an upper triangular matrix
    // will be a upper triangular matrix
    // represented as an vector of vectors
    // without the known 0 entries
    // i.e.
    //
    // [a b c d]
    // [0 e f g]
    // [0 0 h i]
    // [0 0 0 j]
    //
    // is represented as
    //
    // [[a, b, c, d], [e, f, g], [h, i], [j]]
    //
    // Note that the method undertaken here takes
    // advantage of the fact that calculating the 
    // inverse of an upper triangular matrix is equal
    // to calculating the inverse its transpose
    // (a lower triangular matrix) then transposing the result.
    fn lower_tri_transpose_times_diag_inv(
        &self,
        lower_tri_inverse: &[Vec<ValueType>], 
        diagonal: &[IndexType::Delta],
    ) -> Vec<Vec<ValueType>> {
        let mut upper_tri_row: Vec<ValueType> = vec![];
        let mut product: Vec<Vec<ValueType>> = vec![];
        let one: ValueType = ValueType::one();

        for i in 0..diagonal.len() {
            upper_tri_row.clear();
            for j in 0..(i + 1) {
                if i == j {
                    upper_tri_row.push(one / diagonal[j].into_value())
                } else {
                    upper_tri_row.push(lower_tri_inverse[i][j] / diagonal[j].into_value());
                }
            }
            product.push(upper_tri_row.clone())
        }

        self.transpose(product)
    }
    
    // Multiply (L^T)^-1 * D^-1 (Upper triangular matrix) with L^-1 (Lower triangular matrix)
    // to finally get the inverse of A = L * D * L^T
    // The product is represented as a vector of vectors.
    //
    // Note that the *transpose* of the lower triangular matrix
    // is used to simplify the matrix multiplication process
    fn upper_tri_times_lower_tri(
        &self, 
        upper_tri_matrix: &[Vec<ValueType>], 
        lower_tri_matrix_transpose: &[Vec<ValueType>]
    ) -> Vec<Vec<ValueType>>
    {
        let mut product: Vec<Vec<ValueType>> = vec![vec![]; upper_tri_matrix.len()];
        let mut matrix_entry: ValueType;

        for i in 0..upper_tri_matrix.len() {
            for j in 0..lower_tri_matrix_transpose.len() {
                matrix_entry = ValueType::zero();
                let lower_diff: usize = (i as i64 - j as i64).max(0) as usize;
                let upper_diff: usize = (j as i64 - i as i64).max(0) as usize;
                for k in 0..(upper_tri_matrix[i].len().min(lower_tri_matrix_transpose[j].len())) {
                    matrix_entry = matrix_entry + upper_tri_matrix[i][k + upper_diff] * lower_tri_matrix_transpose[j][k + lower_diff];
                }
                product[i].push(matrix_entry);
            }
        }
        product
    }
    
    // Compute the spline value at a given point.
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
            .map(|x| (x[1] - x[0]))
            .collect();

        let (diag_matrix, lower_tri_matrix) = self.compute_lower_tri_and_diagonal_matrices(&self.time_steps);

        let mut inv_lower_tri_matrix: Vec<Vec<ValueType>> = self.invert_lower_tri_matrix(&lower_tri_matrix);

        let upper_tri_matrix: Vec<Vec<ValueType>> = self.lower_tri_transpose_times_diag_inv(
            &inv_lower_tri_matrix,
            &diag_matrix,
        );

        inv_lower_tri_matrix = self.transpose(inv_lower_tri_matrix);
        let tridiagonal_inverse: Vec<Vec<ValueType>> = self.upper_tri_times_lower_tri(
            &upper_tri_matrix,
            &inv_lower_tri_matrix
        );

        let rhs_vector: Vec<ValueType> = self.xs.iter().zip(self.ys.iter())
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

        self.second_derivatives = vec![ValueType::zero(); tridiagonal_inverse.len()];
        let mut matrix_entry: ValueType;

        for i in 0..tridiagonal_inverse.len() {
            matrix_entry = self.second_derivatives[i];
            for j in 0..tridiagonal_inverse[i].len() {
                matrix_entry = matrix_entry + tridiagonal_inverse[i][j] * rhs_vector[j];
            }
            self.second_derivatives[i] = matrix_entry;
        }

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
        let _ = self.fit();
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
            36.64909523809524,
            interpolator.interpolate(2.5).unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_natural_cubic_interpolation_dates() {
        let now: time::OffsetDateTime = time::OffsetDateTime::now_utc();

        let xs: Vec<time::OffsetDateTime> = vec![
            now,
            now + time::Duration::days(1),
            now + time::Duration::days(2),
            now + time::Duration::days(3),
            now + time::Duration::days(4),
        ];

        let ys: Vec<f64> = vec![0., 1., 16., 81., 256.];

        let mut interpolator: CubicSplineInterpolator<time::OffsetDateTime, f64> = CubicSplineInterpolator::new(xs.clone(), ys).unwrap();
        let _ = interpolator.fit();

        assert_approx_equal!(
            36.64909523809524,
            interpolator
                .interpolate(xs[2] + time::Duration::hours(12))
                .unwrap(),
            RUSTQUANT_EPSILON
        );
    }

    #[test]
    fn test_cubic_interpolation_out_of_range() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys = vec![1., 2., 3., 4., 5.];

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
            39.966361318634604,
            interpolator.interpolate(2.5).unwrap(),
            RUSTQUANT_EPSILON
        );
    }
}

#[cfg(test)]
mod tests_cubic_spline_helper_functions {
    use super::*;

    #[test]
    fn test_compute_lower_tri_and_diagonal_matrices() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let time_steps: Vec<f64> = xs.windows(2)
            .map(|x| (x[1] - x[0]))
            .collect();

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let (lower_triangular, diagonal) = interpolator.compute_lower_tri_and_diagonal_matrices(&time_steps);

        assert!(lower_triangular == vec![4.0, 3.75, 3.7333333333333334]);
        assert!(diagonal == vec![0.25, 0.26666666666666666]);
    }

    #[test]
    fn test_invert_lower_tri_matrix() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let inv_lower_tri_matrix: Vec<Vec<f64>> = interpolator.invert_lower_tri_matrix(&[3.0, 2.0, 1.0]);

        assert!(
            inv_lower_tri_matrix == vec![
                vec![1.0],
                vec![-3.0, 1.0],
                vec![6.0, -2.0, 1.0],
                vec![-6.0, 2.0, -1.0, 1.0]
            ]
        )
    }

    #[test]
    fn test_transpose() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let transposed_matrix = interpolator.transpose(vec![
            vec![1.0],
            vec![2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]);

        assert!(
            transposed_matrix == vec![
                vec![1.0, 2.0, 4.0],
                vec![3.0, 5.0],
                vec![6.0],
            ]
        );
    }

    #[test]
    fn lower_tri_transpose_times_diag_inv() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let lower_tri_inverse: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let diagonal: Vec<f64> = vec![1.0, 2.0, 3.0];

        assert!(
            interpolator.lower_tri_transpose_times_diag_inv(&lower_tri_inverse, &diagonal) == vec![
                vec![1.0, 2.0, 4.0],
                vec![0.5, 2.5],
                vec![0.3333333333333333]
            ]
        );
    }

    #[test]
    fn test_upper_tri_times_lower_tri() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let ys: Vec<f64> = vec![1., 2., 3., 4., 5.];

        let interpolator: CubicSplineInterpolator<f64, f64> = CubicSplineInterpolator::new(xs, ys).unwrap();
        let upper_tri_matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0],
            vec![6.0],
        ];
        let lower_tri_matrix_transpose = vec![
            vec![-1.0, -2.0, -3.0],
            vec![-4.0, -5.0],
            vec![-6.0],
        ];

        let product: Vec<Vec<f64>> = interpolator.upper_tri_times_lower_tri(
            &upper_tri_matrix,
            &lower_tri_matrix_transpose
        );

        assert!(
            product == vec![
                [-14.0, -23.0, -18.0],
                [-23.0, -41.0, -30.0],
                [-18.0, -30.0, -36.0]
            ]
        );
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

        assert!(spline_result == 36.64909523809524);
    }
}
