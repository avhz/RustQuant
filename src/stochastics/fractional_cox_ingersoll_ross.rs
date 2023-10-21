// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::*;
use rayon::prelude::*;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
pub struct FractionalCoxIngersollRoss {
    /// The long-run mean ($\mu$).
    pub mu: f64,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: f64,

    /// Mean reversion parameter ($\theta$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub theta: f64,

    /// Hurst parameter of the process.
    /// The Hurst parameter is a measure of the long-term memory of the process.
    pub hurst: f64,

    /// Method to generate Fractional Gaussian Noise.
    pub method: FractionalProcessGeneratorMethod,
}

impl FractionalCoxIngersollRoss {
    /// Create a new Ornstein-Uhlenbeck process.
    pub fn new(
        mu: f64,
        sigma: f64,
        theta: f64,
        hurst: f64,
        method: FractionalProcessGeneratorMethod,
    ) -> Self {
        assert!(sigma >= 0.0);
        assert!((0.0..=1.0).contains(&hurst));
        Self {
            mu,
            sigma,
            theta,
            hurst,
            method,
        }
    }
}

impl StochasticProcess for FractionalCoxIngersollRoss {
    fn drift(&self, x: f64, _t: f64) -> f64 {
        self.theta * (self.mu - x)
    }

    fn diffusion(&self, _x: f64, _t: f64) -> f64 {
        self.sigma * _x.sqrt()
    }

    fn jump(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }

    fn euler_maruyama(
        &self,
        x_0: f64,
        t_0: f64,
        t_n: f64,
        n_steps: usize,
        m_paths: usize,
        parallel: bool,
    ) -> Trajectories {
        let fgn = match self.method {
            FractionalProcessGeneratorMethod::CHOLESKY => {
                let fbm = FractionalBrownianMotion::new(
                    self.hurst,
                    FractionalProcessGeneratorMethod::CHOLESKY,
                );
                FractionalBrownianMotion::fgn_cholesky(&fbm, n_steps, t_n)
            }
            FractionalProcessGeneratorMethod::FFT => {
                let fbm = FractionalBrownianMotion::new(
                    self.hurst,
                    FractionalProcessGeneratorMethod::FFT,
                );
                FractionalBrownianMotion::fgn_fft(&fbm, n_steps, t_n)
            }
        };

        let dt: f64 = (t_n - t_0) / (n_steps as f64);

        // Initialise empty paths and fill in the time points.
        let mut paths = vec![vec![x_0; n_steps + 1]; m_paths];
        let times: Vec<f64> = (0..=n_steps).map(|t| t_0 + dt * (t as f64)).collect();

        let path_generator = |path: &mut Vec<f64>| {
            for t in 0..n_steps {
                path[t + 1] = path[t]
                    + self.drift(path[t], times[t]) * dt
                    + self.diffusion(path[t], times[t]) * fgn[t];
            }
        };

        if parallel {
            paths.par_iter_mut().for_each(path_generator);
        } else {
            paths.iter_mut().for_each(path_generator);
        }

        Trajectories { times, paths }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_fractional_cir {
    use super::*;

    #[test]
    #[ignore = "Hard to test."]
    fn test_fractional_cir() -> Result<(), Box<dyn std::error::Error>> {
        let fou = FractionalOrnsteinUhlenbeck::new(
            0.15,
            0.45,
            0.01,
            0.7,
            FractionalProcessGeneratorMethod::FFT,
        );

        #[allow(dead_code)]
        let output = fou.euler_maruyama(10.0, 0.0, 0.5, 100, 100, false);

        std::result::Result::Ok(())
    }
}
