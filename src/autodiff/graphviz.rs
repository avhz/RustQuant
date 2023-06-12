// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module is for visualising a Tape.

use super::Tape;
use std::fmt;

/// Convert a tape to a Graphviz dot string.
pub fn to_dot(tape: &Tape) -> String {
    let mut dot = String::new();
    dot.push_str("digraph Tape {\n");
    for (i, vertex) in tape.vertices.borrow().iter().enumerate() {
        dot.push_str(&format!("    {} [label=\"{}\"];\n", i, i));
        for &parent in vertex.parents.iter() {
            if parent != i {
                dot.push_str(&format!("    {} -> {};\n", parent, i));
            }
        }
    }
    dot.push_str("}\n");
    dot
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vertices = self.vertices.borrow();
        writeln!(f, "digraph Tape {{")?;
        for (i, vertex) in vertices.iter().enumerate() {
            writeln!(f, "    {} [label=\"{}\"];", i, i)?;
            if vertex.parents[0] != i {
                writeln!(f, "    {} -> {};", vertex.parents[0], i)?;
            }
            if vertex.parents[1] != i {
                writeln!(f, "    {} -> {};", vertex.parents[1], i)?;
            }
        }
        writeln!(f, "}}")
    }
}

/// graph
pub fn to_dot2(tape: &Tape) -> String {
    let mut dot = String::from("digraph Tape {\n\trankdir=\"LR\";\n\tnode [shape=box];\n");

    let vertices = tape.vertices.borrow();

    // Define the nodes
    for (index, vertex) in vertices.iter().enumerate() {
        dot.push_str(&format!(
            "\t{} [label=\"ID: {}, Adjoints: {:.4?}\"];\n",
            index, index, vertex.partials
        ));
    }

    // Define the edges
    for (index, vertex) in vertices.iter().enumerate() {
        for parent in &vertex.parents {
            dot.push_str(&format!("\t{} -> {};\n", parent, index));
        }
    }

    dot.push_str("}\n");

    dot
}

#[cfg(test)]
mod test_graphviz {
    use super::*;
    use crate::autodiff::{Gradient, Tape};

    #[test]
    fn test_graphviz_1() {
        let tape = Tape::new();
        let x = tape.var(2.0);
        let y = tape.var(3.0);
        let z = x * y;
        let u = (z.exp()).sin();
        // let g = z.accumulate();
        // let dot = to_dot(&tape);

        // print!("{}", dot);
        print!("{}", to_dot2(&tape));
        // print!("{}", format!("{}", tape));
        // assert_eq!(
        //     dot,
        //     "digraph Tape {\n    0 [label=\"0\"];\n    1 [label=\"1\"];\n    2 [label=\"2\"];\n    3 [label=\"3\"];\n    4 [label=\"4\"];\n    0 -> 4;\n    1 -> 4;\n    2 -> 4;\n    3 -> 4;\n}\n"
        // );
    }

    #[test]
    fn test_graphviz_2() {
        let tape = Tape::new();
        let a = tape.var(1.0);
        let b = tape.var(2.0);
        let c = tape.var(3.0);
        let d = tape.var(4.0);

        let f = (a + b).sin() * (c + d).ln();

        // let g = z.accumulate();
        // let dot = to_dot(&tape);

        // print!("{}", dot);
        print!("{}", to_dot2(&tape));
        // print!("{}", format!("{}", tape));
        // assert_eq!(
        //     dot,
        //     "digraph Tape {\n    0 [label=\"0\"];\n    1 [label=\"1\"];\n    2 [label=\"2\"];\n    3 [label=\"3\"];\n    4 [label=\"4\"];\n    0 -> 4;\n    1 -> 4;\n    2 -> 4;\n    3 -> 4;\n}\n"
        // );
    }
}
