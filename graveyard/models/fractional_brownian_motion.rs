// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::stochastics::fractional_brownian_motion::FractionalProcessGeneratorMethod;

/// Struct containing the Fractional Brownian Motion parameters.
#[derive(Debug)]
pub struct FractionalBrownianMotion {
    /// Hurst parameter of the process.
    pub hurst: f64,

    /// Method used to generate the process.
    pub method: FractionalProcessGeneratorMethod,
}

impl Default for FractionalBrownianMotion {
    fn default() -> Self {
        Self::new(0.5, FractionalProcessGeneratorMethod::FFT)
    }
}

impl FractionalBrownianMotion {
    /// Create a new Fractional Brownian Motion process.
    ///
    /// # Panics
    ///
    /// Will panic if Hurst parameter is not in [0, 1].
    #[must_use]
    pub fn new(hurst: f64, method: FractionalProcessGeneratorMethod) -> Self {
        assert!((0.0..=1.0).contains(&hurst));

        Self { hurst, method }
    }
}
