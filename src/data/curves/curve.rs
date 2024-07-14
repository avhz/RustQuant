// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::error::RustQuantError;
use crate::math::{
    interpolation::{Interpolator, LinearInterpolator},
    InterpolationIndex,
};
use std::{collections::BTreeMap, hash::Hash};

/// Curve index trait.
pub trait CurveIndex: Ord + Hash + InterpolationIndex + Clone + Copy {}
impl<T> CurveIndex for T where T: Ord + Hash + InterpolationIndex + Clone + Copy {}

/// Curve data structure.
pub struct Curve<C>
where
    C: CurveIndex,
{
    /// The nodes of the curve.
    pub nodes: BTreeMap<C, f64>,
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

            /// Get the first key in the curve.
            pub fn first_key(&self) -> Option<&$index> {
                self.nodes.keys().next()
            }

            /// Get the last key in the curve.
            pub fn last_key(&self) -> Option<&$index> {
                self.nodes.keys().next_back()
            }

            /// Get the first value in the curve.
            pub fn first_value(&self) -> Option<&f64> {
                self.nodes.values().next()
            }

            /// Get the last value in the curve.
            pub fn last_value(&self) -> Option<&f64> {
                self.nodes.values().next_back()
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

            /// Get the bracketing indices for a specific index.
            pub fn get_brackets(&self, index: $index) -> ($index, $index) {
                let first = self.first_key().unwrap();
                let last = self.last_key().unwrap();

                if index <= *first {
                    return (*first, *first);
                }

                if index >= *last {
                    return (*last, *last);
                }

                let left = self.nodes.range(..index).next_back().unwrap().0;
                let right = self.nodes.range(index..).next().unwrap().0;

                return (*left, *right);
            }

            /// Shift the curve by a constant value.
            pub fn shift(&mut self, shift: f64) {
                for value in self.nodes.values_mut() {
                    *value += shift;
                }
            }

            /// Interpolate the curve at a specific index.
            ///
            /// Note: This method modifies the curve by adding the interpolated value.
            pub fn interpolate(&mut self, index: $index) {
                let xs: Vec<$index> = self.nodes.keys().cloned().collect();
                let ys: Vec<f64> = self.nodes.values().cloned().collect();

                let interpolator = LinearInterpolator::new(xs, ys).unwrap();

                self.insert(index, interpolator.interpolate(index).unwrap());
            }

            /// Interpolate the curve at multiple indices.
            ///
            /// Note: This method modifies the curve by adding the interpolated values.
            pub fn interpolate_many(&mut self, indices: &[$index]) {
                let xs: Vec<$index> = self.nodes.keys().cloned().collect();
                let ys: Vec<f64> = self.nodes.values().cloned().collect();

                let interpolator = LinearInterpolator::new(xs, ys).unwrap();

                for index in indices {
                    self.insert(*index, interpolator.interpolate(*index).unwrap());
                }
            }
        }
    };
}

// Implement the Curve for temporal types.
impl_curve!(time::Date);
impl_curve!(time::Time);
impl_curve!(time::OffsetDateTime);
impl_curve!(time::PrimitiveDateTime);

// THE FOLLOWING CANNOT BE IMPLEMENTED DUE TO RESTRICTIONS WITHIN THE INTERPOLATION MODULE.

// Implement the Curve for unsigned integer types.
// impl_curve!(u64);
// impl_curve!(u32);
// impl_curve!(u16);
// impl_curve!(u8);
// impl_curve!(usize);

// Implement the Curve for signed integer types.
// impl_curve!(i64);
// impl_curve!(i32);
// impl_curve!(i16);
// impl_curve!(i8);
// impl_curve!(isize);
