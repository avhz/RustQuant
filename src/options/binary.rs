#![deny(missing_docs)]

use crate::distributions::gaussian::*;


//! This module contains various 'binary', or 'digital', option types.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug)]
struct GapOption {
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
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let K = self.strike_price;
        let T = self.time_to_maturity;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

        let v_a = v / (3_f64).sqrt();
        let b = r - q;
        let b_a = 0.5 * (b - v * v / 6.0);

        let d1 = ((S / K).ln() + (b_a + 0.5 * v_a * v_a) * T) / (v_a * (T).sqrt());
        let d2 = d1 - v_a * (T).sqrt();

        let N = Gaussian::default();

        let c = S * ((b_a - r) * T).exp() * N.cdf(d1) - K * (-r * T).exp() * N.cdf(d2);
        let p = -S * ((b_a - r) * T).exp() * N.cdf(-d1) + K * (-r * T).exp() * N.cdf(-d2);

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
            dividend_rate: 0,
        };

        let prices = gap.price();

        // Value from Haug's book.
        assert_approx_equal!(prices.1, 4.6922, 0.0001);
    }
}
