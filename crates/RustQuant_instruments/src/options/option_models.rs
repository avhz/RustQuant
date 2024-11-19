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

/// Heston (1993) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heston93 {
    s: f64,
    v: f64,
    r: f64,
    q: f64,
    rho: f64,
    kappa: f64,
    theta: f64,
    sigma: f64,
}

/// Bachelier (1900) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Bachelier {
    f: f64,
    r: f64,
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

impl Heston93 {
    /// Create a new Heston (1993) option pricing parameters.
    pub fn new(
        s: f64,
        v: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> Self {
        Self {
            s,
            v,
            r,
            q,
            rho,
            kappa,
            theta,
            sigma,
        }
    }
}

impl Bachelier {
    /// Create a new Bachelier (1900) option pricing parameters.
    pub fn new(f: f64, r: f64, v: f64) -> Self {
        Self { f, r, v }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS AND TRAIT IMPLS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod bsm {
    use RustQuant_math::{Distribution, N};

    #[inline]
    pub(crate) fn d1(s: f64, k: f64, t: f64, b: f64, v: f64) -> f64 {
        ((s / k).ln() + (b + 0.5 * v.powi(2)) * t) / (v * t.sqrt())
    }

    #[inline]
    pub(crate) fn d2(s: f64, k: f64, t: f64, b: f64, v: f64) -> f64 {
        d1(s, k, t, b, v) - v * t.sqrt()
    }

    #[inline]
    pub(crate) fn call_price(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        s * ((b - r) * t).exp() * N.cdf(d1) - k * (-r * t).exp() * N.cdf(d2)
    }

    #[inline]
    pub(crate) fn put_price(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        -s * ((b - r) * t).exp() * N.cdf(-d1) + k * (-r * t).exp() * N.cdf(-d2)
    }

    #[inline]
    pub(crate) fn call_delta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);

        ((b - r) * t).exp() * N.cdf(d1)
    }

    #[inline]
    pub(crate) fn put_delta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);

