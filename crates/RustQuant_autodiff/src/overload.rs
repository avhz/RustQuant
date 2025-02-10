// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::DiffOps;
use crate::{variable::Variable, vertex::Arity};
use std::iter::{Product, Sum};
use std::ops::Neg;
use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

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
impl<'v, T> Add<Variable<'v, T>> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;
    /// ```
    /// # use RustQuant_autodiff::*;
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
    fn add(self, other: Variable<'v, T>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        Variable {
            graph: self.graph,
            value: self.value + other.value,
            index: self.graph.push(
                Arity::Binary,
                &[self.index, other.index],
                &[T::one(), T::one()],
            ),
        }
    }
}

/// Variable<'v> + f64
impl<'v, T> Add<f64> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[T::one(), T::zero()],
            ),
        }
    }
}

/// f64 + Variable<'v>
impl<'v, T> Add<Variable<'v, T>> for f64
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;    
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
    fn add(self, other: Variable<'v, T>) -> Self::Output {
        other + self
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard division operator (`/`).
/// d/dx x/y = 1/y
/// d/dy x/y = -x/y^2

/// `DivAssign`: Variable<'v> /= Variable<'v>
impl<'v> DivAssign<Variable<'v>> for Variable<'v> {
    #[inline]
    fn div_assign(&mut self, other: Variable<'v>) {
        assert!(std::ptr::eq(self.graph, other.graph));

        *self = *self / other;
    }
}

/// `DivAssign`: Variable<'v> /= f64
impl<'v> DivAssign<f64> for Variable<'v> {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

/// `DivAssign`: f64 /= Variable<'v>
impl<'v> DivAssign<Variable<'v>> for f64 {
    #[inline]
    fn div_assign(&mut self, other: Variable<'v>) {
        *self = *self / other.value;
    }
}

/// Variable<'v> / Variable<'v>
impl<'v, T> Div<Variable<'v, T>> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
    fn div(self, other: Variable<'v, T>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        self * other.recip()
    }
}

