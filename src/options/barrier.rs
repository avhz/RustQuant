#![allow(non_snake_case)]
#![deny(missing_docs)]

use crate::prelude::normal_distribution::*;

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
    pub fn price(&self, type_flag: &str) -> f64 {
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

        let norm = StandardgNormal::new();

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
                "cdi" if S >= H => C(1., 1.) + E(1.),
                "cui" if S <= H => A(1.) + E(-1.),
                // Knock-In puts:
                "pdi" if S >= H => B(-1.) - C(-1., 1.) + D(-1., 1.) + E(1.),
                "pui" if S <= H => A(-1.) - B(-1.) + D(-1., -1.) + E(-1.),
                // Knock-Out calls:
                "cdo" if S >= H => A(1.) - C(1., 1.) + F(1.),
                "cuo" if S <= H => F(-1.),
                // Knock-Out puts:
                "pdo" if S >= H => A(-1.) - B(-1.) + C(-1., 1.) - D(-1., 1.) + F(1.),
                "puo" if S <= H => B(-1.) - D(-1., -1.) + F(-1.),

                _ => panic!("Barrier touched - check barrier and type flag."),
            }
        }
        // Strike below barrier (X < H):
        else {
            match type_flag {
                // Knock-In calls:
                "cdi" if S >= H => A(1.) - B(1.) + D(1., 1.) + E(1.),
                "cui" if S <= H => B(1.) - C(1., -1.) + D(1., -1.) + E(-1.),
                // Knock-In puts:
                "pdi" if S >= H => A(-1.) + E(1.),
                "pui" if S <= H => C(-1., -1.) + E(-1.),
                // Knock-Out calls:
                "cdo" if S >= H => B(1.) - D(1., 1.) + F(1.),
                "cuo" if S <= H => A(1.) - B(1.) + C(1., -1.) - D(1., -1.) + F(-1.),
                // Knock-Out puts:
                "pdo" if S >= H => F(1.),
                "puo" if S <= H => A(-1.) - C(-1., -1.) + F(-1.),

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
    use crate::assert_approx_equal;

    use super::*;
    // use crate::*;

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
        let cdi = S_ABOVE_H.price("cdi");
        let cdo = S_ABOVE_H.price("cdo");
        let pdi = S_ABOVE_H.price("pdi");
        let pdo = S_ABOVE_H.price("pdo");

        assert_approx_equal!(cdi, 9.504815, 0.000001);
        assert_approx_equal!(cdo, 7.295022, 0.000001);
        assert_approx_equal!(pdi, 3.017298, 0.000001);
        assert_approx_equal!(pdo, 0.000000, 0.000001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cui_panic() {
        S_ABOVE_H.price("cui");
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cuo_panic() {
        S_ABOVE_H.price("cuo");
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pui_panic() {
        S_ABOVE_H.price("pui");
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn puo_panic() {
        S_ABOVE_H.price("puo");
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
        let cui = S_BELOW_H.price("cui");
        let cuo = S_BELOW_H.price("cuo");
        let pui = S_BELOW_H.price("pui");
        let puo = S_BELOW_H.price("puo");

        assert_approx_equal!(cui, 4.692603, 0.000001);
        assert_approx_equal!(cuo, 0.022449, 0.000001);
        assert_approx_equal!(pui, 1.359553, 0.000001);
        assert_approx_equal!(puo, 9.373956, 0.000001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cdi_panic() {
        S_BELOW_H.price("cdi");
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cdo_panic() {
        S_BELOW_H.price("cdo");
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pdi_panic() {
        S_BELOW_H.price("pdi");
    }
    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pdo_panic() {
        S_BELOW_H.price("pdo");
    }
}
