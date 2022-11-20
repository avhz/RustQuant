use crate::prelude::{
    geometric_brownian_motion::GeometricBrownianMotion, mean::mean, option::PathDependentOption,
    process::StochasticProcess,
};

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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LOOKBACK OPTION IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl PathDependentOption for LookbackOptionFloatingStrike {
    fn call_payoff(&self, path: &Vec<f64>) -> f64 {
        // let s_min = path.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let s_min = path.iter().copied().fold(f64::NAN, f64::min);
        let s_n = path.last().unwrap();
        s_n - s_min
    }

    fn put_payoff(&self, path: &Vec<f64>) -> f64 {
        // let s_max = path.iter().fold(f64::INFINITY, |a, &b| a.max(b));
        let s_max = path.iter().copied().fold(f64::NAN, f64::max);
        let s_n = path.last().unwrap();
        s_max - s_n
    }

    fn prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64) {
        let x_0 = self.initial_price;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let t_n = self.time_to_maturity;

        let gbm = GeometricBrownianMotion::new(r, v);

        let paths = (&gbm).euler_maruyama(x_0, 0.0, t_n, n_steps, n_sims, parallel);

        let mut call_payoffs = Vec::with_capacity(n_sims);
        let mut put_payoffs = Vec::with_capacity(n_sims);

        for i in 0..paths.trajectories.len() {
            let path = paths.trajectories[i].clone();
            call_payoffs.push(Self::call_payoff(&self, &path));
            put_payoffs.push(Self::put_payoff(&self, &path));
        }

        let mean_call_payoff = mean(&call_payoffs);
        // let mean_call_payoff = call_payoffs.iter().sum::<f64>() as f64 / call_payoffs.len() as f64;
        let mean_put_payoff = put_payoffs.iter().sum::<f64>() as f64 / put_payoffs.len() as f64;

        (
            (-r * t_n).exp() * mean_call_payoff,
            (-r * t_n).exp() * mean_put_payoff,
        )
    }
}

impl PathDependentOption for LookbackOptionFixedStrike {
    fn call_payoff(&self, path: &Vec<f64>) -> f64 {
        let s_max = path.iter().copied().fold(f64::NAN, f64::max);
        let k = self.strike_price;
        f64::max(s_max - k, 0.0)
    }

    fn put_payoff(&self, path: &Vec<f64>) -> f64 {
        let s_min = path.iter().copied().fold(f64::NAN, f64::min);
        let k = self.strike_price;
        f64::max(k - s_min, 0.0)
    }

    fn prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64) {
        let x_0 = self.initial_price;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let t_n = self.time_to_maturity;

        let gbm = GeometricBrownianMotion::new(r, v);

        let paths = (&gbm).euler_maruyama(x_0, 0.0, t_n, n_steps, n_sims, parallel);

        let mut call_payoffs = Vec::with_capacity(n_sims);
        let mut put_payoffs = Vec::with_capacity(n_sims);

        for i in 0..paths.trajectories.len() {
            let path = paths.trajectories[i].clone();
            call_payoffs.push(Self::call_payoff(&self, &path));
            put_payoffs.push(Self::put_payoff(&self, &path));
        }

        let mean_call_payoff = call_payoffs.iter().sum::<f64>() as f64 / call_payoffs.len() as f64;
        let mean_put_payoff = put_payoffs.iter().sum::<f64>() as f64 / put_payoffs.len() as f64;

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
        };

        let prices = lbo_floating.prices(1000, 1000, false);

        println!(
            "Lookback call (floating): {}, Lookback put (floating): {}",
            prices.0, prices.1
        );

        assert!(1 == 0);
    }

    #[test]
    fn test_lookback_fixed() {
        let lbo_fixed = LookbackOptionFixedStrike {
            initial_price: 10.0,
            strike_price: 15.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
        };

        let prices = lbo_fixed.prices(1000, 1000, false);

        println!(
            "Lookback call (fixed): {}, Lookback put (fixed): {}",
            prices.0, prices.1
        );

        assert!(1 == 0);
    }
}
