// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FORWARD START OPTION STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::{Distribution, Gaussian};

/// Forward Start Option parameters struct
#[derive(Debug)]
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
    /// `start` - Time until the start of the option (`T` in most literature).
    pub start: f64,
    /// `end` - Time until the end of the option (`t` in most literature).
    pub end: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FORWARD START OPTION IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl ForwardStartOption {
    /// Rubinstein (1990) Forward Start Option Price formula.
    /// Returns a tuple: `(call_price, put_price)`
    /// # Note:
    /// * `b = r - q` - The cost of carry.
    pub fn price(&self) -> (f64, f64) {
        let S = self.initial_price;
        let a = self.alpha;
        let T = self.end;
        let t = self.start;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let q = self.dividend_rate;

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
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn TEST_forward_start_option() {
        let ForwardStart = ForwardStartOption {
            initial_price: 60.0,
            alpha: 1.1,
            risk_free_rate: 0.08,
            volatility: 0.3,
            dividend_rate: 0.04,
            start: 0.25,
            end: 1.0,
        };

        let prices = ForwardStart.price();

        // Call price example from Haug's book.
        assert_approx_equal!(prices.0, 4.4064, 0.0001);
    }
}