/// Variable<'v> / f64
impl<'v, T> Div<f64> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
    /// # use RustQuant_autodiff::*;
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
// OVERLOADING: STANDARD MATH OPERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v, T> std::ops::Neg for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: PRIMITIVE FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v, T> Variable<'v, T>
where
    T: DiffOps,
{
    /// Absolute value function.
    /// d/dx abs(x) = sign(x)
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x1 = g.var(1.0);
    /// let z1 = x1.abs();
    /// let grad1 = z1.accumulate();
    /// assert!((z1.value - 1.0).abs() <= 1e-15);
    /// assert!((grad1.wrt(&x1) - 1.0).abs() <= 1e-15);
    ///
    /// let x2 = g.var(-1.0);
    /// let z2 = x2.abs();
    /// let grad2 = z2.accumulate();
    /// assert!((z2.value - 1.0).abs() <= 1e-15);
    /// assert!((grad2.wrt(&x2) - (-1.0)).abs() <= 1e-15);
    /// ```
    #[must_use]
    #[inline]
    pub fn abs(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.abs(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.signum()]),
        }
    }

    /// Inverse cosine function.
    /// d/dx cos^-1(x) = - 1 / sqrt(1 - x^2)
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x1 = g.var(0.0);
    /// let z1 = x1.acos();
    /// let grad1 = z1.accumulate();
    /// assert!((z1.value - 1.5707963267948966).abs() <= 1e-15);
    /// assert!((grad1.wrt(&x1) - (-1.0)).abs() <= 1e-15);
    /// ```
    #[must_use]
    #[inline]
    pub fn acos(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.acos(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((T::one() - self.value.powi(2)).sqrt()).recip().neg()],
            ),
        }
    }

    /// Inverse hyperbolic cosine function.
    /// d/dx cosh^-1(x) = 1 / ( sqrt(x-1) * sqrt(x+1) )
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let z = x.acosh();
    /// let grad = z.accumulate();
    /// assert!((z.value - 2.2924316695611777).abs() <= 1e-15);
    /// assert!((grad.wrt(&x) - 0.20412414523193150818).abs() <= 1e-15);
    /// ```
    #[must_use]
    #[inline]
    pub fn acosh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.acosh(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[
                    ((self.value - T::one()).sqrt() * (self.value + T::one()).sqrt())
                        .recip(),
                ],
            ),
        }
    }

    /// Inverse sine function.
    /// d/dx sin^-1(x) = 1 / sqrt(1 - x^2)
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(0.0);
    /// let z = x.asin();
    /// let grad = z.accumulate();
    ///
    /// //assert_eq!(z.value, std::f64::consts::PI / 2.0);
    /// assert_eq!(z.value, 0.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    #[must_use]
    #[inline]
    pub fn asin(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.asin(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.asin_diff()]),
        }
    }

    /// Inverse hyperbolic sine function.
    /// d/dx sinh^-1(x) = 1 / sqrt(1 + x^2)
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(0.0);
    /// let z = x.asinh();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 0.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    #[must_use]
    #[inline]
    pub fn asinh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.asinh(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((self.value.powi(2) + 1.0).sqrt()).recip()],
            ),
        }
    }

    /// Inverse tangent function.
    /// d/dx tan^-1(x) = 1 / (1 + x^2)
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(0.0);
    /// let z = x.atan();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 0.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    #[must_use]
    #[inline]
    pub fn atan(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.atan(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((self.value.powi(2) + 1.0).recip())],
            ),
        }
    }

    /// Inverse hyperbolic tangent function.
    /// d/dx tanh^-1(x) = 1 / (1 + x^2)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(0.0);
    /// let z = x.atanh();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.00000000000, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 1.00000000000, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn atanh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.atanh(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[-(self.value.powi(2) - 1.0).recip()],
            ),
        }
    }

    /// Cuberoot function.
    /// d/dx cuberoot(x) = 1 / ( 3 * x^(2/3) )
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.cbrt();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      1.00000000000, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 0.33333333333, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn cbrt(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cbrt(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.cbrt() / (self.value * 3.0)],
            ),
        }
    }

    /// Cosine function.
    /// d/dx cos(x) = -sin(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.cos();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,       0.54030230586, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), -0.84147098480, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn cos(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cos(),
            // index: self.graph.push_unary(self.index, self.value.sin().neg()),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.sin().neg()]),
        }
    }

    /// Inverse hyperbolic cosine function.
    /// d/dx cosh(x) = sinh(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.cosh();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      1.54308063481, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 1.17520119364, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn cosh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cosh(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.sinh()]),
        }
    }

    /// Exponential function (base *e*).
    /// d/dx exp(x) = exp(x) = y
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///  
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.exp();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, std::f64::consts::E);
    /// assert_eq!(grad.wrt(&x), std::f64::consts::E);
    /// ```
    #[must_use]
    #[inline]
    pub fn exp(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.exp()]),
        }
    }

    /// Exponential function (base 2)
    /// d/dx 2^x = 2^x * ln(2)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.exp2();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      2.00000000000, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 1.38629436111, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn exp2(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp2(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.exp2() * 2_f64.ln()],
            ),
        }
    }

    /// Exponential function minus 1 function.
    /// d/dx exp(x) - 1 = exp(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.exp_m1();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      1.71828182845, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 2.71828182845, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn exp_m1(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp_m1(),
            // index: self.graph.push_unary(self.index, self.value.exp()),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.exp()]),
        }
    }

    /// Logarithm (natural)  of `x`.
    /// d/dx ln(x) = 1 / x
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///   
    /// let g = Graph::new();
    ///
    /// let x = g.var(std::f64::consts::E);
    /// let z = x.ln();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 1.0);
    /// assert_eq!(grad.wrt(&x), 0.36787944117144233);
    /// ```
    #[must_use]
    #[inline]
    pub fn ln(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.ln(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.recip()]),
        }
    }

    /// Logarithm (natural) of `1 + x`.
    /// d/dx ln(1+x) = 1 / (1+x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.ln_1p();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.69314718055, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 0.50000000000, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn ln_1p(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.ln_1p(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[(self.value + 1.0).recip()],
            ),
        }
    }

    /// Logarithm (base 10).
    /// d/dx log_10(x) = 1 / x
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.log10();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.00000000000, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 1.00000000000, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn log10(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.log10(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.recip()]),
        }
    }

    /// Logarithm (base 2).
    /// d/dx log_2(x) = 1 / x
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.log2();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.00000000000, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 1.00000000000, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn log2(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.log2(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.recip()]),
        }
    }

    // /// Maximum value.
    // /// f(x) = max(x, y)
    // /// d/dx log_2(x) = 1 / x
    // #[inline]
    // pub fn log2(self) -> Self {
    //     Variable {
    //         graph: self.graph,
    //         value: self.value.log2(),
    //         index: self.graph.push_unary(self.index, self.value.recip()),
    //     }
    // }

    /// Reciprocal function.
    /// d/dx 1 / x =  - 1 / x^2
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.recip();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 1.0);
    /// assert_eq!(grad.wrt(&x), -1.0);
    /// ```
    #[must_use]
    #[inline]
    pub fn recip(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.recip(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.powi(2).recip().neg()],
            ),
        }
    }

    /// Sine function.
    /// d/dx sin(x) = cos(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.sin();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.84147098480, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 0.54030230586, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn sin(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.sin(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.cos()]),
        }
    }

    /// Hyperbolic sine function.
    /// d/dx sinh(x) =  cosh(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.sinh();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      1.17520119364, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 1.54308063481, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn sinh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.sinh(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.cosh()]),
        }
    }

    /// Square root function.
    /// d/dx sqrt(x) =  1 / 2*sqrt(x)
    ///
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(2.0);
    /// let z = x.sqrt();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, std::f64::consts::SQRT_2);
    /// assert_eq!(grad.wrt(&x), 1.0 / (2.0 * std::f64::consts::SQRT_2));
    /// ```
    #[must_use]
    #[inline]
    pub fn sqrt(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.sqrt(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[(self.value.sqrt() * 2.0).recip()],
            ),
        }
    }

    /// Tangent function.
    /// d/dx tan(x) = 1 / cos^2(x) = sec^2(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.tan();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      1.55740772465, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 3.42551882081, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn tan(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.tan(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[(self.value.cos().powi(2)).recip()],
            ),
        }
    }

    /// Hyperbolic tangent function.
    /// d/dx tanh(x) = sech^2(x) = 1 / cosh^2(x)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(1.0);
    /// let z = x.tanh();
    /// let grad = z.accumulate();
    ///
    /// assert_approx_equal!(z.value,      0.7615941559, 1e-10);
    /// assert_approx_equal!(grad.wrt(&x), 0.4199743416, 1e-10);
    /// ```
    #[must_use]
    #[inline]
    pub fn tanh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.tanh(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[(self.value.cosh().powi(2)).recip()],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: ITERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Sum<Variable<'v>> for Variable<'v> {
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let params = (0..100).map(|x| g.var(x as f64)).collect::<Vec<_>>();
    ///
    /// let sum = params.iter().copied().sum::<Variable>();
    ///
    /// let derivs = sum.accumulate();
    ///
    /// for i in derivs.wrt(&params) {
    ///     assert_eq!(i, 1.0);
    /// }
    /// ```
    #[inline]
    fn sum<I: Iterator<Item = Variable<'v>>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y)
            .expect("Cannot call sum() since vector is empty. Exiting ...")
    }
}

