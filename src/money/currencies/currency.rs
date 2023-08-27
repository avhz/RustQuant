// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains implementations of currencies and money.
//! The currencies are based on the ISO 4217 standard.
//! The `Money` struct is a combination of a currency and an amount.
//! Basic arithmetic operations can be performed  on `Money` instances with the
//! same underlying currency.

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

/// Money struct.
#[derive(Debug, Clone, Copy)]
pub struct Money {
    /// The underlying currency.
    pub currency: Currency,
    /// The amount.
    pub amount: f64,
}

/// ISO 4217 codes enum.
/// Format:
///     - First two letters are the ISO 3166-1 alpha-2 country code. e.g. US = United States
///     - Third letter is the first letter of the currency name. e.g. USD = United States Dollar
///     - The number is the ISO numeric code. e.g. 840 = USD
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct ISO_4217 {
    /// The ISO 4217 alphabetic code.
    pub alphabetic: &'static str,
    /// The ISO 4217 numeric code.
    pub numeric: &'static str,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Eq for Currency {}
impl Eq for Money {}
impl Eq for ISO_4217 {}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency.code == other.currency.code && self.amount == other.amount
    }
}

impl PartialEq for ISO_4217 {
    fn eq(&self, other: &Self) -> bool {
        self.alphabetic == other.alphabetic && self.numeric == other.numeric
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

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Currency:\t{}\nISO Code:\t{:?}", self.name, self.code)
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
    pub fn name(&self) -> &str {
        self.name
    }

    /// Get the currency symbol.
    pub fn symbol(&self) -> &str {
        self.symbol
    }

    /// Get the currency code.
    pub fn code(&self) -> ISO_4217 {
        self.code
    }

    /// Get the minor unit.
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Get the fractions per unit.
    pub fn fractions(&self) -> usize {
        self.fractions
    }
}

impl Money {
    /// Create a new money instance.
    pub fn new(currency: Currency, amount: f64) -> Self {
        Self { currency, amount }
    }

    /// Get the currency.
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Get the amount.
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl ISO_4217 {
    /// Create a new ISO 4217 code.
    pub fn new(alphabetic: &'static str, numeric: &'static str) -> Self {
        Self {
            alphabetic,
            numeric,
        }
    }

    /// Get the ISO 4217 alphabetic code.
    pub fn alphabetic(&self) -> &str {
        self.alphabetic
    }

    /// Get the ISO 4217 numeric code.
    pub fn numeric(&self) -> &str {
        self.numeric
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
    use crate::money::currencies::*;

    #[test]
    fn test_fmt() {
        println!("{}", USD);
    }
}
