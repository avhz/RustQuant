// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Cashflows module.s

use time::OffsetDateTime;

/// Cashflow trait.
pub trait Cashflow {
    /// Amount of the cashflow.
    fn amount(&self) -> f64;
    /// Date of the cashflow.
    fn date(&self) -> OffsetDateTime;
    /// Net present value (NPV) of the cashflow.
    fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64;
}

/// Simple cashflow type.
pub struct SimpleCashflow {
    amount: f64,
    date: OffsetDateTime,
}

impl SimpleCashflow {
    /// Create a new simple cashflow.
    pub fn new(amount: f64, date: OffsetDateTime) -> Self {
        SimpleCashflow { amount, date }
    }
}

impl Cashflow for SimpleCashflow {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn date(&self) -> OffsetDateTime {
        self.date
    }

    fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64,
    {
        self.amount * df(self.date)
    }
}
