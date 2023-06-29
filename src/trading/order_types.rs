// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{order::OrderID, order_side::Side};
use time::OffsetDateTime;

/// Order type enum.
/// Definitions from:
///     - https://www.investor.gov/introduction-investing/investing-basics/glossary
///     - https://www.nasdaqtrader.com/content/productsservices/trading/ordertypesg.pdf
#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    /// A market order is an order to buy or sell a stock at the
    /// urrent market price. Unless you specify otherwise, your broker
    /// will enter your order as a market order. The advantage of a market
    /// order is that as long as there are willing buyers and sellers,
    /// you are almost always guaranteed your order will be executed.
    /// The disadvantage is the price you pay when your order is
    /// executed may not be the price you expected.
    Market {
        /// Order ID number.
        id: OrderID,
        /// Order side (bid, ask).
        side: Side,
        /// Order volume.
        volume: usize,
        /// Order timestamp.
        timestamp: OffsetDateTime,
    },

    /// A limit order is an order to buy or sell a security at a specific price.
    /// A buy limit order can only be executed at the limit price or lower,
    /// and a sell limit order can only be executed at the limit price or higher.
    Limit {
        /// Order ID number.
        id: OrderID,
        /// Order side (bid, ask).
        side: Side,
        /// Order volume.
        volume: usize,
        /// Order price.
        price: f64,
        /// Order timestamp.
        timestamp: OffsetDateTime,
    },

    /// A stop order, also referred to as a stop-loss order.
    Stop,
    // StopLimit,

    // TrailingStop,

    // TrailingStopLimit,
}
