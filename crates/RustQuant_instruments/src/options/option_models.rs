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
