use crate::{helpers::*, math::*, options::*, stochastics::*};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct containing Floating-Strike Lookback Option parameters.
#[derive(Debug)]
pub struct LookbackOptionFloatingStrike {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
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
}

/// Struct containing Fixed-Strike Lookback Option parameters.
#[derive(Debug)]
pub struct LookbackOptionFixedStrike {
    /// `S` - Initial price of the underlying.
    pub initial_price: f64,
    /// `K` - Strike price.
    pub strike_price: f64,
    /// `r` - Risk-free rate parameter.
    pub risk_free_rate: f64,
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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl PathDependentOption for LookbackOptionFloatingStrike {
    fn closed_form_prices(&self) -> (f64, f64) {
        let s = self.initial_price;
        let r = self.risk_free_rate;
        let t = self.time_to_maturity;
        let v = self.volatility;
        let q = self.dividend_yield;
        let s_min = self.s_min;
        let s_max = self.s_max;

        let b = r - q; // Cost of carry

        let call: f64;
        let put: f64;

        let a1 = ((s / s_min).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
        let a2 = a1 - v * t.sqrt();

        let b1 = ((s / s_max).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
        let b2 = b1 - v * t.sqrt();

        let norm = StandardNormal::new();

        if b != 0.0 {
            call = s * ((b - r) * t).exp() * norm.cdf(a1) - s_min * (-r * t).exp() * norm.cdf(a2)
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
            call = s * (-r * t).exp() * norm.cdf(a1) - s_min * (-r * t).exp() * norm.cdf(a2)
                + s * (-r * t).exp() * v * t.sqrt() * (norm.pdf(a1) + a1 * (norm.cdf(a1) - 1.0));

            put = -s * ((b - r) * t).exp() * norm.cdf(-b1)
                + s_max * (-r * t).exp() * norm.cdf(-b2)
                + s * (-r * t).exp() * v * t.sqrt() * (norm.pdf(b1) + b1 * norm.cdf(b1));
        }

        (call, put)
    }

    fn call_payoff(&self, path: &[f64]) -> f64 {
        // let s_min = path.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let s_min = path.iter().copied().fold(f64::NAN, f64::min);
        let s_n = path.last().unwrap();
        s_n - s_min
    }

    fn put_payoff(&self, path: &[f64]) -> f64 {
        // let s_max = path.iter().fold(f64::INFINITY, |a, &b| a.max(b));
        let s_max = path.iter().copied().fold(f64::NAN, f64::max);
        let s_n = path.last().unwrap();
        s_max - s_n
    }

    fn monte_carlo_prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64) {
        let x_0 = self.initial_price;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let t_n = self.time_to_maturity;

        let gbm = GeometricBrownianMotion::new(r, v);

        let paths = gbm.euler_maruyama(x_0, 0.0, t_n, n_steps, n_sims, parallel);

        let mut call_payoffs = Vec::with_capacity(n_sims);
        let mut put_payoffs = Vec::with_capacity(n_sims);

        for i in 0..paths.trajectories.len() {
            let path = paths.trajectories[i].clone();
            call_payoffs.push(Self::call_payoff(self, &path));
            put_payoffs.push(Self::put_payoff(self, &path));
        }

        let mean_call_payoff = mean(&call_payoffs);
        // let mean_call_payoff = call_payoffs.iter().sum::<f64>() as f64 / call_payoffs.len() as f64;
        let mean_put_payoff = put_payoffs.iter().sum::<f64>() / put_payoffs.len() as f64;

        (
            (-r * t_n).exp() * mean_call_payoff,
            (-r * t_n).exp() * mean_put_payoff,
        )
    }
}

impl PathDependentOption for LookbackOptionFixedStrike {
    fn closed_form_prices(&self) -> (f64, f64) {
        let s = self.initial_price;
        let r = self.risk_free_rate;
        let t = self.time_to_maturity;
        let v = self.volatility;
        let q = self.dividend_yield;
        let s_min = self.s_min;
        let s_max = self.s_max;

        let b = r - q; // Cost of carry

        let call: f64;
        let put: f64;

        let a1 = ((s / s_min).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
        let a2 = a1 - v * t.sqrt();

        let b1 = ((s / s_max).ln() + (b + v * v / 2.0) * t) / (v * t.sqrt());
        let b2 = b1 - v * t.sqrt();

        let norm = StandardNormal::new();

        if b != 0.0 {
            call = s * ((b - r) * t).exp() * norm.cdf(a1) - s_min * (-r * t).exp() * norm.cdf(a2)
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
            call = s * (-r * t).exp() * norm.cdf(a1) - s_min * (-r * t).exp() * norm.cdf(a2)
                + s * (-r * t).exp() * v * t.sqrt() * (norm.pdf(a1) + a1 * (norm.cdf(a1) - 1.0));

            put = -s * ((b - r) * t).exp() * norm.cdf(-b1)
                + s_max * (-r * t).exp() * norm.cdf(-b2)
                + s * (-r * t).exp() * v * t.sqrt() * (norm.pdf(b1) + b1 * norm.cdf(b1));
        }

        (call, put)
    }

    fn call_payoff(&self, path: &[f64]) -> f64 {
        let s_max = path.iter().copied().fold(f64::NAN, f64::max);
        let k = self.strike_price;
        f64::max(s_max - k, 0.0)
    }

    fn put_payoff(&self, path: &[f64]) -> f64 {
        let s_min = path.iter().copied().fold(f64::NAN, f64::min);
        let k = self.strike_price;
        f64::max(k - s_min, 0.0)
    }

    fn monte_carlo_prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64) {
        let x_0 = self.initial_price;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let t_n = self.time_to_maturity;

        let gbm = GeometricBrownianMotion::new(r, v);

        let paths = gbm.euler_maruyama(x_0, 0.0, t_n, n_steps, n_sims, parallel);

        let mut call_payoffs = Vec::with_capacity(n_sims);
        let mut put_payoffs = Vec::with_capacity(n_sims);

        for i in 0..paths.trajectories.len() {
            let path = paths.trajectories[i].clone();
            call_payoffs.push(Self::call_payoff(self, &path));
            put_payoffs.push(Self::put_payoff(self, &path));
        }

        let mean_call_payoff = call_payoffs.iter().sum::<f64>() / call_payoffs.len() as f64;
        let mean_put_payoff = put_payoffs.iter().sum::<f64>() / put_payoffs.len() as f64;

        (
            (-r * t_n).exp() * mean_call_payoff,
            (-r * t_n).exp() * mean_put_payoff,
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookback_floating() {
        let lbo_floating = LookbackOptionFloatingStrike {
            initial_price: 10.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            s_min: 10.0,
            s_max: 10.0,
            dividend_yield: 0.0,
        };

        let prices_mc = lbo_floating.monte_carlo_prices(2000, 10000, true);
        let prices_cf = lbo_floating.closed_form_prices();

        println!(
            "MONTE CARLO\nLookback call (float): {}, Lookback put (float): {}",
            prices_mc.0, prices_mc.1
        );
        println!(
            "CLOSED FORM\nLookback call (float): {}, Lookback put (float): {}",
            prices_cf.0, prices_cf.1
        );
    }
}
