// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Nelson-Siegel (1987) model parameters.
pub struct NelsonSiegel {
    /// $\beta_0$
    pub beta0: f64,

    /// $\beta_1$
    pub beta1: f64,

    /// $\beta_2$
    pub beta2: f64,

    /// $\lambda$
    pub lambda: f64,
}

impl NelsonSiegel {
    /// Create a new Nelson-Siegel model.
    #[must_use]
    pub const fn new(beta0: f64, beta1: f64, beta2: f64, lambda: f64) -> Self {
        Self {
            beta0,
            beta1,
            beta2,
            lambda,
        }
    }
}
