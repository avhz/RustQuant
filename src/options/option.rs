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
    // fn call_payoff(&self) -> f64;
    // fn put_payoff(&self) -> f64;

    /// Path-dependent option prices (call and put).
    fn prices(&self, n_steps: usize, n_sims: usize, parallel: bool) -> (f64, f64);
}
