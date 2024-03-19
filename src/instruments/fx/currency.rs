// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains implementations of currencies and money.
//! The currencies are based on the ISO 4217 standard.
//! The `Money` struct is a combination of a currency and an amount.
//! Basic arithmetic operations can be performed  on `Money` instances with the
//! same underlying currency.

use crate::iso::ISO_4217;
use crate::{instruments::Instrument, time::today};
use std::fmt::{self, Formatter};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Currency data struct.
#[derive(Debug, Clone, Copy)]
pub struct Currency {
    /// Currency name. e.g. United States Dollar
    pub name: &'static str,
    /// Currency symbol. e.g. $
    pub symbol: &'static str,
    /// ISO 4217 currency code. e.g. USD = 840
    pub code: ISO_4217,
    /// Minor unit: digits after decimal separator. Usually D = 2.
    pub minor: usize,
    /// Fractions per unit. e.g. 100 cents = 1 dollar.
    pub fractions: usize,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Instrument for Currency {
    fn price(&self) -> f64 {
        1.0
    }

    fn error(&self) -> Option<f64> {
        None
    }

    fn valuation_date(&self) -> time::Date {
        today()
    }

    fn instrument_type(&self) -> &'static str {
        self.name
    }
}

impl Eq for Currency {}
impl Eq for ISO_4217 {}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl PartialEq for ISO_4217 {
    fn eq(&self, other: &Self) -> bool {
        self.alphabetic == other.alphabetic && self.numeric == other.numeric
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Currency:\t{}\nISO Code:\t{:?}", self.name, self.code)
    }
}

impl fmt::Display for ISO_4217 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Alphabetic: {}, Numeric: {}",
            self.alphabetic, self.numeric
        )
    }
}

impl Currency {
    /// Create a new currency.
    #[must_use]
    pub fn new(
        name: &'static str,
        symbol: &'static str,
        code: ISO_4217,
        minor: usize,
        fractions: usize,
    ) -> Self {
        Self {
            name,
            symbol,
            code,
            minor,
            fractions,
        }
    }

    /// Get the currency name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the currency symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.symbol
    }

    /// Get the currency code.
    #[must_use]
    pub fn code(&self) -> ISO_4217 {
        self.code
    }

    /// Get the minor unit.
    #[must_use]
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Get the fractions per unit.
    #[must_use]
    pub fn fractions(&self) -> usize {
        self.fractions
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_currencies {
    use super::*;
    use crate::assert_approx_equal;
    use crate::instruments::fx::money::Money;
    use std::f64::EPSILON as EPS;

    // Setup some example currencies and money for testing
    const USD: Currency = Currency {
        name: "United States Dollar",
        symbol: "$",
        code: ISO_4217 {
            alphabetic: "USD",
            numeric: "840",
        },
        minor: 2,
        fractions: 100,
    };

    const EUR: Currency = Currency {
        name: "Euro",
        symbol: "€",
        code: ISO_4217 {
            alphabetic: "EUR",
            numeric: "978",
        },
        minor: 2,
        fractions: 100,
    };

    #[test]
    fn test_currency_display() {
        let display_output = format!("{}", USD);
        assert!(display_output.contains("United States Dollar"));
        assert!(display_output.contains("USD"));
        assert!(display_output.contains("840"));
    }

    #[test]
    fn test_money_display() {
        let money = Money::new(USD, 20.5);
        let display_output = format!("{}", money);
        assert!(display_output.contains("20.5"));
        assert!(display_output.contains("United States Dollar"));
        assert!(display_output.contains("USD"));
    }

    #[test]
    fn test_iso_4217_display() {
        let display_output = format!("{}", USD.code);
        assert!(display_output.contains("USD"));
        assert!(display_output.contains("840"));
    }

    #[test]
    fn test_currency_equality() {
        let another_usd = USD;
        let euro = EUR;
        assert_eq!(USD, another_usd);
        assert_ne!(USD, euro);
    }

    #[test]
    fn test_money_equality() {
        let money1 = Money::new(USD, 20.5);
        let money2 = Money::new(USD, 20.5);
        let money3 = Money::new(USD, 10.5);
        assert_eq!(money1, money2);
        assert_ne!(money1, money3);
    }

    #[test]
    fn test_money_addition() {
        let money1 = Money::new(USD, 20.5);
        let money2 = Money::new(USD, 10.5);
        let result = money1 + money2;
        assert_approx_equal!(result.amount(), 31.0, EPS);
    }

    #[test]
    #[should_panic(expected = "Cannot add two different currencies.")]
    fn test_money_addition_different_currencies() {
        let money1 = Money::new(USD, 20.5);
        let money2 = Money::new(EUR, 10.5);
        let _ = money1 + money2;
    }

    #[test]
    fn test_money_subtraction() {
        let money1 = Money::new(USD, 20.5);
        let money2 = Money::new(USD, 10.5);
        let result = money1 - money2;
        assert_approx_equal!(result.amount(), 10.0, EPS);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract two different currencies.")]
    fn test_money_subtraction_different_currencies() {
        let money1 = Money::new(USD, 20.5);
        let money2 = Money::new(EUR, 10.5);
        let _ = money1 - money2;
    }

    #[test]
    fn test_money_multiplication() {
        let money1 = Money::new(USD, 20.0);
        let money2 = Money::new(USD, 2.0);
        let result = money1 * money2;
        assert_approx_equal!(result.amount(), 40.0, EPS);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply two different currencies.")]
    fn test_money_multiplication_different_currencies() {
        let money1 = Money::new(USD, 20.5);
        let money2 = Money::new(EUR, 2.5);
        let _ = money1 * money2;
    }

    #[test]
    fn test_money_division() {
        let money1 = Money::new(USD, 40.0);
        let money2 = Money::new(USD, 2.0);
        let result = money1 / money2;
        assert_approx_equal!(result.amount(), 20.0, EPS);
    }

    #[test]
    #[should_panic(expected = "Cannot divide two different currencies.")]
    fn test_money_division_different_currencies() {
        let money1 = Money::new(USD, 40.0);
        let money2 = Money::new(EUR, 2.0);
        let _ = money1 / money2;
    }
}
