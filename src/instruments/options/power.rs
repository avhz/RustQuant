// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! # Power Contracts
//!
//! Power contracts are options with the payoff: (S/K)^i
//! where i is the (fixed) power of the contract.

use crate::time::{today, DayCountConvention, DayCounter};
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Power Option contract.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct PowerOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `K` - Strike price.
    pub strike_price: f64,
    /// `i` - Power of the contract.
    pub power: f64,

    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `b` - Cost of carry.
    pub cost_of_carry: f64,
    /// `v` - Volatility parameter.
    pub volatility: f64,

    /// `valuation_date` - Valuation date.
    pub evaluation_date: Option<Date>,
    /// `expiry_date` - Expiry date.
    pub expiration_date: Date,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl PowerOption {
    /// New Power Option contract.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        initial_price: f64,
        strike_price: f64,
        power: f64,
        risk_free_rate: f64,
        cost_of_carry: f64,
        volatility: f64,
        evaluation_date: Option<Date>,
        expiration_date: Date,
    ) -> Self {
        Self {
            initial_price,
            strike_price,
            power,
            risk_free_rate,
            cost_of_carry,
            volatility,
            evaluation_date,
            expiration_date,
        }
    }

    /// Power Option price.
    #[must_use]
    pub fn price(&self) -> f64 {
        let S = self.initial_price;
        let K = self.strike_price;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let b = self.cost_of_carry;
        let i = self.power;

        // Compute time to maturity.
        let T = DayCountConvention::default().day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        );

        (S / K).powf(i) * (((b - 0.5 * v.powi(2)) * i - r + 0.5 * (i * v).powi(2)) * T).exp()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_power_contract {
    use super::*;
    use crate::assert_approx_equal;
    use time::Duration;

    use std::f64::EPSILON as EPS;

    #[test]
    fn test_power() {
        let power_option = PowerOption {
            initial_price: 400.,
            strike_price: 450.,
            power: 2.,
            risk_free_rate: 0.08,
            cost_of_carry: 0.06,
            volatility: 0.25,
            evaluation_date: None,
            expiration_date: today() + Duration::days(182),
        };

        assert_approx_equal!(power_option.price(), 0.831_322_640_144_985_4, EPS);
    }
}
