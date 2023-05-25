// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::distributions::{gaussian::*, Distribution};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BARRIER OPTION STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Barrier Option struct for parameters and pricing methods.
#[derive(Debug)]
pub struct BarrierOption {
    /// * `S` - Initial underlying price.
    pub initial_price: f64,
    /// * `X` - Strike price.
    pub strike_price: f64,
    /// * `H` - Barrier.
    pub barrier: f64,
    /// * `t` - Time to expiry.
    pub time_to_expiry: f64,
    /// * `r` - Risk-free rate.
    pub risk_free_rate: f64,
    /// * `v` - Volatility.
    pub volatility: f64,
    /// * `K` - Rebate (paid if the option is not able to be exercised).
    pub rebate: f64,
    /// * `q` - Dividend yield.
    pub dividend_yield: f64,
}

/// Barrier option type enum.
pub enum BarrierType {
    /// Call (up-and-in)
    /// Payoff: `max(S_T - X, 0) * I(max(S_t) > H)`
    CUI,
    /// Call (down-and-in)
    /// Payoff: `max(S_T - X, 0) * I(min(S_t) < H)`
    CDI,
    /// Call (up-and-out)
    /// Payoff: `max(S_T - X, 0) * I(max(S_t) < H)`
    CUO,
    /// Call (down-and-out)
    /// Payoff: `max(S_T - X, 0) * I(min(S_t) > H)`
    CDO,
    /// Put (up-and-in)
    /// Payoff: `max(X - S_T, 0) * I(max(S_t) > H)`
    PUI,
    /// Put (down-and-in)
    /// Payoff: `max(X - S_T, 0) * I(min(S_t) < H)`
    PDI,
    /// Put (up-and-out)
    /// Payoff: `max(X - S_T, 0) * I(max(S_t) < H)`
    PUO,
    /// Put (down-and-out)
    /// Payoff: `max(X - S_T, 0) * I(min(S_t) > H)`
    PDO,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BARRIER OPTION IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl BarrierOption {
    /// Closed-form solution for path-dependent barrier options.
    ///
    /// Adapted from Haug's *Complete Guide to Option Pricing Formulas*.
    ///
    /// # Arguments:
    ///
    /// * `type_flag` - One of: `cui`, `cuo`, `pui`, `puo`, `cdi`, `cdo`, `pdi`, `pdo`.
    ///
    /// # Note:
    /// * `b = r - q` - The cost of carry.
    pub fn price(&self, type_flag: BarrierType) -> f64 {
        let S = self.initial_price;
        let X = self.strike_price;
        let H = self.barrier;
        let t = self.time_to_expiry;
        let r = self.risk_free_rate;
        let v = self.volatility;
        let K = self.rebate;
        let q = self.dividend_yield;

        let b: f64 = r - q;

        // Common terms:
        let mu: f64 = (b - v * v / 2.) / (v * v);
        let lambda: f64 = (mu * mu + 2. * r / (v * v)).sqrt();
        let z: f64 = (H / S).ln() / (v * t.sqrt()) + lambda * v * t.sqrt();

        let x1: f64 = (S / X).ln() / v * t.sqrt() + (1. + mu) * v * t.sqrt();
        let x2: f64 = (S / H).ln() / v * t.sqrt() + (1. + mu) * v * t.sqrt();

        let y1: f64 = (H * H / (S * X)).ln() / (v * t.sqrt()) + (1. + mu) * v * t.sqrt();
        let y2: f64 = (H / S).ln() / (v * t.sqrt()) + (1. + mu) * v * t.sqrt();

        let norm = Gaussian::default();

        // Common functions:
        let A = |phi: f64| -> f64 {
            let term1: f64 = phi * S * ((b - r) * t).exp() * norm.cdf(phi * x1);
            let term2: f64 = phi * X * (-r * t).exp() * norm.cdf(phi * x1 - phi * v * (t).sqrt());
            term1 - term2
        };

        let B = |phi: f64| -> f64 {
            let term1: f64 = phi * S * ((b - r) * t).exp() * norm.cdf(phi * x2);
            let term2: f64 = phi * X * (-r * t).exp() * norm.cdf(phi * x2 - phi * v * (t).sqrt());
            term1 - term2
        };

        let C = |phi: f64, eta: f64| -> f64 {
            let term1: f64 =
                phi * S * ((b - r) * t).exp() * (H / S).powf(2. * (mu + 1.)) * norm.cdf(eta * y1);
            let term2: f64 = phi
                * X
                * (-r * t).exp()
                * (H / S).powf(2. * mu)
                * norm.cdf(eta * y1 - eta * v * t.sqrt());
            term1 - term2
        };

        let D = |phi: f64, eta: f64| -> f64 {
            let term1: f64 =
                phi * S * ((b - r) * t).exp() * (H / S).powf(2. * (mu + 1.)) * norm.cdf(eta * y2);
            let term2: f64 = phi
                * X
                * (-r * t).exp()
                * (H / S).powf(2. * mu)
                * norm.cdf(eta * y2 - eta * v * (t).sqrt());

            term1 - term2
        };

        let E = |eta: f64| -> f64 {
            let term1: f64 = norm.cdf(eta * x2 - eta * v * (t).sqrt());
            let term2: f64 = (H / S).powf(2. * mu) * norm.cdf(eta * y2 - eta * v * t.sqrt());

            K * (-r * t).exp() * (term1 - term2)
        };

        let F = |eta: f64| -> f64 {
            let term1: f64 = (H / S).powf(mu + lambda) * norm.cdf(eta * z);
            let term2: f64 =
                (H / S).powf(mu - lambda) * norm.cdf(eta * z - 2. * eta * lambda * v * t.sqrt());

            K * (term1 + term2)
        };

        // Strike above barrier (X >= H):
        if X >= H {
            match type_flag {
                // Knock-In calls:
                BarrierType::CDI if S >= H => C(1., 1.) + E(1.),
                BarrierType::CUI if S <= H => A(1.) + E(-1.),
                // Knock-In puts:
                BarrierType::PDI if S >= H => B(-1.) - C(-1., 1.) + D(-1., 1.) + E(1.),
                BarrierType::PUI if S <= H => A(-1.) - B(-1.) + D(-1., -1.) + E(-1.),
                // Knock-Out calls:
                BarrierType::CDO if S >= H => A(1.) - C(1., 1.) + F(1.),
                BarrierType::CUO if S <= H => F(-1.),
                // Knock-Out puts:
                BarrierType::PDO if S >= H => A(-1.) - B(-1.) + C(-1., 1.) - D(-1., 1.) + F(1.),
                BarrierType::PUO if S <= H => B(-1.) - D(-1., -1.) + F(-1.),

                _ => panic!("Barrier touched - check barrier and type flag."),
            }
        }
        // Strike below barrier (X < H):
        else {
            match type_flag {
                // Knock-In calls:
                BarrierType::CDI if S >= H => A(1.) - B(1.) + D(1., 1.) + E(1.),
                BarrierType::CUI if S <= H => B(1.) - C(1., -1.) + D(1., -1.) + E(-1.),
                // Knock-In puts:
                BarrierType::PDI if S >= H => A(-1.) + E(1.),
                BarrierType::PUI if S <= H => C(-1., -1.) + E(-1.),
                // Knock-Out calls:
                BarrierType::CDO if S >= H => B(1.) - D(1., 1.) + F(1.),
                BarrierType::CUO if S <= H => A(1.) - B(1.) + C(1., -1.) - D(1., -1.) + F(-1.),
                // Knock-Out puts:
                BarrierType::PDO if S >= H => F(1.),
                BarrierType::PUO if S <= H => A(-1.) - C(-1., -1.) + F(-1.),

                _ => panic!("Barrier touched - check barrier and type flag."),
            }
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Initial underlying price ABOVE the barrier.
    //
    // If S > H, then:
    //      - "down-in" and "down-out" options have a defined price.
    //      - "up-in" and "up-out" options make no sense.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    static S_ABOVE_H: BarrierOption = BarrierOption {
        initial_price: 110.0,
        strike_price: 100.0,
        barrier: 105.0,
        time_to_expiry: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        rebate: 0.0,
        dividend_yield: 0.01,
    };

    #[test]
    fn test_S_above_H() {
        let cdi = S_ABOVE_H.price(BarrierType::CDI);
        let cdo = S_ABOVE_H.price(BarrierType::CDO);
        let pdi = S_ABOVE_H.price(BarrierType::PDI);
        let pdo = S_ABOVE_H.price(BarrierType::PDO);

        assert_approx_equal!(cdi, 9.504815, 0.000001);
        assert_approx_equal!(cdo, 7.295022, 0.000001);
        assert_approx_equal!(pdi, 3.017298, 0.000001);
        assert_approx_equal!(pdo, 0.000000, 0.000001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cui_panic() {
        S_ABOVE_H.price(BarrierType::CUI);
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cuo_panic() {
        S_ABOVE_H.price(BarrierType::CUO);
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pui_panic() {
        S_ABOVE_H.price(BarrierType::PUI);
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn puo_panic() {
        S_ABOVE_H.price(BarrierType::PUO);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Initial underlying price BELOW the barrier.
    //
    // If S < H, then:
    //      - "down-in" and "down-out" options make no sense.
    //      - "up-in" and "up-out" options have a defined price.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    static S_BELOW_H: BarrierOption = BarrierOption {
        initial_price: 90.0,
        strike_price: 100.0,
        barrier: 105.0,
        time_to_expiry: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        rebate: 0.0,
        dividend_yield: 0.01,
    };

    #[test]
    fn test_S_below_H() {
        let cui = S_BELOW_H.price(BarrierType::CUI);
        let cuo = S_BELOW_H.price(BarrierType::CUO);
        let pui = S_BELOW_H.price(BarrierType::PUI);
        let puo = S_BELOW_H.price(BarrierType::PUO);

        assert_approx_equal!(cui, 4.692603, 0.000001);
        assert_approx_equal!(cuo, 0.022449, 0.000001);
        assert_approx_equal!(pui, 1.359553, 0.000001);
        assert_approx_equal!(puo, 9.373956, 0.000001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cdi_panic() {
        S_BELOW_H.price(BarrierType::CDI);
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cdo_panic() {
        S_BELOW_H.price(BarrierType::CDO);
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pdi_panic() {
        S_BELOW_H.price(BarrierType::PDI);
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pdo_panic() {
        S_BELOW_H.price(BarrierType::PDO);
    }
}
