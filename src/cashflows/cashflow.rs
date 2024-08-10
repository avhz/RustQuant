// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Cashflows module.

use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Cashflow type.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cashflow {
    /// Amount of the cashflow.
    pub amount: f64,

    /// Date of the cashflow.
    pub date: Date,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Cashflow {
    /// Create a new simple cashflow.
    pub fn new(amount: f64, date: Date) -> Self {
        Self { amount, date }
    }

    /// Returns the amount of the cashflow.
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Returns the date of the cashflow.
    pub fn date(&self) -> Date {
        self.date
    }

    /// Returns the Net Present Value (NPV) of the cashflow given a discount rate.
    pub fn npv(&self, discount_rate: f64) -> f64 {
        self.amount * discount_rate
    }
}

impl std::ops::Add for Cashflow {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.date, rhs.date, "Dates must match.");

        Self {
            amount: self.amount + rhs.amount,
            date: self.date,
        }
    }
}

impl std::ops::AddAssign for Cashflow {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.date, rhs.date);

        self.amount += rhs.amount;
    }
}

impl std::ops::Sub for Cashflow {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.date, rhs.date);

        Self {
            amount: self.amount - rhs.amount,
            date: self.date,
        }
    }
}

impl std::ops::SubAssign for Cashflow {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.date, rhs.date);
        self.amount -= rhs.amount;
    }
}

impl std::ops::Mul<f64> for Cashflow {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            date: self.date,
        }
    }
}

impl std::ops::MulAssign<f64> for Cashflow {
    fn mul_assign(&mut self, rhs: f64) {
        self.amount *= rhs;
    }
}

impl std::ops::Div<f64> for Cashflow {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            amount: self.amount / rhs,
            date: self.date,
        }
    }
}

impl std::ops::DivAssign<f64> for Cashflow {
    fn div_assign(&mut self, rhs: f64) {
        self.amount /= rhs;
    }
}

impl std::ops::Neg for Cashflow {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            date: self.date,
        }
    }
}

impl std::ops::Neg for &Cashflow {
    type Output = Cashflow;

    fn neg(self) -> Self::Output {
        Cashflow {
            amount: -self.amount,
            date: self.date,
        }
    }
}

impl std::ops::Neg for &mut Cashflow {
    type Output = Cashflow;

    fn neg(self) -> Self::Output {
        Cashflow {
            amount: -self.amount,
            date: self.date,
        }
    }
}

impl std::fmt::Display for Cashflow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cashflow({}, {})", self.amount, self.date)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_cashflows {
    use super::*;
    use time::Duration;

    use crate::{assert_approx_equal, time::today};
    use std::f64::EPSILON as EPS;

    // Test to verify the `amount` method.
    #[test]
    fn test_amount() {
        let cf = Cashflow::new(100.0, today());
        assert_approx_equal!(cf.amount(), 100.0, EPS);
    }

    // Test to verify the `date` method.
    #[test]
    fn test_date() {
        let now = today();
        let cf = Cashflow::new(100.0, now);
        assert_eq!(cf.date(), now);
    }

    // Test to verify the `npv` method.
    #[test]
    fn test_npv() {
        let cf = Cashflow::new(100.0, today());

        assert_approx_equal!(cf.npv(0.9), 90.0, EPS);
    }

    // Test to verify the `npv` method with a zero discount rate.
    #[test]
    fn test_npv_zero_discount() {
        let now = today();
        let cf = Cashflow::new(100.0, now);

        // Discount function that keeps value the same.
        let df = 1.0;
        assert_approx_equal!(cf.npv(df), 100.0, EPS);
    }

    // Test to verify addition of cashflows with the same date.
    #[test]
    fn test_add_cashflows() {
        let date = today();
        let cf1 = Cashflow::new(100.0, date);
        let cf2 = Cashflow::new(50.0, date);
        let result = cf1 + cf2;
        assert_approx_equal!(result.amount(), 150.0, EPS);
        assert_eq!(result.date(), date);
    }

    // Test to verify subtraction of cashflows with the same date.
    #[test]
    fn test_sub_cashflows() {
        let date = today();
        let cf1 = Cashflow::new(100.0, date);
        let cf2 = Cashflow::new(50.0, date);
        let result = cf1 - cf2;
        assert_approx_equal!(result.amount(), 50.0, EPS);
        assert_eq!(result.date(), date);
    }

    // Test for negative cashflows.
    #[test]
    fn test_negative_cashflow() {
        let date = today();
        let cf = Cashflow::new(-100.0, date);
        assert_approx_equal!(cf.amount(), -100.0, EPS);
    }

    // Test for zero cashflows.
    #[test]
    fn test_zero_cashflow() {
        let date = today();
        let cf = Cashflow::new(0.0, date);
        assert_approx_equal!(cf.amount(), 0.0, EPS);
    }

    // Test for non-matching dates during addition (should panic).
    #[test]
    #[should_panic(expected = "Dates must match.")]
    fn test_non_matching_dates_add() {
        let date1 = today();
        let date2 = date1 + Duration::days(1);
        let cf1 = Cashflow::new(100.0, date1);
        let cf2 = Cashflow::new(50.0, date2);
        let _ = cf1 + cf2;
    }
}
