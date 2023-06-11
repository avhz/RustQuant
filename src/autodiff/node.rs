// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt;

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

/// Enumeration for the operation type.
/// This is used to determine the number of parents.
/// For example:
///     - A binary operation has two parents.
///     - A unary operation has one parent.
///     - A nullary operation has no parents.
pub enum Operation {
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

impl Node {
    /// Get the partials of the node.
    pub fn get_partials(&self) -> [f64; 2] {
        self.partials
    }

    /// Get the parents of the node.
    pub fn get_parents(&self) -> [usize; 2] {
        self.parents
    }

    /// Instantiate a new node from a binary operation.
    pub fn new_binary(partial_x: f64, parent_x: usize, partial_y: f64, parent_y: usize) -> Self {
        Self {
            partials: [partial_x, partial_y],
            parents: [parent_x, parent_y],
        }
    }

    /// Instantiate a new node from a unary operation.
    pub fn new_unary(partial_x: f64, parent_x: usize) -> Self {
        Self {
            partials: [partial_x, 0.0],
            parents: [parent_x, 0],
        }
    }

    /// Instantiate a new node from a nullary operation.
    pub fn new_nullary() -> Self {
        Self {
            partials: [0.0; 2],
            parents: [0; 2],
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.partials == other.partials && self.parents == other.parents
    }
}

impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node {{ partials: [{}, {}], parents: [{}, {}] }}",
            self.partials[0], self.partials[1], self.parents[0], self.parents[1]
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_node {
    use super::*;

    #[test]
    fn test_new_binary() {
        let node = Node::new_binary(1.0, 0, 2.0, 1);
        assert_eq!(node.get_partials(), [1.0, 2.0]);
        assert_eq!(node.get_parents(), [0, 1]);
    }

    #[test]
    fn test_new_unary() {
        let node = Node::new_unary(1.0, 0);
        assert_eq!(node.get_partials(), [1.0, 0.0]);
        assert_eq!(node.get_parents(), [0, 0]);
    }

    #[test]
    fn test_new_nullary() {
        let node = Node::new_nullary();
        assert_eq!(node.get_partials(), [0.0, 0.0]);
        assert_eq!(node.get_parents(), [0, 0]);
    }

    #[test]
    fn test_node_equality() {
        let node1 = Node::new_binary(1.0, 0, 2.0, 1);
        let node2 = Node::new_binary(1.0, 0, 2.0, 1);
        assert_eq!(node1, node2);

        let node3 = Node::new_binary(1.0, 0, 3.0, 1);
        assert_ne!(node1, node3);
    }

    #[test]
    fn test_node_display() {
        let node = Node::new_binary(1.0, 0, 2.0, 1);
        assert_eq!(
            format!("{}", node),
            "Node { partials: [1, 2], parents: [0, 1] }"
        );
    }
}
