// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Submodule of cashflows for defining legs.
//! A leg is a sequence of cashflows.

use time::OffsetDateTime;

pub struct Leg {
    cashflows: Vec<Box<dyn Cashflow>>,
}

impl Leg {
    pub fn new(cashflows: Vec<Box<dyn Cashflow>>) -> Self {
        Leg { cashflows }
    }

    pub fn size(&self) -> usize {
        self.cashflows.len()
    }

    pub fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64,
    {
        self.cashflows.iter().map(|x| x.npv(&df)).sum()
    }

    pub fn start_date(&self) -> Option<OffsetDateTime> {
        self.cashflows.iter().map(|x| x.date()).min()
    }

    pub fn end_date(&self) -> Option<OffsetDateTime> {
        self.cashflows.iter().map(|x| x.date()).max()
    }

    pub fn is_active(&self, current_date: OffsetDateTime) -> bool {
        match (self.start_date(), self.end_date()) {
            (Some(start), Some(end)) => current_date >= start && current_date <= end,
            _ => false,
        }
    }
}
