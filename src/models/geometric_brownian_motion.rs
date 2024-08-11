// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Geometric Brownian Motion parameters.
pub struct GeometricBrownianMotion {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,

    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,
}

impl GeometricBrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new(mu: impl Into<ModelParameter>, sigma: impl Into<ModelParameter>) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
        }
    }

    /// Return the parameters as a Vec<f64>.
    pub fn parameters(&self) -> Vec<f64> {
        vec![self.mu.0(0.0), self.sigma.0(0.0)]
    }

    /// Unpack the parameters from a Vec<f64>.
    pub fn unpack(&self) -> (f64, f64) {
        let p = self.parameters();

        (p[0], p[1])
    }
}
