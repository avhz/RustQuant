// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Black-Derman-Toy process parameters.
pub struct BlackDermanToy {
    /// Instantaneous volatility
    pub sigma: ModelParameter,

    /// Value of underlying at option expiry
    pub theta: ModelParameter,
}

impl BlackDermanToy {
    /// Create a new Black-Derman-Toy process.
    pub fn new(sigma: impl Into<ModelParameter>, theta: impl Into<ModelParameter>) -> Self {
        Self {
            sigma: sigma.into(),
            theta: theta.into(),
        }
    }
}
