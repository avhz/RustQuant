// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Thin wrapper for data to be fed into `ml` methods

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use nalgebra::{DMatrix, DMatrixView, DVector, DVectorView};
use rand::prelude::*;
use std::ops::Index;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// a priori there's no way to distinguish an augmented matrix having
// a response vector attached to it. This field will keep track of when
// one is present or not.
/// Indicates presence of response vector in MLData
#[derive(Clone, Debug, Copy)]
pub enum Response {
    /// Response data is present
    Present,
    /// Response data is absent
    Absent,
}

/// Denotes type of MLData, i.e. for train or test
#[derive(Clone, Debug)]
pub enum InputClass {
    /// Train data
    Train,
    /// Test data
    Test,
}

/// Thin wrapper for input data to be fed into `ml` algorithms
/// Type T is generic in principle, but in practice will only be
/// f32 or f64 to satisfy nalgebra::ComplexField trait
#[derive(Clone, Debug)]
pub struct MLData<T: nalgebra::ComplexField + Clone + Default> {
    /// Data matrix, possibly augmented by response vector
    /// Data points corresponds to rows, with
    /// features along the column entries
    pub data: DMatrix<T>,

    /// Marker for if data has response vector
    pub y: Response,

    /// Number of samples
    pub samples: usize,

    /// Number of features
    pub features: usize,

    /// Denotes if data is for train or test
    pub data_type: InputClass,
}

/// Routines for initialization of data
pub trait InitializeData<T> {
    /// New MLData struct, with no response vector
    fn new(X: DMatrix<T>, data_type: InputClass) -> Self;

    /// New MLData with response vector y
    /// Length of response vector must equal the number of samples
    fn with_response(X: DMatrix<T>, y: &DVector<T>, data_type: InputClass) -> Self;

    /// New MLData from augmented data matrix.
    /// Last column of Xy is assumed to be a response vector
    fn from_augmented(Xy: DMatrix<T>, data_type: InputClass) -> Self;
}

impl<T: nalgebra::ComplexField + Default + Clone> InitializeData<T> for MLData<T> {
    fn new(X: DMatrix<T>, data_type: InputClass) -> Self {
        let samples = X.nrows();
        let features = X.ncols();
        Self {
            data: X,
            y: Response::Absent,
            samples,
            features,
            data_type,
        }
    }

    fn with_response(X: DMatrix<T>, y: &DVector<T>, data_type: InputClass) -> Self {
        let samples = X.nrows();
        let features = X.ncols();

        assert_eq!(
            samples,
            y.nrows(),
            "Design matrix has nrows {} but response vector has length {}",
            samples,
            y.nrows()
        );

        let data = organize_data(X, y);

        Self {
            data,
            y: Response::Present,
            samples,
            features,
            data_type,
        }
    }

    fn from_augmented(Xy: DMatrix<T>, data_type: InputClass) -> Self {
        let samples = Xy.nrows();
        let features = Xy.ncols() - 1;

        Self {
            data: Xy,
            y: Response::Present,
            samples,
            features,
            data_type,
        }
    }
}

impl<T: nalgebra::ComplexField + Default + Clone> MLData<T> {
    /// Returns (feat)ure (matrix)
    pub fn featmatrix(&self) -> DMatrixView<T> {
        self.data.view((0, 0), (self.samples, self.features))
    }

    /// Returns (resp)onse (vector) if present
    pub fn respvector(&self) -> Option<DVectorView<T>> {
        match &self.y {
            Response::Absent => None,
            Response::Present => Some(self.data.column(self.features)),
        }
    }

    /// Samples n random rows from data matrix
    pub fn sample(&self, n: usize, seed: Option<u64>) -> MLData<T> {
        assert!(
            n <= self.samples,
            "Desired number of samples {} exceeds samples {} in self",
            n,
            self.samples
        );

        let indices = self.sample_indexes(n, seed);
        let sampled_data = self.data.select_rows(indices.iter());

        match &self.y {
            Response::Present => MLData::from_augmented(sampled_data, self.data_type.clone()),
            Response::Absent => MLData::new(sampled_data, self.data_type.clone()),
        }
    }

    /// Samples indexes from range 0..samples
    /// Panics if n > self.samples
    fn sample_indexes(&self, n: usize, seed: Option<u64>) -> Vec<usize> {
        assert!(
            n < self.samples,
            "Desired number of samples {} exceeds samples {} in self.",
            n,
            self.samples
        );

        let mut rng: StdRng;
        if let Some(s) = seed {
            rng = StdRng::seed_from_u64(s);
        } else {
            rng = StdRng::from_entropy();
        }

        (0..self.samples).choose_multiple(&mut rng, n)
    }

