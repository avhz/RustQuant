// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt::{self, Formatter};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// A struct that wraps constants and functions into a single type in order
/// to allow for all processes to have time-dependent parameters.
pub struct ModelParameter(pub Box<dyn Fn(f64) -> f64 + Send + Sync>);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl fmt::Debug for ModelParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TimeDependent")
    }
}

impl From<f64> for ModelParameter {
    fn from(x: f64) -> Self {
        Self(Box::new(move |_| x))
    }
}

impl<F> From<F> for ModelParameter
where
    F: Fn(f64) -> f64 + 'static + Send + Sync,
{
    fn from(func: F) -> Self {
        Self(Box::new(func))
    }
}

// impl Ord for ModelParameter {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

// impl PartialOrd for ModelParameter {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         let x = self.0(0.0);
//         let y = other.0(0.0);
//         x.partial_cmp(&y)
//     }
// }
