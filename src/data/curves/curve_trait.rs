// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
