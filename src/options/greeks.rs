// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::{Distribution, Gaussian};
use crate::options::european::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GREEKS STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain common Black-Scholes Greeks/sensitivities.
/// Implemented using the closed-form derivatives from the Black-Scholes model.
#[derive(Debug)]

pub struct Greeks {
    /// Price sensitivity.
    pub Delta: (f64, f64),
    /// Price elasticity (measure of leverage, gearing).
    pub Lambda: (f64, f64),
    /// Price convexity.
    pub Gamma: (f64, f64),
    /// Volatility sensitivity.
    pub Vega: (f64, f64),
    /// Time sensitivity.
    pub Theta: (f64, f64),
    /// Driftless theta
    pub Driftless_theta: (f64, f64),
    /// Interest rate sensitivity.
    pub Rho: (f64, f64),
    /// Dividend sensitivity.
    pub Phi: (f64, f64),
    /// In-the-money probabilities: N(d2), N(-d2).
    pub Zeta: (f64, f64),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GREEKS IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Greeks {
    /// Function that computes the Black-Scholes Greeks/sensitivities.
    ///
    /// # Arguments:
    /// * `option` - A `EuropeanOption` struct containing the parameters.
    pub fn compute(option: EuropeanOption) -> Self {
        let S = option.initial_price;
        let K = option.strike_price;
        let T = option.time_to_maturity;
        let r = option.risk_free_rate;
        let v = option.volatility;
        let q = option.dividend_rate;

        let sqrtT: f64 = T.sqrt();
        let df: f64 = (-r * T).exp();
        let b: f64 = r - q;
        let ebrT: f64 = ((b - r) * T).exp();
        let Fp: f64 = S * (b * T).exp();
        let std: f64 = v * sqrtT;
        let d: f64 = (Fp / K).ln() / std;
        let d1: f64 = d + 0.5 * std;
        let d2: f64 = d1 - std;

        let norm = Gaussian::default();

        let nd1: f64 = norm.pdf(d1);
        // let nd2: f64 = norm.pdf(d2);
        let Nd1: f64 = norm.cdf(d1);
        let Nd2: f64 = norm.cdf(d2);

        // let nd1_: f64 = norm.pdf(-d1);
        // let nd2_: f64 = norm.pdf(-d2);
        let Nd1_: f64 = norm.cdf(-d1);
        let Nd2_: f64 = norm.cdf(-d2);

        let VanillaOption = EuropeanOption {
            initial_price: S,
            strike_price: K,
            risk_free_rate: r,
            volatility: v,
            dividend_rate: q,
            time_to_maturity: T,
        };
        let BS = VanillaOption.price();

        Greeks {
            Delta: (ebrT * Nd1, ebrT * (Nd1 - 1.0)),
            Lambda: (ebrT * Nd1 * S / BS.0, ebrT * (Nd1 - 1.0) * S / BS.1),
            Gamma: (
                (nd1 * ebrT) / (S * v * sqrtT),
                (nd1 * ebrT) / (S * v * sqrtT),
            ),
            Vega: (S * ebrT * nd1 * sqrtT, S * ebrT * nd1 * sqrtT),
            Theta: (
                -(S * ebrT * nd1 * v) / (2. * T.sqrt())
                    - (b - r) * S * ebrT * Nd1
                    - r * K * df * Nd2,
                -(S * ebrT * nd1 * v) / (2. * T.sqrt())
                    + (b - r) * S * ebrT * Nd1_
                    + r * K * df * Nd2_,
            ),
            Driftless_theta: (
                -(S * nd1 * v) / (2.0 * T.sqrt()),
                -(S * nd1 * v) / (2.0 * T.sqrt()),
            ),
            Rho: (T * K * df * Nd2, -T * K * df * Nd2_),
            Phi: (-T * S * ebrT * Nd1, T * S * ebrT * Nd1_),
            Zeta: (Nd2, Nd2_),
        }
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
    fn TEST_greeks() {
        for strike in 1..=100 {
            let option = EuropeanOption {
                initial_price: 100.0,
                strike_price: strike as f64,
                risk_free_rate: 0.05,
                volatility: 0.2,
                dividend_rate: 0.03,
                time_to_maturity: 1.0,
            };

            let g = Greeks::compute(option);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // DELTA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Delta: long call (short put) in (0,1)
            assert!(g.Delta.0 >= 0.0 && g.Delta.0 <= 1.0);

            // Delta: short call (long put) in (-1,0)
            assert!(g.Delta.1 >= -1.0 && g.Delta.1 <= 0.0);

            // Delta: D_call - D_put = 1
            assert_approx_equal!(g.Delta.0 - g.Delta.1, 1.0, 0.1);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // VEGA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vega is the same for calls/puts, and always positive.
            assert_approx_equal!(g.Vega.0, g.Vega.1, 1e-10);
            assert!(g.Vega.0 >= 0.0 && g.Vega.1 >= 0.0);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // GAMMA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Gamma always within (0,1)
            assert!(g.Gamma.0 >= 0.0 && g.Gamma.0 <= 1.0);
            assert!(g.Gamma.1 >= 0.0 && g.Gamma.1 <= 1.0);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // RHO
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Rho > 0 for long calls
            assert!(g.Rho.0 > 0.0);

            // Rho < 0 for long puts
            assert!(g.Rho.1 < 0.0);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // LAMBDA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            assert!(g.Lambda.0 > 1.0);
            assert!(g.Lambda.1 < 1.0);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // DRIFTLESS THETA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            assert!(g.Driftless_theta.0 <= 0.0);
            assert!(g.Driftless_theta.1 <= 0.0);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // THETA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // PHI/EPSILON
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            assert!(g.Phi.0 < 0.0);
            assert!(g.Phi.1 > 0.0);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // ZETA
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            assert!(g.Zeta.0 > 0.0);
            assert!(g.Zeta.1 > 0.0);
        }
    }
}