impl<'v> Product<Variable<'v>> for Variable<'v> {
    /// ```
    /// # use RustQuant_autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let params = (1..=5).map(|x| g.var(x as f64)).collect::<Vec<_>>();
    ///
    /// let prod = params.iter().copied().product::<Variable>();
    ///
    /// let derivs = prod.accumulate();
    /// let true_gradient = vec![120.0, 60.0, 40.0, 30.0, 24.0];
    ///
    /// let n = derivs.wrt(&params).len();
    /// let m = true_gradient.len();
    /// assert_eq!(n, m);
    ///
    /// for i in 0..n {
    ///     assert_eq!(derivs.wrt(&params)[i], true_gradient[i]);
    /// }
    /// ```
    #[inline]
    fn product<I: Iterator<Item = Variable<'v>>>(iter: I) -> Self {
        iter.reduce(|x, y| x * y)
            .expect("Cannot call product() since vector is empty. Exiting ...")
    }
}

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: MIN
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Overload the `Min` trait.
pub trait Min<T> {
    /// Return type of `Min`
    type Output;

    /// Overloaded `min` function.
    fn min(&self, other: T) -> Self::Output;
}

// min{ Variable<'v> , Variable<'v> }
impl<'v> Min<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn min(&self, rhs: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, rhs.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.min(rhs.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, rhs.index],
                &[
                    if self.value < rhs.value { 1.0 } else { 0.0 },
                    if self.value > rhs.value { 1.0 } else { 0.0 },
                ],
            ),
        }
    }
}

