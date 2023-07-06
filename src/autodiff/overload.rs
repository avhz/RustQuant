// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the overloaded operators and primitive functions.
//! Operations such as `+` and `*` are redefined, along with primitive
//! functions such as `sin`, `exp`, and `log`.
//!
//! Each overload has an associated test to ensure functionality.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::*;
use std::{
    f64::consts::PI,
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Neg, Sub},
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: STANDARD MATH OPERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Neg for Variable<'v> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard addition operator (`+`).
/// d/dx x + y = 1
/// d/dy x + y = 1

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
            index: self.graph.push(
                OperationArity::Binary,
                &[self.index, other.index],
                &[1.0, 1.0],
            ),
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
            index: self.graph.push(
                OperationArity::Binary,
                &[self.index, self.index],
                &[1.0, 0.0],
            ),
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
/// Overload the standard subtraction operator (`-`).
/// d/dx x - y = 1
/// d/dy x - y = -1

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
                OperationArity::Binary,
                &[other.index, other.index],
                &[0.0, -1.0],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Overload the standard multiplication operator (`*`).
/// d/dx x * y = y
/// d/dy x * y = x

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
                OperationArity::Binary,
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
            index: self.graph.push(
                OperationArity::Binary,
                &[self.index, self.index],
                &[other, 0.0],
            ),
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
/// Overload the standard division operator (`/`).
/// d/dx x/y = 1/y
/// d/dy x/y = -x/y^2

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
                OperationArity::Binary,
                &[other.index, other.index],
                &[0.0, -self / (other.value * other.value)],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: ITERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Sum<Variable<'v>> for Variable<'v> {
    /// ```
    /// use RustQuant::autodiff::*;
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
    /// use RustQuant::autodiff::*;
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
// OVERLOADING: POWER FUNCTION
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
                OperationArity::Binary,
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
                OperationArity::Binary,
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
                OperationArity::Binary,
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
                OperationArity::Binary,
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
                OperationArity::Binary,
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
                OperationArity::Binary,
                &[other.index, other.index],
                &[0.0, other.value * f64::powf(*self, other.value - 1.0)],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: PRIMITIVE FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Variable<'v> {
    /// Absolute value function.
    /// d/dx abs(x) = sign(x)
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn abs(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.abs(),
            index: self
                .graph
                .push(OperationArity::Unary, &[self.index], &[self.value.signum()]),
        }
    }

    /// Inverse cosine function.
    /// d/dx cos^-1(x) = - 1 / sqrt(1 - x^2)
    ///
    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x1 = g.var(0.0);
    /// let z1 = x1.acos();
    /// let grad1 = z1.accumulate();
    /// assert!((z1.value - 1.5707963267948966).abs() <= 1e-15);
    /// assert!((grad1.wrt(&x1) - (-1.0)).abs() <= 1e-15);
    /// ```
    #[inline]
    pub fn acos(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.acos(),
            index: self.graph.push(
                OperationArity::Unary,
                &[self.index],
                &[((1.0 - self.value.powi(2)).sqrt()).recip().neg()],
            ),
        }
    }

    /// Inverse hyperbolic cosine function.
    /// d/dx cosh^-1(x) = 1 / ( sqrt(x-1) * sqrt(x+1) )
    ///
    /// ```
    /// use RustQuant::autodiff::*;
    ///
    /// let g = Graph::new();
    ///
    /// let x = g.var(5.0);
    /// let z = x.acosh();
    /// let grad = z.accumulate();
    /// assert!((z.value - 2.2924316695611777).abs() <= 1e-15);
    /// assert!((grad.wrt(&x) - 0.20412414523193150818).abs() <= 1e-15);
    /// ```
    #[inline]
    pub fn acosh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.acosh(),
            index: self.graph.push(
                OperationArity::Unary,
                &[self.index],
                &[((self.value - 1.0).sqrt() * (self.value + 1.0).sqrt()).recip()],
            ),
        }
    }

    /// Inverse sine function.
    /// d/dx sin^-1(x) = 1 / sqrt(1 - x^2)
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn asin(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.asin(),
            index: self.graph.push_unary(
                self.index,
                if (self.value > -1.0) && (self.value < 1.0) {
                    ((1.0 - self.value.powi(2)).sqrt()).recip()
                } else {
                    f64::NAN
                },
            ),
        }
    }

    /// Inverse hyperbolic sine function.
    /// d/dx sinh^-1(x) = 1 / sqrt(1 + x^2)
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn asinh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.asinh(),
            index: self
                .graph
                .push_unary(self.index, ((1.0 + self.value.powi(2)).sqrt()).recip()),
        }
    }

    /// Inverse tangent function.
    /// d/dx tan^-1(x) = 1 / (1 + x^2)
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn atan(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.atan(),
            index: self
                .graph
                .push_unary(self.index, (1.0 + self.value.powi(2)).recip()),
        }
    }

    /// Inverse hyperbolic tangent function.
    /// d/dx tanh^-1(x) = 1 / (1 + x^2)
    #[inline]
    pub fn atanh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.atanh(),
            index: self
                .graph
                .push_unary(self.index, (1.0 - self.value.powi(2)).recip()),
        }
    }

    /// Cuberoot function.
    /// d/dx cuberoot(x) = 1 / ( 3 * x^(2/3) )
    #[inline]
    pub fn cbrt(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cbrt(),
            index: self
                .graph
                .push_unary(self.index, (3.0 * self.value.powf(2.0 / 3.0)).recip()),
        }
    }

    /// Cosine function.
    /// d/dx cos(x) = -sin(x)
    #[inline]
    pub fn cos(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cos(),
            index: self.graph.push_unary(self.index, self.value.sin().neg()),
        }
    }

    /// Inverse hyperbolic cosine function.
    /// d/dx cosh(x) = sinh(x)
    #[inline]
    pub fn cosh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cosh(),
            index: self.graph.push_unary(self.index, self.value.sinh()),
        }
    }

    /// Error function.
    /// d/dx erf(x) = 2e^(-x^2) / sqrt(PI)
    #[inline]
    pub fn erf(self) -> Self {
        use statrs::function::erf::erf;

        Variable {
            graph: self.graph,
            value: erf(self.value),
            index: self
                .graph
                .push_unary(self.index, 2.0 * self.value.powi(2).neg().exp() / PI.sqrt()),
        }
    }

    /// Error function (complementary).
    /// d/dx erfc(x) = -2e^(-x^2) / sqrt(PI)
    #[inline]
    pub fn erfc(self) -> Self {
        use statrs::function::erf::erfc;

        Variable {
            graph: self.graph,
            value: erfc(self.value),
            index: self.graph.push_unary(
                self.index,
                (2.0 * self.value.powi(2).neg().exp()).neg() / PI.sqrt(),
            ),
        }
    }

    /// Exponential function (base *e*).
    /// d/dx exp(x) = exp(x) = y
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn exp(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp(),
            index: self.graph.push_unary(self.index, self.value.exp()),
        }
    }

    /// Exponential function (base 2)
    /// d/dx 2^x = 2^x * ln(2)
    #[inline]
    pub fn exp2(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp2(),
            index: self
                .graph
                .push_unary(self.index, 2_f64.powf(self.value) * 2_f64.ln()),
        }
    }

    /// Exponential function minus 1 function.
    /// d/dx exp(x) - 1 = exp(x)
    #[inline]
    pub fn exp_m1(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp_m1(),
            index: self.graph.push_unary(self.index, self.value.exp()),
        }
    }

    /// Logarithm (natural)  of `x`.
    /// d/dx ln(x) = 1 / x
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn ln(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.ln(),
            index: self.graph.push_unary(self.index, self.value.recip()),
        }
    }

    /// Logarithm (natural) of `1 + x`.
    /// d/dx ln(1+x) = 1 / (1+x)
    #[inline]
    pub fn ln_1p(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.ln_1p(),
            index: self
                .graph
                .push_unary(self.index, (1.0 + self.value).recip()),
        }
    }

    /// Logarithm (base 10).
    /// d/dx log_10(x) = 1 / x
    #[inline]
    pub fn log10(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.log10(),
            index: self.graph.push_unary(self.index, self.value.recip()),
        }
    }

    /// Logarithm (base 2).
    /// d/dx log_2(x) = 1 / x
    #[inline]
    pub fn log2(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.log2(),
            index: self.graph.push_unary(self.index, self.value.recip()),
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
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn recip(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.recip(),
            index: self
                .graph
                .push_unary(self.index, self.value.powi(2).recip().neg()),
        }
    }

    /// Sine function.
    /// d/dx sin(x) = cos(x)
    #[inline]
    pub fn sin(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.sin(),
            index: self.graph.push_unary(self.index, self.value.cos()),
        }
    }

    /// Hyperbolic sine function.
    /// d/dx sinh(x) =  cosh(x)
    #[inline]
    pub fn sinh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.sinh(),
            index: self.graph.push_unary(self.index, self.value.cosh()),
        }
    }

    /// Square root function.
    /// d/dx sqrt(x) =  1 / 2*sqrt(x)
    ///
    /// ```
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn sqrt(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.sqrt(),
            index: self
                .graph
                .push_unary(self.index, (2.0 * self.value.sqrt()).recip()),
        }
    }

    /// Tangent function.
    /// d/dx tan(x) = 1 / cos^2(x) = sec^2(x)
    #[inline]
    pub fn tan(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.tan(),
            index: self
                .graph
                .push_unary(self.index, (self.value.cos().powi(2)).recip()),
        }
    }

    /// Hyperbolic tangent function.
    /// d/dx tanh(x) = sech^2(x) = 1 / cosh^2(x)
    #[inline]
    pub fn tanh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.tanh(),
            index: self
                .graph
                .push_unary(self.index, (self.value.cosh().powi(2)).recip()),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overload {
    // use std::f64::EPSILON;
    // use crate::utils::assert_approx_eq;
    use crate::autodiff::Gradient;

    use super::*;

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

    #[test]
    fn test_sum() {
        let g = Graph::new();

        let params = (0..100).map(|x| g.var(x as f64)).collect::<Vec<_>>();
        let sum = params.iter().copied().sum::<Variable>();
        let derivs = sum.accumulate();

        for i in derivs.wrt(&params) {
            assert_eq!(i, 1.0);
        }
    }

    #[test]
    fn test_product() {
        let g = Graph::new();

        let params = (1..=5).map(|x| g.var(x as f64)).collect::<Vec<_>>();
        let prod = params.iter().copied().product::<Variable>();

        let derivs = prod.accumulate();
        let true_gradient = vec![120.0, 60.0, 40.0, 30.0, 24.0];

        let n = derivs.wrt(&params).len();
        let m = true_gradient.len();
        assert_eq!(n, m);

        for i in 0..n {
            assert_eq!(derivs.wrt(&params)[i], true_gradient[i]);
        }
    }

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
