// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BINOMIAL OPTION PRICING PARAMETER STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{ExerciseFlag, TypeFlag};

/// Struct containing the parameters to price an option via binomial tree method.
#[derive(Debug, Clone, Copy)]
pub struct BinomialOption {
    initial_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    dividend_yield: f64,
    volatility: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BINOMIAL OPTION PRICING IMPLEMENTATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl BinomialOption {
    /// Cox-Ross-Rubinstein binomial option pricing model.
    ///
    /// Adapted from Haug's *Complete Guide to Option Pricing Formulas*.
    ///
    /// # Arguments:
    ///
    /// * `output_flag` - `&str`: one of `p` (price), `d` (delta), `g` (gamma), or `t` (theta).
    /// * `ame_eur_flag` - `AmericanEuropeanFlag`: either `American` or `European`.
    /// * `call_put_flag` - `TypeFlag`: either `Call` or `Put`.
    /// * `n` - Height of the binomial tree.
    ///
    /// # Note:
    ///
    /// * `b = r - q` - The cost of carry.
    pub fn price_CoxRossRubinstein(
        &self,
        output_flag: &str,
        ame_eur_flag: ExerciseFlag,
        call_put_flag: TypeFlag,
        n: usize,
    ) -> f64 {
        let S = self.initial_price;
        let K = self.strike_price;
        let T = self.time_to_expiry;
        let r = self.risk_free_rate;
        let q = self.dividend_yield;
        let v = self.volatility;

        let mut option_value: Vec<f64> = Vec::with_capacity(n + 1);

        let mut return_value: Vec<f64> = vec![0.0; 5];

        let b: f64 = r - q;
        let (u, d, p, dt, Df): (f64, f64, f64, f64, f64);

        let z = match call_put_flag {
            TypeFlag::Call => 1,
            TypeFlag::Put => -1,
        };

        dt = T / n as f64;
        u = (v * dt.sqrt()).exp();
        d = 1.0 / u;
        p = ((b * dt).exp() - d) / (u - d);
        Df = (-r * dt).exp();

        for i in 0..option_value.capacity() {
            option_value
                .push((z as f64 * (S * u.powi(i as i32) * d.powi((n - i) as i32) - K)).max(0.0));
        }

        for j in (0..n).rev() {
            for i in 0..=j {
                match ame_eur_flag {
                    ExerciseFlag::American => {
                        option_value[i] = (z as f64
                            * (S * u.powi(i as i32) * d.powi(j as i32 - i as i32) - K))
                            .max(Df * (p * (option_value[i + 1]) + (1.0 - p) * option_value[i]));
                    }
                    ExerciseFlag::European => {
                        option_value[i] =
                            Df * (p * (option_value[i + 1]) + (1.0 - p) * option_value[i]);
                    }
                    ExerciseFlag::Bermudan => {
                        panic!("Bermudan option pricing not implemented yet.");
                    }
                }
            }
            if j == 2 {
                return_value[2] = (option_value[2] - option_value[1]) / (S * u * u - S)
                    - (option_value[1] - option_value[0])
                        / (S - S * d * d)
                        / (0.5 * (S * u * u - S * d * d));
                return_value[3] = option_value[1];
            }
            if j == 1 {
                return_value[1] = (option_value[1] - option_value[0]) / (S * u - S * d);
            }
        }

        return_value[3] = (option_value[3] - option_value[0]) / (2.0 * dt) / 365.0;
        return_value[0] = option_value[0];

        match output_flag {
            // Return the option value.
            "p" => return_value[0],
            // Return the Delta.
            "d" => return_value[1],
            // Return the Gamma.
            "g" => return_value[2],
            // Return the Theta.
            "t" => return_value[3],
            // Capture edge cases.
            _ => panic!("Check OutputFlag. Should be one of: 'p', 'd', 'g', 't'."),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_binomial {
    use crate::{
        assert_approx_equal,
        instruments::{BinomialOption, ExerciseFlag, TypeFlag},
    };

    #[test]
    fn TEST_CRRBinomial() {
        let BinOpt = BinomialOption {
            initial_price: 100.0,
            strike_price: 95.0,
            time_to_expiry: 0.5,
            risk_free_rate: 0.08,
            dividend_yield: 0.0,
            volatility: 0.3,
        };

        let c = BinOpt.price_CoxRossRubinstein("p", ExerciseFlag::American, TypeFlag::Call, 100);
        let p = BinOpt.price_CoxRossRubinstein("p", ExerciseFlag::American, TypeFlag::Put, 100);

        let c_intrinsic = (100_f64 - 95_f64).max(0.0);
        let p_intrinsic = (95_f64 - 100_f64).max(0.0);
        let parity = c - p - 100.0 + 95.0 * (-0.08_f64 * 0.5).exp();

        println!("CRR Call price = {}", c);
        println!("CRR Put price = {}", p);
        println!("CRR Parity = {}", parity);

        assert!(c >= c_intrinsic);
        assert!(p >= p_intrinsic);

        // Very weak parity due to discrete time steps.
        assert_approx_equal!(parity, 0.0, 0.5);
    }
}
