// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Data management module.
//!
//! 'Data' includes data used for pricing, such as curves and surfaces.
//!
//!
//!
//! Disabled by default, due to Polars increasing compile times.
//!
//! You can:
//!
//! - Download data from Yahoo! Finance into a Polars `DataFrame`.
//! - Compute returns on the `DataFrame` you just downloaded.
//!
//! ```rust
//! use RustQuant::data::*;
//! use time::macros::date;
//!
//! // New YahooFinanceData instance.
//! // By default, date range is: 1970-01-01 to present.
//! let mut yfd = YahooFinanceData::new("AAPL".to_string());
//!
//! // Can specify custom dates (optional).
//! yfd.set_start_date(time::macros::datetime!(2019 - 01 - 01 0:00 UTC));
//! yfd.set_end_date(time::macros::datetime!(2020 - 01 - 01 0:00 UTC));
//!
//! // Download the historical data.
//! yfd.get_price_history();
//!
//! // Compute the returns.
//! // Specify the type of returns to compute (Simple, Logarithmic, Absolute)
//! // You don't need to run .get_price_history() first, .compute_returns()
//! // will do it for you if necessary.
//! yfd.compute_returns(ReturnsType::Logarithmic);
//!
//! println!("Apple's quotes: {:?}", yfd.price_history);
//! println!("Apple's returns: {:?}", yfd.returns);
//! ```
//!
//! ```bash
//! Apple's quotes: Some(shape: (252, 7)
//! ┌────────────┬───────────┬───────────┬───────────┬───────────┬────────────┬───────────┐
//! │ date       ┆ open      ┆ high      ┆ low       ┆ close     ┆ volume     ┆ adjusted  │
//! │ ---        ┆ ---       ┆ ---       ┆ ---       ┆ ---       ┆ ---        ┆ ---       │
//! │ date       ┆ f64       ┆ f64       ┆ f64       ┆ f64       ┆ f64        ┆ f64       │
//! ╞════════════╪═══════════╪═══════════╪═══════════╪═══════════╪════════════╪═══════════╡
//! │ 2019-01-02 ┆ 38.7225   ┆ 39.712502 ┆ 38.557499 ┆ 39.48     ┆ 1.481588e8 ┆ 37.994499 │
//! │ 2019-01-03 ┆ 35.994999 ┆ 36.43     ┆ 35.5      ┆ 35.547501 ┆ 3.652488e8 ┆ 34.209969 │
//! │ 2019-01-04 ┆ 36.1325   ┆ 37.137501 ┆ 35.950001 ┆ 37.064999 ┆ 2.344284e8 ┆ 35.670372 │
//! │ 2019-01-07 ┆ 37.174999 ┆ 37.2075   ┆ 36.474998 ┆ 36.982498 ┆ 2.191112e8 ┆ 35.590965 │
//! │ …          ┆ …         ┆ …         ┆ …         ┆ …         ┆ …          ┆ …         │
//! │ 2019-12-26 ┆ 71.205002 ┆ 72.495003 ┆ 71.175003 ┆ 72.477501 ┆ 9.31212e7  ┆ 70.798401 │
//! │ 2019-12-27 ┆ 72.779999 ┆ 73.4925   ┆ 72.029999 ┆ 72.449997 ┆ 1.46266e8  ┆ 70.771545 │
//! │ 2019-12-30 ┆ 72.364998 ┆ 73.172501 ┆ 71.305    ┆ 72.879997 ┆ 1.441144e8 ┆ 71.191582 │
//! │ 2019-12-31 ┆ 72.482498 ┆ 73.419998 ┆ 72.379997 ┆ 73.412498 ┆ 1.008056e8 ┆ 71.711739 │
//! └────────────┴───────────┴───────────┴───────────┴───────────┴────────────┴───────────┘)
//! ```
//!
//! ```bash
//! Apple's returns: Some(shape: (252, 7)
//! ┌────────────┬────────────┬───────────────┬───────────────┬───────────────┬──────────────┬──────────────┐
//! │ date       ┆ volume     ┆ open_logarith ┆ high_logarith ┆ low_logarithm ┆ close_logari ┆ adjusted_log │
//! │ ---        ┆ ---        ┆ mic           ┆ mic           ┆ ic            ┆ thmic        ┆ arithmic     │
//! │ date       ┆ f64        ┆ ---           ┆ ---           ┆ ---           ┆ ---          ┆ ---          │
//! │            ┆            ┆ f64           ┆ f64           ┆ f64           ┆ f64          ┆ f64          │
//! ╞════════════╪════════════╪═══════════════╪═══════════════╪═══════════════╪══════════════╪══════════════╡
//! │ 2019-01-02 ┆ 1.481588e8 ┆ null          ┆ null          ┆ null          ┆ null         ┆ null         │
//! │ 2019-01-03 ┆ 3.652488e8 ┆ -0.073041     ┆ -0.086273     ┆ -0.082618     ┆ -0.104924    ┆ -0.104925    │
//! │ 2019-01-04 ┆ 2.344284e8 ┆ 0.003813      ┆ 0.019235      ┆ 0.012596      ┆ 0.041803     ┆ 0.041803     │
//! │ 2019-01-07 ┆ 2.191112e8 ┆ 0.028444      ┆ 0.001883      ┆ 0.014498      ┆ -0.002228    ┆ -0.002229    │
//! │ …          ┆ …          ┆ …             ┆ …             ┆ …             ┆ …            ┆ …            │
//! │ 2019-12-26 ┆ 9.31212e7  ┆ 0.000457      ┆ 0.017709      ┆ 0.006272      ┆ 0.019646     ┆ 0.019646     │
//! │ 2019-12-27 ┆ 1.46266e8  ┆ 0.021878      ┆ 0.013666      ┆ 0.011941      ┆ -0.00038     ┆ -0.00038     │
//! │ 2019-12-30 ┆ 1.441144e8 ┆ -0.005718     ┆ -0.004364     ┆ -0.010116     ┆ 0.005918     ┆ 0.005918     │
//! │ 2019-12-31 ┆ 1.008056e8 ┆ 0.001622      ┆ 0.003377      ┆ 0.014964      ┆ 0.00728      ┆ 0.00728      │
//! └────────────┴────────────┴───────────────┴───────────────┴───────────────┴──────────────┴──────────────┘)
//! ```
//!
//! ### Read/write data
//!
//! ```ignore
//! use RustQuant::data::*;
//!
//! // New `Data` instance.
//! let mut data = Data::new(
//!     DataFormat::CSV,                        // Can also be JSON or PARQUET.
//!     String::from("./file/path/read.csv"),   // Path to read from.
//! );
//!
//! // Read from the given file.
//! data.read().unwrap();
//!
//! // New path to write the data to.
//! data.path = String::from("./file/path/write.csv");
//! data.write().unwrap();
//!
//! println!("{:?}", data.data);
//! ```

