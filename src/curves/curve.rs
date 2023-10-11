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

use std::{collections::BTreeMap, time::Duration};
use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structs, enums, and traits
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Base trait for all curves to implement.
pub trait Curve {
    /// Initial date of the curve.
    fn initial_date(&self) -> OffsetDateTime;

    /// Final date of the curve.
    fn terminal_date(&self) -> OffsetDateTime;

    /// Updates the rate for the given date.
    fn update_rate(&mut self, date: OffsetDateTime, rate: f64);

    /// Create a new curve from a set of dates and rates.
    fn from_dates_and_rates(dates: &[OffsetDateTime], rates: &[f64]) -> Self;

    /// Create a new curve from an initial date, a set of rates, and a set of
    /// durations.
    /// The dates are calculated as the initial date plus the duration, thus
    /// there must be:
    /// - One initial date
    /// - n rates
    /// - n-1 durations
    fn from_initial_date_rates_and_durations(
        initial_date: OffsetDateTime,
        rates: &[f64],
        durations: &[Duration],
    ) -> Self;

    /// Function to find the interval of dates that contains the given date.
    /// The interval is defined by the two dates that are closest to the given
    /// date, just before and just after.
    fn find_date_interval(&self, date: OffsetDateTime) -> (OffsetDateTime, OffsetDateTime);

    // /// Function to find the three points that are closest to the given date,
    // /// to be used for a quadratic interpolation.
    // fn find_three_points(
    //     &self,
    //     date: OffsetDateTime,
    // ) -> (OffsetDateTime, OffsetDateTime, OffsetDateTime);

    /// Returns the discount factor for the given date, using linear
    /// interpolation for dates between the curve's initial and terminal dates.
    /// If the date is outside the curve's range, we panic.
    ///
    /// We use the following formula for the interpolation:
    /// y = [y0 (x1 - x) + y1 (x - x0)] / (x1 - x0)
    ///
    /// Note: there must be at least two points in the curve, otherwise
    /// we consider the curve to be a flat rate, and return the same rate
    /// for all dates.
    fn discount_factor(&self, date: OffsetDateTime) -> f64;

    /// Returns multiple discount factors for the given dates.
    /// This is a convenience function that calls [discount_factor] for each
    /// date.
    fn discount_factors(&self, dates: &[OffsetDateTime]) -> Vec<f64> {
        dates
            .iter()
            .map(|date| self.discount_factor(*date))
            .collect::<Vec<f64>>()
    }
}

/// Yield curve struct.
pub struct YieldCurve {
    /// Map of dates and rates.
    /// The dates are the keys and the rates are the values.
    /// The reason for using a [BTreeMap] is that it is sorted by date,
    /// which makes sense for a term structure.
    pub rates: BTreeMap<OffsetDateTime, f64>,
}

/// Curve error enum.
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
    pub fn new(rates: BTreeMap<OffsetDateTime, f64>) -> Self {
        Self { rates }
    }
}

impl Curve for YieldCurve {
    fn initial_date(&self) -> OffsetDateTime {
        *self.rates.keys().min().unwrap()
    }

    fn terminal_date(&self) -> OffsetDateTime {
        *self.rates.keys().max().unwrap()
    }

    fn update_rate(&mut self, date: OffsetDateTime, rate: f64) {
        self.rates.insert(date, rate);
    }

    fn from_dates_and_rates(dates: &[OffsetDateTime], rates: &[f64]) -> Self {
        let mut rates_map = BTreeMap::new();

        for (date, rate) in dates.iter().zip(rates.iter()) {
            rates_map.insert(*date, *rate);
        }

        Self { rates: rates_map }
    }

    fn from_initial_date_rates_and_durations(
        initial_date: OffsetDateTime,
        rates: &[f64],
        durations: &[Duration],
    ) -> Self {
        let mut dates = vec![initial_date];

        for duration in durations.iter() {
            dates.push(*dates.last().unwrap() + *duration);
        }

        Self::from_dates_and_rates(&dates, rates)
    }

