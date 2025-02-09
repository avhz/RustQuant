use std::f64::consts::PI;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use crate::Initial;

/// A symbolic expression.
#[derive(Debug, Clone, Copy)]
pub enum Expression {
    /// A constant real value.
    Constant(f64),
    /// A variable.
    Variable(&'static str),
    /// A function applied to an expression, where the positive integer is the index of the expression on the graph.
    Unary(&'static str, usize),
    /// A function applied to two expressions.
    Binary(&'static str, usize, usize),
}

impl Initial for Expression {
    fn zero() -> Self {
        Self::Constant(0.0)
    }
}
