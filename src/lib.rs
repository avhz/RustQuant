//! RustQuant: A Rust library for quantitative finance tools.
//!
//! Copyright (C) 2022-2024 <https://github.com/avhz>
//!
//! Dual licensed under Apache 2.0 and MIT.
//!
//! See:
//! - [LICENSE-APACHE.md](https://github.com/avhz/RustQuant/blob/main/LICENSE-APACHE.md)
//! - [LICENSE-MIT.md](https://github.com/avhz/RustQuant/blob/main/LICENSE-MIT.md)
//!
//! Contact me at: <RustQuantContact@gmail.com>
//!
//! Any contributions are greatly appreciated. Make a PR or open an issue !
//!
//! I'm particularly interested in hearing from people with strong experience
//! in implementing quantitative software in a professional setting.
//!
//! # Installation
//!
//! In your Rust project's root directory, simply run:
//!
//! ```bash
//! cargo add RustQuant
//! ```
//!
//! This will add the latest version to your project.
//!
//! If you require a specific version, add the following to your Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! RustQuant = "*"
//! ```
//!
//! replacing `"*"` with the version number you require, such as `"0.0.17"`.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GLOBAL SETTINGS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Strictly enforce documentation.
#![forbid(missing_docs)]
//
// When writing mathematical equations in documentation, Clippy suggests to
// put backticks inside the LaTeX block. This suppresses that behavior.
#![allow(clippy::doc_markdown)]
//
// Allow snake case.
// This is because much of this library is based on mathematics, so I
// want to adhere to the standard mathematical notation.
#![allow(non_snake_case)]
//
// Strictly enforce SAFETY comments.
// There is no unsafe code currently, but for anyone to add any, it must be
// documented with a SAFETY comment.
#![forbid(clippy::undocumented_unsafe_blocks)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RUSTQUANT MODULES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub mod autodiff;
pub mod data;
pub mod error;
pub mod instruments;
pub mod iso;
#[macro_use]
pub mod macros;
pub mod cashflows;
pub mod math;
pub mod ml;
pub mod models;
pub mod portfolio;
pub mod pricing;
pub mod stochastics;
pub mod time;
pub mod trading;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Epsilon constant for use in testing.
// It is set to: f64::sqrt(f64::EPSILON)
// Once `f64::sqrt()` is `const`, this can be updated.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Epsilon constant for use in testing.
pub const RUSTQUANT_EPSILON: f64 = 0.000_000_014_901_161_193_847_656;
