/*
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
RustQuant: A Rust library for quantitative finance tools.
Copyright (C) 2023 https://github.com/avhz
Dual licensed under Apache 2.0 and MIT.
See:
     - LICENSE-APACHE.md
     - LICENSE-MIT.md
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
*/

//! Module for K nearest neighbors (KNN) algorithms.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use nalgebra::{DMatrix, DVector};
use std::collections::HashMap;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// KNN Classifier struct
#[derive(Clone, Debug)]
pub struct KNearestClassifier<T> {
    /// Input data matrix.
    /// Rows correspond to data points, and each column is a different
    /// feature of the data.
    pub x: DMatrix<T>,

    /// Class labels for each row of the data matrix.
    /// Class labels are assumed to be integers, but they are read in
    /// as f64
    pub y: DVector<T>,

    /// Type of metric to compute distances
    pub metric: Metric,
}

/// Metric for computing distances
#[derive(Clone, Debug)]
pub enum Metric {
    /// Euclidean distance (default).
    /// Equivalent to Minkowski at $p=2$.
    Euclidean,
    /// Manhattan metric, also called the L1 or taxicab metric
    /// Equivalent to Minkowski at $p=1$.
    Manhattan,
    /// Minkowski metric.
    /// Parameter is the power value taken in the computation
    Minkowski(i32),
}

impl KNearestClassifier<f64> {
    /// New KNN
    pub fn new(x: DMatrix<f64>, y: DVector<f64>, metric: Metric) -> Self {
        assert!(x.nrows() == y.len());

        Self { x, y, metric }
    }

    /// Predict classes of a matrix of new data
    pub fn predict(&self, xprime: &DMatrix<f64>, k: &usize) -> f64 {
        let neighbors = self.find_neighbors(xprime, k);

        let mut classes: Vec<f64> = vec![];
        for tup in neighbors.iter() {
            classes.push((self.y)[tup.0]);
        }

        let mut counts = HashMap::new();

        classes
            .iter()
            .copied()
            .max_by_key(|&val| {
                let count = counts.entry(val as i64).or_insert(0);
                *count += 1;
                *count
            })
            .unwrap()
    }

    /// Find neighbors of collection of data
    fn find_neighbors(&self, xprime: &DMatrix<f64>, k: &usize) -> std::vec::Vec<(usize, f64)> {
        let mut distances: Vec<(usize, f64)> = vec![];

        // for (idx, point) in (self.x).iter().enumerate() {
        //     distances.push((idx, dist(xprime, point, &self.metric)))
        // }
        let (n_samples, _n_feats) = self.x.shape();

        for i in 0..n_samples {
            distances.push((
                i,
                match self.metric {
                    Metric::Euclidean => self.x.metric_distance(xprime),

                    Metric::Manhattan => self
                        .x
                        .apply_metric_distance(xprime, &nalgebra::base::LpNorm(1)),

                    Metric::Minkowski(p) => self
                        .x
                        .apply_metric_distance(xprime, &nalgebra::base::LpNorm(p)),
                },
            ));
        }

        // let comp = |(_x, y): (usize, f64 : RealField)| y;

        distances.sort_by(|(_x, y), (_z, w)| y.partial_cmp(w).unwrap());
        distances.into_iter().take(*k).collect()

        // todo!();
    }
}

// pub(crate) fn dist<f64: Scalar + ComplexField>(
//     x: &DMatrix<f64>,
//     y: &DMatrix<f64>,
//     met: &Metric,
// ) -> f64::RealField {
//     match met {
//         Metric::Euclidean => x.metric_distance(y),
//         Metric::Manhattan => x.apply_metric_distance(y, &nalgebra::base::LpNorm(1)),
//         Metric::Minkowski(p) => x.apply_metric_distance(y, &nalgebra::base::LpNorm(*p)),
//     }
// }

#[cfg(test)]
mod tests_knnclassifier {

    use super::*;

    #[test]
    fn test_knn_classifier() {}
}
