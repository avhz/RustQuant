// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::{gaussian::*, Distribution};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Type of Asian option (fixed or floating strike).
pub enum AsianStrike {
    /// Floating strike Asian option.
    /// Payoffs:
    /// - Call: `max(S_T - A, 0)`
    /// - Put: `max(A - S_T, 0)`
    Floating,
    /// Fixed strike Asian option.
    /// Payoffs:
    /// - Call: `max(A - K, 0)`
    /// - Put: `max(K - A, 0)`
    Fixed,
}

/// Method of averaging (arithmetic or geometric, and continuous or discrete).
pub enum AveragingMethod {
    /// Arithmetic Asian option with discrete averaging.
    ArithmeticDiscrete,
    /// Arithmetic Asian option with continuous averaging.
    ArithmeticContinuous,
    /// Geometric Asian option with discrete averaging.
    GeometricDiscrete,
    /// Geometric Asian option with continuous averaging.
    GeometricContinuous,
}

/// Asian Option struct.
#[derive(Debug)]
pub struct AsianOption {
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
    /// `T` - Time to expiry/maturity.
    pub time_to_maturity: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl AsianOption {
    /// Geometric Continuous Average-Rate Price
    pub fn price_geometric_average(&self) -> (f64, f64) {
        let S = self.initial_price;
        let K = self.strike_price;
        let T = self.time_to_maturity;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

        let v_a = v / 3_f64.sqrt();
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
    fn test_asian_geometric() {
        let AsianOption = AsianOption {
            initial_price: 80.0,
            strike_price: 85.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            time_to_maturity: 0.25,
            dividend_rate: -0.03,
        };

        let prices = AsianOption.price_geometric_average();

        // Value from Haug's book.
        assert_approx_equal!(prices.1, 4.6922, 0.0001);
    }
}
