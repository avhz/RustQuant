// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variable::Variable, vertex::Arity};
use std::ops::{Div, DivAssign};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard division operator (`/`).
/// d/dx x/y = 1/y
/// d/dy x/y = -x/y^2

/// DivAssign: Variable<'v> /= Variable<'v>
impl<'v> DivAssign<Variable<'v>> for Variable<'v> {
    #[inline]
    fn div_assign(&mut self, other: Variable<'v>) {
        assert!(std::ptr::eq(self.graph, other.graph));

        *self = *self / other;
    }
}

/// DivAssign: Variable<'v> /= f64
impl<'v> DivAssign<f64> for Variable<'v> {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

/// DivAssign: f64 /= Variable<'v>
impl<'v> DivAssign<Variable<'v>> for f64 {
    #[inline]
    fn div_assign(&mut self, other: Variable<'v>) {
        *self = *self / other.value;
    }
}

/// Variable<'v> / Variable<'v>
impl<'v> Div<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///  
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let y = g.var(2.0);
    /// let z = x / y;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 5.0 / 2.0);
    /// assert_eq!(grad.wrt(&x), 1.0 / 2.0);
    /// assert_eq!(grad.wrt(&y), - 5.0 / (2.0 * 2.0));
    /// ```
    #[inline]
    fn div(self, other: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        self * other.recip()
    }
}

/// Variable<'v> / f64
impl<'v> Div<f64> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let a = 2.0;
    /// let z = x / a;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 5.0 / 2.0);
    /// assert_eq!(grad.wrt(&x), 1.0 / 2.0);
    /// ```
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: f64) -> Self::Output {
        self * other.recip()
    }
}

/// f64 / Variable<'v>
impl<'v> Div<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///   
    /// let g = Graph::new();
    ///
    /// let a = 5.0;
    /// let x = g.var(2.0);
    /// let z = a / x;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 5.0 / 2.0);
    /// assert_eq!(grad.wrt(&x), - 5.0 / (2.0*2.0));
    /// ```
    #[inline]
    fn div(self, other: Variable<'v>) -> Self::Output {
        Variable {
            graph: other.graph,
            value: self / other.value,
            index: other.graph.push(
                Arity::Binary,
                &[other.index, other.index],
                &[0.0, -self / (other.value * other.value)],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    use crate::autodiff::{Gradient, Graph};

    #[test]
    fn test_div() {
        // Variable / Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x / y;
        let grad = z.accumulate();

        assert_eq!(z.value, 0.5);
        assert_eq!(grad.wrt(&x), 0.5);
        assert_eq!(grad.wrt(&y), -0.25);

        // Variable / f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x / y;
        let grad = z.accumulate();

        assert_eq!(z.value, 0.5);
        assert_eq!(grad.wrt(&x), 0.5);

        // f64 / Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x / y;
        let grad = z.accumulate();

        assert_eq!(z.value, 0.5);
        assert_eq!(grad.wrt(&y), -0.25);
    }
}
