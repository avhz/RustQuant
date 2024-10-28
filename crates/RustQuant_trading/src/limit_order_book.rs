// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod limit;
mod order;
mod test;

use limit::Limit;
use order::Order;
use std::{
    collections::{btree_map::BTreeMap, HashMap},
    fmt,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Limit order book
pub struct Book {
    buy_limits: BTreeMap<u64, Limit>,
    sell_limits: BTreeMap<u64, Limit>,
    order_map: HashMap<u64, Order>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ERRORS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Error for when the parameter id exists in the book
#[derive(Debug)]
pub struct ExistingIdError {
    id: u64,
}

impl ExistingIdError {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

impl fmt::Display for ExistingIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Book already contains id {}", self.id)
    }
}

/// Error for when parameter id does not exist in book and should
#[derive(Debug)]
pub struct NonExistingIdError {
    id: u64,
}

impl NonExistingIdError {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

impl fmt::Display for NonExistingIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Book does not contain id {}", self.id)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Book {
    /// Returns new empty book
    #[must_use]
    pub fn new() -> Self {
        Self {
            buy_limits: BTreeMap::new(),
            sell_limits: BTreeMap::new(),
            order_map: HashMap::new(),
        }
    }

    /// Adds order to Book
    /// `order_id` must be a unique id.
    /// `is_buy` order is buy or sell.
    /// `shares` number of shares in order.
    /// `limit_value` value of shares. Typically `$ x 1000` (NASDAQ default). e.g. $4.50 -> 45000
    /// `timestamp` timestamp of order.
    ///
    /// # Errors
    ///
    /// `ExistingIdError` when order book already contains order with `order_id`
    pub fn add_order(
        &mut self,
        order_id: u64,
        is_buy: bool,
        shares: u64,
        limit_value: u64,
        timestamp: u64,
    ) -> Result<(), ExistingIdError> {
        if self.order_map.contains_key(&order_id) {
            return Err(ExistingIdError::new(order_id));
        }

        let order = Order::new(order_id, is_buy, shares, limit_value, timestamp);
        self.order_map.insert(order_id, order);

        let limit_tree = if is_buy {
            &mut self.buy_limits
        } else {
            &mut self.sell_limits
        };

        if let Some(l) = limit_tree.get_mut(&limit_value) {
            l.add(order_id);
        } else {
            let mut limit = Limit::new(limit_value);
            limit.add(order_id);
            let _ = limit_tree.insert(limit_value, limit);
        };

        Ok(())
    }

    /// Cancels order in book
    /// `order_id` order id to cancel.
    ///
    /// # Errors
    ///
    /// `NonExistingIdError` when `order_id` is not found in the `order_map`
    ///
    /// # Panics
    ///
    /// Panics if limit is not in `limit_tree`.
    pub fn cancel_order(&mut self, order_id: u64) -> Result<(), NonExistingIdError> {
        match self.order_map.remove(&order_id) {
            Some(o) => {
                let limit_tree = if o.is_buy {
                    &mut self.buy_limits
                } else {
                    &mut self.sell_limits
                };

                let is_empty = match limit_tree.get_mut(&o.limit) {
                    Some(l) => l.cancel(o.order_id),
                    None => panic!(""),
                };

                if is_empty {
                    limit_tree.remove(&o.limit);
                }

                Ok(())
            }
            None => Err(NonExistingIdError::new(order_id)),
        }
    }

    /// Executes order at market value.
    /// `shares` number of shares to buy.
    /// `is_buy` buy or sell shares.
    /// Returns tuple:
    /// First item is a bool, if false, not enough volume to fulfill order. If true, bought all
    /// shares.
    /// Second item is vector of tuples. First item in tuple is price executed, second item is
    /// number of shares executed at price.
    pub fn execute_market_order(&mut self, shares: u64, is_buy: bool) -> (bool, Vec<(u64, u64)>) {
        let mut shares_left = shares;
        let mut result: Vec<(u64, u64)> = vec![];

        let limit_tree = if is_buy {
            &mut self.sell_limits
        } else {
            &mut self.buy_limits
        };

        while shares_left > 0 {
            let limit_key_value = if is_buy {
                limit_tree.values_mut().next()
            } else {
                limit_tree.values_mut().last()
            };

            let Some(limit) = limit_key_value else {
                return (false, result);
            };

            let (shares_executed, is_empty) = limit.execute(shares_left, &mut self.order_map);

            shares_left -= shares_executed;
            result.push((limit.limit_price, shares_executed));

            if is_empty {
                if is_buy {
                    limit_tree.pop_first();
                } else {
                    limit_tree.pop_last();
                }
            }
        }

        (true, result)
    }
}

impl Default for Book {
    fn default() -> Self {
        Self::new()
    }
}
