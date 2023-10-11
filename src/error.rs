// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Error type for RustQuant.
#[derive(Debug, thiserror::Error)]
pub enum RustQuantError {
    /// This error indicates that an problem occurred in the computation.
    #[error("Computation error: {text:?}")]
    ComputationError {
        /// Text to include in error message.
        text: String,
    },

    /// This error indicates an invalid parameter/argument was passed.
    #[error("Invalid parameter: {text:?}")]
    InvalidParameter {
        /// Text to include in error message.
        text: String,
    },

    /// This error indicates that a condition is violated.
    #[error("Condition violated: {text:?}")]
    ConditionViolated {
        /// Text to include in error message.
        text: String,
    },
}

/// Create a `RustQuantError` with the text to include in the output.
/// You would use it as follows:
/// `return Err(error!(ComputationError, "Linear Regression: Singular Value Decomposition failed."));`
#[macro_export]
macro_rules! error {
    ($error_type:ident, $msg:expr) => {
        $crate::RustQuant::$error_type {
            text: $msg.to_string(),
        }
        .into()
    };
}
