// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Vasicek's model for zero-coupon bond prices.
//!
//! The risk-neutral short rate follows the process:
//!
//! dr(t) = k[θ − r(t)]dt + σdW (t)
//! r(0) = r0
//!
//! It incorporates a mean-reversion factor into the drift term:
//!
//! - `k`: is the rate at which it gets pulled.
//! - `θ`: is the level to which it gets pulled.
//! - `σ`: is the diffusion coefficient.

use time::OffsetDateTime;

use crate::{
    instruments::bonds::*,
    time::{DayCountConvention, DayCounter},
};

/// Struct containing the Vasicek model parameters.
pub struct Vasicek {
    r0: f64,
    k: f64,
    theta: f64,
    sigma: f64,

    /// `valuation_date` - Valuation date.
    pub valuation_date: Option<OffsetDateTime>,
    /// `expiry_date` - Expiry date.
    pub expiry_date: OffsetDateTime,
}

impl ZeroCouponBond for Vasicek {
    fn price(&self) -> f64 {
        let k = self.k;
        let theta = self.theta;
        let sigma = self.sigma;
        let r0 = self.r0;

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

        let B = || (1.0 - (-k * tau).exp()) / k;
        let A = || {
            (((B() - tau) * (k.powi(2) * theta - sigma.powi(2) / 2.0)) / k.powi(2)
                - (sigma.powi(2) * B().powi(2)) / (4.0 * k))
                .exp()
        };

        A() * (-B() * r0).exp()

        // Return the option price on the zero coupon bond?
        // let N = Gaussian::default();

        // let P_tS = self.price();
        // self.time_T = maturity;
        // let P_tT = self.price();

        // let sigma_p = self.sigma * ().sqrt();
        // let h = ;

        // (
        //     P_tS * N(h) - strike * P_tT * N(h - sigma_p),
        //     -P_tS * N(-h) + strike * P_tT * N(sigma_p - h),
        // )
    }

    // fn duration(&self) -> f64 {}
    // fn convexity(&self) -> f64 {}
}

#[cfg(test)]
mod tests_bond_vasicek {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_vasicek_zero_coupon_bond() {
        let expiry_date = OffsetDateTime::now_utc() + time::Duration::days(365);

        let vasicek = Vasicek {
            r0: 0.03,
            k: 0.3,
            theta: 0.1,
            sigma: 0.03,
            valuation_date: None,
            expiry_date,
        };

        let vasicek_price = vasicek.price();

        assert_approx_equal!(vasicek_price, 0.9615, 1e-4);
    }
}
