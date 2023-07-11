// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::variable::Variable;
use nalgebra::{DMatrix, DVector};

/// A matrix of `Variable`s.
pub struct VariableMatrix<'v> {
    _data: DMatrix<Variable<'v>>,
}

/// A vector of `Variable`s.
pub struct VariableVector<'v> {
    _data: DVector<Variable<'v>>,
}

impl<'v> VariableMatrix<'v> {
    // pub fn new(rows: usize, cols: usize) -> Self {
    //     VariableMatrix {
    //         data: DMatrix::zeros(rows, cols),
    //     }
    // }

    // pub fn from_row_slice(rows: usize, cols: usize, elements: &[Variable]) -> Self {
    //     VariableMatrix {
    //         data: DMatrix::from_row_slice(rows, cols, elements),
    //     }
    // }

    // pub fn from_column_slice(rows: usize, cols: usize, data: &[Variable]) -> Self {
    //     VariableMatrix {
    //         data: DMatrix::from_column_slice(rows, cols, data),
    //     }
    // }

    // pub fn from_fn<F>(rows: usize, cols: usize, f: F) -> Self
    // where
    //     F: Fn(usize, usize) -> Variable<'v>,
    // {
    //     VariableMatrix {
    //         data: DMatrix::from_fn(rows, cols, f),
    //     }
    // }

    // pub fn map<F>(&self, f: F) -> Self
    // where
    //     F: Fn(Variable<'v>) -> Variable<'v>,
    // {
    //     VariableMatrix {
    //         data: self.data.map(f),
    //     }
    // }

    // pub fn map_mut<F>(&mut self, f: F)
    // where
    //     F: FnMut(Variable<'v>) -> Variable<'v>,
    // {
    //     self.data.map_mut(f);
    // }

    // pub fn rows(&self) -> usize {
    //     self.data.nrows()
    // }

    // pub fn cols(&self) -> usize {
    //     self.data.ncols()
    // }

    // pub fn get(&self, row: usize, col: usize) -> Variable<'v> {
    //     self.data[(row, col)]
    // }

    // pub fn set(&mut self, row: usize, col: usize, value: Variable<'v>) {
    //     self.data[(row, col)] = value;
    // }

    // pub fn component_mul(&self, other: &VariableMatrix<'v>) -> VariableMatrix<'v> {
    //     VariableMatrix {
    //         data: self.data.component_mul(&other.data),
    //     }
    // }

    // pub fn matrix_mul(&self, other: &VariableMatrix<'v>) -> VariableMatrix<'v> {
    //     VariableMatrix {
    //         data: self.data * &other.data,
    //     }
    // }

    // pub fn transpose(&self) -> VariableMatrix<'v> {
    //     VariableMatrix {
    //         data: self.data.transpose(),
    //     }
    // }

    // pub fn trace(&self) -> Variable<'v> {
    //     self.data.trace()
    // }

    // pub fn determinant(&self) -> Variable<'v> {
    //     self.data.determinant()
    // }

    // pub fn inverse(&self) -> VariableMatrix<'v> {
    //     VariableMatrix {
    //         data: self.data.try_inverse().unwrap(),
    //     }
    // }

    // pub fn solve(&self, rhs: &VariableVector<'v>) -> VariableVector<'v> {
    //     VariableVector {
    //         data: self.data.solve(&rhs.data).unwrap(),
    //     }
    // }
}

// // pub type VariableMatrix<'a> = nalgebra::DMatrix<Variable<'a>>;
// // pub type VariableVector<'a> = nalgebra::DVector<Variable<'a>>;

// /// Trait to implement the `Matrix` struct.
// pub trait Matrix<'v> {
//     // fn new(graph: &'v Graph, rows: usize, cols: usize) -> Self;
//     // fn get(&self, row: usize, col: usize) -> Variable<'v>;
//     // fn set(&mut self, row: usize, col: usize, value: Variable<'v>);

//     /// Component-wise multiplication.
//     fn component_mul(&self, other: &DMatrix<Variable<'v>>) -> DMatrix<Variable<'v>>;

//     // /// Matrix multiplication.
//     // fn matrix_mul(&self, other: &DMatrix<Variable<'v>>) -> DMatrix<Variable<'v>>;
// }

// impl Matrix<'v> for DMatrix<Variable<'v>> {
//     fn component_mul(&self, other: &DMatrix<Variable<'v>>) -> DMatrix<Variable<'v>> {
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
