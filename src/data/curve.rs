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

use crate::time::DayCountConvention;
use std::{collections::BTreeMap, time::Duration};
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structs, enums, and traits
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// A trait for curve models.
#[allow(clippy::module_name_repetitions)]
pub trait CurveModel {
    /// Returns the forward rate for a given date.
    fn forward_rate(&self, date: Date) -> f64;

    /// Returns the spot rate for a given date.
    fn spot_rate(&self, date: Date) -> f64;

    /// Returns the discount factor for a given date.
    fn discount_factor(&self, date: Date) -> f64;

    /// Calibrates the model to a set of market rates.
    #[must_use]
    fn calibrate<C: Curve>(&self, curve: C) -> Self;
}

/// Base trait for all curves to implement.
pub trait Curve {
    /// Initial date of the curve.
    fn initial_date(&self) -> Date;

    /// Final date of the curve.
    fn terminal_date(&self) -> Date;

    /// Updates the rate for the given date.
    fn update_rate(&mut self, date: Date, rate: f64);

    /// Create a new curve from a set of dates and rates.
    fn from_dates_and_rates(dates: &[Date], rates: &[f64]) -> Self;

    /// Create a new curve from an initial date, a set of rates, and a set of
    /// durations.
    /// The dates are calculated as the initial date plus the duration, thus
    /// there must be:
    /// - One initial date
    /// - n rates
    /// - n-1 durations
    fn from_initial_date_rates_and_durations(
        initial_date: Date,
        rates: &[f64],
        durations: &[Duration],
    ) -> Self;

    /// Function to find the interval of dates that contains the given date.
    /// The interval is defined by the two dates that are closest to the given
    /// date, just before and just after.
    fn find_date_interval(&self, date: Date) -> (Date, Date);

    /// Returns the rate for the given date, using linear interpolation for
    /// dates between the curve's initial and terminal dates.
    /// If the date is outside the curve's range, we panic.
    ///
    /// We use the following formula for the interpolation:
    ///
    /// $$
    /// y = \frac{y_0 (x_1 - x) + y_1 (x - x_0)}{x_1 - x_0}
    /// $$
    ///
    /// Note: there must be at least two points in the curve, otherwise
    /// we consider the curve to be a flat rate, and return the same rate
    /// for all dates.
    fn rate(&self, date: Date) -> f64;

    /// Returns the discount factor for the given date.
    /// This is a convenience function that calls [`rate`](Curve::rate) to get the rate for
    /// the given date, and then calculates the discount factor using the
    /// formula:
    /// $$
    /// p(t) = e^{- r \cdot t}
    /// $$
    fn discount_factor(&self, date: Date) -> f64 {
        let t = DayCountConvention::default().day_count_factor(self.initial_date(), date);

        f64::exp(-self.rate(date) * t)
    }

    /// Returns multiple discount factors for the given dates.
    /// This is a convenience function that calls [`discount_factor`](Curve::discount_factor) for each
    /// date.
    fn discount_factors(&self, dates: &[Date]) -> Vec<f64> {
        dates
            .iter()
            .map(|date| self.discount_factor(*date))
            .collect::<Vec<f64>>()
    }
}

#[allow(clippy::module_name_repetitions)]
/// Yield curve struct.
pub struct YieldCurve {
    /// Map of dates and rates.
    /// The dates are the keys and the rates are the values.
    /// The reason for using a [BTreeMap] is that it is sorted by date,
    /// which makes sense for a term structure.
    pub rates: BTreeMap<Date, f64>,
    // /// A model for the curve.
    // pub model: Option<M>,
}

/// Curve error enum.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub enum CurveError {
    /// The date is outside the curve's range.
    DateOutsideRange,

    /// The curve has no points.
    NoPoints,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Implementations, functions, and macros
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl YieldCurve {
    /// Creates a new yield curve.
    #[must_use]
    pub fn new(rates: BTreeMap<Date, f64>) -> Self {
        Self { rates }
    }
}

impl Curve for YieldCurve {
    fn initial_date(&self) -> Date {
        *self.rates.keys().min().unwrap()
    }

    fn terminal_date(&self) -> Date {
        *self.rates.keys().max().unwrap()
    }

    #[allow(clippy::similar_names)]
    fn update_rate(&mut self, date: Date, rate: f64) {
        self.rates.insert(date, rate);
    }

