// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the implementation of the `Variable` structure.
//!
//! `Variable`s are used to create inpug.variables and contain:
//!     - a pointer to their computation graph,
//!     - an index to their vertex,
//!     - an associated value.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::graph::Graph;
use std::fmt::Display;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCT AND IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the initial variables.
#[derive(Clone, Copy, Debug)]
pub struct Variable<'v> {
    /// Pointer to the graph.
    pub graph: &'v Graph,
    /// Index to the vertex.
    pub index: usize,
    /// Value associated to the vertex.
    pub value: f64, // Value,
}

// /// Value of the Variable.
// #[derive(Clone, Debug, PartialEq, PartialOrd)]
// pub enum Value {
//     /// Scalar valued Variable.
//     Scalar(f64),
//     /// Vector valued Variable.
//     Vector(nalgebra::DVector<f64>),
//     /// Matrix valued Variable.
//     Matrix(nalgebra::DMatrix<f64>),
// }

impl<'v> Variable<'v> {
    /// Instantiate a new variable.
    #[inline]
    pub fn new(graph: &'v Graph, index: usize, value: f64) -> Self {
        Variable {
            graph,
            index,
            value,
        }
    }

    /// Function to reverse accumulate the gradient.
    /// 1. Allocate the array of adjoints.
    /// 2. Set the seed (dx/dx = 1).
    /// 3. Traverse the graph backwards, updating the adjoints for the parent vertices.
    #[inline]
    pub fn accumulate(&self) -> Vec<f64> {
        // Set the seed.
        // The seed is the derivative of the output with respect to itself.
        // dy/dy = 1
        let mut adjoints = vec![0.0; self.graph.len()];
        adjoints[self.index] = 1.0; // SEED

        // Traverse the graph backwards and update the adjoints for the parent vertices.
        // This is simply the generalised chain rule.
        for (index, vertex) in self.graph.vertices.borrow().iter().enumerate().rev() {
            let deriv = adjoints[index];

            adjoints[vertex.parents[0]] += vertex.partials[0] * deriv;
            adjoints[vertex.parents[1]] += vertex.partials[1] * deriv;
        }

        adjoints
    }

    // /// Returns a zero variable.
    // #[inline]
    // pub fn zero(graph: &'v Graph) -> Self {
    //     Variable {
    //         graph,
    //         index: graph.push(Arity::Nullary, &[], &[]),
    //         value: 0.0,
    //     }
    // }

    /// Function to return the value contained in a vertex.
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Function to return the index of a vertex.
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Function to return the graph.
    #[inline]
    pub fn graph(&self) -> &'v Graph {
        self.graph
    }

    /// Check if variable is finite.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    /// Check if variable is infinite.
    #[inline]
    pub fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }

    /// Check if variable is NaN.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    /// Check if variable is normal.
    #[inline]
    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    /// Check if variable is subnormal.
    #[inline]
    pub fn is_subnormal(&self) -> bool {
        self.value.is_subnormal()
    }

    /// Check if variable is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    /// Check if variable is positive.
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.value.is_sign_positive()
    }

    /// Check if variable is negative.
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    /// Round variable to nearest integer.
    #[inline]
    pub fn round(&mut self) {
        self.value = self.value.round();
    }

    /// Returns the sign of the variable.
    #[inline]
    pub fn signum(&self) -> f64 {
        self.value.signum()
    }
}

/// Implement formatting for the `Variable` struct.
impl<'v> Display for Variable<'v> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<'v> PartialEq<f64> for Variable<'v> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl<'v> PartialEq for Variable<'v> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.graph, other.graph)
            && self.index == other.index
            && self.value == other.value
    }
}

impl<'v> Eq for Variable<'v> {}

impl<'v> PartialOrd for Variable<'v> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'v> Ord for Variable<'v> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .partial_cmp(&other.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_variable {
    use super::*;

    #[test]
    fn test_value() {
        let graph = Graph::new(); // assuming a `new` method in `Graph`
        let var = Variable {
            graph: &graph,
            index: 5,
            value: 3.14,
        };
        assert_eq!(var.value(), 3.14);
    }

    #[test]
    fn test_index() {
        let graph = Graph::new();
        let var = Variable {
            graph: &graph,
            index: 5,
            value: 3.14,
        };
        assert_eq!(var.index(), 5);
    }

    #[test]
    fn test_graph() {
        let graph = Graph::new();
        let var = Variable {
            graph: &graph,
            index: 5,
            value: 3.14,
        };
        assert_eq!(var.graph() as *const _, &graph as *const _);
    }

    #[test]
    fn test_cmp() {
        let graph = Graph::new();
        let var1 = Variable {
            graph: &graph,
            index: 5,
            value: 3.14,
        };
        let var2 = Variable {
            graph: &graph,
            index: 5,
            value: 2.71,
        };
        assert_eq!(var1.cmp(&var2), std::cmp::Ordering::Greater);
        assert_eq!(var2.cmp(&var1), std::cmp::Ordering::Less);
        assert_eq!(var1.cmp(&var1), std::cmp::Ordering::Equal);
    }
}
