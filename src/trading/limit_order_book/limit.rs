// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::order::Order;
use std::collections::{HashMap, VecDeque};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[derive(Debug)]
pub struct Limit {
    pub limit_price: u64,
    orders: VecDeque<u64>,
}
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Limit {
    pub fn new(limit_price: u64) -> Self {
        Self {
            limit_price,
            orders: VecDeque::new(),
        }
    }

    pub fn add(&mut self, order_id: u64) {
        self.orders.push_back(order_id);
    }

    pub fn cancel(&mut self, order_id: &u64) -> bool {
        let index = self
            .orders
            .iter()
            .position(|id| id == order_id)
            .expect("This limit should have this id but doesn't");

        self.orders.remove(index);

        self.orders.is_empty()
    }

    pub fn execute(&mut self, shares: &u64, order_map: &mut HashMap<u64, Order>) -> (u64, bool) {
        let mut executed_shares = 0;

        while &executed_shares < shares && !self.orders.is_empty() {
            let order_id = self.orders.front().unwrap();
            let order_shares = order_map.get(order_id).unwrap().shares;

            if order_shares > shares - executed_shares {
                let order = order_map.get_mut(order_id).unwrap();
                order.shares -= shares - executed_shares;
                executed_shares += shares - executed_shares;
            } else {
                order_map.remove(order_id);
                self.orders.pop_front();
                executed_shares += order_shares;
            }
        }

        (executed_shares, self.orders.is_empty())
    }
}
