// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Submodule of cashflows for defining legs.
//! A leg is a sequence of cashflows.

/// Leg trait.
pub trait Leg {
    fn size(&self) -> usize;
    fn npv(&self) -> f64;
    fn start_date(&self) -> OffsetDateTime;
    fn end_date(&self) -> OffsetDateTime;
    fn maturity_date(&self) -> OffsetDateTime;
    fn is_active(&self) -> bool;
    fn is_inactive(&self) -> bool;
}

pub struct SimpleLeg {
    cashflows: Vec<SimpleCashflow>,
}
