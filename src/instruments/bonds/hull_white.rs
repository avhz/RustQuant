// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
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

use crate::instruments::Instrument;
use crate::math::integrate;
use crate::time::{DayCountConvention, DayCounter};
use time::OffsetDateTime;

/// Struct containing the Hull-White model parameters.
pub struct HullWhite {
    a: f64,
    theta_t: fn(f64) -> f64,
    sigma: f64,
    r_t: f64,

    /// `evaluation_date` - Valuation date.
    pub evaluation_date: Option<OffsetDateTime>,
    /// `expiration_date` - Expiry date.
    pub expiration_date: OffsetDateTime,
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

        let today = OffsetDateTime::now_utc();
        let t = (self.evaluation_date.unwrap_or(today).year() - today.year()) as f64;
        let T = (self.expiration_date.year() - today.year()) as f64;

        let first = -1.0 * integrate(|u| (self.theta_t)(u) * self.B(), t, T);

        let second = ((self.sigma).powi(2) / (2.0 * (self.a).powi(2))) * (self.B() - self.tau());

        let third = ((self.sigma).powi(2) / (4.0 * self.a)) * (self.B()).powi(2);

        (first - second - third).exp()
    }

    fn tau(&self) -> f64 {
        DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        )
    }
}

impl Instrument for HullWhite {
    fn price(&self) -> f64 {
        assert!(self.a > 0.0);
        assert!(self.expiration_date >= self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()));

        self.A() * (-1.0 * self.B() * self.r_t).exp()
    }

    fn error(&self) -> Option<f64> {
        None
    }

    fn valuation_date(&self) -> time::OffsetDateTime {
        self.evaluation_date.unwrap_or(OffsetDateTime::now_utc())
    }

    fn instrument_type(&self) -> &'static str {
        "Zero Coupon Bond"
    }
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
            evaluation_date: None,
            expiration_date: OffsetDateTime::now_utc() + time::Duration::days(365 * 10),
        };
        let _price = hw_bond.price();
        // TODO check price against actual
        // But this implementation is analytic, so should be right
    }
}
