#![allow(non_snake_case)]

use crate::{cumsum, linspace, rnorm};

// ############################################################################
// STRUCTS
// ############################################################################

// pub struct GeometricBrownianMotionParameters {

// }

// ############################################################################
// FUNCTIONS
// ############################################################################

pub fn GeometricBrownianMotion(
    S: f64,   // initial stock price
    T: f64,   // time to expiry
    r: f64,   // risk free rate
    v: f64,   // volatility
    N: usize, // number of steps between 0 and T
) -> Vec<f64> {
    // length of each time step
    let dt: f64 = T / N as f64;

    // Vector for GBM paths
    let mut St: Vec<f64> = vec![0.0; N + 1];
    St[0] = S;

    // vector of time points
    let time = linspace(0.0, T, N);

    // standard normal sample of N elements
    let Z: Vec<f64> = rnorm(N);

    // Brownian motion increments
    let mut dW: Vec<f64> = vec![0.0; Z.len()];

    for i in 0..(Z.len()) {
        dW[i] = Z[i] * dt.sqrt();
    }

    // Brownian motion at each time (N+1 elements)
    let mut W: Vec<f64> = cumsum(dW);
    W.insert(0, 0.0);

    for i in 1..(St.len()) {
        St[i] = S * ((r - v * v / 2.0) * time[i] + v * W[i]).exp();
    }

    // St = St.pop();

    return (St);
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::*;

    use std::process;

    #[test]
    fn TEST_GBM() {
        let v: Vec<f64> = GeometricBrownianMotion(100.0, 1.0, 0.05, 0.2, 100);

        if let Err(err) = write_vector(v) {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