    /// Bootstrap samples in B bags of size N with replacement
    /// Return is a vector of MLDatas with randomly sampled subsets of
    /// original data, with response vectors when needed
    pub fn bootstrap(&self, B: usize, N: usize, seed: Option<u64>) -> Vec<Self> {
        let mut samples: Vec<MLData<T>> = Vec::with_capacity(B);
        for _ in 0..B {
            samples.push(self.sample(N, seed));
        }
        samples
    }
}

/// Implement subscripting for MLData type
/// Subscripts the augmented Xy matrix with
/// response vector along the rightmost column (if applicable)
impl<T: nalgebra::ComplexField + Clone + Default> Index<(usize, usize)> for MLData<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        assert!(i <= self.features, "Index i out of range.");
        assert!(j <= self.samples, "Index j out of range.");
        if i == self.features {
            match &self.y {
                Response::Absent => {
                    panic!("Cannot subscript response vector: response vector is None")
                }
                Response::Present => return &self.data[(i, j)],
            }
        }

        &self.data[(i, j)]
    }
}

impl<T: nalgebra::ComplexField + Clone + Default> std::fmt::Display for MLData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

/// Appends response data to feature matrix and return augmented design matrix
pub(crate) fn organize_data<T: nalgebra::ComplexField + Clone + Default>(
    X: DMatrix<T>,
    y: &DVector<T>,
) -> DMatrix<T> {
    let ncols = X.ncols();

    assert_eq!(
        X.nrows(),
        y.nrows(),
        "Design matrix has nrows {} but response vector has length {}",
        X.nrows(),
        y.nrows()
    );
    let mut design = X.insert_column(ncols, T::default());
    design.set_column(ncols, y);
    design
}
#[cfg(test)]
mod tests_mlinput {
    use super::*;
    use nalgebra::{dmatrix, dvector};
    use rayon::prelude::*;

    fn init_rand_X_f32(nrows: usize, ncols: usize, parallel: bool) -> DMatrix<f32> {
        let N = nrows * ncols;
        match parallel {
            true => {
                let randos: Vec<f32> = (0..N)
                    .into_par_iter()
                    .map_init(|| rand::thread_rng(), |rng, _| rng.gen::<f32>())
                    .collect::<Vec<f32>>();

                DMatrix::from_iterator(nrows, ncols, randos.into_iter())
            }
            false => {
                let mut rng: StdRng;
                rng = StdRng::from_entropy();
                let randos: Vec<f32> = (0..N).into_iter().map(|_| rng.gen::<f32>()).collect();
                DMatrix::from_iterator(nrows, ncols, randos)
            }
        }
    }

    #[test]
    fn mlinput_init_data() {
        // Initialize identity matrix augmented by y
        let X = dmatrix![1.,0.,0.;
                         0.,1.,0.;
                         0.,0.,1.];
        let y = dvector![1., 0., 1.];

        let input = MLData::with_response(X, &y, InputClass::Train);
        let augmented = input.data;
        assert_eq!(
            augmented,
            dmatrix![1.,0.,0.,1.;
                     0.,1.,0.,0.;
                     0.,0.,1.,1.]
        );
    }

    #[test]
    fn mlinput_test_sample() {
        let seed = 14;

        // Define data size
        let nrows = 5000;
        let ncols = 500;

        // Initialize mldata
        let X: DMatrix<f32> = init_rand_X_f32(nrows, ncols, false);
        let input = MLData::new(X, InputClass::Train);

        // Sample desired number of samples
        let n = 200;
        let samp = input.sample(n, Some(seed));

        assert_eq!(samp.samples, n);
        assert_eq!(samp.features, ncols);
    }

    #[test]
    fn mlinput_bootstrap() {
        let seed = 16;
        let mut rng = StdRng::seed_from_u64(seed);

        // Define data sizes
        let nrows = 15000;
        let ncols = 100;
        let N = nrows * ncols;

        // Generate random numbers to fill for data matrix
        let randos: Vec<f32> = (0..N)
            .into_par_iter()
            .map_init(|| rand::thread_rng(), |rng, _| rng.gen::<f32>())
            .collect::<Vec<f32>>();

        // Make data to initialize test data
        let X: DMatrix<f32> = DMatrix::from_iterator(nrows, ncols, randos.into_iter());
        let mut y = DVector::zeros(nrows);
        for row in y.iter_mut() {
            *row = rng.gen::<i8>() as f32;
        }

        let input = MLData::with_response(X, &y, InputClass::Train);

        let B = 100;
        let N = 50;
        let samples = input.bootstrap(B, N, None);

        assert_eq!(samples.len(), B);
        for sample in samples {
            assert_eq!(sample.samples, N);
        }
    }
}
