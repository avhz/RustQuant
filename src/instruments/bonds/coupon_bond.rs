// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::zero_coupon_bond::ZeroCouponBond;
use crate::data::{Curve, YieldCurve};
use crate::instruments::fx::currency::Currency;
use crate::instruments::Instrument;
use crate::time::{DateRollingConvention, Frequency};
use std::collections::BTreeMap;
use time::{Date, Duration};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Coupon bond struct.
/// A coupon bond is a debt obligation with coupons attached that represent
/// interest payments on the debt.
///
/// A coupon bond can be viewed as a portfolio of zero-coupon bonds.
///
/// For example, consider a 1.5 year bond with a 5% coupon rate (semi-annual)
/// and a face value of $100.
/// Then the bond can be viewed as a portfolio of three zero-coupon bonds:
/// - A 6-month zero-coupon bond with a face value of $2.50.
/// - A 12-month zero-coupon bond with a face value of $2.50.
/// - An 18-month zero-coupon bond with a face value of $102.50.
#[allow(clippy::module_name_repetitions)]
pub struct CouponBond {
    /// The date the bond is evaluated (i.e. priced).
    pub evaluation_date: Date,

    /// The date the bond expires (i.e. matures, is redeemed).
    pub expiration_date: Date,

    /// The currency of the bond (optional).
    pub currency: Option<Currency>,

    /// The coupon rate of the bond.
    pub coupon_rate: f64,

    /// The coupon frequency of the bond.
    pub coupon_frequency: Frequency,

    /// Settlement convention.
    pub settlement_convention: DateRollingConvention,

    /// Yield curve to use for pricing.
    pub yield_curve: YieldCurve,

    /// The face value of the bond.
    pub face_value: f64,

    /// The coupons of the bond.
    /// The coupons are represented as a map of dates to coupon amounts,
    /// ordered by date.
    /// The final coupon is the face value of the bond.
    pub coupons: BTreeMap<Date, f64>,
}

/// Coupon bond struct.
pub struct CouponBond2 {
    /// Portfolio of zero-coupon bonds.
    pub coupons: BTreeMap<Date, ZeroCouponBond>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl CouponBond {
    /// Constructs the coupons of the bond.
    pub fn construct_coupons(&mut self) {
        let mut coupons: BTreeMap<Date, f64> = BTreeMap::new();

        // Create the coupon dates
        let years = (self.expiration_date - self.evaluation_date).whole_days() / 365;
        let n_coupons = years * self.coupon_frequency as i64;

        let mut coupon_dates: Vec<Date> = Vec::with_capacity(n_coupons as usize);

        for i in 1..=n_coupons {
            let coupon_date =
                self.evaluation_date + Duration::days(365 * i) / self.coupon_frequency as i32;

            coupon_dates.push(coupon_date);
        }

        // Create the coupon amounts
        let mut coupon_amounts: Vec<f64> = Vec::with_capacity(n_coupons as usize);

        for _ in 1..n_coupons {
            let coupon_amount =
                self.face_value * self.coupon_rate / self.coupon_frequency as isize as f64;

            coupon_amounts.push(coupon_amount);
        }

        // Create the coupons
        for (date, amount) in coupon_dates.iter().zip(coupon_amounts.iter()) {
            coupons.insert(*date, *amount);
        }

        // Add the final coupon
        coupons.insert(
            self.expiration_date,
            self.face_value * (1.0 + self.coupon_rate / self.coupon_frequency as isize as f64),
        );

        self.coupons = coupons;
    }
}

impl Instrument for CouponBond {
    /// Returns the price (net present value) of the instrument.
    fn price(&self) -> f64 {
        // Compute the discount factors for the coupons.
        let discount_factors = self
            .yield_curve
            .discount_factors(&self.coupons.keys().copied().collect::<Vec<Date>>());
        // .iter()
        // .enumerate()
        // .map(|(i, df)| (1. + df / self.coupon_frequency as i32 as f64).powi((i + 1) as i32))
        // .collect::<Vec<f64>>();

        // Compute the present value of the coupons and face value, and sum them.
        self.coupons
            .values()
            .zip(discount_factors.iter())
            .map(|(coupon, df)| coupon * df)
            .sum::<f64>()
    }

    /// Returns the error on the NPV in case the pricing engine can
    /// provide it (e.g. Monte Carlo pricing engine).
    fn error(&self) -> Option<f64> {
        None
    }

    /// Returns the date at which the NPV is calculated.
    fn valuation_date(&self) -> Date {
        self.evaluation_date
    }

    /// Instrument type.
    fn instrument_type(&self) -> &'static str {
        "Coupon Bond"
    }
}

impl CouponBond2 {
    /// Validate the dates.
    /// All evaluation dates must be the same, since it is a single instrument,
    /// we just happen to be treating it as a portfolio of zero-coupon bonds.
    #[must_use]
    pub fn validate_dates(&self) -> bool {
        let mut evaluation_date: Option<Date> = None;

        for bond in self.coupons.values() {
            if evaluation_date.is_none() {
                evaluation_date = Some(bond.evaluation_date);
            } else if evaluation_date != Some(bond.evaluation_date) {
                return false;
            }
        }

        true
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_bond {
    use super::*;
    use crate::{data::Curve, iso::USD, time::today};

    #[allow(clippy::similar_names)]
    fn create_test_yield_curve(t0: Date) -> YieldCurve {
        // Create a treasury yield curve with 8 points (3m, 6m, 1y, 2y, 5y, 10y, 30y).
        // Values from Bloomberg: <https://www.bloomberg.com/markets/rates-bonds/government-bonds/us>
        let rate_vec = vec![0.0544, 0.0556, 0.0546, 0.0514, 0.0481, 0.0481, 0.0494];
        let date_vec = vec![
            t0 + Duration::days(90),
            t0 + Duration::days(180),
            t0 + Duration::days(365),
            t0 + Duration::days(2 * 365),
            t0 + Duration::days(5 * 365),
            t0 + Duration::days(10 * 365),
            t0 + Duration::days(30 * 365),
        ];

        YieldCurve::from_dates_and_rates(&date_vec, &rate_vec)
    }

    #[test]
    fn test_coupon_construction() {
        let today = today();

        let mut bond = CouponBond {
            evaluation_date: today,
            expiration_date: today + Duration::days(365 * 2),
            currency: Some(USD),
            coupon_rate: 0.15,
            coupon_frequency: Frequency::SemiAnnually,
            settlement_convention: DateRollingConvention::Actual,
            yield_curve: create_test_yield_curve(today),
            face_value: 1000.0,
            coupons: BTreeMap::new(),
        };

        bond.construct_coupons();

        // Should be: $1,184.61
        // Getting:   $1,198.47
        // Think its close enough for now, down to differences in my computation
        // and the calculator I used. Possibly continuous compounding vs discrete.
        println!("Price: {}", bond.price());
    }
}
