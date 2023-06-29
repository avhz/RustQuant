// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{order::Order, order_side::Side};
use std::collections::VecDeque;
// use time::OffsetDateTime;

/// Orderbook struct containing the two 'half-books' (bid and ask sides).
///
/// The half-books are double-ended queues,
/// one for the bids (buy orders) and one for the asks (sell orders).
///
/// VecDeque<T> is not the most efficient choice, but it is convenient
/// since we can push/pop from both the front and back easily.
#[derive(Debug, Clone)]
pub struct OrderBook {
    /// Orderbook bid (buy) side.
    pub bids: VecDeque<Order>,

    /// Orderbook ask (sell) side.
    pub asks: VecDeque<Order>,
}

impl OrderBook {
    /// New `OrderBook` instance.
    pub fn new() -> Self {
        Self {
            bids: VecDeque::new(),
            asks: VecDeque::new(),
        }
    }

    /// Insert an `Order` into an existing `OrderBook`.
    pub fn insert_order(&mut self, order: Order) {
        match order.side {
            Side::BID => self.bids.push_back(order),
            Side::ASK => self.asks.push_back(order),
        }
    }

    /// Check if `OrderBook` is empty.
    pub fn is_empty(&self) -> bool {
        self.bids.is_empty() && self.asks.is_empty()
    }

    /// Get the size of the `OrderBook`.
    pub fn len(&self) -> usize {
        self.bids.len() + self.asks.len()
    }

    // /// Get the best bid price.
    // pub fn best_bid(&self) -> Option<f64> {
    //     self.bids.front().map(|order| order.price)
    // }

    // /// Get the best ask price.
    // pub fn best_ask(&self) -> Option<f64> {
    //     self.asks.front().map(|order| order.price)
    // }

    // /// Cancel an `Order` within the `OrderBook`.
    // pub fn cancel_order(&mut self, id: Order::ID) {}

    // /// Amend an `Order` within the `OrderBook`.
    // pub fn amend_order(&mut self, id: Order::ID, price: f64, volume: i32) {}

    // /// Match orders within an existing `OrderBook`.
    // pub fn match_orders(&mut self) {
    //     while let (Some(bid), Some(ask)) = (self.bids.pop_front(), self.asks.pop_front()) {
    //         // Match order when bid > ask.
    //         if bid.price >= ask.price {
    //             // Get quantity.
    //             let quantity = std::cmp::min(bid.quantity, ask.quantity);

    //             // Calculate bid-ask midprice.
    //             let price = 0.5 * (bid.price + ask.price);

    //             self.bids.push_front(Order {
    //                 ID: bid.ID,
    //                 side: bid.side,
    //                 price,
    //                 quantity: bid.quantity - quantity,
    //                 timestamp: OffsetDateTime::now_utc(),
    //             });
    //             self.asks.push_front(Order {
    //                 ID: ask.ID,
    //                 side: ask.side,
    //                 price,
    //                 quantity: ask.quantity - quantity,
    //                 timestamp: OffsetDateTime::now_utc(),
    //             });
    //         }
    //         // If price is not right, add to `OrderBook`.
    //         else {
    //             self.bids.push_front(bid);
    //             self.asks.push_front(ask);
    //             break;
    //         }
    //     }
    // }
}