// min{ Variable<'v> , f64 }
impl<'v> Min<f64> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn min(&self, rhs: f64) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: self.value.min(rhs),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[if self.value < rhs { 1.0 } else { 0.0 }, 0.0],
            ),
        }
    }
}

// min{ f64 , Variable<'v> }
impl<'v> Min<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn min(&self, rhs: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: rhs.graph,
            value: f64::min(*self, rhs.value),
            index: rhs.graph.push(
                Arity::Binary,
                &[rhs.index, rhs.index],
                &[0.0, if self < &rhs.value { 1.0 } else { 0.0 }],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: MAX
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Overload the `Max` trait.
pub trait Max<T> {
    /// Return type of `Max`
    type Output;

    /// Overloaded `max` function.
    fn max(&self, other: T) -> Self::Output;
}

// max{ Variable<'v> , Variable<'v> }
impl<'v> Max<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn max(&self, rhs: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, rhs.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.max(rhs.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, rhs.index],
                &[
                    if self.value > rhs.value { 1.0 } else { 0.0 },
                    if self.value < rhs.value { 1.0 } else { 0.0 },
                ],
            ),
        }
    }
}

// max{ Variable<'v> , f64 }
impl<'v> Max<f64> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn max(&self, rhs: f64) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: self.value.max(rhs),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[if self.value > rhs { 1.0 } else { 0.0 }, 0.0],
            ),
        }
    }
}

// max{ f64 , Variable<'v> }
impl<'v> Max<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn max(&self, rhs: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: rhs.graph,
            value: f64::max(*self, rhs.value),
            index: rhs.graph.push(
                Arity::Binary,
                &[rhs.index, rhs.index],
                &[0.0, if self > &rhs.value { 1.0 } else { 0.0 }],
            ),
        }
    }
}

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
impl<'v, T> Mul<Variable<'v, T>> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
    fn mul(self, other: Variable<'v, T>) -> Self::Output {
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
impl<'v, T> Mul<f64> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[T::one() * other, T::zero()],
            ),
        }
    }
}

