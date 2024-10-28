// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Ho-Lee process parameters.
pub struct HoLee {
    /// The diffusion, or instantaneous volatility ($\sigma$).
    pub sigma: ModelParameter,

    /// Non-negative time-varying mean reversion function ($\theta_t$)
    pub theta: ModelParameter,
}

impl HoLee {
    /// Create a new Ho-Lee process.
    pub fn new(sigma: impl Into<ModelParameter>, theta: impl Into<ModelParameter>) -> Self {
        Self {
            sigma: sigma.into(),
            theta: theta.into(),
        }
    }
}
