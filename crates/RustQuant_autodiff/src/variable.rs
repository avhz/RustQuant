// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the implementation of the `Variable` structure.
//!
//! `Variable`s are used to create inpug.variables and contain:
//!     - a pointer to their computation graph,
//!     - an index to their vertex,
//!     - an associated value.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::graph::Graph;
use std::fmt::Display;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCT AND IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the initial variables.
#[derive(Clone, Copy, Debug)]
pub struct Variable<'v> {
    /// Pointer to the graph.
    pub graph: &'v Graph,
    /// Index to the vertex.
    pub index: usize,
    /// Value associated to the vertex.
    pub value: f64, // Value,
}

impl<'v> Variable<'v> {
    /// Instantiate a new variable.
    #[must_use]
    #[inline]
    pub const fn new(graph: &'v Graph, index: usize, value: f64) -> Self {
        Variable {
            graph,
            index,
            value,
        }
    }

    /// Function to return the value contained in a vertex.
    #[must_use]
    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Function to return the index of a vertex.
    #[must_use]
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Function to return the graph.
    #[must_use]
    #[inline]
    pub fn graph(&self) -> &'v Graph {
        self.graph
    }

    /// Check if variable is finite.
    #[must_use]
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    /// Check if variable is infinite.
    #[must_use]
    #[inline]
    pub fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }

    /// Check if variable is NaN.
    #[must_use]
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    /// Check if variable is normal.
    #[must_use]
    #[inline]
    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    /// Check if variable is subnormal.
    #[must_use]
    #[inline]
    pub fn is_subnormal(&self) -> bool {
        self.value.is_subnormal()
    }

    /// Check if variable is zero.
    #[must_use]
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    /// Check if variable is positive.
    #[must_use]
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.value.is_sign_positive()
    }

    /// Check if variable is negative.
    #[must_use]
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.value.is_sign_negative()
    }

    /// Round variable to nearest integer.
    #[inline]
    pub fn round(&mut self) {
        self.value = self.value.round();
    }

    /// Returns the sign of the variable.
    #[must_use]
    #[inline]
    pub fn signum(&self) -> f64 {
        self.value.signum()
    }
}

/// Implement formatting for the `Variable` struct.
impl<'v> Display for Variable<'v> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<'v> PartialEq<f64> for Variable<'v> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl<'v> PartialEq for Variable<'v> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.graph, other.graph)
            && self.index == other.index
            && self.value == other.value
    }
}

impl<'v> Eq for Variable<'v> {}

impl<'v> PartialOrd for Variable<'v> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'v> Ord for Variable<'v> {
    #[inline]
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

    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn test_value() {
        let graph = Graph::new(); // assuming a `new` method in `Graph`
        let var = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        assert_approx_equal!(var.value(), std::f64::consts::PI, f64::EPSILON);
    }

    #[test]
    fn test_index() {
        let graph = Graph::new();
        let var = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        assert_eq!(var.index(), 5);
    }

    #[test]
    fn test_graph() {
        let graph = Graph::new();
        let var = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        assert_eq!(var.graph() as *const _, std::ptr::addr_of!(graph));
    }

