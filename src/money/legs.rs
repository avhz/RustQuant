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
use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Leg (sequence of cashflows).
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Leg<C: Cashflow> {
    cashflows: Vec<C>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<C: Cashflow> Leg<C> {
    /// Creates a new leg with given cashflows.
    pub fn new(cashflows: Vec<C>) -> Self {
        Self { cashflows }
    }

    /// Returns the number of cashflows in the leg.
    pub fn size(&self) -> usize {
        self.cashflows.len()
    }

    /// Returns the Net Present Value (NPV) of the leg given a discount function.
    pub fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64,
    {
        self.cashflows.iter().map(|cf| cf.npv(&df)).sum()
    }

    /// Adds a cashflow to the leg.
    pub fn add_cashflow(&mut self, cashflow: C) {
        self.cashflows.push(cashflow);
    }

    /// Returns a slice of all the cashflows in the leg.
    pub fn cashflows(&self) -> &[C] {
        &self.cashflows
    }

    /// Returns the start date of the leg.
    pub fn start_date(&self) -> Option<OffsetDateTime> {
        self.cashflows.iter().map(|x| x.date()).min()
    }

    /// Returns the end date of the leg.
    pub fn end_date(&self) -> Option<OffsetDateTime> {
        self.cashflows.iter().map(|x| x.date()).max()
    }

    /// Returns true if the leg is active at the given date.
    pub fn is_active(&self, current_date: OffsetDateTime) -> bool {
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
    use super::super::SimpleCashflow;
    use super::*;
    use time::Duration;

    // Utility function to generate a simple leg for testing.
    fn generate_simple_leg(now: OffsetDateTime) -> Leg<SimpleCashflow> {
        let cashflows = vec![
            SimpleCashflow::new(100.0, now),
            SimpleCashflow::new(200.0, now + Duration::days(30)),
            SimpleCashflow::new(300.0, now + Duration::days(60)),
        ];
        Leg::new(cashflows)
    }

    // Test to verify the `size` method.
    #[test]
    fn test_size() {
        let now = OffsetDateTime::now_utc();
        let leg = generate_simple_leg(now);
        assert_eq!(leg.size(), 3);
    }

    // Test to verify the `npv` method.
    #[test]
    fn test_npv() {
        let now = OffsetDateTime::now_utc();
        let leg = generate_simple_leg(now);

        // Discount function that reduces value by 10%.
        let df = |_| 0.9;
        assert_eq!(leg.npv(df), 540.0);
    }

    // Test to verify the `add_cashflow` method.
    #[test]
    fn test_add_cashflow() {
        let now = OffsetDateTime::now_utc();
        let mut leg = generate_simple_leg(now);
        let new_cashflow = SimpleCashflow::new(400.0, now + Duration::days(90));
        leg.add_cashflow(new_cashflow.clone());
        assert_eq!(leg.size(), 4);
        assert_eq!(
            leg.cashflows().last().unwrap().amount(),
            new_cashflow.amount()
        );
    }

    // Test to verify the `start_date` and `end_date` methods.
    #[test]
    fn test_start_end_date() {
        let now = OffsetDateTime::now_utc();
        let leg = generate_simple_leg(now);
        let start = leg.start_date().unwrap();
        let end = leg.end_date().unwrap();
        assert_eq!(start, now);
        assert_eq!(end, now + Duration::days(60));
    }

    // Test to verify the `is_active` method.
    #[test]
    fn test_is_active() {
        let now = OffsetDateTime::now_utc();
        let leg = generate_simple_leg(now);
        assert!(leg.is_active(now));
        assert!(leg.is_active(now + Duration::days(30)));
        assert!(leg.is_active(now + Duration::days(60)));
        assert!(!leg.is_active(now - Duration::days(1)));
        assert!(!leg.is_active(now + Duration::days(61)));
    }
}
