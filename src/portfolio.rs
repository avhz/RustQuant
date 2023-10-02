// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Portfolio module.
//! A portfolio is a collection of [Position]s, which are simply a combination
//! of an [Instrument], a quantity, a purchase price, and a current price.
//! You may also specify the [Currency] of the instrument.
//!
//! # Example
//!
//! ```
//! # use RustQuant::portfolio::{Portfolio, Position};
//! # use RustQuant::instruments::options::{BlackScholesMerton, TypeFlag};
//! # use RustQuant::money::USD;
//! # use time::{Duration, OffsetDateTime};
//! # use std::collections::HashMap;
//! # use RustQuant::assert_approx_equal;
//!
//! // Create a position of 100 call options.
//! let position_1 = Position {
//!    instrument: BlackScholesMerton::new(
//!         0.08,
//!         60.0,
//!         65.0,
//!         0.3,
//!         0.08,
//!         None,
//!         OffsetDateTime::now_utc() + Duration::days(91),
//!         TypeFlag::Call,
//!     ),
//!     quantity: 100,
//!     purchase_price: 2.1045,
//!     current_price: 3.5,
//!     currency: Some(USD),
//! };
//!
//! // Create a position of 100 put options.
//! let position_2 = Position {
//!    instrument: BlackScholesMerton::new(
//!         0.1 - 0.05,
//!         100.0,
//!         95.0,
//!         0.2,
//!         0.1,
//!         None,
//!         OffsetDateTime::now_utc() + Duration::days(182),
//!         TypeFlag::Put,
//!     ),
//!     quantity: 100,
//!     purchase_price: 2.4524,
//!     current_price: 2.0,
//!     currency: Some(USD),
//! };
//!
//! let positions = HashMap::from([
//!     ("Call Options".to_string(), position_1),
//!     ("Put Options".to_string(), position_2),
//! ]);
//!
//! // Create a portfolio.
//! let portfolio = Portfolio::new(positions);
//!     
//! // Check the value of the portfolio.
//! assert_approx_equal!(portfolio.value(), 100.0 * 3.5 + 100.0 * 2.0, 1e-10);
//!     
//! // Check the profit of the portfolio.
//! assert_approx_equal!(portfolio.profit(), 550.0 - portfolio.cost(), 1e-10);
//! ```

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{instruments::Instrument, money::Currency};
use std::collections::HashMap;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Portfolio type. Simply a collection of positions.
pub struct Portfolio<I: Instrument> {
    /// HashMap of positions.
    pub positions: HashMap<String, Position<I>>,
}

/// Position type.
pub struct Position<I: Instrument> {
    /// Instrument.
    pub instrument: I,

    /// Quantity.
    pub quantity: u64,

    /// Purchase price of the instrument (per unit).
    pub purchase_price: f64,

    /// Current price of the instrument (per unit).
    pub current_price: f64,

    /// Currency of the instrument.
    pub currency: Option<Currency>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<I> Position<I>
where
    I: Instrument,
{
    /// Create a new position.
    pub fn new(
        instrument: I,
        quantity: u64,
        purchase_price: f64,
        current_price: f64,
        currency: Option<Currency>,
    ) -> Self {
        Self {
            instrument,
            quantity,
            purchase_price,
            current_price,
            currency,
        }
    }

    /// Returns the value of the position.
    pub fn value(&self) -> f64 {
        self.quantity as f64 * self.current_price
    }

    /// Returns the profit (or loss) of the position.
    pub fn profit(&self) -> f64 {
        self.value() - self.quantity as f64 * self.purchase_price
    }

    /// Update the price of the position.
    pub fn update_price(&mut self, new_price: f64) {
        self.current_price = new_price;
    }

    /// Update the quantity of the position.
    pub fn update_quantity(&mut self, new_quantity: u64) {
        self.quantity = new_quantity;
    }
}

impl<I> Portfolio<I>
where
    I: Instrument,
{
    /// Create a new portfolio.
    pub fn new(positions: HashMap<String, Position<I>>) -> Self {
        Self { positions }
    }

    /// Returns the value of the portfolio.
    pub fn value(&self) -> f64 {
        self.positions
            .values()
            .map(|position| position.value())
            .sum()
    }

    /// Returns the cost of the portfolio.
    pub fn cost(&self) -> f64 {
        self.positions
            .values()
            .map(|position| position.quantity as f64 * position.purchase_price)
            .sum()
    }

    /// Returns the profit (or loss) of the portfolio.
    pub fn profit(&self) -> f64 {
        self.positions
            .values()
            .map(|position| position.profit())
            .sum()
    }

    /// Update the price of a position in the portfolio.
    pub fn update_price(&mut self, instrument_name: &str, new_price: f64) {
        self.positions
            .get_mut(instrument_name)
            .unwrap()
            .update_price(new_price);
    }

    /// Update the quantity of a position in the portfolio.
    pub fn update_quantity(&mut self, instrument_name: &str, new_quantity: u64) {
        self.positions
            .get_mut(instrument_name)
            .unwrap()
            .update_quantity(new_quantity);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_portfolio {
    use super::*;
    use crate::{
        instruments::options::{BlackScholesMerton, TypeFlag},
        money::USD,
    };
    use time::{Duration, OffsetDateTime};

    #[test]
    fn test_portfolio() {
        // Create a position of 100 call options.
        let position_1 = Position {
            instrument: BlackScholesMerton::new(
                0.08,
                60.0,
                65.0,
                0.3,
                0.08,
                None,
                OffsetDateTime::now_utc() + Duration::days(91),
                TypeFlag::Call,
            ),
            quantity: 100,
            purchase_price: 2.1045,
            current_price: 3.5,
            currency: Some(USD),
        };

        // Create a position of 100 put options.
        let position_2 = Position {
            instrument: BlackScholesMerton::new(
                0.1 - 0.05,
                100.0,
                95.0,
                0.2,
                0.1,
                None,
                OffsetDateTime::now_utc() + Duration::days(182),
                TypeFlag::Put,
            ),
            quantity: 100,
            purchase_price: 2.4524,
            current_price: 2.0,
            currency: Some(USD),
        };

        let positions = HashMap::from([
            ("Call Options".to_string(), position_1),
            ("Put Options".to_string(), position_2),
        ]);

        // Create a portfolio.
        let portfolio = Portfolio::new(positions);

        // Check the value of the portfolio.
        assert_approx_equal!(portfolio.value(), 100.0 * 3.5 + 100.0 * 2.0, 1e-10);

        // Check the profit of the portfolio.
        assert_approx_equal!(portfolio.profit(), 550.0 - portfolio.cost(), 1e-10);
    }
}
