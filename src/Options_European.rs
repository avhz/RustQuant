#![allow(non_snake_case)]

use super::*;

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Black-Scholes European Call Option Price
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
pub fn BlackScholesCall(S: f64, K: f64, v: f64, r: f64, T: f64, q: f64) -> f64 {
    let df: f64 = (-r * T).exp();
    let b: f64 = r - q;
    let Fp: f64 = S * (b * T).exp();
    let std: f64 = v * T.sqrt();
    let d: f64 = (Fp / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;
    let nd1: f64 = pnorm(d1);
    let nd2: f64 = pnorm(d2);
    let c: f64 = df * (Fp * nd1 - K * nd2);

    return c;
}

/// Black-Scholes European Put Option Price
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
pub fn BlackScholesPut(S: f64, K: f64, v: f64, r: f64, T: f64, q: f64) -> f64 {
    let df: f64 = (-r * T).exp();
    let b: f64 = r - q;
    let Fp: f64 = S * (b * T).exp();
    let std: f64 = v * T.sqrt();
    let d: f64 = (Fp / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;
    let nd1: f64 = pnorm(-d1);
    let nd2: f64 = pnorm(-d2);
    let p: f64 = df * (-Fp * nd1 + K * nd2);

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
        assert_approx_equal(BSC, 2.586, 0.001);
    }

    #[test]
    fn black_scholes_put() {
        let BSP = BlackScholesPut(100.0, 110.0, 0.2, 0.05, 0.5, 0.02);
        assert_approx_equal(BSP, 10.865, 0.001);
    }
}
