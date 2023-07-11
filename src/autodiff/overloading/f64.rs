// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variable::Variable, vertex::Arity};
use std::ops::Neg;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: STANDARD MATH OPERATORS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<'v> std::ops::Neg for Variable<'v> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
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
                .push(Arity::Unary, &[self.index], &[self.value.signum()]),
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
                Arity::Unary,
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
                Arity::Unary,
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[if (self.value > -1.0) && (self.value < 1.0) {
                    ((1.0 - self.value.powi(2)).sqrt()).recip()
                } else {
                    f64::NAN
                }],
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((1.0 + self.value.powi(2)).sqrt()).recip()],
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((1.0 + self.value.powi(2)).recip())],
            ),
        }
    }

    /// Inverse hyperbolic tangent function.
    /// d/dx tanh^-1(x) = 1 / (1 + x^2)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn atanh(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.atanh(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((1.0 - self.value.powi(2)).recip())],
            ),
        }
    }

    /// Cuberoot function.
    /// d/dx cuberoot(x) = 1 / ( 3 * x^(2/3) )
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn cbrt(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.cbrt(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[((3.0 * self.value.powf(2.0 / 3.0)).recip())],
            ),
        }
    }

    /// Cosine function.
    /// d/dx cos(x) = -sin(x)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.exp()]),
        }
    }

    /// Exponential function (base 2)
    /// d/dx 2^x = 2^x * ln(2)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn exp2(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.exp2(),
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[2_f64.powf(self.value) * 2_f64.ln()],
            ),
        }
    }

    /// Exponential function minus 1 function.
    /// d/dx exp(x) - 1 = exp(x)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[self.value.recip()]),
        }
    }

    /// Logarithm (natural) of `1 + x`.
    /// d/dx ln(1+x) = 1 / (1+x)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    #[inline]
    pub fn ln_1p(self) -> Self {
        Variable {
            graph: self.graph,
            value: self.value.ln_1p(),
            index: self
                .graph
                .push(Arity::Unary, &[self.index], &[(1.0 + self.value).recip()]),
        }
    }

    /// Logarithm (base 10).
    /// d/dx log_10(x) = 1 / x
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[(2.0 * self.value.sqrt()).recip()],
            ),
        }
    }

    /// Tangent function.
    /// d/dx tan(x) = 1 / cos^2(x) = sec^2(x)
    ///
    /// ```
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
    /// use RustQuant::assert_approx_equal;
    /// use RustQuant::autodiff::*;
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
