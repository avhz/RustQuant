// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::instruments::Instrument;

use super::option_flags::*;
use time::Date;

/// Option contract data.
#[derive(Debug, Clone)]
pub struct OptionContract {
    /// The option's type flag (call or put).
    pub type_flag: TypeFlag,

    /// The option's strike type (fixed or floating).
    pub strike_flag: StrikeFlag,

    /// The option's exercise type (European, American, Bermudan).
    pub exercise_flag: ExerciseFlag,

    /// The option's settlement type (cash or physical).
    pub settlement_flag: SettlementFlag,
}

/// Vanilla option.
#[derive(Debug, Clone)]
pub struct VanillaOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,
}

impl Instrument for VanillaOption {
    fn price(&self) -> f64 {
        1.
    }

    fn error(&self) -> Option<f64> {
        None
    }

    fn valuation_date(&self) -> Date {
        todo!()
    }

    fn instrument_type(&self) -> &'static str {
        todo!()
    }
}

/// Barrier option.
#[derive(Debug, Clone)]
pub struct BarrierOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Barrier type (up-and-out, down-and-out, up-and-in, down-and-in).
    pub barrier_type: BarrierType,

    /// Barrier level.
    pub barrier: f64,

    /// Rebate amount.
    pub rebate: f64,

    /// Strike price of the option.
    pub strike: f64,
}

/// Binary option.
#[derive(Debug, Clone)]
pub struct BinaryOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,
}

/// Binary type enum.
#[derive(Debug, Clone, Copy)]
pub enum BinaryType {
    /// Asset-or-nothing binary option.
    AssetOrNothing,

    /// Cash-or-nothing binary option.
    CashOrNothing,
}

/// Gap option.
#[derive(Debug, Clone)]
pub struct GapOption {
    /// The option contract.
    pub contract: OptionContract,

    /// First strike price (barrier strike).
    pub strike_1: f64,

    /// Second strike price (payoff strike).
    pub strike_2: f64,
}

/// Supershare option.
#[derive(Debug, Clone)]
pub struct SupershareOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Lower strike price.
    pub strike_1: f64,

    /// Upper strike price.
    pub strike_2: f64,
}

/// Forward start option.
#[derive(Debug, Clone)]
pub struct ForwardStartOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Forward start date.
    pub start_date: Date,
}

/// Asian option.
#[derive(Debug, Clone)]
pub struct AsianOption {
    /// The option contract.
    pub contract: OptionContract,
}

/// Barrier type flag.
#[derive(Clone, Copy, Debug)]
pub enum BarrierType {
    /// Up-and-out barrier option.
    UpAndOut,

    /// Down-and-out barrier option.
    DownAndOut,

    /// Up-and-in barrier option.
    UpAndIn,

    /// Down-and-in barrier option.
    DownAndIn,
}
