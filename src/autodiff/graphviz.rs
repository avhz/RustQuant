// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module is for visualising a Graph.
//!
//! THIS IS A WORK IN PROGRESS !
//!
//! Ideally, I want to be able to visualise the graph by using the `dot`
//! language. At the moment, I'm simply trying to make a function that outputs
//! `dot` code for a given graph.

use crate::autodiff::*;

// impl std::fmt::Display for Graph {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let vertices = self.vertices.borrow();
//         writeln!(f, "digraph Graph {{")?;
//         for (i, vertex) in vertices.iter().enumerate() {
//             writeln!(f, "    {} [label=\"{}\"];", i, i)?;
//             if vertex.parents[0] != i {
//                 writeln!(f, "    {} -> {};", vertex.parents[0], i)?;
//             }
//             if vertex.parents[1] != i {
//                 writeln!(f, "    {} -> {};", vertex.parents[1], i)?;
//             }
//         }
//         writeln!(f, "}}")
//     }
// }

/// Graphviz dot string.
pub fn graphviz(graph: &Graph, vars: &[Variable]) -> String {
    let mut dot = String::from(
        "digraph Graph {\n\tbgcolor=\"transparent\";\n\trankdir=\"LR\";\n\tnode [shape=box3d];\n",
    );

    let vertices = graph.vertices.borrow();

    // Initialize a HashSet with variable indices for quick lookup
    let var_indices: std::collections::HashSet<_> = vars.iter().map(|var| var.index).collect();

    // Define the nodes.
    for (index, _vertex) in vertices.iter().enumerate() {
        if var_indices.contains(&index) {
            let var_value = vars.iter().find(|var| var.index == index).unwrap().value();
            dot.push_str(&format!(
                "\t{} [label=\"Input: x_{}, Value: {:.2}\", color=\"red\"];\n",
                index, index, var_value
            ));
        } else {
            dot.push_str(&format!("\t{} [label=\"Op: #{}\"];\n", index, index));
        }
    }

    // Define the edges.
    for (index, vertex) in vertices.iter().enumerate() {
        for (i, parent) in vertex.parents.iter().enumerate() {
            if parent != &index {
                let label = vertex.partials[i];
                dot.push_str(&format!(
                    "\t{} -> {} [label=\"\u{2202}_{}: {:.2?}\"];\n",
                    parent, index, i, label
                ));
            }
        }
    }

    dot.push_str("}\n");

    dot
}

#[cfg(test)]
mod test_graphviz {
    use super::*;

    // RUN THESE TESTS VIA: cargo t test_graphviz -- --nocapture
    // This will print the graphviz output to stdout,
    // which you can then copy and paste into your Graphviz viewer of choice.

    #[test]
    fn test_graphviz_1() {
        let graph = Graph::new();
        let x = graph.var(2.0);
        let y = graph.var(3.0);
        let z = x * y;
        let _u = (z.exp()).sin();

        print!("{}", graphviz(&graph, &[x, y]));
    }

    #[test]
    fn test_graphviz_2() {
        let graph = Graph::new();
        let a = graph.var(1.0);
        let b = graph.var(2.0);
        let c = graph.var(3.0);
        let d = graph.var(4.0);

        let f1 = (a.exp() + b.cbrt()).sin();

        let e = graph.var(5.0);

        let f2 = e * (c.sqrt() + d.powf(2.0)).ln();

        let f3 = f1 + f2;

        print!("{}", graphviz(&graph, &[a, b, c, d, e]));
        println!("Gradient: {:.4?}", f3.accumulate().wrt(&[a, b, c, d, e]));
    }

    #[test]
    fn test_graphviz_3() {
        let graph = Graph::new();
        let x = graph.var(1.0);
        let y = graph.var(2.0);

        let z = x * y + y.sin();

        let _g = z.accumulate();

        print!("{}", graphviz(&graph, &[x, y]));
    }
}
