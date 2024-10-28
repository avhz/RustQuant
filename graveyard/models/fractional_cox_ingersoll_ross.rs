// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use self::fractional_brownian_motion::FractionalProcessGeneratorMethod;
use crate::models::model_parameter::ModelParameter;
use crate::stochastics::*;

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