use crate::math::{
    InterpolationIndex,
};
use std::{collections::BTreeMap, hash::Hash};

/// File reading and writing.
pub mod io;
pub use io::*;

/// Yahoo! Finance data reader.
pub mod yahoo;
pub use yahoo::*;

/// Curves module.
/// Curves (in the financial sense) are functions that map
/// a time to a value, such as a yield curve or a swap curve.
/// They may also be known as term structures.
pub mod curve;
// pub use curve::*;

// /// Surface implementations.
// /// Surfaces are simply [Curve]s with an additional dimension.
// /// For example, a volatility surface is a function of time and strike/moneyness.
// pub mod surface;
// pub use surface::*;

// /// Term structure data.
// pub mod term_structure;
// pub use term_structure::*;

/// Curve index trait.
pub trait CurveIndex {}
impl<T> CurveIndex for T where T: Ord + Hash + InterpolationIndex {}

/// Curve data structure.
pub struct Curve<C>
where
    C: CurveIndex,
{
    /// The nodes of the curve.
    pub nodes: BTreeMap<C, f64>,
}

/// Surface data structure.
pub struct Surface<S, C>
where
    S: CurveIndex,
    C: CurveIndex,
{
    /// The curves that make up the surface.
    pub nodes: BTreeMap<S, Curve<C>>,
}

