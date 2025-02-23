// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{
    fractional_brownian_motion::FractionalProcessGeneratorMethod,
    process::{StochasticProcess, Trajectories},
    StochasticProcessConfig,
};
use crate::fractional_brownian_motion::FractionalBrownianMotion;
use crate::model_parameter::ModelParameter;
use rayon::prelude::*;

/// Struct containing the Ornstein-Uhlenbeck process parameters.
pub struct FractionalCoxIngersollRoss {
    /// The long-run mean ($\mu$).
    pub mu: ModelParameter,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: ModelParameter,

    /// Mean reversion parameter ($\theta$).
    /// Defines the speed at which the process reverts to the long-run mean.
    pub theta: ModelParameter,

    /// Hurst parameter of the process.
    /// The Hurst parameter is a measure of the long-term memory of the process.
    pub hurst: f64,

    /// Method to generate Fractional Gaussian Noise.
    pub method: FractionalProcessGeneratorMethod,
}

impl FractionalCoxIngersollRoss {
    /// Create a new Ornstein-Uhlenbeck process.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        theta: impl Into<ModelParameter>,
        hurst: f64,
        method: FractionalProcessGeneratorMethod,
    ) -> Self {
        assert!((0.0..=1.0).contains(&hurst));
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            theta: theta.into(),
            hurst,
            method,
        }
    }
}

impl StochasticProcess for FractionalCoxIngersollRoss {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.theta.0(t) * (self.mu.0(t) - x)
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        self.sigma.0(t) * x.sqrt()
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        Some(0.0)
    }

    fn parameters(&self) -> Vec<f64> {
        vec![
            self.mu.0(0.0),
            self.sigma.0(0.0),
            self.theta.0(0.0),
            self.hurst,
        ]
    }

    fn monte_carlo(&self, config: &StochasticProcessConfig) -> Trajectories {
        let (t_0, x_0, t_n, n_steps, m_paths, parallel) = config.unpack();

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
                // FractionalBrownianMotion::fgn_fft(&fbm, n_steps, t_n)
                fbm.fgn_fft(n_steps, t_n)
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
    use crate::{fractional_ornstein_uhlenbeck::FractionalOrnsteinUhlenbeck, StochasticScheme};

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

        let config = StochasticProcessConfig::new(
            10.0, 0.0, 0.5, 100, StochasticScheme::EulerMaruyama, 100, false, None
        );

        #[allow(dead_code)]
        let _output = fou.monte_carlo(&config);

        std::result::Result::Ok(())
    }
}
