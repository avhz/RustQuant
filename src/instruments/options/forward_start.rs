// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FORWARD START OPTION STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::Date;

use crate::{
    math::distributions::{Distribution, Gaussian},
    time::{today, DayCountConvention},
};

/// Forward Start Option parameters struct
#[allow(clippy::module_name_repetitions)]
#[derive(derive_builder::Builder, Debug)]
pub struct ForwardStartOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `alpha` - The proportion of S to set the strike price.
    /// Three possibilities:
    ///     - alpha < 1: call (put) will start (1 - alpha)% in-the-money (out-of-the-money).
    ///     - alpha = 1: the option starts at-the-money.
    ///     - alpha > 1: call (put) will start (alpha - 1)% out-of-the-money (in-the-money).
    pub alpha: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `v` - Volatility parameter.
    pub volatility: f64,
    /// `q` - Dividend rate.
    pub dividend_rate: f64,

    /// `valuation_date` - Valuation date.
    #[builder(default = "None")]
    pub valuation_date: Option<Date>,

    /// `start` - Time until the start of the option (`T` in most literature).
    pub start: Date,
    /// `end` - Time until the end of the option (`t` in most literature).
    pub end: Date,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FORWARD START OPTION IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ForwardStartOption {
    /// Rubinstein (1990) Forward Start Option Price formula.
    /// Returns a tuple: `(call_price, put_price)`
    /// # Note:
    /// * `b = r - q` - The cost of carry.
    #[must_use]
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let a = self.alpha;

        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

        let T = DayCountConvention::default()
            .day_count_factor(self.valuation_date.unwrap_or(today()), self.end);

        let t = DayCountConvention::default()
            .day_count_factor(self.valuation_date.unwrap_or(today()), self.start);

        let b = r - q;

        let d1 = ((1. / a).ln() + (b + v * v / 2.) * (T - t)) / (v * (T - t).sqrt());
        let d2 = d1 - v * (T - t).sqrt();

        let norm = Gaussian::default();

        let Nd1: f64 = norm.cdf(d1);
        let Nd2: f64 = norm.cdf(d2);

        let Nd1_: f64 = norm.cdf(-d1);
        let Nd2_: f64 = norm.cdf(-d2);

        let c: f64 = S
            * ((b - r) * t).exp()
            * (((b - r) * (T - t)).exp() * Nd1 - a * (-r * (T - t)).exp() * Nd2);
        let p: f64 = S
            * ((b - r) * t).exp()
            * (-((b - r) * (T - t)).exp() * Nd1_ + a * (-r * (T - t)).exp() * Nd2_);

        (c, p)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_forward_start {
    use super::*;
    use crate::{assert_approx_equal, RUSTQUANT_EPSILON};

    #[test]
    fn TEST_forward_start_option() {
        let start = today() + time::Duration::days(91);
        let end = today() + time::Duration::days(365);

        let ForwardStart = ForwardStartOption {
            initial_price: 60.0,
            alpha: 1.1,
            risk_free_rate: 0.08,
            volatility: 0.3,
            dividend_rate: 0.04,
            valuation_date: None,
            start,
            end,
        };

        let prices = ForwardStart.price();

        // Call price example from Haug's book.
        assert_approx_equal!(prices.0, 4.402888269001168, 1e-2);
    }
}
