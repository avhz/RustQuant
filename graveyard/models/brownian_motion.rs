// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct containing the Geometric Brownian Motion parameters.
#[derive(Debug)]
pub struct BrownianMotion {}

impl Default for BrownianMotion {
    fn default() -> Self {
        Self::new()
    }
}

impl BrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}
