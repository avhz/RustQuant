// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt;

/// Enum to indicate which side of the book the order falls on.
///
/// `BID`: The side containing buy orders.
/// `ASK`: The side containing sell orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    /// Bid (buy) side.
    BID,
    /// Ask (sell) side.
    ASK,
}

impl std::ops::Not for OrderSide {
    type Output = OrderSide;

    fn not(self) -> Self::Output {
        match self {
            OrderSide::BID => OrderSide::ASK,
            OrderSide::ASK => OrderSide::BID,
        }
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::BID => write!(f, "BID (buy)"),
            OrderSide::ASK => write!(f, "ASK (sell)"),
        }
    }
}
