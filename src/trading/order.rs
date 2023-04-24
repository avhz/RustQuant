// use time::{self, OffsetDateTime};

use super::order_side::Side;

/// Order struct containing parameters for a given order in the LOB.
#[derive(Debug)]
pub struct Order {
    /// Order ID number.
    pub ID: u64,
    /// Order side (bid, ask).
    pub side: Side,
    /// Order price.
    pub price: f64,
    /// Order quantity.
    pub quantity: u64,
    /// Order timestamp.
    pub timestamp: time::OffsetDateTime,
}
