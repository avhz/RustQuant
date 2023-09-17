// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// EUROPEAN OPTION STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::OffsetDateTime;

use crate::{
    statistics::distributions::{Distribution, Gaussian},
    time::{DayCountConvention, DayCounter},
};

/// Black-Scholes Vanilla European Option
#[derive(Debug, Clone, Copy)]
pub struct EuropeanOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `K` - Strike price.
    pub strike_price: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `v` - Volatility parameter.
    pub volatility: f64,
    /// `q` - Dividend rate.
    pub dividend_rate: f64,
    /// `valuation_date` - Valuation date.
    pub evaluation_date: Option<OffsetDateTime>,
    /// `expiry_date` - Expiry date.
    pub expiration_date: OffsetDateTime,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// EUROPEAN OPTION IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl EuropeanOption {
    /// New European Option
    pub fn new(
        initial_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        dividend_rate: f64,
        evaluation_date: Option<OffsetDateTime>,
        expiration_date: OffsetDateTime,
    ) -> Self {
        Self {
            initial_price,
            strike_price,
            risk_free_rate,
            volatility,
            dividend_rate,
            evaluation_date,
            expiration_date,
        }
    }

    /// Black-Scholes European Call Option Price
    /// Returns a tuple: `(call_price, put_price)`
    /// # Note:
    /// * `b = r - q` - The cost of carry.
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let K = self.strike_price;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

        // Compute time to maturity.
        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let df: f64 = (-r * T).exp();
        let b: f64 = r - q;
        let Fp: f64 = S * (b * T).exp();
        let std: f64 = v * T.sqrt();
        let d: f64 = (Fp / K).ln() / std;
        let d1: f64 = d + 0.5 * std;
        let d2: f64 = d1 - std;

        let norm = Gaussian::default();

        let Nd1: f64 = norm.cdf(d1);
        let Nd2: f64 = norm.cdf(d2);

        let Nd1_: f64 = norm.cdf(-d1);
        let Nd2_: f64 = norm.cdf(-d2);

        let c: f64 = df * (Fp * Nd1 - K * Nd2);
        let p: f64 = df * (-Fp * Nd1_ + K * Nd2_);

        (c, p)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_black_scholes {
    use super::*;
    use crate::assert_approx_equal;
    use time::Duration;

    #[test]
    fn TEST_black_scholes() {
        let expiry_date = OffsetDateTime::now_utc() + Duration::days(182);

        let vanilla_option = EuropeanOption::new(
            100.,        // Underlying price
            110.,        // Strike price
            0.05,        // Risk-free rate
            0.2,         // Volatility
            0.0,         // Dividend rate
            None,        // Valuation date
            expiry_date, // Expiry date
        );

        let prices = vanilla_option.price();
        assert_approx_equal!(prices.0, 2.8, 0.1);
        assert_approx_equal!(prices.1, 10.18, 0.01);
    }
}
