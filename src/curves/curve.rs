// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::collections::BTreeMap;

use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structs, enums, and traits
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Base trait for all curves to implement.
pub trait Curve {
    /// Initial date of the curve.
    fn initial_date(&self) -> OffsetDateTime;

    /// Final date of the curve.
    fn final_date(&self) -> OffsetDateTime;

    /// Returns the discount factor for the given date.
    /// Uses linear interpolation between the dates, and linear
    /// extrapolation outside the dates.
    ///
    /// That is:
    ///
    /// - Linear interpolation: y = [y0 (x1 - x) + y1 (x - x0)] / (x1 - x0)
    /// - Linear extrapolation: y = y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    fn discount_factor(&self, date: OffsetDateTime) -> f64;
}

/// Yield curve struct.
pub struct YieldCurve {
    rates: BTreeMap<OffsetDateTime, f64>,
}

/// Swap curve struct.
pub struct SwapCurve {
    rates: BTreeMap<OffsetDateTime, f64>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Implementations, functions, and macros
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl YieldCurve {
    /// Creates a new yield curve.
    pub fn new(rates: BTreeMap<OffsetDateTime, f64>) -> Self {
        Self { rates }
    }
}

impl Curve for YieldCurve {
    fn initial_date(&self) -> OffsetDateTime {
        *self.rates.keys().min().unwrap()
    }

    fn final_date(&self) -> OffsetDateTime {
        *self.rates.keys().max().unwrap()
    }

    fn discount_factor(&self, date: OffsetDateTime) -> f64 {
        let mut iter = self.rates.iter();

        let (mut x0, mut y0) = iter.next().unwrap();
        let (mut x1, mut y1) = iter.next().unwrap();

        while x1 < &date {
            x0 = x1;
            y0 = y1;
            let next = iter.next();
            if next.is_none() {
                break;
            }
            x1 = next.unwrap().0;
            y1 = next.unwrap().1;
        }

        println!("x0: {:?}", x0);
        println!("y0: {:?}", y0);
        println!("x1: {:?}", x1);
        println!("y1: {:?}", y1);

        match date {
            _ if date < *x0 => *y0 + (y1 - y0) * (date - *x0) / (*x1 - *x0),
            _ if date > *x1 => y1 + (y1 - y0) * (date - *x1) / (*x1 - *x0),
            _ => (*y0 * (*x1 - date) + *y1 * (date - *x0)) / (*x1 - *x0),
        }

        // (*y0 * (*x1 - date) + *y1 * (date - *x0)) / (*x1 - *x0)
        // y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_curves {
    use super::*;
    use std::collections::BTreeMap;
    use time::Duration;

    #[test]
    fn test_yield_curve_creation() {
        let mut rates = BTreeMap::new();
        rates.insert(OffsetDateTime::now_utc() + Duration::days(30), 0.025);
        rates.insert(OffsetDateTime::now_utc() + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates.clone());

        assert_eq!(yield_curve.rates, rates);
    }

    #[test]
    fn test_yield_curve_initial_date() {
        let mut rates = BTreeMap::new();
        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(30), 0.025);
        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);
        let initial_date = yield_curve.initial_date();

        assert_eq!(
            initial_date,
            OffsetDateTime::UNIX_EPOCH + Duration::days(30)
        );
    }

    #[test]
    fn test_yield_curve_final_date() {
        let mut rates = BTreeMap::new();
        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(30), 0.025);
        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);
        let final_date = yield_curve.final_date();

        assert_eq!(final_date, OffsetDateTime::UNIX_EPOCH + Duration::days(60));
    }

    #[test]
    fn test_yield_curve_discount_factor() {
        let mut rates = BTreeMap::new();

        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(30), 0.025);
        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);

        let date1 = OffsetDateTime::UNIX_EPOCH + Duration::days(45);
        let date2 = OffsetDateTime::UNIX_EPOCH + Duration::days(90);

        let df1 = yield_curve.discount_factor(date1);
        let df2 = yield_curve.discount_factor(date2);

        println!("df1: {:?}", df1);
        println!("df2: {:?}", df2);

        assert!(df1 > 0.0);
        assert!(df2 > 0.0);
        assert!(df1 < df2);
    }
}
