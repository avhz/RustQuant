// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains various 'binary', or 'digital', option types.

use crate::distributions::{gaussian::*, Distribution};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Gap option parameters.
#[derive(Debug)]
pub struct GapOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `K_1` - First strike price (barrier strike).
    pub strike_1: f64,
    /// `K_2` - Second strike price (payoff strike).
    pub strike_2: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `v` - Volatility parameter.
    pub volatility: f64,
    /// `b` - Cost-of-carry.
    pub cost_of_carry: f64,
    /// `T` - Time to expiry/maturity.
    pub time_to_maturity: f64,
}

/// Cash-or-Nothing option parameters.
pub struct CashOrNothingOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `X` - Strike price.
    pub strike_price: f64,
    /// `K` - Cash payout amount.
    pub payout_value: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `v` - Volatility parameter.
    pub volatility: f64,
    /// `b` - Cost-of-carry.
    pub cost_of_carry: f64,
    /// `T` - Time to expiry/maturity.
    pub time_to_maturity: f64,
}

// pub struct AssetOrNothingOption {}
// pub struct SupershareOption {}
// pub struct BinaryBarrierOption {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl GapOption {
    /// Gap option pricer.
    /// The payoff from a call is 0 if S < K_1 and S — K_2 if S > K_1.
    /// Similarly, the payoff from a put is 0 if S > K_1 and K_2 — S if S < K_1.
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let K_1 = self.strike_1;
        let K_2 = self.strike_2;
        let T = self.time_to_maturity;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let b = self.cost_of_carry;

        let d1 = ((S / K_1).ln() + (b + 0.5 * v * v) * T) / (v * (T).sqrt());
        let d2 = d1 - v * (T).sqrt();

        let N = Gaussian::default();

        let c = S * ((b - r) * T).exp() * N.cdf(d1) - K_2 * (-r * T).exp() * N.cdf(d2);
        let p = -S * ((b - r) * T).exp() * N.cdf(-d1) + K_2 * (-r * T).exp() * N.cdf(-d2);

        (c, p)
    }
}

impl CashOrNothingOption {
    /// Cah-or-Nothing option pricer.
    /// The payoff from a call is 0 if S < X and K if S > X.
    /// The payoff from a put is 0 if S > X and K if S < X.
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let X = self.strike_price;
        let K = self.payout_value;
        let T = self.time_to_maturity;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let b = self.cost_of_carry;

        let d = ((S / X).ln() + (b - 0.5 * v * v) * T) / (v * (T).sqrt());

        let N = Gaussian::default();

        let c = K * (-r * T).exp() * N.cdf(d);
        let p = K * (-r * T).exp() * N.cdf(-d);

        (c, p)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_gap_option() {
        let gap = GapOption {
            initial_price: 50.0,
            strike_1: 50.0,
            strike_2: 57.0,
            risk_free_rate: 0.09,
            volatility: 0.2,
            time_to_maturity: 0.5,
            cost_of_carry: 0.09,
        };

        let prices = gap.price();

        // Value from Haug's book (note: gap option payoffs can be negative).
        assert_approx_equal!(prices.0, -0.0053, 0.0001);
    }

    #[test]
    fn test_cash_or_nothing_option() {
        let CON = CashOrNothingOption {
            initial_price: 100.0,
            payout_value: 10.0,
            strike_price: 80.0,
            risk_free_rate: 0.06,
            volatility: 0.35,
            time_to_maturity: 0.75,
            cost_of_carry: 0.0,
        };

        let prices = CON.price();

        // Value from Haug's book.
        assert_approx_equal!(prices.1, 2.6710, 0.0001);
    }
}
