/// Option type enum.
pub enum TypeFlag {
    /// Call option (right to BUY the underlying asset).
    CALL,
    /// Put option (right to SELL the underlying asset).
    PUT,
    /// Used to return both call and put prices.
    BOTH,
}

/// Generic option parameters struct.
/// Contains the common parameters (as in Black-Scholes).
/// Other option types may have additional parameters,
/// such as lookback options (S_min, S_max).
pub struct OptionParameters {
    /// `S` - Initial price of the underlying.
    pub S: f64,
    /// `K` - Strike price.
    pub K: f64,
    /// `T` - Time to expiry/maturity.
    pub T: f64,
    /// `r` - Risk-free rate parameter.
    pub r: f64,
    /// `v` - Volatility parameter.
    pub v: f64,
    /// `q` - Dividend rate.
    pub q: f64,
}

impl OptionParameters {
    /// New option parameters struct initialiser.
    pub fn new(
        initial_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        dividend_rate: f64,
        time_to_maturity: f64,
    ) -> Self {
        Self {
            S: initial_price,
            K: strike_price,
            T: time_to_maturity,
            r: risk_free_rate,
            v: volatility,
            q: dividend_rate,
        }
    }
}

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
