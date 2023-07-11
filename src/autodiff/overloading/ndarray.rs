// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{graph::Graph, variable::Variable, vertex::Arity};
use nalgebra::DMatrix;
use ndarray::{Array, Ix2};

/// A matrix of `Variable`s.
#[derive(Debug, Clone, PartialEq, Eq)]
struct VariableArray<'v> {
    data: Array<Variable<'v>, Ix2>,
}

struct ARRAY<'v> {
    graph: &'v Graph,
    index: usize,
    value: Array<f64, Ix2>,
}

struct MATRIX<'v> {
    graph: &'v Graph,
    index: usize,
    value: DMatrix<f64>,
}

impl<'v> std::ops::Mul<MATRIX<'v>> for MATRIX<'v> {
    type Output = MATRIX<'v>;

    fn mul(self, rhs: MATRIX<'v>) -> Self::Output {
        MATRIX {
            graph: self.graph,
            value: self.value * rhs.value,
            index: self
                .graph
                .push(Arity::Binary, &[self.index, rhs.index], &[1.0, 1.0]),
        }
    }
}

impl<'v> std::ops::Mul<ARRAY<'v>> for ARRAY<'v> {
    type Output = ARRAY<'v>;

    fn mul(self, rhs: ARRAY<'v>) -> Self::Output {
        ARRAY {
            graph: self.graph,
            value: self.value * rhs.value,
            index: self
                .graph
                .push(Arity::Binary, &[self.index, rhs.index], &[1.0, 1.0]),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// `ndarray` ops implementations
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Component-wise multiplication.
impl<'v> std::ops::Mul<VariableArray<'v>> for VariableArray<'v> {
    type Output = VariableArray<'v>;

    fn mul(self, rhs: VariableArray<'v>) -> Self::Output {
        VariableArray {
            data: self.data * rhs.data,
        }
    }
}

// Component-wise addition.
impl<'v> std::ops::Add<VariableArray<'v>> for VariableArray<'v> {
    type Output = VariableArray<'v>;

    fn add(self, rhs: VariableArray<'v>) -> Self::Output {
        VariableArray {
            data: self.data + rhs.data,
        }
    }
}

// Component-wise subtraction.
impl<'v> std::ops::Sub<VariableArray<'v>> for VariableArray<'v> {
    type Output = VariableArray<'v>;

    fn sub(self, rhs: VariableArray<'v>) -> Self::Output {
        VariableArray {
            data: self.data - rhs.data,
        }
    }
}

// Component-wise division.
impl<'v> std::ops::Div<VariableArray<'v>> for VariableArray<'v> {
    type Output = VariableArray<'v>;

    fn div(self, rhs: VariableArray<'v>) -> Self::Output {
        VariableArray {
            data: self.data / rhs.data,
        }
    }
}

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

#[cfg(test)]
mod test_ndarray {
    use crate::autodiff::gradient::Gradient;

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
        let c = &x * &y; // <--- This works fine.
        let c_values = c.map(|x_i| x_i.value);
        let c_expected = ndarray::array![[5., 12.], [21., 32.]];

        // MATRIX MULTIPLICATION
        // let dot = x.dot(&y); // <--- This does not work.

        assert_eq!(c, c_expected);

        println!("c: {:?}", c);
        println!("c_values: {:?}", c_values);
        println!("c_expected: {:?}", c_expected);
        println!("gradient: {:?}", c[[0, 0]].accumulate().wrt(&a));
    }
}
