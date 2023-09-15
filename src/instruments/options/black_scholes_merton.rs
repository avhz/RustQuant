// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::instruments::options::TypeFlag;
use crate::statistics::distributions::{Distribution, Gaussian};
use crate::time::{DayCountConvention, DayCounter};

use time::{Duration, OffsetDateTime};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Generalised Black-Scholes-Merton European Option pricing model.
pub struct BlackScholesMerton {
    /// The cost of carry factor.
    /// For the generalised Black-Scholes-Merton model,
    /// there are five possibilities for the cost of carry factor:
    ///     1. b = r:           Black-Scholes 1973 stock option model.
    ///     2. b = r - q:       Merton 1973 stock option model with continuous dividend yield.
    ///     3. b = 0:           Black 1976 futures option model.
    ///     4. b = 0, r = 0:    Asay 1982 margined futures option model.
    ///     5. b = r_d - r_f:   Garman and Kohlhagen 1983 currency option model.
    pub cost_of_carry: f64,
    /// The underlying asset price.
    pub underlying_price: f64,
    /// The options strike price.
    pub strike_price: f64,
    /// The underlying asset's volatility.
    pub volatility: f64,
    /// The risk-free interest rate.
    pub risk_free_rate: f64,
    /// Evaluation date (optional, defaults to today t = 0).
    pub evaluation_date: Option<OffsetDateTime>,
    /// The options expiration date.
    pub expiration_date: OffsetDateTime,
    /// Call or put flag.
    pub option_type: TypeFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl BlackScholesMerton {
    /// New European Option
    pub fn new(
        cost_of_carry: f64,
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        risk_free_rate: f64,
        evaluation_date: OffsetDateTime,
        expiration_date: OffsetDateTime,
        option_type: TypeFlag,
    ) -> Self {
        Self {
            cost_of_carry,
            underlying_price,
            strike_price,
            volatility,
            risk_free_rate,
            evaluation_date: Some(evaluation_date),
            expiration_date,
            option_type,
        }
    }

    /// Generalised Black-Scholes European Option Price.
    pub fn price(&self) -> f64 {
        let S = self.underlying_price;
        let K = self.strike_price;
        let b = self.cost_of_carry;
        let v = self.volatility;
        let r = self.risk_free_rate;

        // Compute time to maturity.
        let T = match self.evaluation_date {
            Some(evaluation_date) => DayCounter::day_count_factor(
                evaluation_date,
                self.expiration_date,
                &DayCountConvention::Actual365,
            ),
            None => DayCounter::day_count_factor(
                OffsetDateTime::now_utc(),
                self.expiration_date,
                &DayCountConvention::Actual365,
            ),
        };

        let d1 = (1.0 / (v * T.sqrt())) * ((S / K).ln() + (b + 0.5 * v.powi(2)) * T);
        let d2 = d1 - v * T.sqrt();

        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => S * ((b - r) * T).exp() * n.cdf(d1) - K * (-r * T).exp() * n.cdf(d2),
            TypeFlag::Put => {
                -S * ((b - r) * T).exp() * n.cdf(-d1) + K * (-r * T).exp() * n.cdf(-d2)
            }
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_black_scholes_merton {
    use super::*;

    #[test]
    fn black_scholes_1973() {
        // Values from Haug
        let bsm = BlackScholesMerton::new(
            0.08,
            60.0,
            65.0,
            0.3,
            0.08,
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc() + Duration::days(91),
            TypeFlag::Call,
        );
        assert_approx_equal!(bsm.price(), 2.1275937711976134, 1e-10);
    }

    #[test]
    fn merton_1973() {
        // Values from Haug
        let bsm = BlackScholesMerton::new(
            0.1 - 0.05,
            100.0,
            95.0,
            0.2,
            0.1,
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc() + Duration::days(182),
            TypeFlag::Put,
        );
        assert_approx_equal!(bsm.price(), 2.4606727356947076, 1e-10);
    }
}
