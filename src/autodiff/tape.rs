// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the implementation of the `Tape`.
//! The tape is also known as a Wengert List.
//!
//! The tape is an abstract data structure that contains `Node`s. These
//! contain the adjoints and indices to the parent nodes.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use {super::variable::Variable, std::cell::RefCell};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// NODE AND TAPE STRUCTS AND IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the nodes.
///
/// Operations are assumed to be binary (e.g. x + y),
/// thus the arrays have two elements.
/// To deal with unary or nullary operations, we just adjust the weights
/// (partials) and the dependencies (parents).
#[derive(Clone, Copy, Debug)]
pub struct Node {
    /// Array that contains the partial derivatives wrt to x and y.
    pub partials: [f64; 2],
    /// Array that contains the indices of the parent nodes.
    pub parents: [usize; 2],
}

/// Struct to contain the tape (Wengert list), as a vector of `Node`s.
#[derive(Debug)]
pub struct Tape {
    /// Vector containing the nodes in the Wengert List.
    pub nodes: RefCell<Vec<Node>>,
}

impl Default for Tape {
    fn default() -> Self {
        Tape {
            nodes: RefCell::new(Vec::new()),
        }
    }
}

/// Implementation for the `Tape` struct.
impl Tape {
    /// Instantiate a new tape.
    pub fn new() -> Self {
        Tape {
            nodes: RefCell::new(Vec::new()),
        }
    }

    /// Add a new variable to to the tape.
    /// Returns a new `Variable` instance (the contents of a node).
    #[inline]
    pub fn var(&self, value: f64) -> Variable {
        Variable {
            tape: self,
            value,
            index: self.push0(),
        }
    }

    /// Add a multiple variables (a slice) to the tape.
    /// Useful for larger functions with many inputs.
    #[inline]
    pub fn vars<'v>(&'v self, values: &[f64]) -> Vec<Variable<'v>> {
        values.iter().map(|&val| self.var(val)).collect()
    }

    /// Returns the length of the tape so new nodes can index to the correct position.
    pub fn len(&self) -> usize {
        self.nodes.borrow().len()
    }

    /// Returns true/false depending on whether the tape is empty or not.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Functions to push values to the tape (Wengert List):
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Nullary operator pushback.
    ///
    /// The node pushed to the tape is the result of a **nullary** operation.
    /// e.g. `x.neg()` ($-x$)
    /// Thus no partials and only the current index are added to the new node.
    ///
    /// 1. Constructs the node,
    /// 2. Pushes the new node onto the tape,
    /// 3. Returns the index of the new node.
    #[inline]
    pub fn push0(&self) -> usize {
        let mut nodes = self.nodes.borrow_mut();
        let len = nodes.len();
        nodes.push(Node {
            partials: [0.0, 0.0],
            parents: [len, len],
        });
        len
    }

    /// Unary operator pushback.
    ///
    /// The node pushed to the tape is the result of a **unary** operation.
    /// e.g. `x.sin()` ($sin(x)$)
    /// Thus one partial and one parent are added to the new node.
    ///
    /// 1. Constructs the node,
    /// 2. Pushes the new node onto the tape,
    /// 3. Returns the index of the new node.
    #[inline]
    pub fn push1(&self, parent0: usize, partial0: f64) -> usize {
        let mut nodes = self.nodes.borrow_mut();
        let len = nodes.len();
        nodes.push(Node {
            partials: [partial0, 0.0],
            parents: [parent0, len],
        });
        len
    }

    /// Binary operator pushback.
    ///
    /// The node pushed to the tape is the result of a **binary** operation.
    /// e.g. `x + y`
    /// Thus two partials and two parents are added to the new node.
    ///
    /// 1. Constructs the node,
    /// 2. Pushes the new node onto the tape,
    /// 3. Returns the index of the new node.
    #[inline]
    pub fn push2(&self, parent0: usize, partial0: f64, parent1: usize, partial1: f64) -> usize {
        let mut nodes = self.nodes.borrow_mut();
        let len = nodes.len();
        nodes.push(Node {
            partials: [partial0, partial1],
            parents: [parent0, parent1],
        });
        len
    }
}
