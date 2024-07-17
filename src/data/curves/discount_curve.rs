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

use super::{Curve, CurveIndex};
use crate::time::{Calendar, DateRollingConvention, DayCountConvention};
use derive_builder::Builder;
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Discount curve data structure.
#[derive(Clone, Builder)]
pub struct DiscountCurve<I, C>
where
    I: CurveIndex,
    C: Calendar,
{
    /// Map of dates and rates.
    pub curve: Curve<I>,

    /// Calendar.
    pub calendar: Option<C>,

    /// Day count convention.
    pub day_count_convention: Option<DayCountConvention>,

    /// Date rolling convention.
    pub date_rolling_convention: Option<DateRollingConvention>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Implementations, functions, and macros
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<C> DiscountCurve<Date, C>
where
    C: Calendar,
{
    /// Creates a new discount curve from a set of `Date`s and rates.
    pub fn new(dates: &[Date], rates: &[f64]) -> Self {
        assert!(dates.len() == rates.len());

        Self {
            curve: Curve::<Date>::new_from_slice(&dates, &rates),
            calendar: None,
            day_count_convention: None,
            date_rolling_convention: None,
        }
    }

    /// Set the calendar for the discount curve.
    pub fn with_calendar(&mut self, calendar: C) {
        self.calendar = Some(calendar);
    }

    /// Set the day count convention for the discount curve.
    pub fn with_day_count_convention(&mut self, day_count_convention: DayCountConvention) {
        self.day_count_convention = Some(day_count_convention);
    }

    /// Set the date rolling convention for the discount curve.
    pub fn with_date_rolling_convention(&mut self, date_rolling_convention: DateRollingConvention) {
        self.date_rolling_convention = Some(date_rolling_convention);
    }

    /// Get the initial date of the discount curve.
    pub fn initial_date(&self) -> Date {
        *self.curve.first_key().unwrap()
    }

    /// Get the terminal date of the discount curve.
    pub fn terminal_date(&self) -> Date {
        *self.curve.last_key().unwrap()
    }

    /// Insert a new rate into the discount curve.
    pub fn insert_rate(&mut self, date: Date, rate: f64) {
        self.curve.insert(date, rate);
    }

    /// Get the rate for a specific date.
    ///
    /// Note: If the date is not in the curve, the rate is interpolated,
    /// and the interpolated rate is also stored in the curve.
    /// This is why a mutable reference to the curve is required.
    pub fn get_rate(&mut self, date: Date) -> f64 {
        self.curve.interpolate(date);

        *self.curve.get(date).unwrap()
    }

    /// Get multiple rates for a set of dates.
    ///
    /// Note: If a date is not in the curve, the rate is interpolated,
    /// and the interpolated rate is also stored in the curve.
    /// This is why a mutable reference to the curve is required.
    pub fn get_rates(&mut self, dates: &[Date]) -> Vec<f64> {
        dates.iter().map(|date| self.get_rate(*date)).collect()
    }

    // #[allow(clippy::similar_names)]
    // fn from_initial_date_rates_and_durations(
    //     initial_date: Date,
    //     rates: &[f64],
    //     durations: &[Duration],
    // ) -> Self {
    //     let mut dates = vec![initial_date];

    //     for duration in durations {
    //         dates.push(*dates.last().unwrap() + *duration);
    //     }

    //     Self::from_dates_and_rates(&dates, rates)
    // }

    // fn find_date_interval(&self, date: Date) -> (Date, Date) {
    //     if date == self.initial_date() || date == self.terminal_date() {
    //         return (date, date);
    //     }

    //     (
    //         *self.rates.range(..date).next_back().unwrap().0,
    //         *self.rates.range(date..).next().unwrap().0,
    //     )
    // }
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
    fn test_discount_curve_creation() {
        let dates = [today() + Duration::days(30), today() + Duration::days(60)];
        let rates = [0.025, 0.03];

        let discount_curve = DiscountCurve::new(&dates, &rates);

        assert_eq!(discount_curve.rates, rates);
    }

    #[test]
    fn test_discount_curve_initial_date() {
        let dates = [
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
        ];

        let rates = [0.025, 0.03];

        let discount_curve = DiscountCurve::new(&dates, &rates);
        let initial_date = discount_curve.initial_date();

        assert_eq!(
            initial_date,
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30)
        );
    }

    #[test]
    fn test_discount_curve_final_date() {
        let dates = [
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
        ];

        let rates = [0.025, 0.03];

        let discount_curve = DiscountCurve::new(&dates, &rates);
        let final_date = discount_curve.terminal_date();

        assert_eq!(
            final_date,
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60)
        );
    }

    #[test]
    fn test_find_date_interval() {
        let dates = [
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
            OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
        ];

        let rates = [0.025, 0.03];

        let discount_curve = DiscountCurve::new(&dates, &rates);

        let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30);
        let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
        let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60);

        let interval1 = discount_curve.find_date_interval(date1);
        let interval2 = discount_curve.find_date_interval(date2);
        let interval3 = discount_curve.find_date_interval(date3);

        assert_eq!(interval1, (date1, date1));
        assert_eq!(interval2, (date1, date3));
        assert_eq!(interval3, (date3, date3));
    }

    #[allow(clippy::similar_names)]
    #[test]
    fn test_discount_curve_discount_factor() {
        // Initial date of the curve.
        let t0 = OffsetDateTime::UNIX_EPOCH.date();

        // Create a discount curve with 8 points.
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

        let discount_curve = DiscountCurve::from_dates_and_rates(&date_vec, &rate_vec);

        println!("Curve: {:?}", discount_curve.rates);

        // Test the discount factor for a dates inside the curve's range.
        let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
        let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(80);
        let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(250);

        let df1 = discount_curve.discount_factor(date1);
        let df2 = discount_curve.discount_factor(date2);
        let df3 = discount_curve.discount_factor(date3);

        println!("df1: {:?}", df1);
        println!("df2: {:?}", df2);
        println!("df3: {:?}", df3);

        assert!(df1 > 0.0 && df1 < 1.0 && df2 > 0.0 && df2 < 1.0 && df3 > 0.0 && df3 < 1.0);

        assert!(df1 > df2 && df2 > df3);
    }
}
