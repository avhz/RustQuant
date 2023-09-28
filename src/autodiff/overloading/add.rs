// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variables::variable::Variable, vertex::Arity};
use std::ops::{Add, AddAssign};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard addition operator (`+`).
/// d/dx x + y = 1
/// d/dy x + y = 1

/// AddAssign: Variable<'v> += Variable<'v>
impl<'v> AddAssign<Variable<'v>> for Variable<'v> {
    #[inline]
    fn add_assign(&mut self, other: Variable<'v>) {
        assert!(std::ptr::eq(self.graph, other.graph));

        *self = *self + other;
    }
}

/// AddAssign: Variable<'v> += f64
impl<'v> AddAssign<f64> for Variable<'v> {
    #[inline]
    fn add_assign(&mut self, other: f64) {
        *self = *self + other;
    }
}

/// AddAssign: f64 += Variable<'v>
impl<'v> AddAssign<Variable<'v>> for f64 {
    #[inline]
    fn add_assign(&mut self, other: Variable<'v>) {
        *self = *self + other.value;
    }
}

/// Variable<'v> + Variable<'v>
impl<'v> Add<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let y = g.var(2.0);
    /// let z = x + y;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 7.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// assert_eq!(grad.wrt(&y), 1.0);
    /// ```
    #[inline]
    fn add(self, other: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        Variable {
            graph: self.graph,
            value: self.value + other.value,
            index: self
                .graph
                .push(Arity::Binary, &[self.index, other.index], &[1.0, 1.0]),
        }
    }
}

/// Variable<'v> + f64
impl<'v> Add<f64> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(2.0);
    /// let a = 5.0;
    /// let z = x + a;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 7.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    #[inline]
    fn add(self, other: f64) -> Self::Output {
        Variable {
            graph: self.graph,
            value: self.value + other,
            index: self
                .graph
                .push(Arity::Binary, &[self.index, self.index], &[1.0, 0.0]),
        }
    }
}

/// f64 + Variable<'v>
impl<'v> Add<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    /// ```
    /// use RustQuant::autodiff::*;    
    ///
    /// let g = Graph::new();
    ///
    /// let a = 5.0;
    /// let x = g.var(2.0);
    /// let z = a + x;
    ///
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 7.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    #[inline]
    fn add(self, other: Variable<'v>) -> Self::Output {
        other + self
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    use crate::autodiff::{Accumulate, Gradient, Graph};

    #[test]
    fn test_add() {
        // Variable + Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x + y;
        let grad = z.accumulate();

        assert_eq!(z.value, 3.0);
        assert_eq!(grad.wrt(&x), 1.0);
        assert_eq!(grad.wrt(&y), 1.0);

        // Variable + f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x + y;
        let grad = z.accumulate();

        assert_eq!(z.value, 3.0);
        assert_eq!(grad.wrt(&x), 1.0);

        // f64 + Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x + y;
        let grad = z.accumulate();

        assert_eq!(z.value, 3.0);
        assert_eq!(grad.wrt(&y), 1.0);
    }
}
