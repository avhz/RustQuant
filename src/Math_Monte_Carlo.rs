#![allow(non_snake_case)]
#![deny(missing_docs)]

use crate::{cumsum, linspace, rnorm};

// ############################################################################
// STRUCTS
// ############################################################################

// pub struct GeometricBrownianMotionParameters {}

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Generates a vector of length `N` of a Geometric Brownian Motion trajectory.
///
/// # Arguments:
///
/// * `S` - Initial value.
/// * `T` - Time period.
/// * `r` - Drift/rate.
/// * `v` - Diffusion/volatility.
/// * `N` - Time steps in the trajectory.
pub fn GeometricBrownianMotion(
    S: f64,   // initial value
    T: f64,   // time
    r: f64,   // drift
    v: f64,   // diffusion
    N: usize, // number of steps between 0 and T
) -> Vec<f64> {
    // Length of each time step.
    let dt: f64 = T / N as f64;

    // Vector for GBM trajectory.
    let mut St: Vec<f64> = vec![0.0; N + 1];

    St[0] = S;

    // Vector of time points.
    let time = linspace(0.0, T, N + 1);

    // Standard normal sample of N elements.
    let Z: Vec<f64> = rnorm(N);

    // Brownian Motion increments.
    let mut dW: Vec<f64> = vec![0.0; Z.len()];

    for i in 0..(Z.len()) {
        dW[i] = Z[i] * dt.sqrt();
    }

    // Brownian Motion at each time (N+1 elements).
    let mut W: Vec<f64> = cumsum(&dW);
    W.insert(0, 0.0);

    for i in 1..(St.len()) {
        St[i] = S * ((r - v * v / 2.0) * time[i] + v * W[i]).exp();
    }

    return St;
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::*;

    #[test]
    fn TEST_GBM() -> Result<(), Box<dyn std::error::Error>> {
        let v: Vec<f64> = GeometricBrownianMotion(10.0, 10.0, 0.1, 0.3, 500);

        let file = "geometric_brownian_motion.png";
        plot_vector(v, file)

        // if let Err(err) = write_vector(v) {
        //     eprintln!("{}", err);
        //     process::exit(1);
        // }
    }
}
