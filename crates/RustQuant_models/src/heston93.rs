use num::Complex;
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_1_PI, PI};
use RustQuant_math::integrate;

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
    let (p1, p2) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

    s * (-q * t).exp() * p1 - k * (-r * t).exp() * p2
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
    let (p1, _) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

    (-q * t).exp() * p1
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

    let c = |j: Switch, phi: f64| -> Complex<f64> {
        (r - q) * i * phi * t
            + (kappa * theta / sigma.powi(2))
                * ((b(j) - rho * sigma * i * phi + d(j, phi)) * t
                    - 2.0 * ((1.0 - g(j, phi) * (d(j, phi) * t).exp()) / (1.0 - g(j, phi))).ln())
    };

    let d = |j: Switch, phi: f64| -> Complex<f64> {
        ((b(j) - rho * sigma * i * phi + d(j, phi)) * (1.0 - (d(j, phi) * t).exp()))
            / (sigma.powi(2) * (1.0 - g(j, phi) * (d(j, phi) * t).exp()))
    };

    // The Heston characteristic functions.
    let f = |j: Switch, phi: f64| -> Complex<f64> {
        (c(j, phi) + d(j, phi) * v + i * phi * s.ln()).exp()
    };

    let re1 = |phi: f64| -> f64 {
        let j = Switch::One;

        ((-i * phi * k.ln()).exp() * f(j, phi)).re
    };

    let dp1 = integrate(re1, 0.00001, 50.0) * (PI * s).recip();

    dp1 * (-q * t).exp()
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
    let (_, p2) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

    k * t * (-r * t).exp() * p2
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
    let (_, p2) = p1p2(s, v, k, t, r, q, rho, kappa, theta, sigma);

    k * t * (-r * t).exp() * (p2 - 1.0)
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

    let c = |j: Switch, phi: f64| -> Complex<f64> {
        (r - q) * i * phi * t
            + (kappa * theta / sigma.powi(2))
                * ((b(j) - rho * sigma * i * phi + d(j, phi)) * t
                    - 2.0 * ((1.0 - g(j, phi) * (d(j, phi) * t).exp()) / (1.0 - g(j, phi))).ln())
    };

    let d = |j: Switch, phi: f64| -> Complex<f64> {
        ((b(j) - rho * sigma * i * phi + d(j, phi)) * (1.0 - (d(j, phi) * t).exp()))
            / (sigma.powi(2) * (1.0 - g(j, phi) * (d(j, phi) * t).exp()))
    };

    // The Heston characteristic functions.
    let f = |j: Switch, phi: f64| -> Complex<f64> {
        (c(j, phi) + d(j, phi) * v + i * phi * s.ln()).exp()
    };

    // These functions return the integrand for P1 and P2.
    let re1 = |phi: f64| -> f64 {
        let j = Switch::One;

        (f(j, phi) * (-i * phi * k.ln()).exp() / (i * phi)).re
    };
    let re2 = |phi: f64| -> f64 {
        let j = Switch::Two;

        (f(j, phi) * (-i * phi * k.ln()).exp() / (i * phi)).re
    };

    // Integration bounds given in Fabrice D. Rouah's book (see tests).
    // The integral decays rapidly so 50 is probably enough.
    let p1 = 0.5 + FRAC_1_PI * integrate(re1, 0.00001, 50.0);
    let p2 = 0.5 + FRAC_1_PI * integrate(re2, 0.00001, 50.0);

    (p1, p2)
}

impl Heston93 {
    /// Price a European option using the Heston model.
    pub fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => call_price(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => put_price(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    /// Delta of a European option using the Heston model.
    pub fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => call_delta(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => put_delta(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    /// Gamma of a European option using the Heston model.
    pub fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => call_gamma(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => put_gamma(s, v, k, t, r, q, rho, kappa, theta, sigma),
        }
    }

    /// Rho of a European option using the Heston model.
    pub fn rho(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        let (s, v, r, q, rho, kappa, theta, sigma) = self.unpack();

        match option_type {
            TypeFlag::Call => call_rho(s, v, k, t, r, q, rho, kappa, theta, sigma),
            TypeFlag::Put => put_rho(s, v, k, t, r, q, rho, kappa, theta, sigma),
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
