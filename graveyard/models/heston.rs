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
pub struct Heston {
    /// The initial variance ($v_0$).
    pub initial_variance: ModelParameter,

    /// The long-run variance ($\theta$).
    pub long_run_variance: ModelParameter,

    /// The mean reversion rate ($\kappa$).
    pub mean_reversion_rate: ModelParameter,

    /// The correlation between the asset and the variance Brownian motions ($\rho$).
    pub correlation: ModelParameter,

    /// The volatility of volatility ($\sigma$).
    pub volatility_of_volatility: ModelParameter,
}

impl Heston {
    /// Create a new Arithmetic Brownian Motion process.
    pub fn new(
        initial_variance: impl Into<ModelParameter>,
        long_run_variance: impl Into<ModelParameter>,
        mean_reversion_rate: impl Into<ModelParameter>,
        correlation: impl Into<ModelParameter>,
        volatility_of_volatility: impl Into<ModelParameter>,
    ) -> Self {
        Self {
            initial_variance: initial_variance.into(),
            long_run_variance: long_run_variance.into(),
            mean_reversion_rate: mean_reversion_rate.into(),
            correlation: correlation.into(),
            volatility_of_volatility: volatility_of_volatility.into(),
        }
    }
}
