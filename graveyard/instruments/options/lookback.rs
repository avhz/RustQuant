// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::option_flags::StrikeFlag;
use super::{BarrierType, OptionContract, TypeFlag};
use crate::instruments::Payoff;

/// Lookback option.
#[derive(Debug, Clone)]
pub struct LookbackOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    /// Required for [StrikeFlag::Fixed] lookback options.
    /// If no strike is provided, the strike will be assumed to be
    /// floating (e.g. strike = S_max).
    pub strike: Option<f64>,
}

impl Payoff for LookbackOption {
    type Underlying = Vec<f64>;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        let s = underlying;
        let k = self.strike;

        let terminal = s.last().unwrap();

        let s_max = s.iter().max_by(|x, y| x.total_cmp(y)).unwrap_or(&terminal);
        let s_min = s.iter().min_by(|x, y| x.total_cmp(y)).unwrap_or(&terminal);

        match self.contract.strike_flag {
            Some(StrikeFlag::Fixed) => match self.contract.type_flag {
                TypeFlag::Call => (s_max - k.unwrap()).max(0.0),
                TypeFlag::Put => (k.unwrap() - s_min).max(0.0),
            },
            Some(StrikeFlag::Floating) => match self.contract.type_flag {
                TypeFlag::Call => (terminal - s_min).max(0.0),
                TypeFlag::Put => (s_max - terminal).max(0.0),
            },
            None => match self.contract.type_flag {
                TypeFlag::Call => (terminal - s_min).max(0.0),
                TypeFlag::Put => (s_max - terminal).max(0.0),
            },
        }
    }
}
