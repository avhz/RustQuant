#![allow(non_snake_case)]
#![deny(missing_docs)]

use {
    crate::{cumsum, linspace, rnorm},
    rayon::prelude::*,
    std::sync::{Arc, Mutex},
};

// ############################################################################
// STRUCTS
// ############################################################################

/// Struct for the Geometric Brownian Motion parameters and methods.
struct GeometricBrownianMotion {
    /// Initial value for the trajectory.
    pub initial_value: f64,
    /// Length of the time period that the trajectory covers.
    pub time_horizon: f64,
    /// Drift parameter.
    pub drift: f64,
    /// Diffusion parameter.
    pub diffusion: f64,
    /// Time steps between 0 and `time_horizon`.
    pub time_steps: usize,
    /// Number of trajectories to generate.
    pub trajectories: usize,
}

// ############################################################################
// IMPLEMENTATION
// ############################################################################

impl Default for GeometricBrownianMotion {
    fn default() -> Self {
        GeometricBrownianMotion {
            initial_value: (0_f64),
            time_horizon: (0_f64),
            drift: (0_f64),
            diffusion: (0_f64),
            time_steps: (0_usize),
            trajectories: (1_usize),
        }
    }
}

impl GeometricBrownianMotion {
    /// Generates a vector of length `N` of a Geometric Brownian Motion trajectory.
    pub fn generate_serial(&self) -> Vec<Vec<f64>> {
        let S = self.initial_value;
        let T = self.time_horizon;
        let r = self.drift;
        let v = self.diffusion;
        let N = self.time_steps;

        let mut output: Vec<Vec<f64>> = Vec::with_capacity(self.trajectories);

        // for i in output.par_iter() {

        for _ in 0..self.trajectories {
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

            output.push(St);
        }

        return output;
    }

    /// Generates a vector of length `N` of a Geometric Brownian Motion trajectory.
    pub fn generate_parallel(&self) -> Vec<Vec<f64>> {
        let S = self.initial_value;
        let T = self.time_horizon;
        let r = self.drift;
        let v = self.diffusion;
        let N = self.time_steps;

        // Length of each time step.
        let dt: f64 = T / N as f64;

        let mut output: Vec<Vec<f64>> = Vec::with_capacity(self.trajectories);

        (0..self.trajectories).into_par_iter().for_each(|i| {
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

            output[i] = St;
        });

        return output;
    }
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
        let GBM = GeometricBrownianMotion {
            initial_value: 10.0,
            time_horizon: 1.0,
            drift: 0.05,
            diffusion: 0.1,
            time_steps: 1000,
            trajectories: 3,
        };

        let v1: Vec<Vec<f64>> = GBM.generate_serial();
        let v2: Vec<Vec<f64>> = GBM.generate_parallel();

        let file = "GBM.png";
        plot_vector(v1[0].clone(), file)

        // if let Err(err) = write_vector(v) {
        //     eprintln!("{}", err);
        //     process::exit(1);
        // }
    }
}
