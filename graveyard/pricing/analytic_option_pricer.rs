// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Analytic option pricing module.

/// Analytic option pricer.
///
/// This struct is used to price options using analytic methods.
#[derive(Debug, derive_builder::Builder)]
pub struct AnalyticOptionPricer<O, M> {
    /// The option to be priced.
    pub option: O,

    /// The model to be used to price the option.
    pub model: M,
}

impl<O, M> AnalyticOptionPricer<O, M> {
    /// Create a new instance of the pricer.
    pub fn new(option: O, model: M) -> Self {
        Self { option, model }
    }

    /// Return a reference to the option.
    pub fn option(&self) -> &O {
        &self.option
    }

    /// Return a reference to the model.
    pub fn model(&self) -> &M {
        &self.model
    }

    /// Update the option.
    pub fn set_option(&mut self, option: O) {
        self.option = option;
    }

    /// Update the model.
    pub fn set_model(&mut self, model: M) {
        self.model = model;
    }
}
