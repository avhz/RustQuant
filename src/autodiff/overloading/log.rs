// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variable::Variable, vertex::Arity};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: LOGARITHM
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Overload the `Log` trait.
pub trait Log<T> {
    /// Return type of `Log`
    type Output;

    /// Overloaded `log` function.
    fn log(&self, base: T) -> Self::Output;
}

// log_{Variable<'v>}(Variable<'v>)
// f(x,y) = log_{x}(y)
// df/dx = -ln(y) / (x * ln^2(x))
// df/dy = 1 / (y * ln(x))
impl<'v> Log<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn log(&self, base: Variable) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: f64::log(self.value, base.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, base.index],
                &[
                    -f64::ln(self.value) / (base.value * f64::ln(base.value).powi(2)),
                    1.0 / (self.value * f64::ln(base.value)),
                ],
            ),
        }
    }
}

// log_{Variable<'v>}(f64)
// f(x,y) = log_{x}(b)
// df/dx = -ln(b) / (x * ln^2(x))
// df/db = 0
impl<'v> Log<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn log(&self, base: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: base.graph,
            value: f64::log(*self, base.value),
            index: base.graph.push(
                Arity::Binary,
                &[base.index, base.index],
                &[
                    -f64::ln(*self) / (base.value * f64::ln(base.value).powi(2)),
                    0.0,
                ],
            ),
        }
    }
}

// log_{f64}(Variable<'v>)
// f(x,y) = log_{b}(y)
// df/db = 0
// df/dy = 1 / (y * ln(b))
impl<'v> Log<f64> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn log(&self, base: f64) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: f64::log(self.value, base),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[0.0, 1.0 / (f64::ln(base) * self.value)],
            ),
        }
    }
}
