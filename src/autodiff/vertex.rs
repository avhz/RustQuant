// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt;

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
    /// Array that contains the indices of the parent vertices.
    pub parents: [usize; 2],
    // /// Operation.
    // pub operation: Operation,
}

/// Enumeration for the operation type.
/// This is used to determine the number of parents.
/// For example:
///     - A binary operation has two parents.
///     - A unary operation has one parent.
///     - A nullary operation has no parents.
pub enum Arity {
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

/// Enumeration for the operation type.
#[derive(Clone, Copy, Debug)]
enum Operation {
    _ADD,
    _SUB,
    _MUL,
    _DIV,
    _POW,
    _SIN,
    _COS,
    _TAN,
    _EXP,
    _LOG,
    _SQRT,
    _ABS,
    _NEG,
    _INV,
    _MAX,
    _MIN,
}

impl Vertex {
    /// Get the partials of the vertex.
    pub fn get_partials(&self) -> [f64; 2] {
        self.partials
    }

    /// Get the parents of the vertex.
    pub fn get_parents(&self) -> [usize; 2] {
        self.parents
    }

    /// Instantiate a new vertex from a binary operation.
    pub fn new_binary(partial_x: f64, parent_x: usize, partial_y: f64, parent_y: usize) -> Self {
        Self {
            partials: [partial_x, partial_y],
            parents: [parent_x, parent_y],
        }
    }

    /// Instantiate a new vertex from a unary operation.
    pub fn new_unary(partial_x: f64, parent_x: usize) -> Self {
        Self {
            partials: [partial_x, 0.0],
            parents: [parent_x, 0],
        }
    }

    /// Instantiate a new vertex from a nullary operation.
    pub fn new_nullary() -> Self {
        Self {
            partials: [0.0; 2],
            parents: [0; 2],
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.partials == other.partials && self.parents == other.parents
    }
}

impl Eq for Vertex {}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vertex {{ partials: [{}, {}], parents: [{}, {}] }}",
            self.partials[0], self.partials[1], self.parents[0], self.parents[1]
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_vertex {
    use super::*;

    #[test]
    fn test_new_binary() {
        let vertex = Vertex::new_binary(1.0, 0, 2.0, 1);
        assert_eq!(vertex.get_partials(), [1.0, 2.0]);
        assert_eq!(vertex.get_parents(), [0, 1]);
    }

    #[test]
    fn test_new_unary() {
        let vertex = Vertex::new_unary(1.0, 0);
        assert_eq!(vertex.get_partials(), [1.0, 0.0]);
        assert_eq!(vertex.get_parents(), [0, 0]);
    }

    #[test]
    fn test_new_nullary() {
        let vertex = Vertex::new_nullary();
        assert_eq!(vertex.get_partials(), [0.0, 0.0]);
        assert_eq!(vertex.get_parents(), [0, 0]);
    }

    #[test]
    fn test_vertex_equality() {
        let vertex1 = Vertex::new_binary(1.0, 0, 2.0, 1);
        let vertex2 = Vertex::new_binary(1.0, 0, 2.0, 1);
        assert_eq!(vertex1, vertex2);

        let vertex3 = Vertex::new_binary(1.0, 0, 3.0, 1);
        assert_ne!(vertex1, vertex3);
    }

    #[test]
    fn test_vertex_display() {
        let vertex = Vertex::new_binary(1.0, 0, 2.0, 1);
        assert_eq!(
            format!("{}", vertex),
            "Vertex { partials: [1, 2], parents: [0, 1] }"
        );
    }
}
