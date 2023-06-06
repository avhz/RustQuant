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

use {
    super::tape::Tape,
    super::variable::Variable,
    std::f64::consts::PI,
    std::fmt::Display,
    std::iter::{Product, Sum},
    std::ops::{Add, Div, Mul, Neg, Sub},
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: STANDARD MATH OPERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Neg for Variable<'v> {
    type Output = Self;
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
    /// let y = t.var(2.0);
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
        assert_eq!(self.tape as *const Tape, other.tape as *const Tape);
        Variable {
            tape: self.tape,
            value: self.value + other.value,
            index: self.tape.push2(self.index, 1.0, other.index, 1.0),
        }
    }
}

/// Variable<'v> + f64
impl<'v> Add<f64> for Variable<'v> {
    type Output = Variable<'v>;
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(2.0);
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
            tape: self.tape,
            value: self.value + other,
            index: self.tape.push2(self.index, 1.0, self.index, 0.0),
        }
    }
}

/// f64 + Variable<'v>
impl<'v> Add<Variable<'v>> for f64 {
    type Output = Variable<'v>;
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let a = 5.0;
    /// let x = t.var(2.0);
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
    /// let y = t.var(2.0);
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
        assert_eq!(self.tape as *const Tape, other.tape as *const Tape);
        self.add(other.neg())
    }
}

/// Variable<'v> - f64
impl<'v> Sub<f64> for Variable<'v> {
    type Output = Variable<'v>;
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let a = 5.0;
    /// let x = t.var(2.0);
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
            tape: other.tape,
            value: self - other.value,
            index: other.tape.push2(other.index, 0.0, other.index, -1.0),
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
    /// let y = t.var(2.0);
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
        assert_eq!(self.tape as *const Tape, other.tape as *const Tape);
        Variable {
            tape: self.tape,
            value: self.value * other.value,
            index: self
                .tape
                .push2(self.index, other.value, other.index, self.value),
        }
    }
}

/// Variable<'v> * f64
impl<'v> Mul<f64> for Variable<'v> {
    type Output = Variable<'v>;
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
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
            tape: self.tape,
            value: self.value * other,
            index: self.tape.push2(self.index, other, self.index, 0.0),
        }
    }
}

/// f64 * Variable<'v>
impl<'v> Mul<Variable<'v>> for f64 {
    type Output = Variable<'v>;
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let a = 5.0;
    /// let x = t.var(2.0);
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
    /// let y = t.var(2.0);
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
        assert_eq!(self.tape as *const Tape, other.tape as *const Tape);
        self * other.recip()
    }
}