    #[allow(clippy::similar_names)]
    fn from_dates_and_rates(dates: &[Date], rates: &[f64]) -> Self {
        let mut rates_map = BTreeMap::new();

        for (date, rate) in dates.iter().zip(rates.iter()) {
            rates_map.insert(*date, *rate);
        }

        Self { rates: rates_map }
    }

    #[allow(clippy::similar_names)]
    fn from_initial_date_rates_and_durations(
        initial_date: Date,
        rates: &[f64],
        durations: &[Duration],
    ) -> Self {
        let mut dates = vec![initial_date];

        for duration in durations {
            dates.push(*dates.last().unwrap() + *duration);
        }

        Self::from_dates_and_rates(&dates, rates)
    }

    fn rate(&self, date: Date) -> f64 {
        let n = self.rates.len();

        match n {
            0 => panic!("The curve has no points."),
            1 => *self.rates.values().next().unwrap(),
            _ => {
                let (x0, x1) = self.find_date_interval(date);
                let (y0, y1) = (*self.rates.get(&x0).unwrap(), *self.rates.get(&x1).unwrap());

                (y0 * (x1 - date) + y1 * (date - x0)) / (x1 - x0)
            }
        }
    }

    fn find_date_interval(&self, date: Date) -> (Date, Date) {
        if date == self.initial_date() || date == self.terminal_date() {
            return (date, date);
        }

        (
            *self.rates.range(..date).next_back().unwrap().0,
            *self.rates.range(date..).next().unwrap().0,
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_curves {
    use super::*;
    use crate::time::today;
    use std::collections::BTreeMap;
    use time::Duration;
    use time::OffsetDateTime;

    #[test]
    fn test_yield_curve_creation() {
        let mut rates = BTreeMap::new();
        rates.insert(today() + Duration::days(30), 0.025);
        rates.insert(today() + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates.clone());

        assert_eq!(yield_curve.rates, rates);
    }

    #[test]
    fn test_yield_curve_initial_date() {
        let mut rates = BTreeMap::new();
        rates.insert(
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
            0.025,
        );
        rates.insert(OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);
        let initial_date = yield_curve.initial_date();

        assert_eq!(
            initial_date,
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30)
        );
    }

    #[test]
    fn test_yield_curve_final_date() {
        let mut rates = BTreeMap::new();
        rates.insert(
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
            0.025,
        );
        rates.insert(OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);
        let final_date = yield_curve.terminal_date();

        assert_eq!(
            final_date,
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60)
        );
    }

    #[test]
    fn test_find_date_interval() {
        let mut rates = BTreeMap::new();

        rates.insert(
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
            0.025,
        );
        rates.insert(OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);

        let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30);
        let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
        let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60);

        let interval1 = yield_curve.find_date_interval(date1);
        let interval2 = yield_curve.find_date_interval(date2);
        let interval3 = yield_curve.find_date_interval(date3);

        assert_eq!(interval1, (date1, date1));
        assert_eq!(interval2, (date1, date3));
        assert_eq!(interval3, (date3, date3));
    }

    #[allow(clippy::similar_names)]
    #[test]
    fn test_yield_curve_discount_factor() {
        // Initial date of the curve.
        let t0 = OffsetDateTime::UNIX_EPOCH.date();

        // Create a yield curve with 8 points.
        let rate_vec = vec![0.025, 0.03, 0.035, 0.04, 0.045, 0.05, 0.055, 0.06];
        let date_vec = vec![
            t0 + Duration::days(30),
            t0 + Duration::days(60),
            t0 + Duration::days(90),
            t0 + Duration::days(120),
            t0 + Duration::days(150),
            t0 + Duration::days(180),
            t0 + Duration::days(210),
            t0 + Duration::days(360),
        ];

        let yield_curve = YieldCurve::from_dates_and_rates(&date_vec, &rate_vec);

        println!("Curve: {:?}", yield_curve.rates);

        // Test the discount factor for a dates inside the curve's range.
        let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
        let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(80);
        let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(250);

        let df1 = yield_curve.discount_factor(date1);
        let df2 = yield_curve.discount_factor(date2);
        let df3 = yield_curve.discount_factor(date3);

        println!("df1: {:?}", df1);
        println!("df2: {:?}", df2);
        println!("df3: {:?}", df3);

        assert!(df1 > 0.0 && df1 < 1.0 && df2 > 0.0 && df2 < 1.0 && df3 > 0.0 && df3 < 1.0);

        assert!(df1 > df2 && df2 > df3);
    }
}
