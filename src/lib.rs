//! RustQuant: A Rust library for quantitative finance tools.
//! Copyright (C) 2022, 2023 <https://github.com/avhz>
//!
//! Dual licensed under Apache 2.0 and MIT.
//! See:
//!      - [LICENSE-APACHE.md](https://github.com/avhz/RustQuant/blob/main/LICENSE-APACHE.md)
//!      - [LICENSE-MIT.md](https://github.com/avhz/RustQuant/blob/main/LICENSE-MIT.md)
//!
//! Contact me at: <RustQuantContact@gmail.com>
//!
//! Any contributions are greatly appreciated. Make a PR or open an issue !
//!
//! I'm particularly interested in hearing from people with strong experience
//! in implementing quantitative software in a professional setting.

// Strictly enforce documentation.
#![forbid(missing_docs)]
// Allow snake case.
// This is because much of this library is based on mathematics, so I
// want to adhere to the standard mathematical notation.
#![allow(non_snake_case)]
// Strictly enforce SAFETY comments.
// There is no unsafe code currently, but for anyone to add any, it must be
// documented with a SAFETY comment.
#![forbid(clippy::undocumented_unsafe_blocks)]

pub mod autodiff;
pub mod curves;
#[cfg(feature = "data")]
pub mod data;
pub mod error;
pub mod instruments;
#[macro_use]
pub mod macros;
pub mod math;
pub mod ml;
pub mod models;
pub mod money;
pub mod portfolio;
pub mod statistics;
pub mod stochastics;
pub mod time;
pub mod trading;