/// Variable<'v> / f64
impl<'v> Div<f64> for Variable<'v> {
    type Output = Variable<'v>;
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let a = 5.0;
    /// let x = t.var(2.0);
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
            tape: other.tape,
            value: self / other.value,
            index: other.tape.push2(
                other.index,
                0.0,
                other.index,
                -self / (other.value * other.value),
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: ITERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Sum<Variable<'v>> for Variable<'v> {
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// # use RustQuant::autodiff::variable::*;
    /// let t = Tape::new();
    ///
    /// let params = (0..100).map(|x| t.var(x as f64)).collect::<Vec<_>>();
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// # use RustQuant::autodiff::variable::*;
    /// let t = Tape::new();
    ///
    /// let params = (1..=5).map(|x| t.var(x as f64)).collect::<Vec<_>>();
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
// OVERLOADING: MISCELLANEOUS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> Display for Variable<'v> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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

impl<'v> Powf<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;
    #[inline]
    fn powf(&self, other: Variable<'v>) -> Self::Output {
        assert_eq!(self.tape as *const Tape, other.tape as *const Tape);
        Self::Output {
            tape: self.tape,
            value: self.value.powf(other.value),
            index: self.tape.push2(
                self.index,
                other.value * f64::powf(self.value, other.value - 1.),
                other.index,
                f64::powf(self.value, other.value) * f64::ln(self.value),
            ),
        }
    }
}

impl<'v> Powf<f64> for Variable<'v> {
    type Output = Variable<'v>;
    #[inline]
    fn powf(&self, n: f64) -> Self::Output {
        Self::Output {
            tape: self.tape,
            value: f64::powf(self.value, n),
            index: self.tape.push2(
                self.index,
                n * f64::powf(self.value, n - 1.0),
                self.index,
                0.0,
            ),
        }
    }
}

impl<'v> Powf<Variable<'v>> for f64 {
    type Output = Variable<'v>;
    #[inline]
    fn powf(&self, other: Variable<'v>) -> Self::Output {
        Self::Output {
            tape: other.tape,
            value: f64::powf(*self, other.value),
            index: other.tape.push2(
                other.index,
                0.,
                other.index,
                other.value * f64::powf(*self, other.value - 1.0),
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x1 = t.var(1.0);
    /// let z1 = x1.abs();
    /// let grad1 = z1.accumulate();
    /// assert!((z1.value - 1.0).abs() <= 1e-15);
    /// assert!((grad1.wrt(&x1) - 1.0).abs() <= 1e-15);
    ///
    /// let x2 = t.var(-1.0);
    /// let z2 = x2.abs();
    /// let grad2 = z2.accumulate();
    /// assert!((z2.value - 1.0).abs() <= 1e-15);
    /// assert!((grad2.wrt(&x2) - (-1.0)).abs() <= 1e-15);
    /// ```
    ///
    pub fn abs(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.abs(),
            index: self.tape.push1(self.index, self.value.signum()),
        }
    }

    /// Inverse cosine function.
    /// d/dx cos^-1(x) = - 1 / sqrt(1 - x^2)
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x1 = t.var(0.0);
    /// let z1 = x1.acos();
    /// let grad1 = z1.accumulate();
    /// assert!((z1.value - 1.5707963267948966).abs() <= 1e-15);
    /// assert!((grad1.wrt(&x1) - (-1.0)).abs() <= 1e-15);
    /// ```
    ///
    pub fn acos(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.acos(),
            index: self.tape.push1(
                self.index,
                ((1.0 - self.value.powi(2)).sqrt()).recip().neg(),
            ),
        }
    }

    /// Inverse hyperbolic cosine function.
    /// d/dx cosh^-1(x) = 1 / ( sqrt(x-1) * sqrt(x+1) )
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(5.0);
    /// let z = x.acosh();
    /// let grad = z.accumulate();
    /// assert!((z.value - 2.2924316695611777).abs() <= 1e-15);
    /// assert!((grad.wrt(&x) - 0.20412414523193150818).abs() <= 1e-15);
    /// ```
    ///
    pub fn acosh(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.acosh(),
            index: self.tape.push1(
                self.index,
                ((self.value - 1.0).sqrt() * (self.value + 1.0).sqrt()).recip(),
            ),
        }
    }

    /// Inverse sine function.
    /// d/dx sin^-1(x) = 1 / sqrt(1 - x^2)
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(0.0);
    /// let z = x.asin();
    /// let grad = z.accumulate();
    ///
    /// //assert_eq!(z.value, std::f64::consts::PI / 2.0);
    /// assert_eq!(z.value, 0.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    ///
    pub fn asin(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.asin(),
            index: self.tape.push1(
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
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(0.0);
    /// let z = x.asinh();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 0.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    ///
    pub fn asinh(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.asinh(),
            index: self
                .tape
                .push1(self.index, ((1.0 + self.value.powi(2)).sqrt()).recip()),
        }
    }

    /// Inverse tangent function.
    /// d/dx tan^-1(x) = 1 / (1 + x^2)
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(0.0);
    /// let z = x.atan();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 0.0);
    /// assert_eq!(grad.wrt(&x), 1.0);
    /// ```
    ///
    pub fn atan(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.atan(),
            index: self
                .tape
                .push1(self.index, (1.0 + self.value.powi(2)).recip()),
        }
    }

    /// Inverse hyperbolic tangent function.
    /// d/dx tanh^-1(x) = 1 / (1 + x^2)
    pub fn atanh(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.atanh(),
            index: self
                .tape
                .push1(self.index, (1.0 - self.value.powi(2)).recip()),
        }
    }

    /// Cuberoot function.
    /// d/dx cuberoot(x) = 1 / ( 3 * x^(2/3) )
    pub fn cbrt(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.cbrt(),
            index: self
                .tape
                .push1(self.index, (3.0 * self.value.powf(2.0 / 3.0)).recip()),
        }
    }