    #[test]
    fn test_cmp() {
        let graph = Graph::new();
        let var1 = Variable {
            graph: &graph,
            index: 5,
            value: std::f64::consts::PI,
        };
        let var2 = Variable {
            graph: &graph,
            index: 5,
            value: 2.71,
        };
        assert_eq!(var1.cmp(&var2), std::cmp::Ordering::Greater);
        assert_eq!(var2.cmp(&var1), std::cmp::Ordering::Less);
        assert_eq!(var1.cmp(&var1), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_variable_impl() {
        let g = Graph::new();

        assert!(g.var(1.0).is_finite());
        assert!(g.var(1.0).is_normal());
        assert!(!g.var(1.0).is_subnormal());
        assert!(!g.var(1.0).is_nan());
        assert!(!g.var(1.0).is_infinite());
        assert!(!g.var(1.0).is_zero());
        assert!(g.var(1.0).is_positive());
        assert!(!g.var(1.0).is_negative());
        assert_approx_equal!(g.var(1.0).signum(), 1.0, f64::EPSILON);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::{variables::variable::Variable, Graph};
// use ndarray::{Array, Ix1, Ix2};

// /// A matrix of `Variable`s.
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct VariableArray<'v> {
//     /// The data of the array.
//     pub data: Array<Variable<'v>, Ix2>,
// }

// /// A vector of `Variable`s.
// pub struct VariableVector<'v> {
//     /// The data of the vector.
//     pub data: Array<Variable<'v>, Ix1>,
// }

// /// Struct to contain the initial variables.
// #[derive(Clone, Debug)]
// pub struct ARRAY<'v> {
//     /// Pointer to the graph.
//     pub graph: &'v Graph,
//     /// Index to the vertex.
//     pub index: usize,
//     /// Value associated to the vertex.
//     pub value: Array<f64, Ix2>, // Value,
// }

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // IMPLEMENTATIONS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// // NEED TO IMPLEMENT:
// // - Dot

// impl<'v> ndarray::linalg::Dot<Variable<'v>> for Variable<'v> {
//     type Output = Variable<'v>;

//     fn dot(&self, rhs: &Variable<'v>) -> Self::Output {
//         (*self) * (*rhs)
//     }
// }

// // impl<'v> ndarray::linalg::Dot<VariableVector<'v>> for VariableVector<'v> {
// //     type Output = VariableVector<'v>;

// //     fn dot(&self, rhs: &VariableVector<'v>) -> Self::Output {
// //         VariableVector {
// //             data: self.data.dot(&rhs.data),
// //         }
// //     }
// // }

// // impl<'v> ndarray::linalg::Dot<VariableArray<'v>> for VariableArray<'v> {
// //     type Output = VariableArray<'v>;

// //     fn dot(&self, rhs: &VariableArray<'v>) -> Self::Output {
// //         VariableArray {
// //             data: self.data.dot(&rhs.data),
// //         }
// //     }
// // }

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // UNIT TESTS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #[cfg(test)]
// mod test_ndarray {
//     use crate::autodiff::{gradient::Gradient, Accumulate};

//     #[test]
//     fn test_vector_dot_product() {
//         let g = crate::autodiff::Graph::new();

//         let (a, b, c, d) = (g.var(1.), g.var(2.), g.var(3.), g.var(4.));
//         let (e, f, g, h) = (g.var(5.), g.var(6.), g.var(7.), g.var(8.));

//         // a = [1, 2, 3, 4]
//         // b = [5, 6, 7, 8]
//         let _x = ndarray::array![a, b, c, d];
//         let _y = ndarray::array![e, f, g, h];

//         // DOT PRODUCT
//         // c = 1*5 + 2*6 + 3*7 + 4*8 = 70
//         // let c = x.dot(&y);
//         // let c_value = c.value;
//         // let c_expected = 70.;

//         // assert_eq!(c_value, c_expected);

//         // println!("c: {:?}", c);
//         // println!("c_value: {:?}", c_value);
//         // println!("c_expected: {:?}", c_expected);
//         // println!("gradient: {:?}", c.accumulate().wrt(&a));
//     }

//     #[test]
//     fn test_component_add() {
//         let g = crate::autodiff::Graph::new();

//         let (a, b, c, d) = (g.var(1.), g.var(2.), g.var(3.), g.var(4.));
//         let (e, f, g, h) = (g.var(5.), g.var(6.), g.var(7.), g.var(8.));

//         // a = [[1, 2],
//         //      [3, 4]]
//         // b = [[5, 6],
//         //      [7, 8]]
//         let x = ndarray::array![[a, b], [c, d]];
//         let y = ndarray::array![[e, f], [g, h]];

//         // COMPONENT-WISE ADDITION
//         // c = [[6 , 8],
//         //      [10, 12]]
//         let c = &x + &y;
//         let c_values = c.map(|x_i| x_i.value);
//         let c_expected = ndarray::array![[6., 8.], [10., 12.]];

//         assert_eq!(c, c_expected);

//         println!("c: {:?}", c);
//         println!("c_values: {:?}", c_values);
//         println!("c_expected: {:?}", c_expected);
//         println!("gradient: {:?}", c[[0, 0]].accumulate().wrt(&a));
//     }

//     #[test]
//     fn test_component_mul() {
//         let g = crate::autodiff::Graph::new();

//         let (a, b, c, d) = (g.var(1.), g.var(2.), g.var(3.), g.var(4.));
//         let (e, f, g, h) = (g.var(5.), g.var(6.), g.var(7.), g.var(8.));

//         // a = [[1, 2],
//         //      [3, 4]]
//         // b = [[5, 6],
//         //      [7, 8]]
//         let x = ndarray::array![[a, b], [c, d]];
//         let y = ndarray::array![[e, f], [g, h]];

//         // COMPONENT-WISE MULTIPLICATION
//         // c = [[5 , 12],
//         //      [21, 32]]
//         let c = &x * &y; // <--- This works fine.
//         let c_values = c.map(|x_i| x_i.value);
//         let c_expected = ndarray::array![[5., 12.], [21., 32.]];

//         // MATRIX MULTIPLICATION
//         // let dot = x.dot(&y); // <--- This does not work.

//         assert_eq!(c, c_expected);

//         println!("c: {:?}", c);
//         println!("c_values: {:?}", c_values);
//         println!("c_expected: {:?}", c_expected);
//         println!("gradient: {:?}", c[[0, 0]].accumulate().wrt(&a));
//     }
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GRAVEYARD
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// struct ARRAY<'v> {
//     graph: &'v Graph,
//     index: usize,
//     value: Array<f64, Ix2>,
// }

// struct MATRIX<'v> {
//     graph: &'v Graph,
//     index: usize,
//     value: DMatrix<f64>,
// }

// impl<'v> std::ops::Mul<MATRIX<'v>> for MATRIX<'v> {
//     type Output = MATRIX<'v>;

//     fn mul(self, rhs: MATRIX<'v>) -> Self::Output {
//         MATRIX {
//             graph: self.graph,
//             value: self.value * rhs.value,
//             index: self
//                 .graph
//                 .push(Arity::Binary, &[self.index, rhs.index], &[1.0, 1.0]),
//         }
//     }
// }

// impl<'v> std::ops::Mul<ARRAY<'v>> for ARRAY<'v> {
//     type Output = ARRAY<'v>;

//     fn mul(self, rhs: ARRAY<'v>) -> Self::Output {
//         ARRAY {
//             graph: self.graph,
//             value: self.value * rhs.value,
//             index: self
//                 .graph
//                 .push(Arity::Binary, &[self.index, rhs.index], &[1.0, 1.0]),
//         }
//     }
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// `ndarray` ops implementations
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// // Component-wise multiplication.
// impl<'v> std::ops::Mul<VariableArray<'v>> for VariableArray<'v> {
//     type Output = VariableArray<'v>;

//     fn mul(self, rhs: VariableArray<'v>) -> Self::Output {
//         VariableArray {
//             data: self.data * rhs.data,
//         }
//     }
// }

// // Component-wise addition.
// impl<'v> std::ops::Add<VariableArray<'v>> for VariableArray<'v> {
//     type Output = VariableArray<'v>;

//     fn add(self, rhs: VariableArray<'v>) -> Self::Output {
//         VariableArray {
//             data: self.data + rhs.data,
//         }
//     }
// }

// // Component-wise subtraction.
// impl<'v> std::ops::Sub<VariableArray<'v>> for VariableArray<'v> {
//     type Output = VariableArray<'v>;

//     fn sub(self, rhs: VariableArray<'v>) -> Self::Output {
//         VariableArray {
//             data: self.data - rhs.data,
//         }
//     }
// }

// // Component-wise division.
// impl<'v> std::ops::Div<VariableArray<'v>> for VariableArray<'v> {
//     type Output = VariableArray<'v>;

//     fn div(self, rhs: VariableArray<'v>) -> Self::Output {
//         VariableArray {
//             data: self.data / rhs.data,
//         }
//     }
// }

// Implementing `num-traits` traits for `Variable`.
// We need:
// - `num::One`
// - `num::Zero`

// static GRAPH: Lazy<Mutex<Graph>> = Lazy::new(|| {
//     Mutex::new(Graph {
//         vertices: RefCell::new(Vec::new()),
//     })
// });

// static ONE: Lazy<Variable> = Lazy::new(|| Variable::new(&*GRAPH.lock().unwrap(), 0, 1.));
// static ZERO: Lazy<Variable> = Lazy::new(|| Variable::new(&*GRAPH.lock().unwrap(), 0, 0.));

// static GRAPH: Lazy<Arc<Mutex<Graph>>> = Lazy::new(|| {
//     Arc::new(Mutex::new(Graph {
//         vertices: Vec::new(),
//     }))
// });

// static ONE: Lazy<Variable> = Lazy::new(|| Variable::new(Arc::clone(&GRAPH).lock().unwrap(), 0, 1.));
// static ZERO: Lazy<Variable> =
//     Lazy::new(|| Variable::new(Arc::clone(&GRAPH).lock().unwrap(), 0, 0.));

// static mut GRAPH: Graph = Graph {
//     vertices: RefCell::new(Vec::new()),
// };

// static ONE: Variable = Variable {
//     graph: unsafe { &GRAPH },
//     index: 0,
//     value: 1.,
// };

// static ZERO: Variable = Variable {
//     graph: unsafe { &GRAPH },
//     index: 0,
//     value: 0.,
// };

// impl<'v> num::One for Variable<'v> {
//     fn one() -> Self {
//         // Variable::new(&*GRAPH.lock().unwrap(), 0, 1.)
//         Variable::new(unsafe { &GRAPH }, 0, 1.)
//         // unsafe { Variable::new(&GRAPH, 0, 1.) }
//     }
// }

// impl<'v> num::One for VariableArray<'v> {
//     fn one() -> Self {
//         Self {
//             data: ndarray::Array2::ones((0, 0)),
//         }
//     }
// }

// impl<'v> num::Zero for Variable<'v> {
//     fn zero() -> Self {
//         // Variable::new(&*GRAPH.lock().unwrap(), 0, 1.)
//         Variable::new(unsafe { &GRAPH }, 0, 0.)
//         // unsafe { Variable::new(&GRAPH, 0, 0.) }
//     }

//     fn is_zero(&self) -> bool {
//         self.is_zero()
//     }
// }

// impl<'v> num::Zero for VariableArray<'v> {
//     fn zero() -> Self {
//         Self {
//             data: ndarray::Array2::zeros((0, 0)),
//         }
//     }

//     fn is_zero(&self) -> bool {
//         self.data.iter().all(|x| x.is_zero())
//     }
// }

// impl<'v> Dot<VariableArray<'v>> for VariableArray<'v> {
//     type Output = VariableArray<'v>;

//     fn dot(&self, rhs: &VariableArray<'v>) -> Self::Output {
//         use num::Zero;

//         assert!(self.data.ncols() == rhs.data.nrows());

//         let mut data = Array::zeros((self.data.nrows(), rhs.data.ncols()));

//         for i in 0..self.data.nrows() {
//             for j in 0..rhs.data.ncols() {
//                 let mut sum = Variable::zero();

//                 for k in 0..self.data.ncols() {
//                     sum += self.data[(i, k)] * rhs.data[(k, j)];
//                 }

//                 data[(i, j)] = sum;
//             }
//         }

//         VariableArray { data }
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::autodiff::variable::Variable;
// use nalgebra::{DMatrix, DVector};

// /// A matrix of `Variable`s.
// pub struct VariableMatrix<'v> {
//     data: DMatrix<Variable<'v>>,
// }

// /// A vector of `Variable`s.
// pub struct VariableVector<'v> {
//     data: DVector<Variable<'v>>,
// }

// impl std::ops::Add<VariableMatrix<'static>> for VariableMatrix<'static> {
//     type Output = VariableMatrix<'static>;

//     fn add(self, rhs: VariableMatrix<'static>) -> Self::Output {
//         VariableMatrix {
//             data: self.data + rhs.data,
//         }
//     }
// }

// #[cfg(test)]
// mod tests_nalgebra_variable {
//     use crate::autodiff::Graph;

//     use super::*;

//     #[test]
//     fn test_matrix() {
//         let graph = Graph::new();

//         let a = VariableMatrix {
//             data: DMatrix::from_row_slice(2, 2, &graph.vars(&[1.0, 2.0, 3.0, 4.0])),
//         };
//         let b = VariableMatrix {
//             data: DMatrix::from_row_slice(2, 2, &graph.vars(&[5.0, 6.0, 7.0, 8.0])),
//         };

//         let c = a + b;

//         println!("{:?}", c.data);

//         // assert_eq!(
//         //     c.data,
//         //     DMatrix::from_row_slice(2, 2, &[5.0, 12.0, 21.0, 32.0])
//         // );
//     }
// }

// impl<'v> VariableMatrix<'v> {
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
// }

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

// impl<'v> Accumulate<Array2<Vec<f64>>> for VariableArray<'v> {
//     /// Function to reverse accumulate the gradient
//     /// for an `Array2<Variable<'v>>`.
//     #[inline]
//     fn accumulate(&self) -> Array2<Vec<f64>> {
//         let mut adjoints = Array2::from_elem(self.data.dim(), Vec::new());

//         for ((row, col), variable) in self.data.indexed_iter() {
//             adjoints[(row, col)] = variable.accumulate();
//         }

//         adjoints
//     }
// }
