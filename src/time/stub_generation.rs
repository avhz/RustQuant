// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
