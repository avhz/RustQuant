#![allow(non_snake_case)]

// mod normalDistribution;
use super::*;

// ############################################################################
// FUNCTIONS
// ############################################################################

// Black-Scholes European Call Option Price
fn BlackScholesCall(
    underlying_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_rate: f64,
    time_to_expiry: f64,
    dividend_yield: f64,
) -> f64 {
    let S0 = underlying_price;
    let K = strike_price;
    let r = risk_free_rate;
    let v = volatility;
    let t = time_to_expiry;
    let q = dividend_yield;

    let df: f64 = (-r * t).exp();
    let Ff: f64 = S0 * ((r - q) * t).exp();
    let std: f64 = v * (t).sqrt();
    let d: f64 = (Ff / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;
    let nd1: f64 = normalCDF(d1);
    let nd2: f64 = normalCDF(d2);
    let c: f64 = df * (Ff * nd1 - K * nd2);

    return c;
}

// Black-Scholes European Put Option Price
fn BlackScholesPut(
    underlying_price: f64,
    strike_price: f64,
    volatility: f64,
    risk_free_rate: f64,
    time_to_expiry: f64,
    dividend_yield: f64,
) -> f64 {
    let S0 = underlying_price;
    let K = strike_price;
    let r = risk_free_rate;
    let v = volatility;
    let t = time_to_expiry;
    let q = dividend_yield;

    let df: f64 = (-r * t).exp();
    let Ff: f64 = S0 * ((r - q) * t).exp();
    let std: f64 = v * (t).sqrt();
    let d: f64 = (Ff / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;
    let nd1: f64 = normalCDF(-d1);
    let nd2: f64 = normalCDF(-d2);
    let p: f64 = df * (-Ff * nd1 + K * nd2);

    return p;
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_scholes_call() {
        let BSC = BlackScholesCall(100.0, 110.0, 0.2, 0.05, 0.5, 0.02);
        assert!(BSC - 2.586 < 0.001);
    }

    #[test]
    fn black_scholes_put() {
        let BSP = BlackScholesPut(100.0, 110.0, 0.2, 0.05, 0.5, 0.02);
        assert!(BSP - 10.865 < 0.001);
    }
}
