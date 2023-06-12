// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// THIS FILE IS NOT IN USE CURRENTLY.

/// Struct defining the vertex of the computational graph.
///
/// Operations are assumed to be binary (e.g. x + y),
/// thus the arrays have two elements.
/// To deal with unary or nullary operations, we just adjust the weights
/// (partials) and the dependencies (parents).
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// Array that contains the partial derivatives wrt to x and y.
    pub partials: [f64; 2],
    /// Array that contains the indices of the parent nodes.
    pub parents: [usize; 2],
}

/// Enumeration for the operation type.
/// This is used to determine the number of parents.
/// For example:
///     - A binary operation has two parents.
///     - A unary operation has one parent.
///     - A nullary operation has no parents.
pub enum OperationArity {
    /// Nullary operation (e.g. a constant).
    /// This has no parents.
    Nullary,
    /// Unary operation (e.g. sin(x)).
    /// This has one parent.
    Unary,
    /// Binary operation (e.g. x + y).
    /// This has two parents.
    Binary,
}
