// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::Date;

/// Option type enum.
#[derive(Debug, Clone, Copy)]
pub enum TypeFlag {
    /// Call option (right to BUY the underlying asset).
    Call = 1,

    /// Put option (right to SELL the underlying asset).
    Put = -1,
}

/// American/European option type enum.
#[derive(Debug, Clone)]
pub enum ExerciseFlag {
    /// European option (can only be exercised at expiry).
    /// Most index options are European.
    European {
        /// The expiry date of the option.
        expiry: Date,
    },

    /// American option (can be exercised at any time before expiry).
    /// Most stock options are American.
    American {
        /// Initial date of the option.
        start: Date,
        /// The terminal date of the option.
        end: Date,
    },

    /// Bermudan option (can be exercised at specific dates before expiry).
    /// Bermudan options are a hybrid of American and European options,
    /// hence the name. These are relatively rare and typically used
    /// in OTC markets.
    Bermudan {
        /// The exercise dates of the option.
        exercise_dates: Vec<Date>,
    },
}

impl ExerciseFlag {
    /// Get the expiry date of the option.
    pub fn expiry(&self) -> Date {
        match self {
            ExerciseFlag::European { expiry } => *expiry,
            ExerciseFlag::American { end, .. } => *end,
            ExerciseFlag::Bermudan { exercise_dates } => exercise_dates[exercise_dates.len() - 1],
        }
    }
}

/// Option strike type enum.
///
/// These are used for options such as
/// Asian options (average) or Lookback options (extreme).
#[derive(Debug, Clone, Copy)]
pub enum StrikeFlag {
    /// Strike is fixed.
    Fixed,

    /// Strike is floating (e.g. strike = S_max).
    Floating,
}

/// Instrument settlement flag.
#[derive(Debug, Clone, Copy)]
pub enum SettlementFlag {
    /// Cash settlement.
    Cash,

    /// Physical settlement.
    Physical,
}

/// Method of averaging (arithmetic or geometric, and continuous or discrete).
#[derive(Debug, Clone, Copy)]
pub enum AveragingMethod {
    /// Arithmetic Asian option with discrete averaging.
    ArithmeticDiscrete,

    /// Arithmetic Asian option with continuous averaging.
    ArithmeticContinuous,

    /// Geometric Asian option with discrete averaging.
    GeometricDiscrete,

    /// Geometric Asian option with continuous averaging.
    GeometricContinuous,
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

/// Binary type enum.
#[derive(Debug, Clone, Copy)]
pub enum BinaryType {
    /// Asset-or-nothing binary option.
    AssetOrNothing,

    /// Cash-or-nothing binary option.
    CashOrNothing,
}
