// pub struct OptionParameters {
//     /// `S` - Initial price of the underlying.
//     pub initial_price: f64,
//     /// `K` - Strike price.
//     pub strike_price: f64,
//     /// `r` - Risk-free rate parameter.
//     pub risk_free_rate: f64,
//     /// `v` - Volatility parameter.
//     pub volatility: f64,
//     /// `q` - Dividend rate.
//     pub dividend_rate: f64,
//     /// `T` - Time to expiry/maturity.
//     pub time_to_maturity: f64,
// }

// pub trait PathIndependentOption {
//     fn price(&self) -> f64;
// }

/// Path-dependent option trait.
pub trait PathDependentOption {
    /// Base method for path-dependent call option payoff.
    fn call_payoff(&self, path: &[f64]) -> f64;

    /// Base method for path-dependent put option payoff.
    fn put_payoff(&self, path: &[f64]) -> f64;

    /// Base method for path-dependent option prices using closed-form solution (call and put).
    fn closed_form_prices(&self) -> (f64, f64);

    /// Base method for path-dependent option prices using Monte Carlo (call and put).
    fn monte_carlo_prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64);
}
