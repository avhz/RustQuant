use crate::{dnorm, pnorm, BlackScholesCall, BlackScholesPut};

/// Struct to contain common Black-Scholes Greeks/sensitivities.
///
/// Implemented using the closed-form derivatives from the Black-Scholes model.
pub struct Greeks {
    /// Price sensitivity.
    Delta: (f64, f64),
    /// Price elasticity (measure of leverage, gearing).
    Lambda: (f64, f64),
    /// Price convexity.
    Gamma: (f64, f64),
    /// Volatility sensitivity.
    Vega: (f64, f64),
    /// Time sensitivity.
    Theta: (f64, f64),
    /// Interest rate sensitivity.
    Rho: (f64, f64),
    /// Dividend sensitivity.
    Phi: (f64, f64),
    /// In-the-money probabilities.
    Zeta: (f64, f64),
}

/// Function that computes the Black-Scholes Greeks/sensitivities.
pub fn Greeks(S: f64, K: f64, v: f64, r: f64, T: f64, q: f64) -> Greeks {
    let sqrtT: f64 = T.sqrt();
    let df: f64 = (-r * T).exp();
    let b: f64 = r - q;
    let ebrT: f64 = ((b - r) * T).exp();
    let Fp: f64 = S * (b * T).exp();
    let std: f64 = v * sqrtT;
    let d: f64 = (Fp / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;

    let nd1: f64 = dnorm(d1);
    let nd2: f64 = dnorm(d2);
    let Nd1: f64 = pnorm(d1);
    let Nd2: f64 = pnorm(d2);

    let nd1_: f64 = dnorm(-d1);
    let nd2_: f64 = dnorm(-d2);
    let Nd1_: f64 = pnorm(-d1);
    let Nd2_: f64 = pnorm(-d2);

    Greeks {
        Delta: (ebrT * Nd1, ebrT * (Nd1 - 1.0)),
        Lambda: (
            ebrT * Nd1 * S / BlackScholesCall(S, K, v, r, T, q),
            ebrT * (Nd1 - 1.0) * S / BlackScholesPut(S, K, v, r, T, q),
        ),
        Gamma: (
            (nd1 * ebrT) / (S * v * sqrtT),
            (nd1 * ebrT) / (S * v * sqrtT),
        ),
        Vega: (S * ebrT * nd1 * sqrtT, S * ebrT * nd1 * sqrtT),
        Theta: (
            -(S * ebrT * nd1 * v) / (2. * T.sqrt()) - (b - r) * S * ebrT * Nd1 - r * K * df * Nd2,
            -(S * ebrT * nd1 * v) / (2. * T.sqrt()) + (b - r) * S * ebrT * Nd1_ + r * K * df * Nd2_,
        ),
        Rho: (T * K * df * Nd2, -T * K * df * Nd2_),
        Phi: (-T * S * ebrT * Nd1, T * S * ebrT * Nd1_),
        Zeta: (Nd2, Nd2_),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::assert_approx_equal;

    #[test]
    fn test_greeks() {
        for strike in 1..=1000 {
            let g = Greeks(100.0, strike as f64, 0.2, 0.05, 1.0, 0.03);

            // Delta: long call (short put) in (0,1)
            assert!(g.Delta.0 >= 0.0 && g.Delta.0 <= 1.0);

            // Delta: short call (long put) in (-1,0)
            assert!(g.Delta.1 >= -1.0 && g.Delta.1 <= 0.0);

            // Vega is the same for calls/puts, and always positive.
            assert_approx_equal(g.Vega.0, g.Vega.1, 1e-10);
            assert!(g.Vega.0 >= 0.0 && g.Vega.1 >= 0.0);

            // Greeks(S, K, v, r, T, q)
            // println!("strike = {}, call delta = {}, put delta = {}", strike, g.Delta.0, g.Delta.1);
            // println!("strike = {}, call theta = {}, put theta = {}",strike, g.Theta.0, g.Theta.1);
            // assert!(g.Theta.0 <= 0.0 && g.Theta.1 <= 0.0);
        }
    }
}
