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
    European {
        /// The expiry date of the option.
        expiry: Date,
    },

    /// American option (can be exercised at any time before expiry).
    American {
        /// Initial date of the option.
        start: Date,
        /// The terminal date of the option.
        end: Date,
    },

    /// Bermudan option (can be exercised at specific dates before expiry).
    Bermudan {
        /// The exercise dates of the option.
        exercise_dates: Vec<Date>,
    },
}

/// Option strike type enum.
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
