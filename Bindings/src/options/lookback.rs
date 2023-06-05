// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{
    distributions::{Distribution, Gaussian},
    options::*,
    stochastics::*,
    utilities::*,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Lookback option strike type enum.
/// The strike can be either fixed or floating.
#[derive(Debug)]
pub enum LookbackStrike {
    /// Floating strike lookback option.
    /// Payoffs:
    /// - Call: `max(S_T - S_min, 0)`
    /// - Put: `max(S_max - S_T, 0)`
    Floating,
    /// Fixed strike lookback option.
    /// Payoffs:
    /// - Call: `max(S_max - K, 0)`
    /// - Put: `max(K - S_min, 0)`
    Fixed,
}

/// Struct containing Lookback Option parameters.
#[derive(Debug)]
pub struct LookbackOption {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
    /// `K` - Strike price (only needed for fixed strike lookbacks).
    /// If the strike is floating, then this is `None`.
    pub strike_price: Option<f64>,
    /// `v` - Volatility parameter.
    pub volatility: f64,
    /// `T` - Time to expiry/maturity.
    pub time_to_maturity: f64,
    /// `q` - dividend yield.
    pub dividend_yield: f64,
    /// Minimum value of the underlying price observed **so far**.
    /// If the contract starts at t=0, then `S_min = S_0`.
    /// Used for the closed-form put price.
    pub s_min: f64,
    /// Maximum value of the underlying price observed **so far**.
    /// If the contract starts at t=0, then `S_max = S_0`.
    /// Used for the closed-form call price.
    pub s_max: f64,
    /// Strike type.
    pub strike_type: LookbackStrike,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl LookbackOption {
    /// Closed-form lookback option price.
    pub fn price_analytic(&self) -> (f64, f64) {
        let s = self.initial_price;
        let r = self.risk_free_rate;
        let t = self.time_to_maturity;
        let v = self.volatility;
        let q = self.dividend_yield;
        let s_min = self.s_min;
        let s_max = self.s_max;

        let b = r - q; // Cost of carry

        let norm = Gaussian::default();

        let call: f64;
        let put: f64;

        match self.strike_type {
            LookbackStrike::Floating => {
                let a1 = ((s / s_min).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
                let a2 = a1 - v * t.sqrt();
                let b1 = ((s / s_max).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
                let b2 = b1 - v * t.sqrt();

                if b != 0.0 {
                    call = s * ((b - r) * t).exp() * norm.cdf(a1)
                        - s_min * (-r * t).exp() * norm.cdf(a2)
                        + s * (-r * t).exp()
                            * (v * v / (2.0 * b))
                            * ((s / s_min).powf(-2.0 * b / (v * v))
                                * norm.cdf(-a1 + 2.0 * b * t.sqrt() / v)
                                - (b * t).exp() * norm.cdf(-a1));

                    put = -s * ((b - r) * t).exp() * norm.cdf(-b1)
                        + s_max * (-r * t).exp() * norm.cdf(-b2)
                        + s * (-r * t).exp()
                            * (v * v / (2.0 * b))
                            * (-(s / s_max).powf(-2.0 * b / (v * v))
                                * norm.cdf(b1 - 2.0 * b * t.sqrt() / v)
                                + (b * t).exp() * norm.cdf(b1));
                } else {
                    call = s * (-r * t).exp() * norm.cdf(a1)
                        - s_min * (-r * t).exp() * norm.cdf(a2)
                        + s * (-r * t).exp()
                            * v
                            * t.sqrt()
                            * (norm.pdf(a1) + a1 * (norm.cdf(a1) - 1.0));

                    put = -s * ((b - r) * t).exp() * norm.cdf(-b1)
                        + s_max * (-r * t).exp() * norm.cdf(-b2)
                        + s * (-r * t).exp() * v * t.sqrt() * (norm.pdf(b1) + b1 * norm.cdf(b1));
                }
                (call, put)
            }
            LookbackStrike::Fixed => {
                let x = self.strike_price.unwrap();

                let d1 = ((s / x).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
                let d2 = d1 - v * t.sqrt();
                let e1 = ((s / s_max).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
                let e2 = e1 - v * t.sqrt();
                let f1 = ((s / s_min).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
                let f2 = f1 - v * t.sqrt();

                let call = if x > s_max {
                    s * ((b - r) * t).exp() * norm.cdf(d1) - x * (-r * t).exp() * norm.cdf(d2)
                        + s * (-r * t).exp()
                            * (v * v / (2.0 * b))
                            * (-(s / x).powf(-2.0 * b / (v * v))
                                * norm.cdf(d1 - 2.0 * b * t.sqrt() / v)
                                + (b * t).exp() * norm.cdf(d1))
                } else {
                    (-r * t).exp() * (s_max - x) + s * ((b - r) * t).exp() * norm.cdf(e1)
                        - s_max * (-r * t).exp() * norm.cdf(e2)
                        + s * (-r * t).exp()
                            * (v * v / (2.0 * b))
                            * (-(s / s_max).powf(-2.0 * b / (v * v))
                                * norm.cdf(e1 - 2.0 * b * t.sqrt() / v)
                                + (b * t).exp() * norm.cdf(e1))
                };

                let put = if x < s_min {
                    -s * ((b - r) * t).exp() * norm.cdf(-d1)
                        + x * (-r * t).exp() * norm.cdf(-d2)
                        + s * (-r * t).exp()
                            * (v * v / (2.0 * b))
                            * ((s / x).powf(-2.0 * b / (v * v))
                                * norm.cdf(-d1 + 2.0 * b * t.sqrt() / v)
                                - (b * t).exp() * norm.cdf(-d1))
                } else {
                    (-r * t).exp() * (x - s_min) - s * ((b - r) * t).exp() * norm.cdf(-f1)
                        + s_min * (-r * t).exp() * norm.cdf(-f2)
                        + s * (-r * t).exp()
                            * (v * v / (2.0 * b))
                            * ((s / s_min).powf(-2.0 * b / (v * v))
                                * norm.cdf(-f1 + 2.0 * b * t.sqrt() / v)
                                - (b * t).exp() * norm.cdf(-f1))
                };

                (call, put)
            }
        }
    }

    fn payoff(&self, option_type: TypeFlag, strike_type: LookbackStrike, path: &[f64]) -> f64 {
        // let S_min = path.iter().copied().fold(path[0] /*f64::NAN*/, f64::min);
        // let S_max = path.iter().copied().fold(path[0] /*f64::NAN*/, f64::max);

        let S_min = *path.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let S_max = *path.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

        let S_T = path.last().unwrap();

        match option_type {
            TypeFlag::CALL => match strike_type {
                LookbackStrike::Fixed => f64::max(S_max - self.strike_price.unwrap(), 0.0),
                LookbackStrike::Floating => f64::max(S_T - S_min, 0.0),
            },
            TypeFlag::PUT => match strike_type {
                LookbackStrike::Fixed => f64::max(self.strike_price.unwrap() - S_min, 0.0),
                LookbackStrike::Floating => f64::max(S_max - S_T, 0.0),
            },
        }
    }

    /// Monte Carlo simulation of the lookback option price.
    pub fn price_simulated(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64) {
        let x_0 = self.initial_price;
        let r = self.risk_free_rate;
        let q = self.dividend_yield;
        let sigma = self.volatility;
        let t_n = self.time_to_maturity;

        // Adjust the drift term to account for the cost of carry.
        let cost_of_carry = r - q;
        let gbm = GeometricBrownianMotion::new(cost_of_carry, sigma);

        let paths = gbm.euler_maruyama(x_0, 0.0, t_n, n_steps, n_sims, parallel);

        let mut call_payoffs = Vec::with_capacity(n_sims);
        let mut put_payoffs = Vec::with_capacity(n_sims);

        match self.strike_type {
            LookbackStrike::Fixed => {
                for path in &paths.paths {
                    call_payoffs.push(Self::payoff(
                        self,
                        TypeFlag::CALL,
                        LookbackStrike::Fixed,
                        path,
                    ));
                    put_payoffs.push(Self::payoff(
                        self,
                        TypeFlag::PUT,
                        LookbackStrike::Fixed,
                        path,
                    ));
                }
            }
            LookbackStrike::Floating => {
                for path in &paths.paths {
                    call_payoffs.push(Self::payoff(
                        self,
                        TypeFlag::CALL,
                        LookbackStrike::Floating,
                        path,
                    ));
                    put_payoffs.push(Self::payoff(
                        self,
                        TypeFlag::PUT,
                        LookbackStrike::Floating,
                        path,
                    ));
                }
            }
        }

        (
            // Discounted mean of the call and put payoffs.
            (-r * t_n).exp() * mean(&call_payoffs, MeanType::Arithmetic),
            (-r * t_n).exp() * mean(&put_payoffs, MeanType::Arithmetic),
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_lookback {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_lookback_floating() {
        let lbo_floating = LookbackOption {
            initial_price: 50.0,
            s_max: 50.0,
            s_min: 50.0,
            time_to_maturity: 0.25,
            risk_free_rate: 0.1,
            dividend_yield: 0.0,
            volatility: 0.4,
            strike_price: None, // Floating strike has no strike price input.
            strike_type: LookbackStrike::Floating,
        };

        let prices_mc = lbo_floating.price_simulated(500, 10000, true);
        let prices_cf = lbo_floating.price_analytic();

        // Analytic and closed form should match.
        assert_approx_equal!(prices_mc.0, prices_cf.0, 0.5);
        assert_approx_equal!(prices_mc.1, prices_cf.1, 0.5);

        // Hull p.630: Floating-Strike Lookback Option Values
        // Monte Carlo prices.
        assert_approx_equal!(prices_mc.0, 8.04, 0.5);
        assert_approx_equal!(prices_mc.1, 7.79, 0.5);
        // Closed-form prices.
        assert_approx_equal!(prices_cf.0, 8.04, 0.2);
        assert_approx_equal!(prices_cf.1, 7.79, 0.2);

        // The following example (which includes q = 0.06) from Haug's book
        // does not work.
        // The cost of carry seems to be the problem.
        // Even though it is accounted for in the drift term of the GBM.
        // let lbo_floating = LookbackOption {
        //     initial_price: 120.0,
        //     s_max: 100.0,
        //     s_min: 100.0,
        //     time_to_maturity: 0.5,
        //     risk_free_rate: 0.1,
        //     dividend_yield: 0.06,
        //     volatility: 0.3,
        //     strike_price: None, // Floating strike has no strike price input.
        //     strike_type: LookbackStrike::Floating,
        // };
        // Haug p.145: Floating-Strike Lookback Option Values
        // Monte Carlo prices.
        // assert_approx_equal!(prices_mc.0, 25.3533, 0.5);
        // assert_approx_equal!(prices_mc.1, 1.0534, 0.5);
        // Analytic prices.
        // assert_approx_equal!(prices_cf.0, 25.3533, 0.4);
        // assert_approx_equal!(prices_cf.1, 1.0534, 0.4);
    }

    #[test]
    fn test_lookback_fixed() {
        let lbo_fixed = LookbackOption {
            initial_price: 100.0,
            s_max: 100.0,
            s_min: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.1,
            volatility: 0.1,
            strike_price: Some(95.0),
            dividend_yield: 0.0,
            strike_type: LookbackStrike::Fixed,
        };

        let prices_mc = lbo_fixed.price_simulated(1000, 10000, true);
        let prices_cf = lbo_fixed.price_analytic();

        // Analytic and closed form should match.
        assert_approx_equal!(prices_mc.0, prices_cf.0, 0.5);
        assert_approx_equal!(prices_mc.1, prices_cf.1, 0.5);

        // Haug p.145: Fixed-Strike Lookback Option Values
        // Monte Carlo prices.
        assert_approx_equal!(prices_mc.0, 18.3241, 0.5);
        assert_approx_equal!(prices_mc.1, 1.0534, 0.5);

        // Haug p.145: Fixed-Strike Lookback Option Values
        // Analytic prices.
        assert_approx_equal!(prices_cf.0, 18.3241, 0.0001);
        assert_approx_equal!(prices_cf.1, 1.0534, 0.0001);
    }

    #[test]
    fn test_lookback_payoff_fixed() {
        let lbo_fixed = LookbackOption {
            initial_price: 50.0,
            s_max: 50.0,
            s_min: 50.0,
            time_to_maturity: 0.25,
            risk_free_rate: 0.1,
            dividend_yield: 0.0,
            volatility: 0.4,
            strike_price: Some(60.0), // Fixed strike has a strike price input.
            strike_type: LookbackStrike::Fixed,
        };

        let path = vec![50.0, 55.0, 52.0, 58.0, 54.0];

        let call_payoff = lbo_fixed.payoff(TypeFlag::CALL, LookbackStrike::Fixed, &path);
        let put_payoff = lbo_fixed.payoff(TypeFlag::PUT, LookbackStrike::Fixed, &path);

        // Payoff values
        assert_approx_equal!(call_payoff, 0.0, 0.1); // call payoff = max(S_max - K, 0) = max(58 - 60, 0) = 0
        assert_approx_equal!(put_payoff, 10.0, 0.1); // put payoff = max(K - S_min, 0) = max(60 - 50, 0) = 10
    }

    #[test]
    fn test_lookback_payoff_floating() {
        let lbo_floating = LookbackOption {
            initial_price: 50.0,
            s_max: 50.0,
            s_min: 50.0,
            time_to_maturity: 0.25,
            risk_free_rate: 0.1,
            dividend_yield: 0.0,
            volatility: 0.4,
            strike_price: None, // Floating strike has no strike price input.
            strike_type: LookbackStrike::Floating,
        };

        let path = vec![50.0, 55.0, 52.0, 58.0, 54.0];

        let call_payoff = lbo_floating.payoff(TypeFlag::CALL, LookbackStrike::Floating, &path);
        let put_payoff = lbo_floating.payoff(TypeFlag::PUT, LookbackStrike::Floating, &path);

        // Payoff values
        assert_approx_equal!(call_payoff, 4.0, 0.1); // call payoff = max(S_T - S_min, 0) = max(54 - 50, 0) = 4
        assert_approx_equal!(put_payoff, 4.0, 0.1); // put payoff = max(S_max - S_T, 0) = max(58 - 54, 0) = 4
    }
}
