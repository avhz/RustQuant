// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Cashflows module.

use time::OffsetDateTime;

/// Cashflow trait.
pub trait Cashflow {
    fn amount(&self) -> f64;
    fn date(&self) -> OffsetDateTime;
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

/// Cashflows type.
pub struct Cashflows {
    cashflows: Vec<Box<dyn Cashflow>>,
}

impl Cashflows {
    pub fn new(cashflows: Vec<Box<dyn Cashflow>>) -> Self {
        Cashflows { cashflows }
    }

    pub fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64,
    {
        self.cashflows.iter().map(|x| x.npv(&df)).sum()
    }
}
