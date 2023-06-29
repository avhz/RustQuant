// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt;

use super::{order_lifespan::OrderTimeInForce, order_side::OrderSide, order_type::OrderType};
use time::OffsetDateTime;

/// Order ID type.
pub type OrderID = u64;

/// Order struct containing parameters for a given order in the LOB.
#[derive(Debug, Clone, Copy)]
pub struct Order {
    // GENERAL INFORMATION
    /// Order Identifier
    pub id: OrderID,
    /// Symbol Identifier
    pub symbol_id: u32,
    /// Order type (limit, market, etc.)
    pub order_type: OrderType,
    /// Order side (bid or ask)
    pub order_side: OrderSide,

    // PRICE INFORMATION
    /// Order price
    pub price: f64,
    /// Order stop price
    pub stop_price: f64,

    // QUANTITY INFORMATION
    /// Order quantity
    pub quantity: u64,
    /// Order executed quantity
    pub executed_quantity: u64,
    /// Order leaves quantity
    pub leaves_quantity: u64,

    // TIME INFORMATION
    /// Time in Force (GTC, IOC, etc.)
    pub time_in_force: OrderTimeInForce,
    /// Order timestamp
    pub timestamp: OffsetDateTime,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order(id={}, symbol_id={}, order_type={}, order_side={}, price={}, stop_price={}, quantity={}, executed_quantity={}, leaves_quantity={}, time_in_force={}, timestamp={})",
            self.id,
            self.symbol_id,
            self.order_type,
            self.order_side,
            self.price,
            self.stop_price,
            self.quantity,
            self.executed_quantity,
            self.leaves_quantity,
            self.time_in_force,
            // self.max_visible_quantity,
            self.timestamp
        )
    }
}

// impl Eq for Order {}

// impl PartialEq for Order {
//     fn eq(&self, other: &Self) -> bool {
//         self.ID == other.ID
//     }
// }

// impl PartialOrd for Order {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.price.partial_cmp(&other.price)
//     }
// }

// impl Ord for Order {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.price
//             .partial_cmp(&other.price)
//             .unwrap_or(std::cmp::Ordering::Equal)
//     }
// }

impl Order {
    // /// Create a new `Order`.
    // pub fn new(ID: u64, side: Side, price: f64, quantity: u64, timestamp: OffsetDateTime) -> Self {
    //     assert!(ID > 0, "Order ID must be positive.");
    //     assert!(quantity > 0, "Order quantity must be positive.");

    //     Self {
    //         ID,
    //         side,
    //         price,
    //         quantity,
    //         timestamp,
    //     }
    // }

    /// Validate the `Order`.
    ///
    /// This should do the following:
    ///     - Check the ID is positive.
    ///     - Check the quantity is valid.
    ///     - Check the type and validate.
    pub fn validate(&self) {
        todo!()
    }

    /// Get the `Order` ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the `Order` symbol ID.
    pub fn symbol_id(&self) -> u32 {
        self.symbol_id
    }

    /// Get the `Order` type.
    pub fn order_type(&self) -> &OrderType {
        &self.order_type
    }

    /// Get the `Order` side.
    pub fn order_side(&self) -> &OrderSide {
        &self.order_side
    }

    /// Get the `Order` price.
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Get the `Order` stop price.
    pub fn stop_price(&self) -> f64 {
        self.stop_price
    }

    /// Get the `Order` quantity.
    pub fn quantity(&self) -> u64 {
        self.quantity
    }

    /// Get the `Order` executed quantity.
    pub fn executed_quantity(&self) -> u64 {
        self.executed_quantity
    }

    /// Get the `Order` leaves quantity.
    pub fn leaves_quantity(&self) -> u64 {
        self.leaves_quantity
    }

    /// Get the `Order` time in force.
    pub fn time_in_force(&self) -> &OrderTimeInForce {
        &self.time_in_force
    }

    // /// Get the `Order` maximum visible quantity.
    // pub fn max_visible_quantity(&self) -> u64 {
    //     self.max_visible_quantity
    // }

    /// Get the `Order` timestamp.
    pub fn timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }
}
