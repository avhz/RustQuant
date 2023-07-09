// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::Mul;

use super::{Graph, Variable};
use ndarray::{linalg::Dot, Array, Array2};

// type VariableMatrix<'v> = Array2<Variable<'v>>;

// type VariableMatrix<'a> = Array<Variable<'a>, ndarray::IxDyn>;
// type VariableVector<'a> = Array<Variable<'a>, ndarray::IxDyn>;

// /// Trait to implement the `Matrix` struct using `ndarray`.
// trait Matrix<'v> {
//     // fn new(graph: &'v Graph, rows: usize, cols: usize) -> Self;
//     // fn get(&self, row: usize, col: usize) -> Variable<'v>;
//     // fn set(&mut self, row: usize, col: usize, value: Variable<'v>);

//     /// Component-wise multiplication.
//     fn component_mul(&self, other: &VariableMatrix<'v>) -> VariableMatrix<'v>;

//     // /// Matrix multiplication.
//     // fn matrix_mul(&self, other: &VariableMatrix<'v>) -> VariableMatrix<'v>;
// }

// impl<'v> Matrix<'v> for VariableMatrix<'v> {
//     fn component_mul(&self, other: &VariableMatrix<'v>) -> VariableMatrix<'v> {
//         self * other
//     }

//     // fn matrix_mul(&self, other: &VariableMatrix<'v>) -> VariableMatrix<'v> {
//     //     self.dot(other)
//     // }
// }

// impl Dot<Array<Variable, ndarray::IxDyn>> for Array<Variable, ndarray::IxDyn> {
//     type Output = Array<Variable, ndarray::IxDyn>;

//     fn dot(&self, rhs: &Array<Variable, ndarray::IxDyn>) -> Self::Output {
//         self.dot(rhs)
//     }
// }

/// A matrix of `Variable`s.
#[derive(Debug, Clone, PartialEq, Eq)]
struct VariableMatrix<'v> {
    data: ndarray::Array2<Variable<'v>>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// `ndarray` ops implementations
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Component-wise multiplication.
impl<'v> std::ops::Mul<VariableMatrix<'v>> for VariableMatrix<'v> {
    type Output = VariableMatrix<'v>;

    fn mul(self, rhs: VariableMatrix<'v>) -> Self::Output {
        VariableMatrix {
            data: self.data * rhs.data,
        }
    }
}

// Component-wise addition.
impl<'v> std::ops::Add<VariableMatrix<'v>> for VariableMatrix<'v> {
    type Output = VariableMatrix<'v>;

    fn add(self, rhs: VariableMatrix<'v>) -> Self::Output {
        VariableMatrix {
            data: self.data + rhs.data,
        }
    }
}

// Component-wise subtraction.
impl<'v> std::ops::Sub<VariableMatrix<'v>> for VariableMatrix<'v> {
    type Output = VariableMatrix<'v>;

    fn sub(self, rhs: VariableMatrix<'v>) -> Self::Output {
        VariableMatrix {
            data: self.data - rhs.data,
        }
    }
}

// Component-wise division.
impl<'v> std::ops::Div<VariableMatrix<'v>> for VariableMatrix<'v> {
    type Output = VariableMatrix<'v>;

    fn div(self, rhs: VariableMatrix<'v>) -> Self::Output {
        VariableMatrix {
            data: self.data / rhs.data,
        }
    }
}

// Implementing `num-traits` traits for `Variable`.
// We need:
// - `num::One`
// - `num::Zero`

// impl<'v> num::One for Variable<'v> {
//     fn one() -> Self {
//         let graph = Graph::new();

//         Variable::new(graph, 0, 1.)
//         // Self::new(&Graph::new(), 0, 1.)
//     }
// }

// impl<'v> num::One for VariableMatrix<'v> {
//     fn one() -> Self {
//         Self {
//             data: ndarray::Array2::ones((0, 0)),
//         }
//     }
// }

// impl num::One for VariableMatrix<'v> {
//     fn one() -> Self {
//         Self {
//             data: ndarray::Array2::ones((0, 0)),
//         }
//     }
// }

// impl num::Zero for VariableMatrix<'v> {
//     fn zero() -> Self {
//         Self {
//             data: ndarray::Array2::zeros((0, 0)),
//         }
//     }

//     fn is_zero(&self) -> bool {
//         self.data.is_zero()
//     }
// }

// impl<'v> Dot<VariableMatrix<'v>> for VariableMatrix<'_> {
//     type Output = VariableMatrix<'v>;

//     fn dot(&self, rhs: &VariableMatrix<'v>) -> Self::Output {
//         self.data.dot(rhs.data)
//     }
// }

// impl<'a> VariableMatrix<'a> {
//     fn new(graph: &'a crate::autodiff::Graph, rows: usize, cols: usize) -> Self {
//         let data = Array::from_elem((rows, cols), Variable::new(graph, 0, 0.));
//         Self { data }
//     }

//     fn get(&self, row: usize, col: usize) -> Variable<'a> {
//         self.data[(row, col)]
//     }

//     fn set(&mut self, row: usize, col: usize, value: Variable<'a>) {
//         self.data[(row, col)] = value;
//     }
// }

#[cfg(test)]
mod test_ndarray {
    use crate::autodiff::Gradient;

    #[test]
    fn test_component_mul() {
        let g = crate::autodiff::Graph::new();

        let (a, b, c, d) = (g.var(1.), g.var(2.), g.var(3.), g.var(4.));
        let (e, f, g, h) = (g.var(5.), g.var(6.), g.var(7.), g.var(8.));

        // a = [[1, 2],
        //      [3, 4]]
        // b = [[5, 6],
        //      [7, 8]]
        let x = ndarray::array![[a, b], [c, d]];
        let y = ndarray::array![[e, f], [g, h]];

        // COMPONENT-WISE MULTIPLICATION
        // c = [[5 , 12],
        //      [21, 32]]
        let c = x * y; // <--- This works fine.
        let c_values = c.map(|x_i| x_i.value);
        let c_expected = ndarray::array![[5., 12.], [21., 32.]];

        // MATRIX MULTIPLICATION
        // let dot = a.dot(&b);                                 // <--- This does not work.

        assert_eq!(c, c_expected);

        println!("c: {:?}", c);
        println!("c_values: {:?}", c_values);
        println!("c_expected: {:?}", c_expected);
        println!("gradient: {:?}", c[[0, 0]].accumulate().wrt(&a));
    }
}
