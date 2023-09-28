// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variables::variable::Variable, vertex::Arity};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: POWER FUNCTION TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Overload the `Powf` trait.
pub trait Powf<T> {
    /// Return type of `Powf`
    type Output;
    /// Overloaded `powf` function.
    fn powf(&self, other: T) -> Self::Output;
}

// Variable<'v> ^ Variable<'v>
impl<'v> Powf<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn powf(&self, other: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.powf(other.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, other.index],
                &[
                    other.value * f64::powf(self.value, other.value - 1.),
                    f64::powf(self.value, other.value) * f64::ln(self.value),
                ],
            ),
        }
    }
}

// Variable<'v> ^ f64
impl<'v> Powf<f64> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn powf(&self, n: f64) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: f64::powf(self.value, n),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[n * f64::powf(self.value, n - 1.0), 0.0],
            ),
        }
    }
}

// f64 ^ Variable<'v>
impl<'v> Powf<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn powf(&self, other: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: other.graph,
            value: f64::powf(*self, other.value),
            index: other.graph.push(
                Arity::Binary,
                &[other.index, other.index],
                &[0.0, other.value * f64::powf(*self, other.value - 1.0)],
            ),
        }
    }
}

/// Overload the `Powi` trait.
pub trait Powi<T> {
    /// Return type of `Powi`
    type Output;

    /// Overloaded `powi` function.
    fn powi(&self, other: T) -> Self::Output;
}

// Variable<'v> ^ Variable<'v>
impl<'v> Powi<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn powi(&self, other: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.powf(other.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, other.index],
                &[
                    other.value * f64::powf(self.value, other.value - 1.),
                    f64::powf(self.value, other.value) * f64::ln(self.value),
                ],
            ),
        }
    }
}

// Variable<'v> ^ f64
impl<'v> Powi<i32> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn powi(&self, n: i32) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: f64::powi(self.value, n),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[n as f64 * f64::powi(self.value, n - 1), 0.0],
            ),
        }
    }
}

// f64 ^ Variable<'v>
impl<'v> Powi<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn powi(&self, other: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: other.graph,
            value: f64::powf(*self, other.value),
            index: other.graph.push(
                Arity::Binary,
                &[other.index, other.index],
                &[0.0, other.value * f64::powf(*self, other.value - 1.0)],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    // use crate::autodiff::{Graph, Variable};

    // #[test]
    // fn test_powf() {
    //     let g = Graph::new();

    //     let x = g.var(2.0);
    //     let y = 3.0.powf(x);

    //     assert!((y.value() - 9.0).abs() < EPSILON);

    //     let derivs = y.accumulate();
    //     assert_approx_equal!(derivs.wrt(&x), (9.0 * (3.0f64).ln()), 1e-8);
    // }

    // #[test]
    // fn test_powf_zero() {
    //     let g = Graph::new();

    //     let x = g.var(0.0);
    //     let y = 3.0.powf(x);

    //     assert_approx_equal!(y.value(), 1.0, 1e-8); // 3^1 = 3.0

    //     let derivs = y.accumulate();
    //     assert!((derivs.wrt(&x) - 0.0).abs() < EPSILON);
    // }

    // #[test]
    // fn test_powf_one() {
    //     let g = Graph::new();

    //     let x = g.var(1.0); // create a variable
    //     let y = 3.0.powf(x); // powf operation

    //     assert_approx_equal!(y.value(), 3.0, 1e-8); // 3^1 = 3.0

    //     let derivs = y.accumulate(); // d/dx 3^x = 3^x * ln(3)
    //     assert_approx_equal!(derivs.wrt(&x), (27.0f64).ln(), 1e-8);
    // }
}
