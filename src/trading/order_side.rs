// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Enum to indicate which side of the book the order falls on.
///
/// `BID`: The side containing buy orders.
/// `ASK`: The side containing sell orders.
#[derive(Debug)]
pub enum Side {
    /// Bid (buy) side.
    BID,
    /// Ask (sell) side.
    ASK,
}

impl std::ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::BID => Side::ASK,
            Side::ASK => Side::BID,
        }
    }
}
