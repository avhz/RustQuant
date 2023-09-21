// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variables::variable::Variable, vertex::Arity, vertex::Operation};
use std::ops::{Add, Neg, Sub, SubAssign};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard subtraction operator (`-`).
/// d/dx x - y = 1
/// d/dy x - y = -1

/// SubAssign: Variable<'v> -= Variable<'v>
impl<'v> SubAssign<Variable<'v>> for Variable<'v> {
    #[inline]
    fn sub_assign(&mut self, other: Variable<'v>) {
        assert!(std::ptr::eq(self.graph, other.graph));

        *self = *self - other;
    }
}

/// SubAssign: Variable<'v> -= f64
impl<'v> SubAssign<f64> for Variable<'v> {
    #[inline]
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other;
    }
}

/// SubAssign: f64 -= Variable<'v>
impl<'v> SubAssign<Variable<'v>> for f64 {
    #[inline]
    fn sub_assign(&mut self, other: Variable<'v>) {
        *self = *self - other.value;
    }
}

/// Variable<'v> - Variable<'v>
impl<'v> Sub<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;    
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let y = g.var(2.0);
    /// let z = x - y;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 3.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// assert_eq!(grad.wrt(&y), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        self.add(other.neg())
    }
}

/// Variable<'v> - f64
impl<'v> Sub<f64> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let a = 2.0;
    /// let z = x - a;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 3.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    #[inline]
    fn sub(self, other: f64) -> Self::Output {
        self.add(other.neg())
    }
}

/// f64 - Variable<'v>
impl<'v> Sub<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;   
    ///  
    /// let g = Graph::new();
    ///
    /// let a = 5.0;
    /// let x = g.var(2.0);
    /// let z = a - x;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 3.0);
    /// assert_eq!(grad.wrt(&x), -1.0);
    /// ```
    #[inline]
    fn sub(self, other: Variable<'v>) -> Self::Output {
        Variable {
            graph: other.graph,
            value: self - other.value,
            index: other.graph.push(
                Arity::Binary,
                &[other.index, other.index],
                &[0.0, -1.0],
                Operation::_SUB,
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    use crate::autodiff::{Accumulate, Gradient, Graph};

    #[test]
    fn test_sub() {
        // Variable - Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x - y;
        let grad = z.accumulate();

        assert_eq!(z.value, -1.0);
        assert_eq!(grad.wrt(&x), 1.0);
        assert_eq!(grad.wrt(&y), -1.0);

        // Variable - f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x - y;
        let grad = z.accumulate();

        assert_eq!(z.value, -1.0);
        assert_eq!(grad.wrt(&x), 1.0);

        // f64 - Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x - y;
        let grad = z.accumulate();

        assert_eq!(z.value, -1.0);
        assert_eq!(grad.wrt(&y), -1.0);
    }
}
