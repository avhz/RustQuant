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
    /// New KNN classifier object
    /// x: data points with features along columns
    /// y: labels of data points
    /// metric: choice of metric to compute distances
    ///
    /// # Panics
    ///
    /// Will panic if x and y have different number of data points.
    #[must_use]
    pub fn new(x: DMatrix<f64>, y: DVector<f64>, metric: Metric) -> Self {
        assert_eq!(x.nrows(), y.nrows());

        Self { x, y, metric }
    }

    /// Predict class of a single test data point
    /// xprime: test data point
    /// k: number of neighbors to consider
    fn predict_one(&self, xprime: &DMatrix<f64>, k: usize) -> f64 {
        let neighbors = self.find_neighbors(xprime, k);

        let mut classes: Vec<f64> = vec![0.0; neighbors.len()];
        for i in 0..neighbors.len() {
            classes[i] = self.y[neighbors[i].0];
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

    /// Predict classes of collection of test points
    /// xprime: test data point
    /// k: number of neighbors to consider
    ///
    /// # Panics
    ///
    /// Panics if x and xprime do not have the same number of columns.
    #[must_use]
    pub fn predict(&self, xprime: &DMatrix<f64>, k: &usize) -> Vec<f64> {
        assert_eq!(self.x.ncols(), xprime.ncols());
        let mut predictions: Vec<f64> = vec![0.0; xprime.nrows()];

        (0..predictions.len()).for_each(|i| {
            predictions[i] =
                self.predict_one(&DMatrix::from(xprime.view((i, 0), (1, xprime.ncols()))), *k);
        });

        predictions
    }

    /// Find distances of neighbors of data points
    /// xprime: test data point
    /// k: number of neighbors to consider
    #[must_use]
    fn find_neighbors(&self, xprime: &DMatrix<f64>, k: usize) -> Vec<(usize, f64)> {
        let (n_samples, _n_feats) = self.x.shape();

        let mut distances: Vec<(usize, f64)> = vec![(0, 0.0); n_samples];

        (0..n_samples).for_each(|i| {
            distances[i] = (
                i,
                match self.metric {
                    Metric::Euclidean => self.x.row(i).metric_distance(xprime),

                    Metric::Manhattan => self
                        .x
                        .row(i)
                        .apply_metric_distance(xprime, &nalgebra::base::LpNorm(1)),

                    Metric::Minkowski(p) => self
                        .x
                        .row(i)
                        .apply_metric_distance(xprime, &nalgebra::base::LpNorm(p)),
                },
            );
        });

        distances.sort_by(|(_x, y), (_z, w)| y.partial_cmp(w).unwrap());
        distances.into_iter().take(k).collect()
    }
}

#[cfg(test)]
mod tests_knnclassifier {

    use RustQuant_utils::assert_approx_equal;

    use super::*;
    use nalgebra::dmatrix;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn iris_dataset_test_knn_classifier() {
        let iris_data = dmatrix![
            // Setosa = 0.0
            5.1,3.5,1.4,0.2;
            4.9,3.0,1.4,0.2;
            4.7,3.2,1.3,0.2;
            4.6,3.1,1.5,0.2;
            5.0,3.6,1.4,0.2;
            5.4,3.9,1.7,0.4;
            4.6,3.4,1.4,0.3;
            5.0,3.4,1.5,0.2;
            4.4,2.9,1.4,0.2;
            4.9,3.1,1.5,0.1;
            5.4,3.7,1.5,0.2;
            4.8,3.4,1.6,0.2;
            4.8,3.0,1.4,0.1;
            4.3,3.0,1.1,0.1;
            5.8,4.0,1.2,0.2;
            5.7,4.4,1.5,0.4;
            5.4,3.9,1.3,0.4;
            5.1,3.5,1.4,0.3;
            5.7,3.8,1.7,0.3;
            5.1,3.8,1.5,0.3;
            5.4,3.4,1.7,0.2;
            5.1,3.7,1.5,0.4;
            4.6,3.6,1.0,0.2;
            5.1,3.3,1.7,0.5;
            4.8,3.4,1.9,0.2;
            5.0,3.0,1.6,0.2;
            5.2,3.5,1.5,0.2;
            5.2,3.4,1.4,0.2;
            4.7,3.2,1.6,0.2;
            4.8,3.1,1.6,0.2;

            // Versicolor = 1.0
            7.0,3.2,4.7,1.4;
            6.4,3.2,4.5,1.5;
            6.9,3.1,4.9,1.5;
            5.5,2.3,4.0,1.3;
            6.5,2.8,4.6,1.5;
            5.7,2.8,4.5,1.3;
            6.3,3.3,4.7,1.6;
            4.9,2.4,3.3,1.0;
            6.6,2.9,4.6,1.3;
            5.2,2.7,3.9,1.4;
            5.0,2.0,3.5,1.0;
            5.9,3.0,4.2,1.5;
            6.0,2.2,4.0,1.0;
            6.1,2.9,4.7,1.4;
            5.6,2.9,3.6,1.3;
            6.7,3.1,4.4,1.4;
            5.6,3.0,4.5,1.5;
            5.8,2.7,4.1,1.0;
            5.6,2.5,3.9,1.1;
            5.9,3.2,4.8,1.8;
            6.1,2.8,4.0,1.3;
            6.3,2.5,4.9,1.5;
            6.1,2.8,4.7,1.2;
            6.4,2.9,4.3,1.3;
            6.6,3.0,4.4,1.4;
            6.8,2.8,4.8,1.4;
            6.7,3.0,5.0,1.7;
            6.0,2.9,4.5,1.5;
            5.7,2.6,3.5,1.0;
            5.5,2.4,3.8,1.1;


            // Virginica = 2.0
            6.3,3.3,6.0,2.5;
            5.8,2.7,5.1,1.9;
            7.1,3.0,5.9,2.1;
            6.3,2.9,5.6,1.8;
            6.5,3.0,5.8,2.2;
            7.6,3.0,6.6,2.1;
            4.9,2.5,4.5,1.7;
            7.3,2.9,6.3,1.8;
            6.7,2.5,5.8,1.8;
            7.2,3.6,6.1,2.5;
            6.5,3.2,5.1,2.0;
            6.4,2.7,5.3,1.9;
            6.8,3.0,5.5,2.1;
            5.7,2.5,5.0,2.0;
            5.8,2.8,5.1,2.4;
            6.4,3.2,5.3,2.3;
            6.5,3.0,5.5,1.8;
            7.7,3.8,6.7,2.2;
            7.7,2.6,6.9,2.3;
            6.0,2.2,5.0,1.5;
            6.9,3.2,5.7,2.3;
            5.6,2.8,4.9,2.0;
            7.7,2.8,6.7,2.0;
            6.3,2.7,4.9,1.8;
            6.7,3.3,5.7,2.1;
            7.2,3.2,6.0,1.8;
            6.2,2.8,4.8,1.8;
            6.1,3.0,4.9,1.8;
            6.4,2.8,5.6,2.1;
            7.4,2.8,6.1,1.9];

        let mut class_labels = vec![0.0; 30];
        class_labels.append(&mut vec![1.0; 30]);
        class_labels.append(&mut vec![2.0; 30]);
        let labels = DVector::from(class_labels);

        let knn: KNearestClassifier<f64> =
            KNearestClassifier::new(iris_data, labels, Metric::Euclidean);

        let test_features = dmatrix![

            // Setosa
            5.4,3.4,1.5,0.4;
            5.2,4.1,1.5,0.1;
            5.5,4.2,1.4,0.2;
            4.9,3.1,1.5,0.1;
            5.0,3.2,1.2,0.2;
            5.5,3.5,1.3,0.2;
            4.9,3.1,1.5,0.1;
            4.4,3.0,1.3,0.2;
            5.1,3.4,1.5,0.2;
            5.0,3.5,1.3,0.3;

            // Versicolor
            5.5,2.4,3.7,1.0;
            5.8,2.7,3.9,1.2;
            6.0,2.7,5.1,1.6;
            5.4,3.0,4.5,1.5;
            6.0,3.4,4.5,1.6;
            6.7,3.1,4.7,1.5;
            6.3,2.3,4.4,1.3;
            5.6,3.0,4.1,1.3;
            5.5,2.5,4.0,1.3;
            5.5,2.6,4.4,1.2;
            6.1,3.0,4.6,1.4;

            // Virginica
            7.9,3.8,6.4,2.0;
            6.4,2.8,5.6,2.2;
            6.3,2.8,5.1,1.5;
            6.1,2.6,5.6,1.4;
            7.7,3.0,6.1,2.3;
            6.3,3.4,5.6,2.4;
            6.4,3.1,5.5,1.8;
            6.0,3.0,4.8,1.8;
            6.9,3.1,5.4,2.1;
            6.7,3.1,5.6,2.4;
            6.9,3.1,5.1,2.3
        ];

        let mut actual_test_labels = vec![0.0; 10];
        actual_test_labels.append(&mut vec![1.0; 10]);
        actual_test_labels.append(&mut vec![2.0; 10]);

        // Predict with k=3 nearest neighbors
        let predictions = knn.predict(&test_features, &9);
        let N = predictions.len();

        let MSE = |x: &Vec<f64>, y: &Vec<f64>| -> f64 {
            x.iter()
                .zip(y.iter())
                .map(|(&xi, &yi)| (xi - yi).powi(2))
                .collect::<Vec<f64>>()
                .iter()
                .sum::<f64>()
                / (N as f64)
        };
        let err = MSE(&predictions, &actual_test_labels);
        assert_approx_equal!(err, 0.0, 0.1);
    }
}
