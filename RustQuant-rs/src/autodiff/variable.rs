// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the implementation of the `Variable` structure.
//!
//! `Variable`s are used to create input variables and contain:
//!     - a pointer to their tape
//!     - an index to their node
//!     - an associated value.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::tape::Tape;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCT AND IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the initial variables.
#[derive(Clone, Copy, Debug)]
pub struct Variable<'v> {
    /// Pointer to the tape.
    pub tape: &'v Tape,
    /// Index to the node.
    pub index: usize,
    /// Value associated to the node.
    pub value: f64,
}

impl<'v> Variable<'v> {
    /// Function to return the value contained in a node.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Function to reverse accumulate the gradient.
    /// 1. Allocate the array of adjoints.
    /// 2. Set the seed (dx/dx = 1).
    /// 3. Traverse the tape backwards, updating the adjoints for the parent nodes.
    #[inline]
    pub fn accumulate(&self) -> Vec<f64> {
        let length = self.tape.len();
        let nodes = self.tape.nodes.borrow();
        let mut adjoints = vec![0.0; length];
        adjoints[self.index] = 1.0;

        for (index, node) in nodes.iter().enumerate().rev() {
            let deriv = adjoints[index];
            adjoints[node.parents[0]] += node.partials[0] * deriv;
            adjoints[node.parents[1]] += node.partials[1] * deriv;
        }
        adjoints
    }
}
