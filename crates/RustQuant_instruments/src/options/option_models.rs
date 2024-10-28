// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::TypeFlag;
use serde::{Deserialize, Serialize};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS & TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Black-Scholes (1973) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackScholes73 {
    s: f64,
    r: f64,
    v: f64,
}

/// Meron (1973) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merton73 {
    s: f64,
    r: f64,
    q: f64,
    v: f64,
}

/// Black (1976) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Black76 {
    f: f64,
    r: f64,
    v: f64,
}

/// Asay (1982) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asay82 {
    f: f64,
    v: f64,
}

/// Garman-Kohlhagen (1983) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarmanKohlhagen83 {
    s: f64,
    r_d: f64,
    r_f: f64,
    v: f64,
}

/// Generalised Black-Scholes-Merton option pricing model and it's Greeks.
pub trait GeneralisedBlackScholesMerton {
    /// Price a European option.
    fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Delta of a European option.
    fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Gamma of a European option.
    fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Theta of a European option.
    fn theta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Vega of a European option.
    fn vega(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Rho of a European option.
    fn rho(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Vanna of a European option.
    fn vanna(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Charm of a European option.
    fn charm(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Lambda of a European option.
    fn lambda(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Zomma of a European option.
    fn zomma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Speed of a European option.
    fn speed(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Color of a European option.
    fn color(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Vomma of a European option.
    fn vomma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Ultima of a European option.
    fn ultima(&self, k: f64, t: f64, option_type: TypeFlag) -> f64;

    /// Calculate d1.
    fn d1(&self, k: f64, t: f64) -> f64;

    /// Calculate d2.
    fn d2(&self, k: f64, t: f64) -> f64;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl BlackScholes73 {
    /// Create a new Black-Scholes (1973) option pricing parameters.
    pub fn new(s: f64, r: f64, v: f64) -> Self {
        Self { s, r, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.s
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r
    }

    #[inline]
    fn b(&self) -> f64 {
        self.r
    }
}

impl Merton73 {
    /// Create a new Merton (1973) option pricing parameters.
    pub fn new(s: f64, r: f64, q: f64, v: f64) -> Self {
        Self { s, r, q, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.s
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r
    }

    #[inline]
    fn b(&self) -> f64 {
        self.r - self.q
    }
}

impl Black76 {
    /// Create a new Black (1976) option pricing parameters.
    pub fn new(f: f64, r: f64, v: f64) -> Self {
        Self { f, r, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.f
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r
    }

    #[inline]
    fn b(&self) -> f64 {
        0.0
    }
}

impl Asay82 {
    /// Create a new Asay (1982) option pricing parameters.
    pub fn new(f: f64, v: f64) -> Self {
        Self { f, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.f
    }

    #[inline]
    fn r(&self) -> f64 {
        0.0
    }

    #[inline]
    fn b(&self) -> f64 {
        0.0
    }
}

impl GarmanKohlhagen83 {
    /// Create a new Garman-Kohlhagen (1983) option pricing parameters.
    pub fn new(s: f64, r_d: f64, r_f: f64, v: f64) -> Self {
        Self { s, r_d, r_f, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.s
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r_d
    }

    #[inline]
    fn b(&self) -> f64 {
        self.r_d - self.r_f
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS AND TRAIT IMPLS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod bsm {
    use RustQuant_math::{Distribution, N};

    #[inline]
    pub(crate) fn d1(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        ((s / k).ln() + (b + 0.5 * v.powi(2)) * t) / (v * t.sqrt())
    }

    #[inline]
    pub(crate) fn d2(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        d1(s, k, t, r, b, v) - v * t.sqrt()
    }

    #[inline]
    pub(crate) fn call_price(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        s * ((b - r) * t).exp() * N.cdf(d1) - k * (-r * t).exp() * N.cdf(d2)
    }

    #[inline]
    pub(crate) fn put_price(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        -s * ((b - r) * t).exp() * N.cdf(-d1) + k * (-r * t).exp() * N.cdf(-d2)
    }

    #[inline]
    pub(crate) fn call_delta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);

        ((b - r) * t).exp() * N.cdf(d1)
    }

    #[inline]
    pub(crate) fn put_delta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);

        ((b - r) * t).exp() * (N.cdf(d1) - 1.0)
    }

    #[inline]
    pub(crate) fn call_gamma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);

        ((b - r) * t).exp() * N.pdf(d1) / (s * v * t.sqrt())
    }

    #[inline]
    pub(crate) fn put_gamma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_gamma(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_theta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        -s * ((b - r) * t).exp() * N.pdf(d1) * v / (2.0 * t.sqrt())
            - (b - r) * s * ((b - r) * t).exp() * N.cdf(d1)
            - r * k * (-r * t).exp() * N.cdf(d2)
    }

    #[inline]
    pub(crate) fn put_theta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        -s * ((b - r) * t).exp() * N.pdf(d1) * v / (2.0 * t.sqrt())
            + (b - r) * s * ((b - r) * t).exp() * N.cdf(-d1)
            + r * k * (-r * t).exp() * N.cdf(-d2)
    }

    #[inline]
    pub(crate) fn call_vega(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);

        s * ((b - r) * t).exp() * N.pdf(d1) * t.sqrt()
    }

    #[inline]
    pub(crate) fn put_vega(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_vega(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_rho(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d2 = d2(s, k, t, r, b, v);

        k * t * (-r * t).exp() * N.cdf(d2)
    }

    #[inline]
    pub(crate) fn put_rho(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d2 = d2(s, k, t, r, b, v);

        -k * t * (-r * t).exp() * N.cdf(-d2)
    }

    #[inline]
    pub(crate) fn call_vanna(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        -((b - r) * t).exp() * N.pdf(d1) * d2 / v
    }

    #[inline]
    pub(crate) fn put_vanna(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_vanna(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_charm(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        ((b - r) * t).exp()
            * (N.pdf(d1) * ((b / (v * t.sqrt())) - (d2 / (2.0 * t))) + (b - r) * N.cdf(d1))
    }

    #[inline]
    pub(crate) fn put_charm(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        ((b - r) * t).exp()
            * (N.pdf(d1) * ((b / (v * t.sqrt())) - (d2 / (2.0 * t))) - (b - r) * N.cdf(-d1))
    }

    #[inline]
    pub(crate) fn call_lambda(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_delta(s, k, t, r, b, v) * s / call_price(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn put_lambda(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        put_delta(s, k, t, r, b, v) * s / put_price(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_zomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        call_gamma(s, k, t, r, b, v) * ((d1 * d2 - 1.0) / v)
    }

    #[inline]
    pub(crate) fn put_zomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        put_gamma(s, k, t, r, b, v) * ((d1 * d2 - 1.0) / v)
    }

    #[inline]
    pub(crate) fn call_speed(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);

        -call_gamma(s, k, t, r, b, v) * (1.0 + d1 / (v * t.sqrt())) / s
    }

    #[inline]
    pub(crate) fn put_speed(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);

        -put_gamma(s, k, t, r, b, v) * (1.0 + d1 / (v * t.sqrt())) / s
    }

    #[inline]
    pub(crate) fn call_color(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        call_gamma(s, k, t, r, b, v)
            * (r - b + b * d1 / (v * t.sqrt()) + (1.0 - d1 * d2) / (2.0 * t))
    }

    #[inline]
    pub(crate) fn put_color(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        put_gamma(s, k, t, r, b, v)
            * (r - b + b * d1 / (v * t.sqrt()) + (1.0 - d1 * d2) / (2.0 * t))
    }

    #[inline]
    pub(crate) fn call_vomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        call_vega(s, k, t, r, b, v) * d1 * d2 / v
    }

    #[inline]
    pub(crate) fn put_vomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        put_vega(s, k, t, r, b, v) * d1 * d2 / v
    }

    #[inline]
    pub(crate) fn call_ultima(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        (call_vomma(s, k, t, r, b, v) / v) * (d1 * d2 - d1 / d2 + d2 / d1 - 1.0)
    }

    #[inline]
    pub(crate) fn put_ultima(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, r, b, v);
        let d2 = d2(s, k, t, r, b, v);

        (put_vomma(s, k, t, r, b, v) / v) * (d1 * d2 - d1 / d2 + d2 / d1 - 1.0)
    }
}

macro_rules! impl_gbsm {
    ($gbsm_variant:ident) => {
        impl GeneralisedBlackScholesMerton for $gbsm_variant {
            /// Price a European option.
            fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_price(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_price(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn d1(&self, k: f64, t: f64) -> f64 {
                bsm::d1(self.s(), k, t, self.r(), self.b(), self.v)
            }

            fn d2(&self, k: f64, t: f64) -> f64 {
                bsm::d2(self.s(), k, t, self.r(), self.b(), self.v)
            }

            fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_delta(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_delta(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_gamma(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_gamma(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn theta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_theta(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_theta(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn vega(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_vega(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_vega(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn rho(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_rho(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_rho(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn vanna(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_vanna(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_vanna(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn charm(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_charm(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_charm(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn lambda(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_lambda(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_lambda(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn zomma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_zomma(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_zomma(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn speed(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_speed(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_speed(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn color(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_color(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_color(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn vomma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_vomma(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_vomma(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }

            fn ultima(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
                match option_type {
                    TypeFlag::Call => bsm::call_ultima(self.s(), k, t, self.r(), self.b(), self.v),
                    TypeFlag::Put => bsm::put_ultima(self.s(), k, t, self.r(), self.b(), self.v),
                }
            }
        }
    };
}

impl_gbsm!(BlackScholes73);
impl_gbsm!(Merton73);
impl_gbsm!(Black76);
impl_gbsm!(Asay82);
impl_gbsm!(GarmanKohlhagen83);
