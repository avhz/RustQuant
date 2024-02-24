// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Nelson-Siegel-Svensson (1994) model parameters.
pub struct NelsonSiegelSvensson {
    beta0: f64,
    beta1: f64,
    beta2: f64,
    beta3: f64,
    lambda1: f64,
    lambda2: f64,
}

impl NelsonSiegelSvensson {
    /// Create a new Nelson-Siegel model.
    #[must_use]
    pub const fn new(
        beta0: f64,
        beta1: f64,
        beta2: f64,
        beta3: f64,
        lambda1: f64,
        lambda2: f64,
    ) -> Self {
        Self {
            beta0,
            beta1,
            beta2,
            beta3,
            lambda1,
            lambda2,
        }
    }
}
