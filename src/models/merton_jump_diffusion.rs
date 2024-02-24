// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::math::Gaussian;
use crate::models::model_parameter::ModelParameter;

/// Struct containing the Merton Jump Diffusion parameters.
/// The Merton Jump Diffusion is a stochastic process that models a path-dependent option.
/// It is a modification of the Geometric Brownian Motion where the end value is known.
pub struct MertonJumpDiffusion {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,

    /// The jump intensity ($\lambda$) in percentage.
    pub lambda: ModelParameter,

    /// The Gaussian distribution for the jump size.
    pub gaussian: Gaussian,
}

impl MertonJumpDiffusion {
    /// Create a new Merton Jump Diffusion process.
    /// # Arguments
    /// * `mu` - The drift ($\mu$) in percentage.
    /// * `sigma` - The volatility ($\sigma$) in percentage.
    /// * `lambda` - The jump intensity ($\lambda$) in percentage.
    /// * `m` - The mean of the Gaussian distribution for the jump size.
    /// * `v` - The variance of the Gaussian distribution for the jump size.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        lambda: impl Into<ModelParameter>,
        m: f64,
        v: f64,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            lambda: lambda.into(),
            gaussian: Gaussian::new(m, v),
        }
    }
}
