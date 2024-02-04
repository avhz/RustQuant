// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Hull-White process parameters.
pub struct HullWhite {
    /// Long run mean ($\alpha)
    pub alpha: ModelParameter,

    /// Non-negative diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: ModelParameter,

    /// Mean reversion function (non-negative) ($\theta(t)$)
    pub theta: ModelParameter,
}

impl HullWhite {
    /// Create a new Hull-White process.
    pub fn new(
        alpha: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        theta: impl Into<ModelParameter>,
    ) -> Self {
        Self {
            alpha: alpha.into(),
            sigma: sigma.into(),
            theta: theta.into(),
        }
    }
}
