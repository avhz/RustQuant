// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::option_flags::*;
use derive_builder::Builder;

/// Option contract data.
#[derive(Debug, Clone, Builder)]
pub struct OptionContract {
    /// Mandatory: Option type (call or put).
    pub type_flag: TypeFlag,

    /// Mandatory: Exercise type (European, American, Bermudan).
    pub exercise_flag: ExerciseFlag,

    /// Optional: Strike type (fixed or floating).
    #[builder(default)]
    pub strike_flag: Option<StrikeFlag>,

    /// Optional: Settlement type (cash or physical).
    #[builder(default)]
    pub settlement_flag: Option<SettlementFlag>,
}
