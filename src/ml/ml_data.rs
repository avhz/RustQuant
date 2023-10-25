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
use nalgebra::{DMatrix, DMatrixView, DVector};
use rand::prelude::*;
use std::ops::Index;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// A priori there's no way to distinguish an augmented matrix having
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

/// Thin wrapper for input data to be fed into ml algorithms
/// Type T is generic in principle, but in practice will only be
/// f32 or f64 to satisfy nalgebra::ComplexField trait
#[derive(Clone, Debug)]
struct MLData<T: nalgebra::ComplexField + Clone + Default> {
    /// Data matrix, possibly augmented by response vector
    /// Data points corresponds to rows, with
    /// features along the column entries
    pub data: DMatrix<T>,

    /// Response vector
    /// For classifiction, this will be a collection of class
    /// labels parameterized by the same data type as the feature
    /// data type.
    /// Optional in case MLData is test data
    pub y: Response,

    /// Number of samples
    pub samples: usize,

    /// Number of features
    pub features: usize,

    /// Denotes if data is for train or test
    pub data_type: InputClass,
}

impl<T: nalgebra::ComplexField + Default + Clone> MLData<T> {
    /// New MLData struct, with no response vector
    pub fn new(X: DMatrix<T>, data_type: InputClass) -> Self {
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

    /// New MLData with response vector
    /// Length of response vector must equal the number of samples
    pub fn with_response(X: DMatrix<T>, y: &DVector<T>, data_type: InputClass) -> Self {
        let samples = X.nrows();
        let features = X.ncols();

        assert_eq!(
            samples,
            (&y).nrows(),
            "Design matrix has nrows {} but response vector has length {}",
            samples,
            (&y).nrows()
        );

        let data = organize_data(X, &y);

        Self {
            data,
            y: Response::Present,
            samples,
            features,
            data_type,
        }
    }

    // New MLData from augmented data matrix.
    // Last column is assumed to be a response vector
    pub fn from_augmented(Xy: DMatrix<T>, data_type: InputClass) -> Self {
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

    /// Returns (feat)ure (matrix)
    pub fn featmatrix(&self) -> DMatrixView<T> {
        self.data.view((0, 0), (self.samples, self.features))
    }

    /// Returns (resp)onse (vector) if present
    pub fn respvector(&self) -> Option<DMatrixView<T>> {
        match &self.y {
            Response::Absent => None,
            Response::Present => Some(self.data.view((0, self.features - 1), (self.samples, 1))),
        }
    }

    /// Samples n random rows from the feature matrix X
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

        let nums = (0..self.samples).choose_multiple(&mut rng, n);
        nums
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

/// Implement subscripting for RFData type
/// Subscripts the augmented Xy matrix with
/// response vector along the rightmost column (if applicable)
impl<T: nalgebra::ComplexField + Clone + Default> Index<(usize, usize)> for MLData<T> {
    type Output = T;
    fn index<'a>(&'a self, (i, j): (usize, usize)) -> &'a Self::Output {
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
        write!(f, "X: {}, y: {:?}", self.featmatrix(), self.respvector())
    }
}

// TODO: mutable indexing

/// Appends response data to feature matrix and return augmented design matrix
pub(crate) fn organize_data<T: nalgebra::ComplexField + Clone + Default>(
    X: DMatrix<T>,
    y: &DVector<T>,
) -> DMatrix<T> {
    let ncols = X.ncols();

    assert_eq!(
        X.nrows(),
        (&y).nrows(),
        "Design matrix has nrows {} but response vector has length {}",
        X.nrows(),
        (&y).nrows()
    );
    let mut design = X.insert_column(ncols, T::default());
    design.set_column(ncols, &y);
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
                let randos: Vec<f32> = (0..N).into_iter().map(|x| rng.gen::<f32>()).collect();
                DMatrix::from_iterator(nrows, ncols, randos)
            }
        }
    }

    fn init_rand_X_f64(nrows: usize, ncols: usize, parallel: bool) -> DMatrix<f64> {
        //let seed = 16;

        let N = nrows * ncols;
        match parallel {
            true => {
                let randos: Vec<f64> = (0..N)
                    .into_par_iter()
                    .map_init(|| rand::thread_rng(), |rng, _| rng.gen::<f64>())
                    .collect::<Vec<f64>>();

                DMatrix::from_iterator(nrows, ncols, randos.into_iter())
            }
            false => {
                let mut rng: StdRng;
                rng = StdRng::from_entropy();
                let randos: Vec<f64> = (0..N).into_iter().map(|_| rng.gen::<f64>()).collect();
                DMatrix::from_iterator(nrows, ncols, randos)
            }
        }
    }

    #[test]
    fn mlinput_init_data() {
        std::env::set_var("RUST_BACKTRACE", "1");

        // Initialize identity matrix
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
        std::env::set_var("RUST_BACKTRACE", "1");
        let seed = 14;

        let nrows = 500000;
        let ncols = 500;

        let X: DMatrix<f32> = init_rand_X_f32(nrows, ncols, false);

        let input = MLData::new(X.clone(), InputClass::Train);

        let samp = input.sample(200, Some(seed));
        // println!("X:\n{}", X);
        println!("Random sample:\n{}", samp);
    }

    #[test]
    fn mlinput_bootstrap() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let seed = 16;
        let mut rng: StdRng;
        rng = StdRng::seed_from_u64(seed);

        let nrows = 15000;
        let ncols = 10;

        let N = nrows * ncols;
        let randos: Vec<f32> = (0..N)
            .into_par_iter()
            .map_init(|| rand::thread_rng(), |rng, _| rng.gen::<f32>())
            .collect::<Vec<f32>>();

        let X: DMatrix<f32> = DMatrix::from_iterator(nrows, ncols, randos.into_iter());

        let mut y = DVector::zeros(nrows);
        for row in y.iter_mut() {
            *row = rng.gen::<i8>() as f32;
        }
        // println!("X: {}", X);

        let input = MLData::with_response(X, &y, InputClass::Train);
        let B = 50;
        let samples = input.bootstrap(B, 10000, None);
        assert_eq!(samples.len(), B);
        for sample in samples.iter().take(10) {
            println!("{}\n", sample);
        }
    }
}
