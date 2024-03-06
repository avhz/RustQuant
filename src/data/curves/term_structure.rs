// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// use crate::time::{Calendar, DateRollingConvention, DayCountConvention};
use std::collections::BTreeMap;
use std::fmt::Display;
use time::Date;

/// Surface data.
pub struct Surface {
    /// Nodes of the surface.
    pub nodes: BTreeMap<f64, TermStructure>,
}

/// Term structure data.
pub struct TermStructure {
    /// Nodes of the term structure.
    pub nodes: BTreeMap<Date, f64>,
    // /// Calendar.
    // pub calendar: C,

    // /// Day counting convention.
    // pub day_count_convention: DayCountConvention,

    // /// Date rolling convention.
    // pub date_rolling_convention: DateRollingConvention,
}

impl TermStructure {
    /// Create a new term structure.
    pub fn new(x_values: &[Date], y_values: &[f64]) -> Self {
        Self {
            nodes: x_values
                .iter()
                .zip(y_values.iter())
                .map(|(&x, &y)| (x, y))
                .collect(),
        }
    }
}

impl Display for TermStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Term Structure")?;

        for (x, y) in &self.nodes {
            write!(f, "\t{}: {}", x, y)?;
        }

        Ok(())
    }
}
