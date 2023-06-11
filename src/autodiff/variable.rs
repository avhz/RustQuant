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

use std::fmt::Display;

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

/// Implement formatting for the `Variable` struct.
impl<'v> Display for Variable<'v> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'v> Variable<'v> {
    /// Function to return the value contained in a node.
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Function to return the index of a node.
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Function to return the tape.
    #[inline]
    pub fn tape(&self) -> &'v Tape {
        self.tape
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

        // Set the seed.
        // The seed is the derivative of the output with respect to itself.
        // dy/dy = 1
        adjoints[self.index] = 1.0;

        // Traverse the tape backwards and update the adjoints for the parent nodes.
        for (index, node) in nodes.iter().enumerate().rev() {
            let deriv = adjoints[index];
            adjoints[node.parents[0]] += node.partials[0] * deriv;
            adjoints[node.parents[1]] += node.partials[1] * deriv;
        }
        adjoints
    }
}

impl<'v> PartialEq for Variable<'v> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.tape, other.tape)
            && self.index == other.index
            && self.value == other.value
    }
}

impl<'v> Eq for Variable<'v> {}

impl<'v> PartialOrd for Variable<'v> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'v> Ord for Variable<'v> {
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
        let tape = Tape::new(); // assuming a `new` method in `Tape`
        let var = Variable {
            tape: &tape,
            index: 5,
            value: 3.14,
        };
        assert_eq!(var.value(), 3.14);
    }

    #[test]
    fn test_index() {
        let tape = Tape::new();
        let var = Variable {
            tape: &tape,
            index: 5,
            value: 3.14,
        };
        assert_eq!(var.index(), 5);
    }

    #[test]
    fn test_tape() {
        let tape = Tape::new();
        let var = Variable {
            tape: &tape,
            index: 5,
            value: 3.14,
        };
        assert_eq!(var.tape() as *const _, &tape as *const _);
    }

    #[test]
    fn test_cmp() {
        let tape = Tape::new();
        let var1 = Variable {
            tape: &tape,
            index: 5,
            value: 3.14,
        };
        let var2 = Variable {
            tape: &tape,
            index: 5,
            value: 2.71,
        };
        assert_eq!(var1.cmp(&var2), std::cmp::Ordering::Greater);
        assert_eq!(var2.cmp(&var1), std::cmp::Ordering::Less);
        assert_eq!(var1.cmp(&var1), std::cmp::Ordering::Equal);
    }
}
