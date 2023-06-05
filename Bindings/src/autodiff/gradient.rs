// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the `Gradient` trait.
//! Each implementation of `wrt` returns the chosen partial derivatives.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::variable::Variable;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GRADIENT STRUCT AND IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// /// Struct containing the gradient (vector of partial derivatives).
// pub struct Gradient {
//     pub adjoints: Vec<f64>,
// }

// impl Gradient {
//     /// Function to retrieve the partial derivative *with-respect-to*
//     /// the chosen input variable.
//     pub fn wrt<'v>(&self, var: Variable<'v>) -> f64 {
//         self.adjoints[var.index]
//     }
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GRADIENT TRAIT AND IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Return the derivative/s *with-respect-to* the chosen variables.
pub trait Gradient<T, S> {
    /// Returns the derivative/s *with-respect-to* the chosen variables.
    fn wrt(&self, v: T) -> S;
}

/// `wrt` a single variable.
impl<'v> Gradient<&Variable<'v>, f64> for Vec<f64> {
    #[inline]
    fn wrt(&self, v: &Variable) -> f64 {
        self[v.index]
    }
}

/// `wrt` a borrowed vector of variables.
impl<'v> Gradient<&Vec<Variable<'v>>, Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, v: &Vec<Variable<'v>>) -> Vec<f64> {
        let mut gradient = Vec::with_capacity(v.len());
        for i in v {
            gradient.push(self.wrt(i));
        }
        gradient
    }
}

/// `wrt` a borrowed slice of variables.
impl<'v> Gradient<&[Variable<'v>], Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, v: &[Variable<'v>]) -> Vec<f64> {
        let mut gradient = Vec::with_capacity(v.len());

        for i in v {
            gradient.push(self.wrt(i));
        }
        gradient
    }
}

/// `wrt` an array of variables.
impl<'v, const N: usize> Gradient<[Variable<'v>; N], Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, v: [Variable<'v>; N]) -> Vec<f64> {
        let mut gradient = Vec::with_capacity(N);
        for i in v {
            gradient.push(self.wrt(&i));
        }
        gradient
    }
}

/// `wrt` a borrowed array of variables.
impl<'v, const N: usize> Gradient<&[Variable<'v>; N], Vec<f64>> for Vec<f64> {
    #[inline]
    fn wrt(&self, v: &[Variable<'v>; N]) -> Vec<f64> {
        let mut gradient = Vec::with_capacity(N);

        for i in v {
            gradient.push(self.wrt(i));
        }
        gradient
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS SECTION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_gradient {
    use crate::{
        assert_approx_equal,
        autodiff::*,
        // distributions::{Distribution, Gaussian},
    };

    #[test]
    fn x_times_y_plus_sin_x() {
        let t = Tape::new();

        let x = t.var(69.0);
        let y = t.var(420.0);

        let z = x * y + x.sin();

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 28979.885215, 1e-6);
        assert_approx_equal!(grad.wrt(&x), y.value + x.value.cos(), 1e-15);
        assert_approx_equal!(grad.wrt(&y), x.value, 1e-15);
    }

    #[test]
    fn x_times_y_plus_tan_x() {
        let t = Tape::new();

        let x = t.var(1.0);
        let y = t.var(2.0);

        let z = x * y + x.tan();

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 3.5574077246549, 1e-14);
        assert_approx_equal!(grad.wrt(&x), 5.4255188208147597, 1e-15);
        assert_approx_equal!(grad.wrt(&y), 1.0, 1e-15);
    }

    #[test]
    fn cosh_x_times_y() {
        let t = Tape::new();

        let x = t.var(1.0);
        let y = t.var(2.0);

        let z = (x * y).cosh();

        let grad = z.accumulate();

        println!("{}", grad.wrt(&x));

        assert_approx_equal!(z.value, 3.762195691083631459, 1e-10);
        assert_approx_equal!(grad.wrt(&x), 7.2537208156940375, 1e-10);
        assert_approx_equal!(grad.wrt(&y), 3.62686040784701, 1e-10);
    }

    #[test]
    fn cosh_xy_div_tanh_x_times_sinh_y() {
        let t = Tape::new();

        let x = t.var(1.0);
        let y = t.var(2.0);

        let z = (x * y).cosh() / (x.tanh() * y.sinh());

        let grad = z.accumulate();

        assert_approx_equal!(z.value, 1.3620308304831552, 1e-8);
        assert_approx_equal!(grad.wrt(&x), 1.87499075136386965, 1e-15);
        assert_approx_equal!(grad.wrt(&y), -0.099819345045613269, 1e-15);
    }

    #[test]
    fn test_block_assign() {
        let t = Tape::new();

        let x = t.var(1.0);
        let y = t.var(2.0);

        let f = {
            let z = x.sin() + y.tan();
            z.exp()
        };

        let grad = f.accumulate();

        println!("Grad wrt x = 1.0: \t{}", grad.wrt(&x));
        println!("Grad wrt y = 2.0: \t{}", grad.wrt(&y));

        assert_approx_equal!(grad.wrt(&x), 0.1409718084254616945815, 1e-15);
        assert_approx_equal!(grad.wrt(&y), 1.5066148885971964908277, 1e-15);
    }

    #[test]
    fn test_closure() {
        let t = Tape::new();

        let x = t.var(1.0);
        let y = t.var(2.0);

        let z = || (x * y).cosh() / (x.tanh() * y.sinh());

        let grad = z().accumulate();

        assert_approx_equal!(z().value, 1.3620308304831552, 1e-8);
        assert_approx_equal!(grad.wrt(&x), 1.87499075136386965, 1e-15);
        assert_approx_equal!(grad.wrt(&y), -0.099819345045613269, 1e-15);
    }

    #[test]
    fn test_function_input() {
        fn diff_fn<'v>(params: &[Variable<'v>], data: &[f64]) -> Variable<'v> {
            params[0].powf(params[1]) + data[0].sin() - params[2].asinh() / data[1]
        }

        let tape = Tape::new();
        let params = tape.vars(&[3.0, 2.0, 1.0]);
        let data = [1., 2.];
        let result = diff_fn(&params, &data);
        let gradients = result.accumulate();
        println!("{:?}", gradients.wrt(&params));
        println!("{:?}", gradients);
    }
}
