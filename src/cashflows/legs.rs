// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Submodule of cashflows for defining legs.
//! A leg is a sequence of cashflows.

use super::Cashflow;
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Leg (sequence of cashflows).
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Leg {
    cashflows: Vec<Cashflow>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Leg {
    /// Creates a new leg with given cashflows.
    pub fn new(cashflows: Vec<Cashflow>) -> Self {
        Self { cashflows }
    }

    /// Returns the number of cashflows in the leg.
    pub fn size(&self) -> usize {
        self.cashflows.len()
    }

    /// Returns the Net Present Value (NPV) of the leg given a discount function.
    pub fn npv(&self, discount_rate: f64) -> f64 {
        self.cashflows.iter().map(|cf| cf.npv(discount_rate)).sum()
    }

    /// Adds a cashflow to the leg.
    pub fn add_cashflow(&mut self, cashflow: Cashflow) {
        self.cashflows.push(cashflow);
    }

    /// Returns a slice of all the cashflows in the leg.
    pub fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    /// Returns the start date of the leg.
    pub fn start_date(&self) -> Option<Date> {
        self.cashflows.iter().map(Cashflow::date).min()
    }

    /// Returns the end date of the leg.
    pub fn end_date(&self) -> Option<Date> {
        self.cashflows.iter().map(Cashflow::date).max()
    }

    /// Returns true if the leg is active at the given date.
    pub fn is_active(&self, current_date: Date) -> bool {
        match (self.start_date(), self.end_date()) {
            (Some(start), Some(end)) => current_date >= start && current_date <= end,
            _ => false,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_legs {
    // use super::super::SimpleCashflow;
    use super::*;
    use crate::assert_approx_equal;
    use crate::time::today;
    use std::f64::EPSILON as EPS;
    use time::Duration;
    use time::OffsetDateTime;

    // Utility function to generate a simple leg for testing.
    fn generate_simple_leg(now: Date) -> Leg {
        let cashflows = vec![
            Cashflow::new(100.0, now),
            Cashflow::new(200.0, now + Duration::days(30)),
            Cashflow::new(300.0, now + Duration::days(60)),
        ];
        Leg::new(cashflows)
    }

    // Test to verify the `size` method.
    #[test]
    fn test_size() {
        let now = today();
        let leg = generate_simple_leg(now);
        assert_eq!(leg.size(), 3);
    }

    // Test to verify the `npv` method.
    #[test]
    fn test_npv() {
        let now = today();
        let leg = generate_simple_leg(now);

        // Discount function that reduces value by 10%.
        let df = 0.9;
        assert_approx_equal!(leg.npv(df), 540.0, EPS);
    }

    // Test to verify the `add_cashflow` method.
    #[test]
    fn test_add_cashflow() {
        let now = today();
        let mut leg = generate_simple_leg(now);
        let new_cashflow = Cashflow::new(400.0, now + Duration::days(90));
        leg.add_cashflow(new_cashflow.clone());
        assert_eq!(leg.size(), 4);
        assert_approx_equal!(
            leg.cashflows().last().unwrap().amount(),
            new_cashflow.amount(),
            EPS
        );
    }

    // Test to verify the `start_date` and `end_date` methods.
    #[test]
    fn test_start_end_date() {
        let now = today();
        let leg = generate_simple_leg(now);
        let start = leg.start_date().unwrap();
        let end = leg.end_date().unwrap();
        assert_eq!(start, now);
        assert_eq!(end, now + Duration::days(60));
    }

    // Test to verify the `is_active` method.
    #[test]
    fn test_is_active() {
        let now = today();
        let leg = generate_simple_leg(now);
        assert!(leg.is_active(now));
        assert!(leg.is_active(now + Duration::days(30)));
        assert!(leg.is_active(now + Duration::days(60)));
        assert!(!leg.is_active(now - Duration::days(1)));
        assert!(!leg.is_active(now + Duration::days(61)));
    }
}
