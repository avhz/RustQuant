// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::models::model_parameter::ModelParameter;

/// Struct containing the Heston model parameters.
pub struct SABR {
    /// The volatility of the volatility ($\alpha$).
    /// Note: $\alpha \in [0, \infty)$.
    pub alpha: ModelParameter,

    /// The beta parameter ($\beta$), which controls the skewness of the volatility.
    /// Note: $\beta \in [0, 1]$.
    pub beta: ModelParameter,

    /// The correlation between the asset and the variance Brownian motions ($\rho$).
    /// Note: $\rho \in [-1, 1]$.
    pub rho: ModelParameter,
}

impl SABR {
    /// Create a new SABR process.
    pub fn new(
        alpha: impl Into<ModelParameter>,
        beta: impl Into<ModelParameter>,
        rho: impl Into<ModelParameter>,
    ) -> Self {
        Self {
            alpha: alpha.into(),
            beta: beta.into(),
            rho: rho.into(),
        }
    }
}
