// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Cashflows module.

/// Cashflow trait.
pub trait Cashflow {
    fn amount(&self) -> f64;
    fn date(&self) -> f64;
    fn npv(&self, df: f64) -> f64;
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

    fn npv(&self, df: f64) -> f64 {
        self.amount * df
    }
}
