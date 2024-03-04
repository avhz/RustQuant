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
struct Surface {
    pub nodes: BTreeMap<f64, TermStructure>,
}

struct TermStructure {
    pub nodes: BTreeMap<Date, f64>,
    // /// Calendar.
    // pub calendar: C,

    // /// Day counting convention.
    // pub day_count_convention: DayCountConvention,

    // /// Date rolling convention.
    // pub date_rolling_convention: DateRollingConvention,
}

impl TermStructure {
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
