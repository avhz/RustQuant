// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{order_side::Side, order_types::OrderType};
use time::OffsetDateTime;

/// Order ID type.
pub type OrderID = u64;

/// Order struct containing parameters for a given order in the LOB.
#[derive(Debug, Clone, Copy)]
pub struct Order {
    /// Order ID number.
    pub ID: OrderID,

    /// Order side (bid, ask).
    pub side: Side,

    /// Order type (limit, market, etc.).
    pub order_type: OrderType,
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

// impl Order {
//     /// Create a new `Order`.
//     pub fn new(ID: u64, side: Side, price: f64, quantity: u64, timestamp: OffsetDateTime) -> Self {
//         assert!(ID > 0, "Order ID must be positive.");
//         assert!(quantity > 0, "Order quantity must be positive.");

//         Self {
//             ID,
//             side,
//             price,
//             quantity,
//             timestamp,
//         }
//     }

//     /// Validate the `Order`.
//     ///
//     /// This should do the following:
//     ///     - Check the ID is positive.
//     ///     - Check the quantity is valid.
//     ///     - Check the type and validate.
//     pub fn validate(&self) {
//         todo!()
//     }
// }
