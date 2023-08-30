// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Cashflows module.

use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Cashflow trait.
pub trait Cashflow {
    /// Amount of the cashflow.
    fn amount(&self) -> f64;
    /// Date of the cashflow.
    fn date(&self) -> OffsetDateTime;
    /// Net present value (NPV) of the cashflow.
    fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64;
}

/// Simple cashflow type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleCashflow {
    amount: f64,
    date: OffsetDateTime,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl SimpleCashflow {
    /// Create a new simple cashflow.
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

    fn npv<F>(&self, df: F) -> f64
    where
        F: Fn(OffsetDateTime) -> f64,
    {
        self.amount * df(self.date)
    }
}

impl std::ops::Add for SimpleCashflow {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.date, rhs.date);
        Self {
            amount: self.amount + rhs.amount,
            date: self.date,
        }
    }
}

impl std::ops::AddAssign for SimpleCashflow {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.date, rhs.date);
        self.amount += rhs.amount;
    }
}

impl std::ops::Sub for SimpleCashflow {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.date, rhs.date);
        Self {
            amount: self.amount - rhs.amount,
            date: self.date,
        }
    }
}

impl std::ops::SubAssign for SimpleCashflow {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.date, rhs.date);
        self.amount -= rhs.amount;
    }
}

impl std::ops::Mul<f64> for SimpleCashflow {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            amount: self.amount * rhs,
            date: self.date,
        }
    }
}

impl std::ops::MulAssign<f64> for SimpleCashflow {
    fn mul_assign(&mut self, rhs: f64) {
        self.amount *= rhs;
    }
}

impl std::ops::Div<f64> for SimpleCashflow {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            amount: self.amount / rhs,
            date: self.date,
        }
    }
}

impl std::ops::DivAssign<f64> for SimpleCashflow {
    fn div_assign(&mut self, rhs: f64) {
        self.amount /= rhs;
    }
}

impl std::ops::Neg for SimpleCashflow {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            amount: -self.amount,
            date: self.date,
        }
    }
}

impl std::ops::Neg for &SimpleCashflow {
    type Output = SimpleCashflow;

    fn neg(self) -> Self::Output {
        SimpleCashflow {
            amount: -self.amount,
            date: self.date,
        }
    }
}

impl std::ops::Neg for &mut SimpleCashflow {
    type Output = SimpleCashflow;

    fn neg(self) -> Self::Output {
        SimpleCashflow {
            amount: -self.amount,
            date: self.date,
        }
    }
}

impl std::fmt::Display for SimpleCashflow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SimpleCashflow({}, {})", self.amount, self.date)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_cashflows {
    use super::*;
    use time::Duration;

    // Test to verify the `amount` method.
    #[test]
    fn test_amount() {
        let cf = SimpleCashflow::new(100.0, OffsetDateTime::now_utc());
        assert_eq!(cf.amount(), 100.0);
    }

    // Test to verify the `date` method.
    #[test]
    fn test_date() {
        let now = OffsetDateTime::now_utc();
        let cf = SimpleCashflow::new(100.0, now);
        assert_eq!(cf.date(), now);
    }

    // Test to verify the `npv` method.
    #[test]
    fn test_npv() {
        let now = OffsetDateTime::now_utc();
        let cf = SimpleCashflow::new(100.0, now);

        // Discount function that reduces value by 10%.
        let df = |date: OffsetDateTime| if date == now { 0.9 } else { 1.0 };
        assert_eq!(cf.npv(df), 90.0);
    }

    // Test to verify the `npv` method with a zero discount rate.
    #[test]
    fn test_npv_zero_discount() {
        let now = OffsetDateTime::now_utc();
        let cf = SimpleCashflow::new(100.0, now);

        // Discount function that keeps value the same.
        let df = |_: OffsetDateTime| 1.0;
        assert_eq!(cf.npv(df), 100.0);
    }

    // Test to verify the `npv` method with future date
    #[test]
    fn test_npv_future_date() {
        let now = OffsetDateTime::now_utc();
        let future_date = now + Duration::days(30);
        let cf = SimpleCashflow::new(100.0, future_date);

        // Discount function that reduces value by 10% for future_date.
        let df = |date: OffsetDateTime| if date == future_date { 0.9 } else { 1.0 };
        assert_eq!(cf.npv(df), 90.0);
    }

    // Test to verify addition of cashflows with the same date.
    #[test]
    fn test_add_cashflows() {
        let date = OffsetDateTime::now_utc();
        let cf1 = SimpleCashflow::new(100.0, date);
        let cf2 = SimpleCashflow::new(50.0, date);
        let result = cf1 + cf2;
        assert_eq!(result.amount(), 150.0);
        assert_eq!(result.date(), date);
    }

    // Test to verify subtraction of cashflows with the same date.
    #[test]
    fn test_sub_cashflows() {
        let date = OffsetDateTime::now_utc();
        let cf1 = SimpleCashflow::new(100.0, date);
        let cf2 = SimpleCashflow::new(50.0, date);
        let result = cf1 - cf2;
        assert_eq!(result.amount(), 50.0);
        assert_eq!(result.date(), date);
    }

    // Test for negative cashflows.
    #[test]
    fn test_negative_cashflow() {
        let date = OffsetDateTime::now_utc();
        let cf = SimpleCashflow::new(-100.0, date);
        assert_eq!(cf.amount(), -100.0);
    }

    // Test for zero cashflows.
    #[test]
    fn test_zero_cashflow() {
        let date = OffsetDateTime::now_utc();
        let cf = SimpleCashflow::new(0.0, date);
        assert_eq!(cf.amount(), 0.0);
    }

    // Test for non-matching dates during addition (should panic).
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_non_matching_dates_add() {
        let date1 = OffsetDateTime::now_utc();
        let date2 = date1 + Duration::days(1);
        let cf1 = SimpleCashflow::new(100.0, date1);
        let cf2 = SimpleCashflow::new(50.0, date2);
        let _ = cf1 + cf2;
    }
}