macro_rules! impl_curve {
    ($index:ty) => {
        impl Curve<$index> {
            /// Create a new curve.
            pub fn new() -> Self {
                Self {
                    nodes: BTreeMap::new(),
                }
            }

            /// Add a node to the curve.
            pub fn insert(&mut self, index: $index, value: f64) {
                self.nodes.insert(index, value);
            }

            /// Get a value for a specific index.
            pub fn get(&self, index: $index) -> Option<&f64> {
                self.nodes.get(&index)
            }

            /// Get a mutable reference to a value for a specific index.
            pub fn get_mut(&mut self, index: $index) -> Option<&mut f64> {
                self.nodes.get_mut(&index)
            }

            /// Create a Curve from a vector of indices and values.
            pub fn new_from_slice(indices: &[$index], values: &[f64]) -> Self {
                let mut curve = Self::new();

                for (index, value) in indices.iter().zip(values.iter()) {
                    curve.insert(*index, *value);
                }

                curve
            }

            /// Create a Curve from a function.
            pub fn new_from_function<F>(f: F, indices: &[$index]) -> Self
            where
                F: Fn($index) -> f64,
            {
                let mut curve = Self::new();

                for index in indices {
                    curve.insert(*index, f(*index));
                }

                curve
            }

            /// Create a Curve from a constant value.
            pub fn new_from_constant(value: f64, indices: &[$index]) -> Self {
                let mut curve = Self::new();

                for index in indices {
                    curve.insert(*index, value);
                }

                curve
            }

            /// Shift the curve by a constant value.
            pub fn shift(&mut self, shift: f64) {
                for value in self.nodes.values_mut() {
                    *value += shift;
                }
            }

            // /// Interpolate the curve at a specific index.
            // pub fn interpolate(&self, index: $index) -> f64 {
            //     // let (left, right) = self.nodes.range(..=index).next_back().unwrap();
            //     // let (left_index, left_value) = left;
            //     // let (right_index, right_value) = right;

            //     // let t = (*index - *left_index) / (*right_index - *left_index);

            //     // left_value + (right_value - left_value) * t
            //     let xs = self.nodes.keys().cloned().collect::<Vec<$index>>();
            //     let ys = self.nodes.values().cloned().collect::<Vec<f64>>();

            //     let interpolator = LinearInterpolator::new(xs, ys);

            //     1.0
            // }

            // /// Create a Curve from a Polars DataFrame.
            // pub fn new_from_polars(df: &DataFrame, index: &str, value: &str) -> Self {
            //     let mut curve = Self::new();

            //     let index_series = df.column(index).unwrap();
            //     let value_series = df.column(value).unwrap();

            //     for i in 0..df.height() {
            //         let index = index_series.get(i).unwrap().get::<$index>().unwrap();
            //         let value = value_series.get(i).unwrap().get::<f64>().unwrap();

            //         curve.insert(index, value);
            //     }

            //     curve
            // }
        }
    };
}

// Implement the Curve for temporal types.
impl_curve!(time::Date);
impl_curve!(time::Time);
impl_curve!(time::OffsetDateTime);
impl_curve!(time::PrimitiveDateTime);

// Implement the Curve for unsigned integer types.
impl_curve!(u64);
impl_curve!(u32);
impl_curve!(u16);
impl_curve!(u8);
impl_curve!(usize);

// Implement the Curve for signed integer types.
impl_curve!(i64);
impl_curve!(i32);
impl_curve!(i16);
impl_curve!(i8);
impl_curve!(isize);
