// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Overloading functions from `statrs` crate.

use crate::autodiff::{variables::variable::Variable, vertex::Arity};
use std::f64::consts::PI;
use std::ops::Neg;

impl<'v> Variable<'v> {
    /// Error function.
    /// d/dx erf(x) = 2e^(-x^2) / sqrt(PI)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.erf();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.84270079294, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 0.41510749742, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn erf(self) -> Self {
        use statrs::function::erf::erf;

        Variable {
            graph: self.graph,
            value: erf(self.value),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[2.0 * self.value.powi(2).neg().exp() / PI.sqrt()],
            ),
        }
    }

    /// Error function (complementary).
    /// d/dx erfc(x) = -2e^(-x^2) / sqrt(PI)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.erfc();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,       0.15729920705, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), -0.41510749742, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn erfc(self) -> Self {
        use statrs::function::erf::erfc;

        Variable {
            graph: self.graph,
            value: erfc(self.value),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((2.0 * self.value.powi(2).neg().exp()).neg() / PI.sqrt())],
            ),
        }
    }
}
