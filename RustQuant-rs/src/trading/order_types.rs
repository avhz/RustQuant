// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use super::order_side::Side;

// /// Order type enum.
// /// Definitions from:
// ///     - https://www.investor.gov/introduction-investing/investing-basics/glossary
// ///     - https://www.nasdaqtrader.com/content/productsservices/trading/ordertypesg.pdf
// pub enum OrderType {
//     /// Cancels an existing order.
//     CANCEL { id: usize },
//     /// Amends (edits) an existing order.
//     AMEND { id: usize },
//     /// A market order is an order to buy or sell a stock at the
//     /// urrent market price. Unless you specify otherwise, your broker
//     /// will enter your order as a market order. The advantage of a market
//     /// order is that as long as there are willing buyers and sellers,
//     /// you are almost always guaranteed your order will be executed.
//     /// The disadvantage is the price you pay when your order is
//     /// executed may not be the price you expected.
//     MARKET {
//         id: usize,
//         side: Side,
//         volume: usize,
//         timestamp: time::OffsetDateTime,
//     },
//     /// A limit order is an order to buy or sell a security at a specific price.
//     /// A buy limit order can only be executed at the limit price or lower,
//     /// and a sell limit order can only be executed at the limit price or higher.
//     LIMIT {
//         id: usize,
//         side: Side,
//         volume: usize,
//         price: usize,
//         timestamp: time::OffsetDateTime,
//     },

//     /// A Fill-Or-Kill order is an order to buy or sell a stock that
//     /// must be executed immediately in its entirety;
//     /// otherwise, the entire order will be cancelled
//     /// (i.e., no partial execution of the order is allowed).
//     FILL_OR_KILL {
//         id: usize,
//         side: Side,
//         volume: usize,
//         price: usize,
//         timestamp: time::OffsetDateTime,
//     },
//     /// An Immediate-Or-Cancel (IOC) order is an order to buy or sell
//     /// a stock that must be executed immediately.
//     /// Any portion of an IOC order that cannot be filled immediately will be cancelled.
//     IMMEDIATE_OR_CANCEL {
//         id: usize,
//         side: Side,
//         volume: usize,
//         price: usize,
//         timestamp: time::OffsetDateTime,
//     },
// }
