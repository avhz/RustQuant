// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use time::{self, OffsetDateTime};

// /// Order struct containing parameters for a given order in the LOB.
// #[derive(Debug)]
// pub struct Order {
//     /// Order ID number.
//     pub ID: u64,
//     /// Order side (bid, ask).
//     pub side: Side,
//     /// Order price.
//     pub price: f64,
//     /// Order quantity.
//     pub quantity: u64,
//     /// Order timestamp.
//     pub timestamp: time::OffsetDateTime,
// }

// /// Enum to indicate which side of the book the order falls on.
// ///
// /// `BID`: The side containing buy orders.
// /// `ASK`: The side containing sell orders.
// #[derive(Debug)]
// pub enum Side {
//     /// Bid (buy) side.
//     BID,
//     /// Ask (sell) side.
//     ASK,
// }

// /// Order type enum.
// pub enum OrderType {
//     /// Market order: submits at best current price for immediate execution.
//     MARKET {
//         id: usize,
//         side: Side,
//         volume: usize,
//         timestamp: time::OffsetDateTime,
//     },
//     /// Limit order: places order outside of the bid-ask spread.
//     LIMIT {
//         id: usize,
//         side: Side,
//         volume: usize,
//         price: usize,
//         timestamp: time::OffsetDateTime,
//     },
//     /// Cancels an existing order.
//     CANCEL { id: usize },
//     /// Fill as much as possible, kill the remaining unfilled volume.
//     FILL_OR_KILL {
//         id: usize,
//         side: Side,
//         volume: usize,
//         price: usize,
//         timestamp: time::OffsetDateTime,
//     },
//     /// Only executes if entire order can be filled, otherwise killed.
//     IMMEDIATE_OR_CANCEL {
//         id: usize,
//         side: Side,
//         volume: usize,
//         price: usize,
//         timestamp: time::OffsetDateTime,
//     },
// }

// use time::OffsetDateTime;

// use super::{order::Order, order_side::Side};

// /// Orderbook struct containing the two 'half-books' (bid and ask sides).
// #[derive(Debug)]
// pub struct OrderBook {
//     /// Orderbook bid (buy) side.
//     pub bids: std::collections::VecDeque<Order>,
//     /// Orderbook ask (sell) side.
//     pub asks: std::collections::VecDeque<Order>,
// }

// impl OrderBook {
//     /// New `OrderBook` instance.
//     pub fn new() -> Self {
//         Self {
//             bids: std::collections::VecDeque::new(),
//             asks: std::collections::VecDeque::new(),
//         }
//     }

//     /// Insert an `Order` into an existing `OrderBook`.
//     pub fn insert_order(&mut self, order: Order) {
//         match order.side {
//             Side::BID => self.bids.push_back(order),
//             Side::ASK => self.asks.push_back(order),
//         }
//     }

//     // /// Cancel an `Order` within the `OrderBook`.
//     // pub fn cancel_order(&mut self, id: Order::ID) {}
//     // /// Amend an `Order` within the `OrderBook`.
//     // pub fn amend_order(&mut self, id: Order::ID, price: f64, volume: i32) {}

//     /// Match orders within an existing `OrderBook`.
//     pub fn match_orders(&mut self) {
//         while let (Some(bid), Some(ask)) = (self.bids.pop_front(), self.asks.pop_front()) {
//             // Match order when bid > ask.
//             if bid.price >= ask.price {
//                 // Get quantity.
//                 let quantity = std::cmp::min(bid.quantity, ask.quantity);

//                 // Calculate bid-ask midprice.
//                 let price = 0.5 * (bid.price + ask.price);

//                 self.bids.push_front(Order {
//                     ID: bid.ID,
//                     side: bid.side,
//                     price,
//                     quantity: bid.quantity - quantity,
//                     timestamp: OffsetDateTime::now_utc(),
//                 });
//                 self.asks.push_front(Order {
//                     ID: ask.ID,
//                     side: ask.side,
//                     price,
//                     quantity: ask.quantity - quantity,
//                     timestamp: OffsetDateTime::now_utc(),
//                 });
//             }
//             // If price is not right, add to `OrderBook`.
//             else {
//                 self.bids.push_front(bid);
//                 self.asks.push_front(ask);
//                 break;
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_limit_orderbook() {
//         let mut LOB = OrderBook::new();

//         println!("Adding orders:");

//         LOB.insert_order(Order {
//             ID: 1,
//             side: Side::BID,
//             price: 100.0,
//             quantity: 10,
//             timestamp: OffsetDateTime::now_utc(),
//         });
//         LOB.insert_order(Order {
//             ID: 2,
//             side: Side::ASK,
//             price: 90.0,
//             quantity: 5,
//             timestamp: OffsetDateTime::now_utc(),
//         });
//         LOB.insert_order(Order {
//             ID: 3,
//             side: Side::ASK,
//             price: 95.0,
//             quantity: 5,
//             timestamp: OffsetDateTime::now_utc(),
//         });
//         LOB.insert_order(Order {
//             ID: 4,
//             side: Side::BID,
//             price: 99.0,
//             quantity: 10,
//             timestamp: OffsetDateTime::now_utc(),
//         });
//         LOB.insert_order(Order {
//             ID: 5,
//             side: Side::ASK,
//             price: 98.0,
//             quantity: 5,
//             timestamp: OffsetDateTime::now_utc(),
//         });

//         println!("Matching orders:");
//         LOB.match_orders();

//         println!("Bids: {:?}", LOB.bids);
//         println!("Asks: {:?}", LOB.asks);

//         assert!(1 == 0);
//     }
// }
