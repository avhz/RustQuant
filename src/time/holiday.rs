// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines a `Holiday` type and its methods.

use time::Date;

/// Holiday type.
#[derive(Debug, Clone)]
pub struct Holiday {
    name: &'static str,
    date: Date,
    description: Option<&'static str>,
}

impl Eq for Holiday {}

impl PartialEq for Holiday {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.name == other.name
    }
}

impl Ord for Holiday {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Holiday {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Holiday {
    /// Create a new holiday.
    pub const fn new(name: &'static str, date: Date) -> Self {
        Self {
            name,
            date,
            description: None,
        }
    }

    /// Create a new holiday with a description.
    pub fn new_with_description(name: &'static str, date: Date, description: &'static str) -> Self {
        Self {
            name,
            date,
            description: Some(description),
        }
    }

    /// Get the name of the holiday.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the date of the holiday.
    pub fn date(&self) -> Date {
        self.date
    }

    /// Get the description of the holiday.
    pub fn description(&self) -> Option<&'static str> {
        self.description
    }
}
