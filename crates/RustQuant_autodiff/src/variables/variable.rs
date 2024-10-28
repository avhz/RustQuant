// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
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

use crate::graph::Graph;
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
    #[must_use]
    #[inline]
    pub const fn new(graph: &'v Graph, index: usize, value: f64) -> Self {
        Variable {
            graph,
            index,
            value,
        }
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
    #[must_use]
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Function to return the index of a vertex.
    #[must_use]
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Function to return the graph.
    #[must_use]
    #[inline]
    pub fn graph(&self) -> &'v Graph {
        self.graph
    }

    /// Check if variable is finite.
    #[must_use]
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    /// Check if variable is infinite.
    #[must_use]
    #[inline]
    pub fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }

    /// Check if variable is NaN.
    #[must_use]
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    /// Check if variable is normal.
    #[must_use]
    #[inline]
    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    /// Check if variable is subnormal.
    #[must_use]
    #[inline]
    pub fn is_subnormal(&self) -> bool {
        self.value.is_subnormal()
    }

    /// Check if variable is zero.
    #[must_use]
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    /// Check if variable is positive.
    #[must_use]
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.value.is_sign_positive()
    }

    /// Check if variable is negative.
    #[must_use]
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
    #[must_use]
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

    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON as EPS};

    #[test]
    fn test_value() {
        let graph = Graph::new(); // assuming a `new` method in `Graph`
        let var = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        assert_approx_equal!(var.value(), std::f64::consts::PI, f64::EPSILON);
    }

    #[test]
    fn test_index() {
        let graph = Graph::new();
        let var = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        assert_eq!(var.index(), 5);
    }

    #[test]
    fn test_graph() {
        let graph = Graph::new();
        let var = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        assert_eq!(var.graph() as *const _, std::ptr::addr_of!(graph));
    }

    #[test]
    fn test_cmp() {
        let graph = Graph::new();
        let var1 = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
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

    #[test]
    fn test_variable_impl() {
        let g = Graph::new();

        assert!(g.var(1.0).is_finite());
        assert!(g.var(1.0).is_normal());
        assert!(!g.var(1.0).is_subnormal());
        assert!(!g.var(1.0).is_nan());
        assert!(!g.var(1.0).is_infinite());
        assert!(!g.var(1.0).is_zero());
        assert!(g.var(1.0).is_positive());
        assert!(!g.var(1.0).is_negative());
        assert_approx_equal!(g.var(1.0).signum(), 1.0, f64::EPSILON);
    }
}