/// f64 * Variable<'v>
impl<'v> Mul<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
impl<'v, T> Powi<Variable<'v, T>> for Variable<'v, T> where T: DiffOps {
    type Output = Variable<'v, T>;

    #[inline]
    fn powi(&self, other: Variable<'v, T>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.powf(other.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, other.index],
                &[
                    other.value * self.value.powf(other.value - 1.0),
                    self.value.powf(other.value) * self.value.ln(),
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
                &[f64::from(n) * f64::powi(self.value, n - 1), 0.0],
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

use std::f64::consts::PI;

impl<'v, T> Variable<'v, T>
where
    T: DiffOps,
{
    /// Error function.
    /// d/dx erf(x) = 2e^(-x^2) / sqrt(PI)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
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
        // use statrs::function::erf::erf;

        Variable {
            graph: self.graph,
            value: self.value.erf(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.powi(2).neg().exp() * 2.0 / PI.sqrt()],
            ),
        }
    }

    /// Error function (complementary).
    /// d/dx erfc(x) = -2e^(-x^2) / sqrt(PI)
    ///
    /// ```
    /// # use RustQuant_utils::assert_approx_equal;
    /// # use RustQuant_autodiff::*;
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
        // use statrs::function::erf::erfc;

        Variable {
            graph: self.graph,
            value: self.value.erfc(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((self.value.powi(2).neg().exp()).neg() * 2.0 / PI.sqrt())],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard subtraction operator (`-`).
/// d/dx x - y = 1
/// d/dy x - y = -1

/// `SubAssign`: Variable<'v> -= Variable<'v>
impl<'v> SubAssign<Variable<'v>> for Variable<'v> {
    #[inline]
    fn sub_assign(&mut self, other: Variable<'v>) {
        assert!(std::ptr::eq(self.graph, other.graph));

        *self = *self - other;
    }
}

/// `SubAssign`: Variable<'v> -= f64
impl<'v> SubAssign<f64> for Variable<'v> {
    #[inline]
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other;
    }
}

/// `SubAssign`: f64 -= Variable<'v>
impl<'v> SubAssign<Variable<'v>> for f64 {
    #[inline]
    fn sub_assign(&mut self, other: Variable<'v>) {
        *self = *self - other.value;
    }
}

/// Variable<'v> - Variable<'v>
impl<'v, T> Sub<Variable<'v, T>> for Variable<'v, T>
where
    T: DiffOps,
{
    type Output = Variable<'v, T>;

    /// ```
    /// # use RustQuant_autodiff::*;    
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
    fn sub(self, other: Variable<'v, T>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, other.graph));

        self.add(other.neg())
    }
}

/// Variable<'v> - f64
impl<'v> Sub<f64> for Variable<'v> {
    type Output = Variable<'v>;

