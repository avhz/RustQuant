// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{OptionContract, TypeFlag};
use crate::Payoff;

/// Power Option.
/// Not to be confused with the [PoweredOption].
#[derive(Debug, Clone)]
pub struct PowerOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Power parameter.
    pub power: f64,
}

/// Powered Option.
/// Not to be confused with the [PowerOption].
#[derive(Debug, Clone)]
pub struct PoweredOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Power parameter.
    pub power: f64,
}

/// Capped Power Option.
#[derive(Debug, Clone)]
pub struct CappedPowerOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Power parameter.
    pub power: f64,

    /// Cap price of the option.
    pub cap: f64,
}

/// Power Contract.
///
///
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

impl Payoff for CappedPowerOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        let payoff = match self.contract.type_flag {
            TypeFlag::Call => (underlying.powf(self.power) - self.strike).max(0.0),
            TypeFlag::Put => (self.strike - underlying.powf(self.power)).max(0.0),
        };

        payoff.min(self.cap)
    }
}

impl Payoff for PoweredOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        let payoff = match self.contract.type_flag {
            TypeFlag::Call => (underlying - self.strike).max(0.0),
            TypeFlag::Put => (self.strike - underlying).max(0.0),
        };

        payoff.powf(self.power)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl PowerOption {
    /// Create a new power option.
    pub fn new(contract: OptionContract, strike: f64, power: f64) -> Self {
        Self {
            contract,
            strike,
            power,
        }
    }
}

impl PoweredOption {
    /// Create a new powered option.
    pub fn new(contract: OptionContract, strike: f64, power: f64) -> Self {
        Self {
            contract,
            strike,
            power,
        }
    }
}

impl CappedPowerOption {
    /// Create a new capped power option.
    pub fn new(contract: OptionContract, strike: f64, power: f64, cap: f64) -> Self {
        Self {
            contract,
            strike,
            power,
            cap,
        }
    }
}

impl PowerContract {
    /// Create a new power contract.
    pub fn new(strike: f64, power: f64) -> Self {
        Self { strike, power }
    }
}