    fn discount_factor(&self, date: OffsetDateTime) -> f64 {
        let n = self.rates.len();

        match n {
            0 => panic!("The curve has no points."),
            1 => *self.rates.values().next().unwrap(),
            _ => {
                let (x0, x1) = self.find_date_interval(date);

                println!("x0: {:?}, x1: {:?}", x0.date(), x1.date());

                let y0 = *self.rates.get(&x0).unwrap();
                let y1 = *self.rates.get(&x1).unwrap();

                (y0 * (x1 - date) + y1 * (date - x0)) / (x1 - x0)

                // MIGHT IMPLEMENT A QUADRATIC INTERPOLATION LATER

                // let t = (date - x0) / (x1 - x0);
                // let t2 = t * t;

                // y0 + (2.0 * t - 1.0) * (y1 - y0) * t2
            }
        }
    }

    fn find_date_interval(&self, date: OffsetDateTime) -> (OffsetDateTime, OffsetDateTime) {
        if date == self.initial_date() {
            return (date, date);
        }

        if date == self.terminal_date() {
            return (date, date);
        }

        (
            *self.rates.range(..date).next_back().unwrap().0,
            *self.rates.range(date..).next().unwrap().0,
        )
    }

    // fn find_three_points(
    //     &self,
    //     date: OffsetDateTime,
    // ) -> (OffsetDateTime, OffsetDateTime, OffsetDateTime) {
    //     let mut dates = self.rates.keys().collect::<Vec<&OffsetDateTime>>();

    //     // dates.sort();

    //     let mut i = 0;

    //     while dates[i] < &date {
    //         i += 1;
    //     }

    //     if i == 0 {
    //         (dates[0], dates[1], dates[2])
    //     } else if i == dates.len() - 1 {
    //         (
    //             dates[dates.len() - 3],
    //             dates[dates.len() - 2],
    //             dates[dates.len() - 1],
    //         )
    //     } else {
    //         (dates[i - 1], dates[i], dates[i + 1])
    //     }
    // }
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
        let final_date = yield_curve.terminal_date();

        assert_eq!(final_date, OffsetDateTime::UNIX_EPOCH + Duration::days(60));
    }

    #[test]
    fn test_find_date_interval() {
        let mut rates = BTreeMap::new();

        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(30), 0.025);
        rates.insert(OffsetDateTime::UNIX_EPOCH + Duration::days(60), 0.03);

        let yield_curve = YieldCurve::new(rates);

        let date1 = OffsetDateTime::UNIX_EPOCH + Duration::days(30);
        let date2 = OffsetDateTime::UNIX_EPOCH + Duration::days(45);
        let date3 = OffsetDateTime::UNIX_EPOCH + Duration::days(60);

        let interval1 = yield_curve.find_date_interval(date1);
        let interval2 = yield_curve.find_date_interval(date2);
        let interval3 = yield_curve.find_date_interval(date3);

        assert_eq!(interval1, (date1, date1));
        assert_eq!(interval2, (date1, date3));
        assert_eq!(interval3, (date3, date3));
    }

    #[test]
    fn test_yield_curve_discount_factor() {
        // Initial date of the curve.
        let t0 = OffsetDateTime::UNIX_EPOCH;

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
        let date1 = OffsetDateTime::UNIX_EPOCH + Duration::days(45);
        let date2 = OffsetDateTime::UNIX_EPOCH + Duration::days(80);
        let date3 = OffsetDateTime::UNIX_EPOCH + Duration::days(250);

        let df1 = yield_curve.discount_factor(date1);
        let df2 = yield_curve.discount_factor(date2);
        let df3 = yield_curve.discount_factor(date3);

        println!("df1: {:?}", df1);
        println!("df2: {:?}", df2);
        println!("df3: {:?}", df3);

        assert!(df1 > 0.0 && df1 < 1.0 && df2 > 0.0 && df2 < 1.0 && df3 > 0.0 && df3 < 1.0);

        assert!(df1 < df2 && df2 < df3);
    }
}
