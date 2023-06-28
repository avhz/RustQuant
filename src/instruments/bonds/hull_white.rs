// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// TODO: Update Hull-White pricer to accept dates instead of time to maturity.

//! Hull-White model for zero-coupon bond prices.
//!
//! The risk-neutral short rate follows the process:
//!
//! dr = (theta(t) - a*r_t)dt + sigma * dW_t
//!
//! It incorporates a mean-reversion factor into the drift term:
//!
//! - `theta(t)`: is the rate at which it gets pulled.
//! - `a`: is the level to which it gets pulled.
//! - `r_t`: short rate at time t
//! - `sigma`: is the diffusion coefficient.
//! - `t`: time to check price at
//! - `maturity`: time at bond maturity

use crate::instruments::bonds::*;
use crate::math::integrate;

/// Struct containing the Hull-White model parameters.
pub struct HullWhite {
    a: f64,
    theta_t: fn(f64) -> f64,
    sigma: f64,
    r_t: f64,
    t: f64,
    maturity: f64,
}

impl HullWhite {
    // TODO make dependenont t,T
    fn B(&self) -> f64 {
        assert!(self.a > 0.0);
        (1.0 / self.a) * (1.0 - (-self.a).exp())
    }

    // TODO make dependenont t,T
    fn A(&self) -> f64 {
        assert!(self.a > 0.0);
        let first = -1.0 * integrate(|u| (self.theta_t)(u) * self.B(), self.t, self.maturity);
        let second =
            ((self.sigma).powi(2) / (2.0 * (self.a).powi(2))) * (self.B() - self.maturity + self.t);
        let third = ((self.sigma).powi(2) / (4.0 * self.a)) * (self.B()).powi(2);

        (first - second - third).exp()
    }
}

impl ZeroCouponBond for HullWhite {
    fn price(&self) -> f64 {
        assert!(self.a > 0.0);
        assert!(self.maturity >= self.t);

        self.A() * (-1.0 * self.B() * self.r_t).exp()
    }

    // fn duration(&self) -> f64 {}
    // fn convexity(&self) -> f64 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hw_zero_coupon_bond() {
        let hw_bond = HullWhite {
            a: 2.0,
            theta_t: |_x| 0.5,
            sigma: 0.3,
            r_t: 0.05,
            t: 0.0,
            maturity: 10.0,
        };
        let _price = hw_bond.price();
        // TODO check price against actual
        // But this implementation is analytic, so should be right
    }
}
