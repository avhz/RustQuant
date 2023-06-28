// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Cox-Ingersoll-Ross model for zero-coupon bond prices.
//!
//! The risk-neutral short rate follows the process:
//!
//! dr = a(b-r)dt + sigma * sqrt(r) * dW
//!
//! It incorporates a mean-reversion factor into the drift term:
//!
//! - `a`: is the rate at which it gets pulled.
//! - `b`: is the level to which it gets pulled.
//! - `sigma`: is the diffusion coefficient.
//!
//! Furthermore, it makes the standard deviation proportional to sqrt(r).
//! This means that, as the short-term interest rate increases,
//! the standard deviation increases.

use crate::{
    instruments::bonds::*,
    time::{DayCountConvention, DayCounter},
};
use time::OffsetDateTime;

/// Struct containing the Cox-Ingersoll-Ross model parameters.
pub struct CoxIngersollRoss {
    a: f64,
    b: f64,
    sigma: f64,
    r: f64,

    /// `valuation_date` - Valuation date.
    pub valuation_date: Option<OffsetDateTime>,
    /// `expiry_date` - Expiry date.
    pub expiry_date: OffsetDateTime,
}

impl ZeroCouponBond for CoxIngersollRoss {
    fn price(&self) -> f64 {
        let a = self.a;
        let b = self.b;
        let sigma = self.sigma;
        let r = self.r;

        // Compute time to maturity.
        let tau = match self.valuation_date {
            Some(valuation_date) => DayCounter::day_count_factor(
                valuation_date,
                self.expiry_date,
                &DayCountConvention::Actual365,
            ),
            None => DayCounter::day_count_factor(
                OffsetDateTime::now_utc(),
                self.expiry_date,
                &DayCountConvention::Actual365,
            ),
        };

        let gamma = (a * a + 2.0 * sigma.powi(2)).sqrt();

        let b_t = 2.0 * ((gamma * tau).exp() - 1.0)
            / ((gamma + a) * ((gamma * tau).exp() - 1.0) + 2.0 * gamma);
        let a_t = (2.0 * gamma * ((a + gamma) * tau / 2.0).exp()
            / ((gamma + a) * ((gamma * tau).exp() - 1.0) + 2.0 * gamma))
            .powf(2.0 * a * b / sigma.powi(2));

        // Price:
        a_t * (-b_t * r).exp()
    }

    // fn duration(&self) -> f64 {}
    // fn convexity(&self) -> f64 {}
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_cir_zero_coupon_bond() {
        let expiry = OffsetDateTime::now_utc() + time::Duration::days(365);

        let cir = CoxIngersollRoss {
            a: 0.3,
            b: 0.1,
            sigma: 0.03,
            r: 0.03,
            valuation_date: None,
            expiry_date: expiry,
        };

        let cir_price = cir.price();

        assert_approx_equal!(cir_price, 0.9613, 1e-4);
    }
}
