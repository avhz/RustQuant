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
use crate::math::distributions::{Distribution, Gaussian};
use crate::time::{today, DayCountConvention, DayCounter};

use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Bachelier European Option pricing model.
pub struct Bachelier {
    /// The underlying asset price.
    pub underlying_price: f64,
    /// The options strike price.
    pub strike_price: f64,
    /// The underlying asset's volatility.
    pub volatility: f64,

    /// Evaluation date (optional, defaults to today t = 0).
    pub evaluation_date: Option<Date>,
    /// The options expiration date.
    pub expiration_date: Date,

    /// Call or put flag.
    pub option_type: TypeFlag,
}

/// Bachelier European Option pricing model.
#[allow(clippy::module_name_repetitions)]
pub struct ModifiedBachelier {
    /// The underlying asset price.
    pub underlying_price: f64,
    /// The options strike price.
    pub strike_price: f64,
    /// The underlying asset's volatility.
    pub volatility: f64,
    /// Risk-free interest rate.
    pub risk_free_rate: f64,
    /// Dividend yield.
    pub dividend_yield: f64,

    /// Evaluation date (optional, defaults to today t = 0).
    pub evaluation_date: Option<Date>,
    /// The options expiration date.
    pub expiration_date: Date,

    /// Call or put flag.
    pub option_type: TypeFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Bachelier {
    /// New Bachelier European Option
    #[must_use]
    pub fn new(
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        evaluation_date: Option<Date>,
        expiration_date: Date,
        option_type: TypeFlag,
    ) -> Self {
        Self {
            underlying_price,
            strike_price,
            volatility,
            evaluation_date,
            expiration_date,
            option_type,
        }
    }

    /// Bachelier European Option price.
    #[must_use]
    pub fn price(&self) -> f64 {
        let S = self.underlying_price;
        let K = self.strike_price;
        let v = self.volatility;

        // Compute time to maturity.
        let T = DayCountConvention::default().day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        );

        let d1 = (S - K) / (v * T.sqrt());

        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => (S - K) * n.cdf(d1) + v * T.sqrt() * n.pdf(d1),
            TypeFlag::Put => (K - S) * n.cdf(-d1) + v * T.sqrt() * n.pdf(-d1),
        }
    }
}

impl ModifiedBachelier {
    /// New Modified Bachelier European Option
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        risk_free_rate: f64,
        dividend_yield: f64,
        evaluation_date: Option<Date>,
        expiration_date: Date,
        option_type: TypeFlag,
    ) -> Self {
        Self {
            underlying_price,
            strike_price,
            volatility,
            risk_free_rate,
            dividend_yield,
            evaluation_date,
            expiration_date,
            option_type,
        }
    }

    /// Modified Bachelier European Option price.
    #[must_use]
    pub fn price(&self) -> f64 {
        let S = self.underlying_price;
        let K = self.strike_price;
        let v = self.volatility;
        let r = self.risk_free_rate;

        // Compute time to maturity.
        let T = DayCountConvention::default().day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        );

        let d1 = (S - K) / (v * T.sqrt());

        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => (S - K * (-r * T).exp()) * n.cdf(d1) + v * T.sqrt() * n.pdf(d1),
            TypeFlag::Put => (K * (-r * T).exp() - S) * n.cdf(-d1) + v * T.sqrt() * n.pdf(-d1),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_bachelier {
    use super::*;
    use crate::assert_approx_equal;
    use time::Duration;

    #[test]
    fn bachelier() {
        let bachelier = Bachelier::new(
            100.0,
            100.0,
            0.2,
            None,
            today() + Duration::days(365),
            TypeFlag::Call,
        );
        assert_approx_equal!(bachelier.price(), 0.079_679_081_860_151_37, 1e-10);
    }

    #[test]
    fn bachelier_modified() {
        let bachelier = ModifiedBachelier::new(
            100.0,
            100.0,
            0.2,
            0.05,
            0.0,
            None,
            today() + Duration::days(365),
            TypeFlag::Call,
        );
        assert_approx_equal!(bachelier.price(), 2.511_692_140_521_875, 1e-10);
    }
}
