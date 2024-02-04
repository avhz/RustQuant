// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Arithmetic Brownian Motion parameters.
pub struct ArithmeticBrownianMotion {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,
}

impl ArithmeticBrownianMotion {
    /// Create a new Arithmetic Brownian Motion process.
    pub fn new(mu: impl Into<ModelParameter>, sigma: impl Into<ModelParameter>) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
        }
    }
}