    /// ```
    /// # use RustQuant_autodiff::*;
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
    /// # use RustQuant_autodiff::*;   
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
            index: other
                .graph
                .push(Arity::Binary, &[other.index, other.index], &[0.0, -1.0]),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overloading {
    use super::*;
    use crate::*;
    use RustQuant_utils::{assert_approx_equal, RUSTQUANT_EPSILON as EPS};

    #[test]
    fn test_div() {
        // Variable / Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x / y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 0.5, EPS);
        assert_approx_equal!(grad.wrt(&x), 0.5, EPS);
        assert_approx_equal!(grad.wrt(&y), -0.25, EPS);

        // Variable / f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x / y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 0.5, EPS);
        assert_approx_equal!(grad.wrt(&x), 0.5, EPS);

        // f64 / Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x / y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 0.5, EPS);
        assert_approx_equal!(grad.wrt(&y), -0.25, EPS);
    }

    #[test]
    fn test_add() {
        // Variable + Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x + y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 3.0, EPS);
        assert_approx_equal!(grad.wrt(&x), 1.0, EPS);
        assert_approx_equal!(grad.wrt(&y), 1.0, EPS);

        // Variable + f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x + y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 3.0, EPS);
        assert_approx_equal!(grad.wrt(&x), 1.0, EPS);

        // f64 + Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x + y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 3.0, EPS);
        assert_approx_equal!(grad.wrt(&y), 1.0, EPS);
    }

    #[test]
    fn test_values() {
        let g = Graph::new();

        let x = g.var(1.0);

        // VALUES
        assert_approx_equal!((-x).value, -1.0, EPS);
        assert_approx_equal!(x.log2().value, 0.0, EPS);
        assert_approx_equal!(x.exp2().value, 2.0, EPS);
        assert_approx_equal!(x.exp_m1().value, 1.718_281_828_459_045, EPS);
        assert_approx_equal!(x.ln().value, 0.0, EPS);
        assert_approx_equal!(x.ln_1p().value, std::f64::consts::LN_2, EPS);
        assert_approx_equal!(x.log10().value, 0.0, EPS);
        assert_approx_equal!(x.log2().value, 0.0, EPS);
        assert_approx_equal!(x.recip().value, 1.0, EPS);
        assert_approx_equal!(x.sqrt().value, 1.0, EPS);
        assert_approx_equal!(x.cbrt().value, 1.0, EPS);
        assert_approx_equal!(x.sin().value, 0.841_470_984_807_896_5, EPS);
        assert_approx_equal!(x.cos().value, 0.540_302_305_868_139_8, EPS);
        assert_approx_equal!(x.tan().value, 1.557_407_724_654_902_3, EPS);
        assert_approx_equal!(x.asin().value, std::f64::consts::FRAC_PI_2, EPS);
        assert_approx_equal!(x.acos().value, 0.0, EPS);
        assert_approx_equal!(x.atan().value, std::f64::consts::FRAC_PI_4, EPS);
        assert_approx_equal!(x.sinh().value, 1.175_201_193_643_801_4, EPS);
        assert_approx_equal!(x.cosh().value, 1.543_080_634_815_243_7, EPS);
        assert_approx_equal!(x.tanh().value, 0.761_594_155_955_764_9, EPS);
        assert_approx_equal!(x.asinh().value, 0.881_373_587_019_543, EPS);
        assert_approx_equal!(x.acosh().value, 0.0, EPS);
        assert!(x.atanh().is_infinite() && x.atanh().is_positive());
        assert_approx_equal!(x.abs().value, 1.0, EPS);
    }

    #[test]
    fn test_gradients() {
        let g = Graph::new();

        let x = g.var(1.0);

        // GRADIENTS
        assert_approx_equal!((-x).accumulate().wrt(&x), -1.0, EPS);
        assert_approx_equal!(x.log2().accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(x.exp2().accumulate().wrt(&x), 1.386_294_361_119_890_6, EPS);
        assert_approx_equal!(x.exp_m1().accumulate().wrt(&x), std::f64::consts::E, EPS);
        assert_approx_equal!(x.ln().accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(x.ln().accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(x.ln_1p().accumulate().wrt(&x), 0.5, EPS);
        assert_approx_equal!(x.log10().accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(x.log2().accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(x.recip().accumulate().wrt(&x), -1.0, EPS);
        assert_approx_equal!(x.sqrt().accumulate().wrt(&x), 0.5, EPS);
        assert_approx_equal!(x.cbrt().accumulate().wrt(&x), 0.333_333_333_333_333_3, EPS);
        assert_approx_equal!(x.sin().accumulate().wrt(&x), 0.540_302_305_868_139_8, EPS);
        assert_approx_equal!(x.cos().accumulate().wrt(&x), -0.841_470_984_807_896_5, EPS);
        assert_approx_equal!(x.tan().accumulate().wrt(&x), 3.425_518_820_814_759, EPS);
        assert_approx_equal!(x.sinh().accumulate().wrt(&x), 1.543_080_634_815_243_7, EPS);
        assert_approx_equal!(x.atan().accumulate().wrt(&x), 0.5, EPS);
        assert_approx_equal!(x.cosh().accumulate().wrt(&x), 1.175_201_193_643_801_4, EPS);
        assert_approx_equal!(x.tanh().accumulate().wrt(&x), 0.419_974_341_614_026_14, EPS);
        assert_approx_equal!(x.asinh().accumulate().wrt(&x), 1.0 / 2_f64.sqrt(), EPS);
        assert_approx_equal!(x.abs().accumulate().wrt(&x), 1.0, EPS);
        assert!(x.atanh().accumulate().wrt(&x).is_nan());
        assert!(x.acosh().accumulate().wrt(&x).is_nan());
        assert!(x.asin().accumulate().wrt(&x).is_nan());
        assert!(x.acos().accumulate().wrt(&x).is_nan());
    }

    #[test]
    fn test_sum() {
        let g = Graph::new();

        let params = (0..100).map(|x| g.var(f64::from(x))).collect::<Vec<_>>();
        let sum = params.iter().copied().sum::<Variable>();
        let derivs = sum.accumulate();

        for i in derivs.wrt(&params) {
            assert_approx_equal!(i, 1.0, EPS);
        }
    }

    #[test]
    fn test_product() {
        let g = Graph::new();

        let params = (1..=5).map(|x| g.var(f64::from(x))).collect::<Vec<_>>();
        let prod = params.iter().copied().product::<Variable>();

        let derivs = prod.accumulate();
        let true_gradient = [120.0, 60.0, 40.0, 30.0, 24.0];

        let expects = derivs.wrt(&params);
        let n = expects.len();
        let m = true_gradient.len();
        assert_eq!(n, m);

        for (&expect, &gradient) in expects.iter().zip(true_gradient.iter()) {
            assert_approx_equal!(expect, gradient, EPS);
        }
    }

    #[test]
    fn test_values_ad() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        // VALUES
        assert!(Min::min(&x, y) == 1.0);
        assert!(Max::max(&x, y) == 2.0);
        assert!(Min::min(&x, 2_f64) == 1.0);
        assert!(Max::max(&x, 2_f64) == 2.0);
        assert!(Max::max(&x, 2_f64) == 2.0);
        assert!(Min::min(&2_f64, x) == 1.0);
        assert!(Max::max(&2_f64, x) == 2.0);
    }

    #[test]
    fn test_gradients_ad() {
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);

        // GRADIENTS
        assert_approx_equal!(Min::min(&x, y).accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(Min::min(&x, y).accumulate().wrt(&y), 0.0, EPS);
        assert_approx_equal!(Max::max(&x, y).accumulate().wrt(&x), 0.0, EPS);
        assert_approx_equal!(Max::max(&x, y).accumulate().wrt(&y), 1.0, EPS);

        assert_approx_equal!(Min::min(&x, 2_f64).accumulate().wrt(&x), 1.0, EPS);
        assert_approx_equal!(Max::max(&x, 2_f64).accumulate().wrt(&x), 0.0, EPS);

        assert_approx_equal!(Min::min(&2_f64, x).accumulate().wrt(&x), 0.0, EPS);
        assert_approx_equal!(Max::max(&2_f64, x).accumulate().wrt(&x), 1.0, EPS);
    }

    #[test]
    fn test_mul() {
        // Variable * Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x * y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, EPS);
        assert_approx_equal!(grad.wrt(&x), 2.0, EPS);
        assert_approx_equal!(grad.wrt(&y), 1.0, EPS);

        // Variable * f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x * y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, EPS);
        assert_approx_equal!(grad.wrt(&x), 2.0, EPS);

        // f64 * Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x * y;
        let grad = z.accumulate();

        assert_approx_equal!(z.value, 2.0, EPS);
        assert_approx_equal!(grad.wrt(&y), 1.0, EPS);
    }

    #[test]
    fn test_sub() {
        // Variable - Variable
        let g = Graph::new();

        let x = g.var(1.0);
        let y = g.var(2.0);
        let z = x - y;
        let grad = z.accumulate();

        assert!((z.value - -1.0).abs() < f64::EPSILON);
        assert!((grad.wrt(&x) - 1.0).abs() < f64::EPSILON);
        assert!((grad.wrt(&y) - -1.0).abs() < f64::EPSILON);

        // Variable - f64
        let g = Graph::new();

        let x = g.var(1.0);
        let y = 2.0;
        let z = x - y;
        let grad = z.accumulate();

        assert!((z.value - -1.0).abs() < f64::EPSILON);
        assert!((grad.wrt(&x) - 1.0).abs() < f64::EPSILON);

        // f64 - Variable
        let g = Graph::new();

        let x = 1.0;
        let y = g.var(2.0);
        let z = x - y;
        let grad = z.accumulate();

        assert!((z.value - -1.0).abs() < f64::EPSILON);
        assert!((grad.wrt(&y) - -1.0).abs() < f64::EPSILON);
    }

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
