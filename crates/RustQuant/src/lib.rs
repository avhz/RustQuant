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

/// The RustQuant prelude.
pub mod prelude {
    pub use RustQuant_autodiff::*;
    pub use RustQuant_cashflows::*;
    pub use RustQuant_data::*;
    pub use RustQuant_error::*;
    pub use RustQuant_instruments::*;
    pub use RustQuant_iso::*;
    pub use RustQuant_math::*;
    pub use RustQuant_ml::*;
    pub use RustQuant_portfolios::*;
    pub use RustQuant_stochastics::*;
    pub use RustQuant_time::*;
    pub use RustQuant_trading::*;
    pub use RustQuant_utils::*;
}

/// The `autodiff` module.
pub mod autodiff {
    pub use RustQuant_autodiff::*;
}

/// The `cashflows` module.
pub mod cashflows {
    pub use RustQuant_cashflows::*;
}

/// The `data` module.
pub mod data {
    pub use RustQuant_data::*;
}

/// The `error` module.
pub mod error {
    pub use RustQuant_error::*;
}

/// The `instruments` module.
pub mod instruments {
    pub use RustQuant_instruments::*;
}

/// The `iso` module.
pub mod iso {
    pub use RustQuant_iso::*;
}

/// The `math` module.
pub mod math {
    pub use RustQuant_math::*;
}

/// The `ml` module.
pub mod ml {
    pub use RustQuant_ml::*;
}

/// The `portfolios` module.
pub mod portfolios {
    pub use RustQuant_portfolios::*;
}

/// The `stochastics` module.
pub mod stochastics {
    pub use RustQuant_stochastics::*;
}

/// The `time` module.
pub mod time {
    pub use RustQuant_time::*;
}

/// The `trading` module.
pub mod trading {
    pub use RustQuant_trading::*;
}

/// The `utils` module.
#[macro_use]
pub mod utils {
    pub use RustQuant_utils::*;
}
