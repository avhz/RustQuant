#![allow(non_snake_case)]
#![deny(missing_docs)]

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Cox-Ross-Rubinstein binomial option pricing model.
///
/// Adapted from Haug's *Complete Guide to Option Pricing Formulas*.
///
/// # Arguments:
///
/// * `S` - Initial underlying price.
/// * `X` - Strike price.
/// * `T` - Time to expiry.
/// * `r` - Risk-free rate.
/// * `v` - Volatility.
/// * `q` - Dividend yield.
/// * `n` - Height of the binomial tree.
///
/// # Note:
///
/// * `b = r - q` - The cost of carry.
pub fn CRRBinomial(
    OutputFlag: &str,
    AmeEurFlag: &str,
    CallPutFlag: &str,
    S: f64,
    X: f64,
    T: f64,
    r: f64,
    q: f64,
    v: f64,
    n: usize,
) -> f64 {
    let mut OptionValue: Vec<f64> = vec![0.0; n + 1];
    let mut ReturnValue: Vec<f64> = vec![0.0; 5];

    let b: f64 = r - q;
    let (u, d, p, dt, Df): (f64, f64, f64, f64, f64);
    let z: isize;

    if CallPutFlag == "c" {
        z = 1;
    } else if CallPutFlag == "p" {
        z = -1;
    } else {
        panic!("Check call/put flag. Should be either 'c' or 'p'.");
    }

    dt = T / n as f64;
    u = (v * dt.sqrt()).exp();
    d = 1.0 / u;
    p = ((b * dt).exp() - d) / (u - d);
    Df = (-r * dt).exp();

    for i in 0..=n {
        OptionValue[i] = (z as f64 * (S * u.powi(i as i32) * d.powi((n - i) as i32) - X)).max(0.0);
    }

    for j in (0..n).rev() {
        for i in 0..=j {
            if AmeEurFlag == "e" {
                OptionValue[i] = Df * (p * (OptionValue[i + 1]) + (1.0 - p) * OptionValue[i]);
            } else if AmeEurFlag == "a" {
                OptionValue[i] = (z as f64
                    * (S * u.powi(i as i32) * d.powi(j as i32 - i as i32) - X))
                    .max(Df * (p * (OptionValue[i + 1]) + (1.0 - p) * OptionValue[i]));
            }
        }
        if j == 2 {
            ReturnValue[2] = (OptionValue[2] - OptionValue[1]) / (S * u * u - S)
                - (OptionValue[1] - OptionValue[0])
                    / (S - S * d * d)
                    / (0.5 * (S * u * u - S * d * d));
            ReturnValue[3] = OptionValue[1];
        }
        if j == 1 {
            ReturnValue[1] = (OptionValue[1] - OptionValue[0]) / (S * u - S * d);
        }
    }

    ReturnValue[3] = (OptionValue[3] - OptionValue[0]) / (2.0 * dt) / 365.0;
    ReturnValue[0] = OptionValue[0];

    if OutputFlag == "p" {
        // Return the option value.
        ReturnValue[0]
    } else if OutputFlag == "d" {
        // Return the Delta ().
        ReturnValue[1]
    } else if OutputFlag == "g" {
        // Return the Gamma ().
        ReturnValue[2]
    } else if OutputFlag == "t" {
        // Return the Theta ().
        ReturnValue[3]
    } else {
        panic!("Check OutputFlag. Should be one of: 'p', 'd', 'g', 't'.")
    }
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn TEST_CRRBinomial() {
        let c = CRRBinomial("p", "a", "c", 100., 95., 0.5, 0.08, 0.0, 0.3, 100);
        let p = CRRBinomial("p", "a", "p", 100., 95., 0.5, 0.08, 0.0, 0.3, 100);

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
