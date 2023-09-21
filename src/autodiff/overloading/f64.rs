// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variables::variable::Variable, vertex::Arity, vertex::Operation};
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.signum()],
                Operation::_ABS,
            ),
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
                Operation::_ACOS,
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
                Operation::_ACOSH,
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
                Operation::_ASIN,
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
                Operation::_ASINH,
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
                Operation::_ATAN,
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
                Operation::_ATANH,
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
                Operation::_CBRT,
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.sin().neg()],
                Operation::_COS,
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.sinh()],
                Operation::_SIN,
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.exp()],
                Operation::_EXP,
            ),
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
                Operation::_EXP2,
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.exp()],
                Operation::_ExpM1,
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.recip()],
                Operation::_LN,
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[(1.0 + self.value).recip()],
                Operation::_P1LN,
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.recip()],
                Operation::_LOG10,
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.recip()],
                Operation::_LOG2,
            ),
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
                Operation::_INV,
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.cos()],
                Operation::_SIN,
            ),
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
            index: self.graph.push(
                Arity::Unary,
                &[self.index],
                &[self.value.cosh()],
                Operation::_SINH,
            ),
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
                Operation::_SQRT,
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
                Operation::_TAN,
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
                Operation::_TANH,
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_overloading_f64 {
    use crate::assert_approx_equal;
    use crate::autodiff::*;

    #[test]
    fn test_values() {
        let g = Graph::new();

        let x = g.var(1.0);

        // VALUES
        assert_approx_equal!((-x).value, -1.0, 1e-10);
        assert_approx_equal!(x.log2().value, 0.0, 1e-10);
        assert_approx_equal!(x.exp2().value, 2.0, 1e-10);
        assert_approx_equal!(x.exp_m1().value, 1.718281828459045, 1e-10);
        assert_approx_equal!(x.ln().value, 0.0, 1e-10);
        assert_approx_equal!(x.ln_1p().value, std::f64::consts::LN_2, 1e-10);
        assert_approx_equal!(x.log10().value, 0.0, 1e-10);
        assert_approx_equal!(x.log2().value, 0.0, 1e-10);
        assert_approx_equal!(x.recip().value, 1.0, 1e-10);
        assert_approx_equal!(x.sqrt().value, 1.0, 1e-10);
        assert_approx_equal!(x.cbrt().value, 1.0, 1e-10);
        assert_approx_equal!(x.sin().value, 0.8414709848078965, 1e-10);
        assert_approx_equal!(x.cos().value, 0.5403023058681398, 1e-10);
        assert_approx_equal!(x.tan().value, 1.5574077246549023, 1e-10);
        assert_approx_equal!(x.asin().value, std::f64::consts::FRAC_PI_2, 1e-10);
        assert_approx_equal!(x.acos().value, 0.0, 1e-10);
        assert_approx_equal!(x.atan().value, std::f64::consts::FRAC_PI_4, 1e-10);
        assert_approx_equal!(x.sinh().value, 1.1752011936438014, 1e-10);
        assert_approx_equal!(x.cosh().value, 1.5430806348152437, 1e-10);
        assert_approx_equal!(x.tanh().value, 0.7615941559557649, 1e-10);
        assert_approx_equal!(x.asinh().value, 0.881373587019543, 1e-10);
        assert_approx_equal!(x.acosh().value, 0.0, 1e-10);
        assert_eq!(x.atanh().value, std::f64::INFINITY);
        assert_approx_equal!(x.abs().value, 1.0, 1e-10);
    }

    #[test]
    fn test_gradients() {
        let g = Graph::new();

        let x = g.var(1.0);

        // GRADIENTS
        assert_approx_equal!((-x).accumulate().wrt(&x), -1.0, 1e-10);
        assert_approx_equal!(x.log2().accumulate().wrt(&x), 1.0, 1e-10);
        assert_approx_equal!(x.exp2().accumulate().wrt(&x), 1.3862943611198906, 1e-10);
        assert_approx_equal!(x.exp_m1().accumulate().wrt(&x), std::f64::consts::E, 1e-10);
        assert_approx_equal!(x.ln().accumulate().wrt(&x), 1.0, 1e-10);
        assert_approx_equal!(x.ln_1p().accumulate().wrt(&x), 0.5, 1e-10);
        assert_approx_equal!(x.log10().accumulate().wrt(&x), 1.0, 1e-10);
        assert_approx_equal!(x.log2().accumulate().wrt(&x), 1.0, 1e-10);
        assert_approx_equal!(x.recip().accumulate().wrt(&x), -1.0, 1e-10);
        assert_approx_equal!(x.sqrt().accumulate().wrt(&x), 0.5, 1e-10);
        assert_approx_equal!(x.cbrt().accumulate().wrt(&x), 0.3333333333333333, 1e-10);
        assert_approx_equal!(x.sin().accumulate().wrt(&x), 0.5403023058681398, 1e-10);
        assert_approx_equal!(x.cos().accumulate().wrt(&x), -0.8414709848078965, 1e-10);
        assert_approx_equal!(x.tan().accumulate().wrt(&x), 3.4255188208149777, 1e-10);
        assert_approx_equal!(x.sinh().accumulate().wrt(&x), 1.5430806348152437, 1e-10);
        assert_approx_equal!(x.atan().accumulate().wrt(&x), 0.5, 1e-10);
        assert_approx_equal!(x.cosh().accumulate().wrt(&x), 1.1752011936438014, 1e-10);
        assert_approx_equal!(x.tanh().accumulate().wrt(&x), 0.41997434161402614, 1e-10);
        assert_approx_equal!(x.asinh().accumulate().wrt(&x), 1.0 / 2_f64.sqrt(), 1e-10);
        assert_approx_equal!(x.abs().accumulate().wrt(&x), 1.0, 1e-10);
        assert!(x.atanh().accumulate().wrt(&x).is_nan());
        assert!(x.acosh().accumulate().wrt(&x).is_nan());
        assert!(x.asin().accumulate().wrt(&x).is_nan());
        assert!(x.acos().accumulate().wrt(&x).is_nan());
    }
}
