//! This module contains various 'binary', or 'digital', option types.

#![deny(missing_docs)]

use crate::distributions::gaussian::*;

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
    /// `q` - Dividend rate.
    pub dividend_rate: f64,
    /// `T` - Time to expiry/maturity.
    pub time_to_maturity: f64,
}

struct CashOrNothingOption {}
struct AssetOrNothingOption {}
struct SupershareOption {}
struct BinaryBarrierOption {}

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
        let q = self.dividend_rate;

        let b = r - q;

        let d1 = ((S / K_1).ln() + (b + 0.5 * v * v) * T) / (v * (T).sqrt());
        let d2 = d1 - v * (T).sqrt();

        let N = Gaussian::default();

        let c = S * ((b - r) * T).exp() * N.cdf(d1) - K_2 * (-r * T).exp() * N.cdf(d2);
        let p = -S * ((b - r) * T).exp() * N.cdf(-d1) + K_2 * (-r * T).exp() * N.cdf(-d2);

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
            dividend_rate: 0.0,
        };

        let prices = gap.price();

        // Value from Haug's book (note: gap option payoffs can be negative).
        assert_approx_equal!(prices.0, -0.0053, 0.0001);
    }
}
