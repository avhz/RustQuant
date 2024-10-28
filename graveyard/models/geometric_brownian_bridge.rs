// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Geometric Brownian Bridge parameters.
/// The Geometric Brownian Bridge is a stochastic process that models a path-dependent option.
/// It is a modification of the Geometric Brownian Motion where the end value is known.
pub struct GeometricBrownianBridge {
    /// The drift ($\mu$) in percentage.
    pub mu: ModelParameter,
    /// The volatility ($\sigma$) in percentage.
    pub sigma: ModelParameter,
    /// The known end value of the process.
    pub end_value: f64,
    /// The known end time of the process.
    pub end_time: f64,
}

impl GeometricBrownianBridge {
    /// Create a new Geometric Brownian Bridge process.
    /// # Arguments
    /// * `mu` - The drift ($\mu$) in percentage.
    /// * `sigma` - The volatility ($\sigma$) in percentage.
    /// * `end_value` - The known end value of the process.
    /// * `end_time` - The known end time of the process.
    pub fn new(
        mu: impl Into<ModelParameter>,
        sigma: impl Into<ModelParameter>,
        end_value: f64,
        end_time: f64,
    ) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            end_value,
            end_time,
        }
    }
}
