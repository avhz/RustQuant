//! ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//! RustQuant: A Rust library for quantitative finance tools.
//! Copyright (C) 2023 https://github.com/avhz
//! Dual licensed under Apache 2.0 and MIT.
//! See:
//!      - LICENSE-APACHE.md
//!      - LICENSE-MIT.md
//! ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
pub use autodiff::*;

pub mod curves;
pub use curves::*;

#[cfg(feature = "data")]
pub mod data;
pub use data::*;

pub mod error;
pub use error::*;

pub mod instruments;
pub use instruments::*;

#[macro_use]
pub mod macros;
pub use macros::*;

pub mod math;
pub use math::*;

pub mod ml;
pub use ml::*;

pub mod money;
pub use money::*;

pub mod portfolio;
pub use portfolio::*;

pub mod statistics;
pub use statistics::*;

pub mod stochastics;
pub use stochastics::*;

pub mod time;
pub use time::*;

pub mod trading;
pub use trading::*;
