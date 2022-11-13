//! RustQuant: A Rust library for quantitative finance tools.

#![allow(non_snake_case)]
// #![deny(missing_docs)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Mathematics and statistics modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod Math_Interpolation;
mod Math_Monte_Carlo;
mod Math_Normal_Distribution;
mod Math_Risk_Reward;

pub use Math_Interpolation::*;
pub use Math_Monte_Carlo::*;
pub use Math_Normal_Distribution::*;
pub use Math_Risk_Reward::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Automatic Differentiation modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod AD_Chain;
mod AD_Tape;

pub use AD_Chain::*;
pub use AD_Tape::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Option pricing modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod Options_American;
mod Options_Barrier;
mod Options_Binomial;
mod Options_European;
mod Options_Greeks;

pub use Options_American::*;
pub use Options_Barrier::*;
pub use Options_Binomial::*;
pub use Options_European::*;
pub use Options_Greeks::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Miscellaneous modules:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod helpers;

pub use helpers::*;
