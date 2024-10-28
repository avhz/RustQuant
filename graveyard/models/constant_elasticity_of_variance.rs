// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the CEV process parameters.
pub struct ConstantElasticityOfVariance {
    /// The long-run mean ($\mu$).
    pub mu: ModelParameter,

    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: ModelParameter,

    /// Elasticity parameter.
    /// Often denoted as $\beta$, $\rho$, or $\gamma$.
    /// Must be in the unit interval $[0, 1]$.
    pub elasticity: ModelParameter,
}

impl ConstantElasticityOfVariance {
    /// Create a new Cox-Ingersoll-Ross process.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        elasticity: impl Into<ModelParameter>,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            elasticity: elasticity.into(),
        }
    }
}
