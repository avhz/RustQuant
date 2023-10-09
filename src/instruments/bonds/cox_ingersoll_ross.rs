// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
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
    instruments::Instrument,
    time::{DayCountConvention, DayCounter},
};
use time::OffsetDateTime;

/// Struct containing the Cox-Ingersoll-Ross model parameters.
pub struct CoxIngersollRoss {
    a: f64,
    b: f64,
    r: f64,
    sigma: f64,

    /// `evaluation_date` - Valuation date.
    pub evaluation_date: Option<OffsetDateTime>,
    /// `expiration_date` - Expiry date.
    pub expiration_date: OffsetDateTime,
}

impl Instrument for CoxIngersollRoss {
    fn price(&self) -> f64 {
        let a = self.a;
        let b = self.b;
        let sigma = self.sigma;
        let r = self.r;

        // Compute time to maturity.
        let tau = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let gamma = (a * a + 2.0 * sigma.powi(2)).sqrt();

        let b_t = 2.0 * ((gamma * tau).exp() - 1.0)
            / ((gamma + a) * ((gamma * tau).exp() - 1.0) + 2.0 * gamma);
        let a_t = (2.0 * gamma * ((a + gamma) * tau / 2.0).exp()
            / ((gamma + a) * ((gamma * tau).exp() - 1.0) + 2.0 * gamma))
            .powf(2.0 * a * b / sigma.powi(2));

        // Price:
        a_t * (-b_t * r).exp()
    }

    fn error(&self) -> Option<f64> {
        None
    }

    fn valuation_date(&self) -> OffsetDateTime {
        self.evaluation_date.unwrap_or(OffsetDateTime::now_utc())
    }

    fn instrument_type(&self) -> &'static str {
        "Zero Coupon Bond"
    }
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
            evaluation_date: None,
            expiration_date: expiry,
        };

        let cir_price = cir.price();

        assert_approx_equal!(cir_price, 0.9613, 1e-4);
    }
}
