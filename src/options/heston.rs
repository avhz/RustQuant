// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::math::*;
use num_complex::Complex;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Heston model for option pricing.
#[allow(clippy::too_many_arguments)]
pub fn heston(
    S0: f64,    // Initial asset value.
    V0: f64,    // Initial variance value.
    K: f64,     // Strike price.
    tau: f64,   // Time to expiry.
    r: f64,     // Risk-free rate.
    q: f64,     // Dividend yield.
    rho: f64,   // Correlation between the two Brownian motions.
    sigma: f64, // Volatility-of-volatility.
    kappa: f64, // Mean reversion rate in the variance process' drift term.
    theta: f64, // Long run mean of the variance process.
) -> (f64, f64) {
    // Market price of volatility risk (set to 0 for simplicity).
    // Should probably include, though, since for equity options it has been shown
    // to be non-zero (Lamoureux & Lastrapes, 1993).
    let lambda = 0.0;

    // i = sqrt(-1). Used frequently, so assign here.
    let i: Complex<f64> = Complex::i();

    let u = |j: u8| -> f64 {
        match j {
            1 => 0.5,
            2 => -0.5,
            _ => panic!("`j` should be: 1 or 2."),
        }
    };

    let b = |j: u8| -> f64 {
        match j {
            1 => kappa + lambda - rho * sigma,
            2 => kappa + lambda,
            _ => panic!("`j` should be: 1 or 2."),
        }
    };

    let d = |j: u8, phi: f64| -> Complex<f64> {
        ((rho * sigma * i * phi - b(j)).powi(2)
            - sigma.powi(2) * (2.0 * u(j) * i * phi - phi.powi(2)))
        .sqrt()
    };

    let g = |j: u8, phi: f64| -> Complex<f64> {
        assert!(j == 1 || j == 2);

        (b(j) - rho * sigma * i * phi + d(j, phi)) / (b(j) - rho * sigma * i * phi - d(j, phi))
    };

    let C = |j: u8, phi: f64| -> Complex<f64> {
        assert!(j == 1 || j == 2);

        (r - q) * i * phi * tau
            + (kappa * theta / sigma.powi(2))
                * ((b(j) - rho * sigma * i * phi + d(j, phi)) * tau
                    - 2.0 * ((1.0 - g(j, phi) * (d(j, phi) * tau).exp()) / (1.0 - g(j, phi))).ln())
    };

    let D = |j: u8, phi: f64| -> Complex<f64> {
        assert!(j == 1 || j == 2);

        ((b(j) - rho * sigma * i * phi + d(j, phi)) * (1.0 - (d(j, phi) * tau).exp()))
            / (sigma.powi(2) * (1.0 - g(j, phi) * (d(j, phi) * tau).exp()))
    };

    // The Heston characteristic functions.
    let f = |j: u8, phi: f64| -> Complex<f64> {
        assert!(j == 1 || j == 2);

        (C(j, phi) + D(j, phi) * V0 + i * phi * S0.ln()).exp()
    };

    // These functions return the integrand for P1 and P2.
    let Re1 = |phi: f64| -> f64 {
        let j = 1;

        (f(j, phi) * (-i * phi * K.ln()).exp() / (i * phi)).re
    };
    let Re2 = |phi: f64| -> f64 {
        let j = 2;

        (f(j, phi) * (-i * phi * K.ln()).exp() / (i * phi)).re
    };

    // Integration bounds given in Fabrice D. Rouah's book (see tests).
    // The integral decays rapidly so 50 is probably enough.
    let P1 = 0.5 + std::f64::consts::FRAC_1_PI * integrate(Re1, 0.00001, 50.0);
    let P2 = 0.5 + std::f64::consts::FRAC_1_PI * integrate(Re2, 0.00001, 50.0);

    // Price call, then use put-call-parity for the put.
    let call = S0 * (-q * tau).exp() * P1 - K * (-r * tau).exp() * P2;
    let put = call + K * (-r * tau).exp() - S0 * (-q * tau).exp();

    (call, put)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/*
TEST VALUES
From: "The Heston Model and Its Extensions in MATLAB and C#"" by Fabrice D. Rouah.

""
For example, the price a 6-month European put with strike K = 100 on a
dividend-paying stock with spot price S = 100 and yield q = 0.02,
when the risk-free rate is r = 0.03 and using the parameters
κ =5, σ =0.5, ρ =−0.8, θ =v0 =0.05, and λ=0,
along with the integration grid φ ∈ [0.00001, 50] in increments of 0.001 is 5.7590.
The price of the call with identical features is 6.2528.
If there is no dividend yield so that q = 0, then as expected,
the put price decreases, to 5.3790, and the call price increases, to 6.8678.
""

*/

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_heston_options() {
        // WITH DIVIDEND YIELD.
        let heston1 = heston(100.0, 0.05, 100.0, 0.5, 0.03, 0.02, -0.8, 0.5, 5.0, 0.05);
        // Call price.
        assert_approx_equal!(heston1.0, 6.2528, 1e-4);
        // Put price.
        assert_approx_equal!(heston1.1, 5.7590, 1e-4);

        // WITHOUT DIVIDEND YIELD.
        let heston2 = heston(100.0, 0.05, 100.0, 0.5, 0.03, 0.0, -0.8, 0.5, 5.0, 0.05);
        // Call price.
        assert_approx_equal!(heston2.0, 6.8678, 1e-4);
        // Put price.
        assert_approx_equal!(heston2.1, 5.3790, 1e-4);
    }
}
