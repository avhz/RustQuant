// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::autodiff::{Graph, Variable};
// // use nalgebra::{DMatrix, DVector};
// use ndarray::Array;

// // pub type VariableMatrix<'a> = nalgebra::DMatrix<Variable<'a>>;
// // pub type VariableVector<'a> = nalgebra::DVector<Variable<'a>>;

// /// Trait to implement the `Matrix` struct.
// pub trait Matrix<'v> {
//     // fn new(graph: &'v Graph, rows: usize, cols: usize) -> Self;
//     // fn get(&self, row: usize, col: usize) -> Variable<'v>;
//     // fn set(&mut self, row: usize, col: usize, value: Variable<'v>);

//     /// Component-wise multiplication.
//     fn component_mul(&self, other: &DMatrix<Variable<'static>>) -> DMatrix<Variable<'static>>;

//     // /// Matrix multiplication.
//     // fn matrix_mul(&self, other: &DMatrix<Variable<'v>>) -> DMatrix<Variable<'v>>;
// }

// impl Matrix<'static> for DMatrix<Variable<'static>> {
//     fn component_mul(&self, other: &DMatrix<Variable<'static>>) -> DMatrix<Variable<'static>> {
//         nalgebra::Matrix::component_mul(self, other)
//     }

//     // fn matrix_mul(&self, other: &DMatrix<Variable<'v>>) -> DMatrix<Variable<'v>> {
//     //     self * other
//     // }
// }

// #[cfg(test)]
// mod test_nalgebra {
//     use super::*;

//     #[test]
//     fn test_component_mul() {
//         let graph = Graph::new();
//         let a = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
//         let b = DMatrix::from_row_slice(2, 2, &[5.0, 6.0, 7.0, 8.0]);
//         let a_var = a.map(|x| Variable::new(&graph, 0, x));
//         let b_var = b.map(|x| Variable::new(&graph, 0, x));
//         let c_var = a_var.component_mul(&b_var);
//         let c = c_var.map(|x| x.value);
//         let c_expected = DMatrix::from_row_slice(2, 2, &[5.0, 12.0, 21.0, 32.0]);
//         assert_eq!(c, c_expected);
//     }
// }
