#![allow(non_snake_case)]
#![deny(missing_docs)]

#[cfg(test)]
use crate::helpers::*;
use crate::normal_distribution::*;

// ############################################################################
// STRUCTS
// ############################################################################

/// Black-Scholes Vanilla European Option
#[derive(Debug)]
pub struct EuropeanOption {
    /// Initial price of the underlying.
    pub initial_price: f64,
    /// Strike price.
    pub strike_price: f64,
    /// Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// Volatility parameter.
    pub volatility: f64,
    /// Dividend rate.
    pub dividend_rate: f64,
    /// Time to expiry/maturity.
    pub time_to_maturity: f64,
}

// ############################################################################
// IMPLEMENTATION
// ############################################################################

impl EuropeanOption {
    /// Black-Scholes European Call Option Price
    ///
    /// Returns a tuple: `(call_price, put_price)`
    ///
    /// # Arguments:
    ///
    /// * `S` - Initial underlying price.
    /// * `K` - Strike price.
    /// * `T` - Time to expiry.
    /// * `r` - Risk-free rate.
    /// * `v` - Volatility.
    /// * `q` - Dividend yield.
    ///
    /// # Note:
    ///
    /// * `b = r - q` - The cost of carry.
    ///
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let K = self.strike_price;
        let T = self.time_to_maturity;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

        let df: f64 = (-r * T).exp();
        let b: f64 = r - q;
        let Fp: f64 = S * (b * T).exp();
        let std: f64 = v * T.sqrt();
        let d: f64 = (Fp / K).ln() / std;
        let d1: f64 = d + 0.5 * std;
        let d2: f64 = d1 - std;

        let Nd1: f64 = pnorm(d1);
        let Nd2: f64 = pnorm(d2);

        let Nd1_: f64 = pnorm(-d1);
        let Nd2_: f64 = pnorm(-d2);

        let c: f64 = df * (Fp * Nd1 - K * Nd2);
        let p: f64 = df * (-Fp * Nd1_ + K * Nd2_);

        (c, p)
    }
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn TEST_black_scholes() {
        let VanillaOption = EuropeanOption {
            initial_price: 100.0,
            strike_price: 110.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_rate: 0.02,
            time_to_maturity: 0.5,
        };

        let prices = VanillaOption.price();
        assert_approx_equal!(prices.0, 2.586, 0.001);
        assert_approx_equal!(prices.1, 10.865, 0.001);
    }
}
