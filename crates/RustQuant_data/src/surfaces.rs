// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::Curve;
use crate::data::TermStructure;
use num::Float;
use std::collections::BTreeMap;
use time::Date;

/// Surface data structure.
pub struct Surface<S, C>
where
    S: CurveIndex,
    C: CurveIndex,
{
    /// The curves that make up the surface.
    pub nodes: BTreeMap<S, Curve<C>>,
}

/// Surface data.
#[allow(dead_code)] // never used
pub struct Surface {
    /// Nodes of the surface.
    pub nodes: BTreeMap<u64, TermStructure>,
}

#[allow(dead_code)]
impl Surface {
    /// Create a new surface.
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Add a term structure node to the surface.
    pub fn add_node(&mut self, time: u64, term_structure: TermStructure) {
        self.nodes.insert(time, term_structure);
    }

    /// Get a term structure for a specific time.
    pub fn get_term_structure(&self, time: u64) -> Option<&TermStructure> {
        self.nodes.get(&time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month};

    #[test]
    fn test_surface_creation() {
        let dates = vec![
            Date::from_calendar_date(2023, Month::January, 1).unwrap(),
            Date::from_calendar_date(2023, Month::February, 1).unwrap(),
            Date::from_calendar_date(2023, Month::March, 1).unwrap(),
        ];
        let rates = [0.05, 0.06, 0.07];

        let term_structure = TermStructure::new(&dates, &rates);

        let mut surface = Surface::new();
        surface.add_node(1, term_structure);

        assert_eq!(surface.nodes.len(), 1);
        assert!(surface.get_term_structure(1).is_some());

        let ts = surface.get_term_structure(1).unwrap();
        assert_eq!(ts.nodes.len(), 3);
        assert_eq!(
            ts.nodes
                .get(&Date::from_calendar_date(2023, Month::January, 1).unwrap()),
            Some(&0.05)
        );
        assert_eq!(
            ts.nodes
                .get(&Date::from_calendar_date(2023, Month::February, 1).unwrap()),
            Some(&0.06)
        );
        assert_eq!(
            ts.nodes
                .get(&Date::from_calendar_date(2023, Month::March, 1).unwrap()),
            Some(&0.07)
        );
    }
}
