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

/// Log Moneyness Contract.
#[derive(Debug, Clone)]
pub struct LogMoneynessContract {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,
}

/// Log Underlying Contract.
#[derive(Debug, Clone)]
pub struct LogUnderlyingContract {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,
}

/// Log Option.
#[derive(Debug, Clone)]
pub struct LogOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,
}

impl Payoff for LogMoneynessContract {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        (underlying / self.strike).ln()
    }
}

impl Payoff for LogUnderlyingContract {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        underlying.ln()
    }
}

impl Payoff for LogOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        (underlying / self.strike).ln().max(0.0)
    }
}