    /// Cosine function.
    /// d/dx cos(x) = -sin(x)
    pub fn cos(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.cos(),
            index: self.tape.push1(self.index, self.value.sin().neg()),
        }
    }

    /// Inverse hyperbolic cosine function.
    /// d/dx cosh(x) = sinh(x)
    pub fn cosh(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.cosh(),
            index: self.tape.push1(self.index, self.value.sinh()),
        }
    }

    /// Error function (complementary).
    /// d/dx erfc(x) = -2e^(-x^2) / sqrt(PI)
    pub fn erfc(self) -> Self {
        use statrs::function::erf::erfc;
        Variable {
            tape: self.tape,
            value: erfc(self.value),
            index: self.tape.push1(
                self.index,
                (2.0 * self.value.powi(2).neg().exp()).neg() / PI.sqrt(),
            ),
        }
    }

    /// Exponential function (base *e*).
    /// d/dx exp(x) = exp(x) = y
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(1.0);
    /// let z = x.exp();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, std::f64::consts::E);
    /// assert_eq!(grad.wrt(&x), std::f64::consts::E);
    /// ```
    ///
    pub fn exp(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.exp(),
            index: self.tape.push1(self.index, self.value.exp()),
        }
    }

    /// Exponential function (base 2)
    /// d/dx 2^x = 2^x * ln(2)
    pub fn exp2(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.exp2(),
            index: self
                .tape
                .push1(self.index, 2_f64.powf(self.value) * 2_f64.ln()),
        }
    }

    /// Exponential function minus 1 function.
    /// d/dx exp(x) - 1 = exp(x)
    pub fn exp_m1(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.exp_m1(),
            index: self.tape.push1(self.index, self.value.exp()),
        }
    }

    /// Logarithm (natural)  of `x`.
    /// d/dx ln(x) = 1 / x
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;    
    /// let t = Tape::new();
    ///
    /// let x = t.var(std::f64::consts::E);
    /// let z = x.ln();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 1.0);
    /// assert_eq!(grad.wrt(&x), 0.36787944117144233);
    /// ```
    ///
    pub fn ln(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.ln(),
            index: self.tape.push1(self.index, self.value.recip()),
        }
    }

    /// Logarithm (natural) of `1 + x`.
    /// d/dx ln(1+x) = 1 / (1+x)
    pub fn ln_1p(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.ln_1p(),
            index: self.tape.push1(self.index, (1.0 + self.value).recip()),
        }
    }

    /// Logarithm (base 10).
    /// d/dx log_10(x) = 1 / x
    pub fn log10(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.log10(),
            index: self.tape.push1(self.index, self.value.recip()),
        }
    }

    /// Logarithm (base 2).
    /// d/dx log_2(x) = 1 / x
    pub fn log2(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.log2(),
            index: self.tape.push1(self.index, self.value.recip()),
        }
    }

    /// Reciprocal function.
    /// d/dx 1 / x =  - 1 / x^2
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;
    /// let t = Tape::new();
    ///
    /// let x = t.var(1.0);
    /// let z = x.recip();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, 1.0);
    /// assert_eq!(grad.wrt(&x), -1.0);
    /// ```
    ///
    pub fn recip(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.recip(),
            index: self
                .tape
                .push1(self.index, self.value.powi(2).recip().neg()),
        }
    }

    /// Sine function.
    /// d/dx sin(x) = cos(x)
    pub fn sin(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.sin(),
            index: self.tape.push1(self.index, self.value.cos()),
        }
    }

    /// Hyperbolic sine function.
    /// d/dx sinh(x) =  cosh(x)
    pub fn sinh(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.sinh(),
            index: self.tape.push1(self.index, self.value.cosh()),
        }
    }

    /// Square root function.
    /// d/dx sqrt(x) =  1 / 2*sqrt(x)
    ///
    /// ```
    /// # use RustQuant::autodiff::gradient::*;
    /// # use RustQuant::autodiff::tape::*;
    /// let t = Tape::new();
    ///
    /// let x = t.var(2.0);
    /// let z = x.sqrt();
    /// let grad = z.accumulate();
    ///
    /// assert_eq!(z.value, std::f64::consts::SQRT_2);
    /// assert_eq!(grad.wrt(&x), 1.0 / (2.0 * std::f64::consts::SQRT_2));
    /// ```
    ///
    pub fn sqrt(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.sqrt(),
            index: self
                .tape
                .push1(self.index, (2.0 * self.value.sqrt()).recip()),
        }
    }

    /// Tangent function.
    /// d/dx tan(x) = 1 / cos^2(x) = sec^2(x)
    pub fn tan(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.tan(),
            index: self
                .tape
                .push1(self.index, (self.value.cos().powi(2)).recip()),
        }
    }

    /// Hyperbolic tangent function.
    /// d/dx tanh(x) = sech^2(x) = 1 / cosh^2(x)
    pub fn tanh(self) -> Self {
        Variable {
            tape: self.tape,
            value: self.value.tanh(),
            index: self
                .tape
                .push1(self.index, (self.value.cosh().powi(2)).recip()),
        }
    }
}
