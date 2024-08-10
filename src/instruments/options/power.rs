// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{OptionContract, TypeFlag};
use crate::instruments::Payoff;

/// Power Option.
#[derive(Debug, Clone)]
pub struct PowerOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Power parameter.
    pub power: f64,
}

/// Power Option.
#[derive(Debug, Clone, Copy)]
pub struct PowerContract {
    /// Strike price of the option.
    pub strike: f64,

    /// Power parameter.
    pub power: f64,
}

impl Payoff for PowerContract {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        (underlying / self.strike).powf(self.power)
    }
}

impl Payoff for PowerOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match self.contract.type_flag {
            TypeFlag::Call => (underlying.powf(self.power) - self.strike).max(0.0),
            TypeFlag::Put => (self.strike - underlying.powf(self.power)).max(0.0),
        }
    }
}
