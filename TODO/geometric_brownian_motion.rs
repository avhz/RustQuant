#![deny(missing_docs)]

use crate::{
    helpers::{cumsum::*, linspace::*},
    normal_distribution::*,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct for the Geometric Brownian Motion parameters and methods.
#[derive(Debug)]
pub struct GeometricBrownianMotion {
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
    pub fn generate(&self) -> Vec<Vec<f64>> {
        let S = self.initial_value;
        let T = self.time_horizon;
        let r = self.drift;
        let v = self.diffusion;
        let N = self.time_steps;
        let dt: f64 = T / N as f64;

        let mut output: Vec<Vec<f64>> = Vec::with_capacity(self.trajectories);

        for _ in 0..self.trajectories {
            // Vector for GBM trajectory.
            let mut St: Vec<f64> = Vec::with_capacity(N + 1);
            St.push(S);

            // Vector of time points.
            let time = linspace(0.0, T, N + 1);

            // Standard normal sample of N elements.
            let Z: Vec<f64> = rnorm(N);

            // Brownian Motion increments.
            let mut dW: Vec<f64> = Vec::with_capacity(Z.len());

            // for i in 0..(Z.len()) {
            for item in Z {
                dW.push(item * dt.sqrt());
            }

            // Brownian Motion at each time (N+1 elements).
            let W: Vec<f64> = cumsum(&dW);

            for i in 1..(St.capacity() - 1) {
                St.push(S * ((r - v * v / 2.0) * time[i] + v * W[i]).exp());
            }
            output.push(St);
        }

        output
        // Trajectories {
        //     trajectories: (output),
        // }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::plot_vector;

    #[test]
    fn test_gbm() -> Result<(), Box<dyn std::error::Error>> {
        let gbm = GeometricBrownianMotion {
            initial_value: 10.0,
            time_horizon: 1.0,
            drift: 0.05,
            diffusion: 0.1,
            time_steps: 1000,
            trajectories: 2,
        };

        let output: Vec<Vec<f64>> = gbm.generate();

        let file1 = "GBM1.png";
        plot_vector(output[0].clone(), file1).unwrap();

        let file2 = "GBM2.png";
        plot_vector(output[1].clone(), file2)

        // if let Err(err) = write_vector(v) {
        //     eprintln!("{}", err);
        //     process::exit(1);
        // }
    }
}
