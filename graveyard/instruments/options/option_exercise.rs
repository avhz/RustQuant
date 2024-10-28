// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// European exercise type.
pub struct EuropeanExercise {
    /// The expiry date of the option.
    pub expiry: Date,
}

/// American exercise type.
pub struct AmericanExercise {
    /// Initial date of the option.
    pub start: Date,

    /// The terminal date of the option.
    pub end: Date,
}

/// Bermudan exercise type.
pub struct BermudanExercise {
    /// The exercise dates of the option.
    pub exercise_dates: Vec<Date>,
}
