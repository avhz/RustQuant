// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variable::Variable, vertex::Arity};
use std::ops::{Mul, MulAssign};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard multiplication operator (`*`).
/// d/dx x * y = y
/// d/dy x * y = x

/// MulAssign: Variable<'v> *= Variable<'v>
impl<'v> MulAssign<Variable<'v>> for Variable<'v> {
    #[inline]
    fn mul_assign(&mut self, other: Variable<'v>) {
        assert!(std::ptr::eq(self.graph, other.graph));

        *self = *self * other;
    }
}

/// MulAssign: Variable<'v> *= f64
impl<'v> MulAssign<f64> for Variable<'v> {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

/// MulAssign: f64 *= Variable<'v>
impl<'v> MulAssign<Variable<'v>> for f64 {
    #[inline]
    fn mul_assign(&mut self, other: Variable<'v>) {
        *self = *self * other.value;
    }
}

/// Variable<'v> * Variable<'v>
impl<'v> Mul<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let y = g.var(2.0);
    /// let z = x * y;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 10.0);
    /// assert_eq!(grad.wrt(&x), 2.0);
    /// assert_eq!(grad.wrt(&y), 5.0);
    /// ```
    #[inline]
    fn mul(self, other: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        Variable {
            graph: self.graph,
            value: self.value * other.value,
            index: self.graph.push(
                Arity::Binary,
                &[self.index, other.index],
                &[other.value, self.value],
            ),
        }
    }
}

/// Variable<'v> * f64
impl<'v> Mul<f64> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let a = 2.0;
    /// let z = x * a;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 10.0);
    /// assert_eq!(grad.wrt(&x), 2.0);
    /// ```
    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        Variable {
            graph: self.graph,
            value: self.value * other,
            index: self
                .graph
                .push(Arity::Binary, &[self.index, self.index], &[other, 0.0]),
        }
    }
}

/// f64 * Variable<'v>
impl<'v> Mul<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let a = 5.0;
    /// let x = g.var(2.0);
    /// let z = a * x;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 10.0);
    /// assert_eq!(grad.wrt(&x), 5.0);
    /// ```
    #[inline]
    fn mul(self, other: Variable<'v>) -> Self::Output {
        other * self
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    use crate::autodiff::{Gradient, Graph};

    #[test]
    fn test_mul() {
        // Variable * Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x * y;
        let grad = z.accumulate();

        assert_eq!(z.value, 2.0);
        assert_eq!(grad.wrt(&x), 2.0);
        assert_eq!(grad.wrt(&y), 1.0);

        // Variable * f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x * y;
        let grad = z.accumulate();

        assert_eq!(z.value, 2.0);
        assert_eq!(grad.wrt(&x), 2.0);

        // f64 * Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x * y;
        let grad = z.accumulate();

        assert_eq!(z.value, 2.0);
        assert_eq!(grad.wrt(&y), 1.0);
    }
}