        ((b - r) * t).exp() * (N.cdf(d1) - 1.0)
    }

    #[inline]
    pub(crate) fn call_gamma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);

        ((b - r) * t).exp() * N.pdf(d1) / (s * v * t.sqrt())
    }

    #[inline]
    pub(crate) fn put_gamma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_gamma(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_theta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        -s * ((b - r) * t).exp() * N.pdf(d1) * v / (2.0 * t.sqrt())
            - (b - r) * s * ((b - r) * t).exp() * N.cdf(d1)
            - r * k * (-r * t).exp() * N.cdf(d2)
    }

    #[inline]
    pub(crate) fn put_theta(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        -s * ((b - r) * t).exp() * N.pdf(d1) * v / (2.0 * t.sqrt())
            + (b - r) * s * ((b - r) * t).exp() * N.cdf(-d1)
            + r * k * (-r * t).exp() * N.cdf(-d2)
    }

    #[inline]
    pub(crate) fn call_vega(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);

        s * ((b - r) * t).exp() * N.pdf(d1) * t.sqrt()
    }

    #[inline]
    pub(crate) fn put_vega(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_vega(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_rho(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d2 = d2(s, k, t, b, v);

        k * t * (-r * t).exp() * N.cdf(d2)
    }

    #[inline]
    pub(crate) fn put_rho(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d2 = d2(s, k, t, b, v);

        -k * t * (-r * t).exp() * N.cdf(-d2)
    }

    #[inline]
    pub(crate) fn call_vanna(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        -((b - r) * t).exp() * N.pdf(d1) * d2 / v
    }

    #[inline]
    pub(crate) fn put_vanna(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        call_vanna(s, k, t, r, b, v)
    }

    #[inline]
    pub(crate) fn call_charm(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        ((b - r) * t).exp()
            * (N.pdf(d1) * ((b / (v * t.sqrt())) - (d2 / (2.0 * t))) + (b - r) * N.cdf(d1))
    }

    #[inline]
    pub(crate) fn put_charm(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

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
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        call_gamma(s, k, t, r, b, v) * ((d1 * d2 - 1.0) / v)
    }

    #[inline]
    pub(crate) fn put_zomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        put_gamma(s, k, t, r, b, v) * ((d1 * d2 - 1.0) / v)
    }

    #[inline]
    pub(crate) fn call_speed(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);

        -call_gamma(s, k, t, r, b, v) * (1.0 + d1 / (v * t.sqrt())) / s
    }

    #[inline]
    pub(crate) fn put_speed(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);

        -put_gamma(s, k, t, r, b, v) * (1.0 + d1 / (v * t.sqrt())) / s
    }

    #[inline]
    pub(crate) fn call_color(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        call_gamma(s, k, t, r, b, v)
            * (r - b + b * d1 / (v * t.sqrt()) + (1.0 - d1 * d2) / (2.0 * t))
    }

    #[inline]
    pub(crate) fn put_color(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        put_gamma(s, k, t, r, b, v)
            * (r - b + b * d1 / (v * t.sqrt()) + (1.0 - d1 * d2) / (2.0 * t))
    }

    #[inline]
    pub(crate) fn call_vomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        call_vega(s, k, t, r, b, v) * d1 * d2 / v
    }

    #[inline]
    pub(crate) fn put_vomma(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        put_vega(s, k, t, r, b, v) * d1 * d2 / v
    }

    #[inline]
    pub(crate) fn call_ultima(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

        (call_vomma(s, k, t, r, b, v) / v) * (d1 * d2 - d1 / d2 + d2 / d1 - 1.0)
    }

    #[inline]
    pub(crate) fn put_ultima(s: f64, k: f64, t: f64, r: f64, b: f64, v: f64) -> f64 {
        let d1 = d1(s, k, t, b, v);
        let d2 = d2(s, k, t, b, v);

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
                bsm::d1(self.s(), k, t, self.b(), self.v)
            }

            fn d2(&self, k: f64, t: f64) -> f64 {
                bsm::d2(self.s(), k, t, self.b(), self.v)
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

mod heston {
    use num::Complex;
    use std::f64::consts::{FRAC_1_PI, PI};
    use RustQuant_math::integrate;

    #[derive(Clone, Copy)]
    enum Switch {
        One = 1,
        Two = 2,
    }

    /// Heston model for option pricing.
    pub(crate) fn call_price(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        let (P1, P2) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

        s * (-q * t).exp() * P1 - k * (-r * t).exp() * P2
    }

    /// Heston model for option pricing.
    pub(crate) fn put_price(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        // Use put-call-parity.
        call_price(s, v, k, t, r, q, rho, kappa, theta, sigma)
            + (k * (-r * t).exp() - s * (-q * t).exp())
    }

    pub(crate) fn call_delta(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        let (P1, _) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

        (-q * t).exp() * P1
    }

    pub(crate) fn put_delta(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        call_delta(s, v, k, t, r, q, rho, kappa, theta, sigma) - (-q * t).exp()
    }

    pub(crate) fn call_gamma(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        // Market price of volatility risk (set to 0 for simplicity).
        // Should probably include, though, since for equity options it has been shown
        // to be non-zero (Lamoureux & Lastrapes, 1993).
        let lambda = 0.0;

        // i = sqrt(-1). Used frequently, so assign here.
        let i: Complex<f64> = Complex::i();

        let u = |j: Switch| -> f64 {
            match j {
                Switch::One => 0.5,
                Switch::Two => -0.5,
            }
        };

        let b = |j: Switch| -> f64 {
            match j {
                Switch::One => kappa + lambda - rho * sigma,
                Switch::Two => kappa + lambda,
            }
        };

        let d = |j: Switch, phi: f64| -> Complex<f64> {
            ((rho * sigma * i * phi - b(j)).powi(2)
                - sigma.powi(2) * (2.0 * u(j) * i * phi - phi.powi(2)))
            .sqrt()
        };

        let g = |j: Switch, phi: f64| -> Complex<f64> {
            (b(j) - rho * sigma * i * phi + d(j, phi)) / (b(j) - rho * sigma * i * phi - d(j, phi))
        };

        let C = |j: Switch, phi: f64| -> Complex<f64> {
            (r - q) * i * phi * t
                + (kappa * theta / sigma.powi(2))
                    * ((b(j) - rho * sigma * i * phi + d(j, phi)) * t
                        - 2.0
                            * ((1.0 - g(j, phi) * (d(j, phi) * t).exp()) / (1.0 - g(j, phi))).ln())
        };

        let D = |j: Switch, phi: f64| -> Complex<f64> {
            ((b(j) - rho * sigma * i * phi + d(j, phi)) * (1.0 - (d(j, phi) * t).exp()))
                / (sigma.powi(2) * (1.0 - g(j, phi) * (d(j, phi) * t).exp()))
        };

        // The Heston characteristic functions.
        let f = |j: Switch, phi: f64| -> Complex<f64> {
            (C(j, phi) + D(j, phi) * v + i * phi * s.ln()).exp()
        };

        let Re1 = |phi: f64| -> f64 {
            let j = Switch::One;

            ((-i * phi * k.ln()).exp() * f(j, phi)).re
        };

        let dP1 = integrate(Re1, 0.00001, 50.0) * (PI * s).recip();

        dP1 * (-q * t).exp()
    }

    pub(crate) fn put_gamma(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        call_gamma(s, v, k, t, r, q, rho, kappa, theta, sigma)
    }

    pub(crate) fn call_rho(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        let (_, P2) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

        k * t * (-r * t).exp() * P2
    }

    pub(crate) fn put_rho(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> f64 {
        let (_, P2) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

        k * t * (-r * t).exp() * (P2 - 1.0)
    }

    pub(crate) fn p1p2(
        s: f64,
        v: f64,
        k: f64,
        t: f64,
        r: f64,
        q: f64,
        rho: f64,
        kappa: f64,
        theta: f64,
        sigma: f64,
    ) -> (f64, f64) {
        // Market price of volatility risk (set to 0 for simplicity).
        // Should probably include, though, since for equity options it has been shown
        // to be non-zero (Lamoureux & Lastrapes, 1993).
        let lambda = 0.0;

        // i = sqrt(-1). Used frequently, so assign here.
        let i: Complex<f64> = Complex::i();

        let u = |j: Switch| -> f64 {
            match j {
                Switch::One => 0.5,
                Switch::Two => -0.5,
            }
        };

        let b = |j: Switch| -> f64 {
            match j {
                Switch::One => kappa + lambda - rho * sigma,
                Switch::Two => kappa + lambda,
            }
        };

        let d = |j: Switch, phi: f64| -> Complex<f64> {
            ((rho * sigma * i * phi - b(j)).powi(2)
                - sigma.powi(2) * (2.0 * u(j) * i * phi - phi.powi(2)))
            .sqrt()
        };

        let g = |j: Switch, phi: f64| -> Complex<f64> {
            (b(j) - rho * sigma * i * phi + d(j, phi)) / (b(j) - rho * sigma * i * phi - d(j, phi))
        };

        let C = |j: Switch, phi: f64| -> Complex<f64> {
            (r - q) * i * phi * t
                + (kappa * theta / sigma.powi(2))
                    * ((b(j) - rho * sigma * i * phi + d(j, phi)) * t
                        - 2.0
                            * ((1.0 - g(j, phi) * (d(j, phi) * t).exp()) / (1.0 - g(j, phi))).ln())
        };

        let D = |j: Switch, phi: f64| -> Complex<f64> {
            ((b(j) - rho * sigma * i * phi + d(j, phi)) * (1.0 - (d(j, phi) * t).exp()))
                / (sigma.powi(2) * (1.0 - g(j, phi) * (d(j, phi) * t).exp()))
        };

        // The Heston characteristic functions.
        let f = |j: Switch, phi: f64| -> Complex<f64> {
            (C(j, phi) + D(j, phi) * v + i * phi * s.ln()).exp()
        };

        // These functions return the integrand for P1 and P2.
        let Re1 = |phi: f64| -> f64 {
            let j = Switch::One;

            (f(j, phi) * (-i * phi * k.ln()).exp() / (i * phi)).re
        };
        let Re2 = |phi: f64| -> f64 {
            let j = Switch::Two;

            (f(j, phi) * (-i * phi * k.ln()).exp() / (i * phi)).re
        };

        // Integration bounds given in Fabrice D. Rouah's book (see tests).
        // The integral decays rapidly so 50 is probably enough.
        let P1 = 0.5 + FRAC_1_PI * integrate(Re1, 0.00001, 50.0);
        let P2 = 0.5 + FRAC_1_PI * integrate(Re2, 0.00001, 50.0);

        (P1, P2)
    }
}

impl Heston93 {
    /// Price a European option using the Heston model.
    pub fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => heston::call_price(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => heston::put_price(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    /// Delta of a European option using the Heston model.
    pub fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => heston::call_delta(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => heston::put_delta(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    /// Gamma of a European option using the Heston model.
    pub fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => heston::call_gamma(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => heston::put_gamma(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    /// Rho of a European option using the Heston model.
    pub fn rho(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => heston::call_rho(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => heston::put_rho(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    fn unpack(&self) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        // s: f64,
        // v: f64,
        // r: f64,
        // q: f64,
        // rho: f64,
        // kappa: f64,
        // theta: f64,
        // sigma: f64,
        (
            self.s, self.v, self.r, self.q, self.rho, self.kappa, self.theta, self.sigma,
        )
    }
}

mod bachelier {
    use std::f64::consts::{FRAC_PI_2, PI};
    use RustQuant_math::{gaussian::N, Distribution};

    /// Price a European call option using the Bacheller model.
    #[inline]
    pub(crate) fn call_price(f: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        let df = df(r, t);
        let price = (f - k) * N.cdf(d) + v * t.sqrt() * N.pdf(d);

        df * price
    }

    /// Price a European put option using the Bacheller model.
    #[inline]
    pub(crate) fn put_price(f: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        let df = df(r, t);
        let price = (k - f) * N.cdf(-d) + v * t.sqrt() * N.pdf(d);

        df * price
    }

    #[inline]
    fn d(f: f64, k: f64, t: f64, v: f64) -> f64 {
        (f - k) / (v * t.sqrt())
    }

    #[inline]
    fn df(r: f64, t: f64) -> f64 {
        (-r * t).exp()
    }

    #[inline]
    pub(crate) fn atm_price(t: f64, v: f64) -> f64 {
        v * (t / (2. * PI)).sqrt()
    }

    #[inline]
    pub(crate) fn atm_vol(price: f64, t: f64) -> f64 {
        price * (2. * PI / t).sqrt()
    }

    const A: [f64; 8] = [
        3.994_961_687_345_13e-1,
        2.100_960_795_068_49e+1,
        4.980_340_217_855_08e+1,
        5.988_761_102_690_99e+2,
        1.848_489_695_437_09e+3,
        6.106_322_407_867_05e+3,
        2.493_415_285_349_36e+4,
        1.266_458_051_348_24e+4,
    ];

    const B: [f64; 9] = [
        4.990_534_153_589_42e+1,
        3.093_573_936_743_11e+1,
        1.495_105_008_310_99e+3,
        1.323_614_537_899_73e+3,
        1.598_919_697_679_74e+4,
        2.392_008_891_720_78e+4,
        3.608_817_108_375_03e+3,
        -2.067_719_486_400_92e+2,
        1.174_240_599_306_01e+1,
    ];

    #[inline]
    pub(crate) fn call_iv(price: f64, f: f64, k: f64, t: f64) -> f64 {
        let v = (f - k).abs() / (2. * price - (f - k));
        let eta = v / v.atanh();

        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        // for k in 0..A.len() {
        for (k, v) in A.iter().enumerate() {
            sum1 += v * eta.powi(k as i32);
        }

        // for k in 0..B.len() {
        for (k, v) in B.iter().enumerate() {
            sum2 += v * eta.powi(k as i32);
        }

        let hn = eta.sqrt() * sum1 / (1. + sum2);

        (FRAC_PI_2 / t).sqrt() * (2. * price - (f - k)) * hn
    }

    #[inline]
    pub(crate) fn put_iv(price: f64, f: f64, k: f64, t: f64) -> f64 {
        let v = (f - k).abs() / (2. * price + (f - k));
        let eta = v / v.atanh();

        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        // for k in 0..A.len() {
        for (k, v) in A.iter().enumerate() {
            sum1 += v * eta.powi(k as i32);
        }

        // for k in 1..=B.len() {
        for (k, v) in B.iter().enumerate().skip(1) {
            sum2 += v * eta.powi(k as i32);
        }

        let hn = eta.sqrt() * sum1 / (1. + sum2);

        (FRAC_PI_2 / t).sqrt() * (2. * price + (f - k)) * hn
    }

    #[inline]
    pub(crate) fn call_delta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        N.cdf(d)
    }

    #[inline]
    pub(crate) fn put_delta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        N.cdf(d) - 1.0
    }

    #[inline]
    pub(crate) fn call_gamma(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        N.pdf(d) / (v * t.sqrt())
    }

    #[inline]
    pub(crate) fn put_gamma(f: f64, k: f64, t: f64, v: f64) -> f64 {
        call_gamma(f, k, t, v)
    }

    #[inline]
    pub(crate) fn call_vega(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        t.sqrt() * N.pdf(d)
    }

    #[inline]
    pub(crate) fn put_vega(f: f64, k: f64, t: f64, v: f64) -> f64 {
        call_vega(f, k, t, v)
    }

    #[inline]
    pub(crate) fn call_theta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);

        -v * N.pdf(d) / (2. * t.sqrt())
    }

    #[inline]
    pub(crate) fn put_theta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);

        -v * N.pdf(d) / (2. * t.sqrt())
    }
}

impl Bachelier {
    /// Price a European option using the Bachelier model.
    pub fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => bachelier::call_price(self.f, k, t, self.r, self.v),
            TypeFlag::Put => bachelier::put_price(self.f, k, t, self.r, self.v),
        }
    }

    /// ATM price of a European option using the Bachelier model.
    pub fn atm_price(&self, t: f64) -> f64 {
        bachelier::atm_price(t, self.v)
    }

    /// ATM volatility of a European option using the Bachelier model.
    pub fn atm_vol(&self, price: f64, t: f64) -> f64 {
        bachelier::atm_vol(price, t)
    }

    /// Implied volatility of a European option using the Bachelier model.
    pub fn iv(&self, price: f64, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => bachelier::call_iv(price, self.f, k, t),
            TypeFlag::Put => bachelier::put_iv(price, self.f, k, t),
        }
    }

    /// Delta of a European option using the Bachelier model.
    pub fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => bachelier::call_delta(self.f, k, t, self.v),
            TypeFlag::Put => bachelier::put_delta(self.f, k, t, self.v),
        }
    }

    /// Gamma of a European option using the Bachelier model.
    pub fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => bachelier::call_gamma(self.f, k, t, self.v),
            TypeFlag::Put => bachelier::put_gamma(self.f, k, t, self.v),
        }
    }

    /// Vega of a European option using the Bachelier model.
    pub fn vega(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => bachelier::call_vega(self.f, k, t, self.v),
            TypeFlag::Put => bachelier::put_vega(self.f, k, t, self.v),
        }
    }

    /// Theta of a European option using the Bachelier model.
    pub fn theta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => bachelier::call_theta(self.f, k, t, self.v),
            TypeFlag::Put => bachelier::put_theta(self.f, k, t, self.v),
        }
    }
}
