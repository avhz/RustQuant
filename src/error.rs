// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::{error::Error, fmt};

// pub enum ErrorType {}

/// Result type for RustQuant.
pub enum RustQuantResult<T, E> {
    /// Ok result.
    Ok(T),
    /// Error result.
    RustQuantErr(E),
}

/// Error type for RustQuant.
#[derive(Debug, Clone)]
pub struct RustQuantError {
    /// Error message.
    pub message: String,
    // pub type: ErrorType,
}

impl RustQuantError {
    /// Create a new RustQuant error.
    pub fn new(message: &str) -> RustQuantError {
        RustQuantError {
            message: message.to_string(),
            // type: ErrorType::new(),
        }
    }
}

impl Error for RustQuantError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for RustQuantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// impl fmt::Debug for RustQuantError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "Error: {}", self.message)
//         write!(f, "{}", self.message)
//     }
// }
