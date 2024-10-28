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

use super::currency::Currency;
use std::fmt::{self, Formatter};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Money struct.
#[derive(Debug, Clone, Copy)]
pub struct Money {
    /// The underlying currency.
    pub currency: Currency,
    /// The amount.
    pub amount: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Eq for Money {}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency.code == other.currency.code && self.amount == other.amount
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency == other.currency {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Amount:\t{}\nName:\t{}\nISO:\t{:?}",
            self.amount, self.currency.name, self.currency.code
        )
    }
}

impl Money {
    /// Create a new money instance.
    #[must_use]
    pub fn new(currency: Currency, amount: f64) -> Self {
        Self { currency, amount }
    }

    /// Get the currency.
    #[must_use]
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Get the amount.
    #[must_use]
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.currency == other.currency {
            Self {
                currency: self.currency,
                amount: self.amount + other.amount,
            }
        } else {
            panic!("Cannot add two different currencies.")
        }
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.currency == other.currency {
            Self {
                currency: self.currency,
                amount: self.amount - other.amount,
            }
        } else {
            panic!("Cannot subtract two different currencies.")
        }
    }
}

impl std::ops::Mul for Money {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.currency == other.currency {
            Self {
                currency: self.currency,
                amount: self.amount * other.amount,
            }
        } else {
            panic!("Cannot multiply two different currencies.")
        }
    }
}

impl std::ops::Div for Money {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.currency == other.currency {
            Self {
                currency: self.currency,
                amount: self.amount / other.amount,
            }
        } else {
            panic!("Cannot divide two different currencies.")
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_currencies {
    use super::*;
    use crate::fx::*;
    use std::f64::EPSILON as EPS;
    use RustQuant_utils::assert_approx_equal;

    // Setup some example currencies and money for testing
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
