// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Reverse accumulation trait.
//! This trait is used to reverse accumulate the gradient for different types.
//!
//! Types that implement this trait will hopefully be:
//!
//! - `Variable<'v>`                <- Done
//! - `DMatrix<Variable<'v>>`       <- Currently not possible due to lifetimes
//! - `DVector<Variable<'v>>`       <- Currently not possible due to lifetimes
//! - `Array<Variable<'v>, Ix2>`    <- Work in progress

use std::ops::AddAssign;

use num_traits::{One, Zero};

use crate::DiffOps;

use super::variable::Variable;

/// Trait to reverse accumulate the gradient for different types.
pub trait Accumulate<OUT> {
    /// Function to reverse accumulate the gradient.
    fn accumulate(&self) -> OUT;
}

impl<T> Accumulate<Vec<T>> for Variable<'_, T> where T: Zero + One + Copy + AddAssign {
    /// Function to reverse accumulate the gradient for a `Variable`.
    /// 1. Allocate the array of adjoints.
    /// 2. Set the seed (dx/dx = 1).
    /// 3. Traverse the graph backwards, updating the adjoints for the parent vertices.
    #[inline]
    fn accumulate(&self) -> Vec<T> {
        // Set the seed.
        // The seed is the derivative of the output with respect to itself.
        // dy/dy = 1
        let mut adjoints = vec![T::zero(); self.graph.len()];
        adjoints[self.index] = T::one(); // SEED

        // Traverse the graph backwards and update the adjoints for the parent vertices.
        // This is simply the generalised chain rule.
        for (index, vertex) in self.graph.vertices.borrow().iter().enumerate().rev() {
            let deriv = adjoints[index];

            adjoints[vertex.parents[0]] += vertex.partials[0] * deriv;
            adjoints[vertex.parents[1]] += vertex.partials[1] * deriv;
        }

        adjoints
    }
}
