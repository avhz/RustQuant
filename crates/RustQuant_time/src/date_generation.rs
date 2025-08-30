// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Date generation conventions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateGenerationConvention {
    /// Forward from the issue date.
    Forward,

    /// Backward from the maturity date.
    Backward,

    /// Zero date generation.
    Zero,
}

/// Stub generation rules.
pub enum StubGeneration {
    /// No stubs.
    None,

    /// Short stub at the beginning.
    ShortFront,

    /// Short stub at the end.
    ShortBack,

    /// Long stub at the beginning.
    LongFront,

    /// Long stub at the end.
    LongBack,

    /// Front and back stubs.
    Both,
}
