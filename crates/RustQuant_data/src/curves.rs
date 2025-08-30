// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Curves module.
//!
//! This module contains the following curve structures:
//! - [Curve](crate::data::curves::curve::Curve)
//! - [DiscountCurve](crate::data::curves::discount_curve::DiscountCurve)
//! - [SpotCurve](crate::data::curves::spot_curve::SpotCurve)
//! - [ForwardCurve](crate::data::curves::forward_curve::ForwardCurve)
//!
//! Curves are constructed from a set of dates and rates, and can be used to
//! interpolate rates for dates that are not in the curve.
//! Currently, the curves are fit to a Nelson-Siegel-Svensson model via
//! particle swarm optimization (thanks to the [argmin](https://docs.rs/argmin/latest/argmin/index.html) crate),
//! and a log-cosh loss function, however these may change in the future.
//!
//! The curves are fit to the models *"lazily"* in the sense that no
//! fitting takes place until the user requests a rate for a date that is not
//! present in the curve. At that point, the curve is fit to the model and the
//! rate is interpolated.

use argmin::{
    core::{CostFunction, Executor, State},
    solver::particleswarm::ParticleSwarm,
};
use derive_builder::Builder;
use plotly::{color::NamedColor, common::Marker, common::Mode, Plot, Scatter};
use std::{collections::BTreeMap, hash::Hash, iter::zip};
use time::Date;
use RustQuant_math::{
    interpolation::{ExponentialInterpolator, Interpolator, LinearInterpolator},
    InterpolationIndex,
};
use RustQuant_stochastics::{CurveModel, NelsonSiegelSvensson};
use RustQuant_time::{Calendar, DateRollingConvention, DayCountConvention};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BASE CURVE DATA STRUCTURE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Curve index trait.
pub trait CurveIndex: Ord + Hash + InterpolationIndex + Clone + Copy {}
impl<T> CurveIndex for T where T: Ord + Hash + InterpolationIndex + Clone + Copy {}

/// Curve data structure.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Curve<C>
where
    C: CurveIndex,
{
    /// The nodes of the curve.
    pub nodes: BTreeMap<C, f64>,
}

macro_rules! impl_curve {
    ($index:ty) => {
        impl Curve<$index> {
            /// Create a new curve.
            pub fn new() -> Self {
                Self {
                    nodes: BTreeMap::new(),
                }
            }

            /// Get the first key in the curve.
            pub fn first_key(&self) -> Option<&$index> {
                self.nodes.keys().next()
            }

            /// Get the last key in the curve.
            pub fn last_key(&self) -> Option<&$index> {
                self.nodes.keys().next_back()
            }

            /// Get the keys of the curve.
            pub fn keys(&self) -> Vec<$index> {
                self.nodes.keys().cloned().collect()
            }

            /// Get the values of the curve.
            pub fn values(&self) -> Vec<f64> {
                self.nodes.values().cloned().collect()
            }

            /// Get the length of the curve.
            pub fn len(&self) -> usize {
                self.nodes.len()
            }

            /// Check if the curve is empty.
            pub fn is_empty(&self) -> bool {
                self.nodes.is_empty()
            }

            /// Get the first value in the curve.
            pub fn first_value(&self) -> Option<&f64> {
                self.nodes.values().next()
            }

            /// Get the last value in the curve.
            pub fn last_value(&self) -> Option<&f64> {
                self.nodes.values().next_back()
            }

            /// Add a node to the curve.
            pub fn insert(&mut self, index: $index, value: f64) {
                self.nodes.insert(index, value);
            }

            /// Get a value for a specific index.
            pub fn get(&self, index: $index) -> Option<&f64> {
                self.nodes.get(&index)
            }

            /// Get a mutable reference to a value for a specific index.
            pub fn get_mut(&mut self, index: $index) -> Option<&mut f64> {
                self.nodes.get_mut(&index)
            }

            /// Create a Curve from a vector of indices and values.
            pub fn new_from_slice(indices: &[$index], values: &[f64]) -> Self {
                let mut curve = Self::new();

                for (index, value) in indices.iter().zip(values.iter()) {
                    curve.insert(*index, *value);
                }

                curve
            }

            /// Create a Curve from a function.
            pub fn new_from_function<F>(f: F, indices: &[$index]) -> Self
            where
                F: Fn($index) -> f64,
            {
                let mut curve = Self::new();

                for index in indices {
                    curve.insert(*index, f(*index));
                }

                curve
            }

            /// Create a Curve from a constant value.
            pub fn new_from_constant(value: f64, indices: &[$index]) -> Self {
                let mut curve = Self::new();

                for index in indices {
                    curve.insert(*index, value);
                }

                curve
            }

            /// Get the bracketing indices for a specific index.
            pub fn get_brackets(&self, index: $index) -> ($index, $index) {
                let first = self.first_key().unwrap();
                let last = self.last_key().unwrap();

                if index <= *first {
                    return (*first, *first);
                }

                if index >= *last {
                    return (*last, *last);
                }

                let left = self.nodes.range(..index).next_back().unwrap().0;
                let right = self.nodes.range(index..).next().unwrap().0;

                return (*left, *right);
            }

            /// Shift the curve by a constant value.
            pub fn shift(&mut self, shift: f64) {
                for value in self.nodes.values_mut() {
                    *value += shift;
                }
            }

            /// Interpolate the curve at a specific index.
            ///
            /// Note: This method modifies the curve by adding the interpolated value.
            pub fn interpolate(&mut self, index: $index) -> f64 {
                if self.nodes.contains_key(&index) {
                    return *self.nodes.get(&index).unwrap();
                }

                let xs: Vec<$index> = self.nodes.keys().cloned().collect();
                let ys: Vec<f64> = self.nodes.values().cloned().collect();

                let interpolator = LinearInterpolator::new(xs, ys).unwrap();

                self.insert(index, interpolator.interpolate(index).unwrap());

                *self.nodes.get(&index).unwrap()
            }

            /// Interpolate the curve at multiple indices.
            ///
            /// Note: This method modifies the curve by adding the interpolated values.
            pub fn interpolate_many(&mut self, indices: &[$index]) {
                let xs: Vec<$index> = self.nodes.keys().cloned().collect();
                let ys: Vec<f64> = self.nodes.values().cloned().collect();

                // let interpolator = LinearInterpolator::new(xs, ys).unwrap();
                let interpolator = ExponentialInterpolator::new(xs, ys).unwrap();

                for index in indices {
                    if !self.nodes.contains_key(index) {
                        self.insert(*index, interpolator.interpolate(*index).unwrap());
                    }
                }
            }

            /// Plot the curve.
            pub fn plot(&self) {
                let mut plot = Plot::new();

                let xs = self
                    .nodes
                    .keys()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

                let ys = self.nodes.values().cloned().collect::<Vec<f64>>();

                let trace = Scatter::new(xs, ys).mode(Mode::LinesMarkers);

                plot.add_trace(trace);
                plot.show();
                // plot
            }

            /// Static method to plot multiple curves.
            pub fn plot_many(curves: &[Self]) {
                let mut plot = Plot::new();

                for curve in curves {
                    let xs = curve
                        .nodes
                        .keys()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();

                    let ys = curve.nodes.values().cloned().collect::<Vec<f64>>();

                    let trace = Scatter::new(xs, ys);

                    plot.add_trace(trace);
                }

                plot.show();
            }
        }
    };
}

// Implement the Curve for temporal types.
impl_curve!(time::Date);
impl_curve!(time::Time);
impl_curve!(time::OffsetDateTime);
impl_curve!(time::PrimitiveDateTime);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// CURVE RELATED CONSTANTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const CURVE_OPTIM_MAX_ITER: u64 = 69;
const CURVE_OPTIM_SWARM_SIZE: usize = 1000;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MACRO TO IMPLEMENT SPECIFIC CURVES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Generic trait for curves.
pub trait Curves {
    /// Create a new curve from a set of `Date`s and rates (`f64`s).
    fn new(dates: &[Date], rates: &[f64]) -> Self;

    /// Get the initial date of the curve.
    fn initial_date(&self) -> Date;

    /// Get the terminal date of the curve.
    fn terminal_date(&self) -> Date;

    /// Get the rate for a specific date.
    fn get_rate(&mut self, date: Date) -> f64;

    /// Get multiple rates for a set of dates.
    fn get_rates(&mut self, dates: &[Date]) -> Vec<f64>;

    /// Insert a new rate into the curve.
    fn insert_rate(&mut self, date: Date, rate: f64);

    /// Fit the curve to a Nelson-Siegel-Svensson model.
    fn fit(&mut self) -> Result<(), argmin::core::Error>;

    /// Plot the curve.
    fn plot(&self);
}

macro_rules! impl_specific_curve_cost_function {
    ($curve:ident, $curve_function:ident) => {
        impl CostFunction for &$curve<Date> {
            type Param = Vec<f64>;
            type Output = f64;

            fn cost(&self, p: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
                let nss = RustQuant_stochastics::NelsonSiegelSvensson::new(
                    p[0], p[1], p[2], p[3], p[4], p[5],
                );

                let n = self.curve.len() as f64;
                let x = self.curve.keys();
                let y = self.curve.values();

                let y_model = x
                    .iter()
                    .map(|date| nss.$curve_function(*date))
                    .collect::<Vec<f64>>();

                let data = zip(y.iter(), y_model.iter());

                // let mae = data.map(|(o, p)| (o - p).abs()).sum::<f64>() / n;
                // let mse = data.map(|(o, p)| (o - p).powi(2)).sum::<f64>() / n;
                let log_cosh_loss = data.map(|(o, p)| (p - o).cosh().ln()).sum::<f64>() / n;

                Ok(log_cosh_loss)
            }
        }
    };
}

macro_rules! impl_specific_curve {
    ($curve:ident, $curve_function:ident) => {
        impl Curves for $curve<Date>
        {
            #[doc = concat!("Fit the ", stringify!($curve))]
            fn fit(&mut self) -> Result<(), argmin::core::Error> {
                let zero = f64::EPSILON;

                let bounds = [
                    (zero, 0.3), // Beta_0
                    (-0.3, 0.3), // Beta_1
                    (-1.0, 1.0), // Beta_2
                    (-1.0, 1.0), // Beta_3
                    (zero, 5.0), // Tau_1
                    (zero, 5.0), // Tau_2
                ].to_vec().into_iter().map(|(a, b)| (a, b)).collect();

                let model = self.clone();

                let solver = ParticleSwarm::new(bounds, CURVE_OPTIM_SWARM_SIZE);//-0.3593 -0.7238 2.0289
                    // .with_inertia_factor(0.1)? // Inertia factor (w)
                    // .with_cognitive_factor(2.)? // Cognitive (personal) factor
                    // .with_social_factor(2.)?; // Social (global) factor

                // let solver = SimulatedAnnealing::new(100_f64);
                let executor =
                    Executor::new(&model, solver).configure(|state| state.max_iters(CURVE_OPTIM_MAX_ITER));

                let result = executor.run()?;
                let params = result.state().get_best_param().unwrap().position.to_vec();

                self.nss = NelsonSiegelSvensson::new(
                    params[0], params[1], params[2], params[3], params[4], params[5],
                );

                self.fitted = true;
                println!("TIME: {:?}", result.state().get_time());

                Ok(())
            }

            #[doc = concat!("Creates a new ", stringify!($curve), " curve from a set of `Date`s and rates.")]
            fn new(dates: &[Date], rates: &[f64]) -> Self {
                assert!(dates.len() == rates.len());

                Self {
                    curve: Curve::<Date>::new_from_slice(&dates, &rates),

                    calendar: None,
                    day_count_convention: None,
                    date_rolling_convention: None,
                    nss: NelsonSiegelSvensson::default(),
                    fitted: false,
                    fitted_curve: None,
                }
            }

            #[doc = concat!("Get the initial date of the ", stringify!($curve))]
            fn initial_date(&self) -> Date {
                *self.curve.first_key().unwrap()
            }

            #[doc = concat!("Get the terminal date of the ", stringify!($curve))]
            fn terminal_date(&self) -> Date {
                *self.curve.last_key().unwrap()
            }

            #[doc = concat!("Insert a new rate into the ", stringify!($curve))]
            fn insert_rate(&mut self, date: Date, rate: f64) {
                self.curve.insert(date, rate);
            }

            /// Get the rate for a specific date.
            ///
            /// Note: If the date is not in the curve, the rate is interpolated,
            /// and the interpolated rate is also stored in the curve.
            /// This is why a mutable reference to the curve is required.
            ///
            /// Note: Interpolation is performed by fitting the curve
            /// to a Nelson-Siegel-Svensson model (if not already fitted),
            /// and then interpolating the rate for the given date.
            fn get_rate(&mut self, date: Date) -> f64 {
                match self.curve.get(date) {
                    Some(rate) => *rate,
                    None => {
                        if !self.fitted {
                            self.fit().unwrap();
                            self.fitted_curve = Some(Curve::<Date>::new());
                        }

                        let rate = self.nss.$curve_function(date);
                        self.insert_rate(date, rate);
                        self.fitted_curve.as_mut().unwrap().insert(date, rate);

                        rate
                    }
                }
            }

            /// Get multiple rates for a set of dates.
            ///
            /// Note: If a date is not in the curve, the rate is interpolated,
            /// and the interpolated rate is also stored in the curve.
            /// This is why a mutable reference to the curve is required.
            ///
            /// Note: Interpolation is performed by fitting the curve
            /// to a Nelson-Siegel-Svensson model (if not already fitted),
            /// and then interpolating the rate for the given date.
            fn get_rates(&mut self, dates: &[Date]) -> Vec<f64> {
                dates.iter().map(|date| self.get_rate(*date)).collect()
            }

            #[doc = concat!("Plot the ", stringify!($curve))]
            fn plot(&self) {
                let mut plot = Plot::new();

                let xs = self
                    .curve
                    .nodes
                    .keys()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

                let ys = self.curve.nodes.values().cloned().collect::<Vec<f64>>();

                let trace = Scatter::new(xs, ys).mode(Mode::Markers).name(
                    concat!(stringify!($curve)),
                );

                plot.add_trace(trace);

                if self.fitted {
                    let xs = self
                        .fitted_curve
                        .as_ref()
                        .unwrap()
                        .nodes
                        .keys()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();

                    let ys = self
                        .fitted_curve
                        .as_ref()
                        .unwrap()
                        .nodes
                        .values()
                        .cloned()
                        .collect::<Vec<f64>>();

                    let trace = Scatter::new(xs, ys)
                        .mode(Mode::LinesMarkers)
                        .marker(Marker::new().color(NamedColor::Red))
                        .name("Fitted curve");

                    plot.add_trace(trace);
                }

                plot.show();
            }
        }
    };
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// DISCOUNT CURVE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Discount curve data structure.
#[derive(Builder, Clone, Debug)]
pub struct DiscountCurve<I>
where
    I: CurveIndex,
{
    /// Map of dates and rates.
    pub curve: Curve<I>,

    /// Calendar.
    pub calendar: Option<Calendar>,

    /// Day count convention.
    pub day_count_convention: Option<DayCountConvention>,

    /// Date rolling convention.
    pub date_rolling_convention: Option<DateRollingConvention>,

    /// Nelson-Siegel-Svensson parameters.
    /// Backend for fitting the curve to interpolate missing rates.
    #[builder(default)]
    pub nss: NelsonSiegelSvensson,

    /// Whether the curve has been fitted.
    #[builder(default = "false")]
    pub fitted: bool,

    /// Fitted curve.
    #[builder(default)]
    pub fitted_curve: Option<Curve<I>>,
}

impl_specific_curve_cost_function!(DiscountCurve, discount_factor);
impl_specific_curve!(DiscountCurve, discount_factor);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// SPOT CURVE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Spot curve data structure.
#[derive(Builder, Clone, Debug)]
pub struct SpotCurve<I>
where
    I: CurveIndex,
{
    /// Map of dates and rates.
    pub curve: Curve<I>,

    /// Calendar.
    pub calendar: Option<Calendar>,

    /// Day count convention.
    pub day_count_convention: Option<DayCountConvention>,

    /// Date rolling convention.
    pub date_rolling_convention: Option<DateRollingConvention>,

    /// Nelson-Siegel-Svensson parameters.
    /// Backend for fitting the curve to interpolate missing rates.
    #[builder(default)]
    pub nss: NelsonSiegelSvensson,

    /// Whether the curve has been fitted.
    #[builder(default = "false")]
    pub fitted: bool,

    /// Fitted curve.
    #[builder(default)]
    pub fitted_curve: Option<Curve<I>>,
}

impl_specific_curve_cost_function!(SpotCurve, spot_rate);
impl_specific_curve!(SpotCurve, spot_rate);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FORWARD CURVE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Forward curve data structure.
#[derive(Builder, Clone, Debug)]
pub struct ForwardCurve<I>
where
    I: CurveIndex,
{
    /// Map of dates and rates.
    pub curve: Curve<I>,

    /// Calendar.
    pub calendar: Option<Calendar>,

    /// Day count convention.
    pub day_count_convention: Option<DayCountConvention>,

    /// Date rolling convention.
    pub date_rolling_convention: Option<DateRollingConvention>,

    /// Nelson-Siegel-Svensson parameters.
    /// Backend for fitting the curve to interpolate missing rates.
    #[builder(default)]
    pub nss: NelsonSiegelSvensson,

    /// Whether the curve has been fitted.
    #[builder(default = "false")]
    pub fitted: bool,

    /// Fitted curve.
    #[builder(default)]
    pub fitted_curve: Option<Curve<I>>,
}

impl_specific_curve_cost_function!(ForwardCurve, forward_rate);
impl_specific_curve!(ForwardCurve, forward_rate);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FLAT CURVE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Flat curve data structure.
#[derive(Builder, Clone, Debug)]
pub struct FlatCurve {
    /// Rate of the curve.
    pub rate: f64,

    /// Calendar.
    pub calendar: Option<Calendar>,

    /// Day count convention.
    pub day_count_convention: Option<DayCountConvention>,

    /// Date rolling convention.
    pub date_rolling_convention: Option<DateRollingConvention>,
}

impl FlatCurve {
    /// Create a new flat curve.
    pub fn new_flat_curve(rate: f64) -> Self {
        Self {
            rate,
            calendar: None,
            day_count_convention: None,
            date_rolling_convention: None,
        }
    }

    /// Get the rate of the curve.
    pub fn get_rate(&self) -> f64 {
        self.rate
    }

    /// Get rate for a specific date.
    pub fn get_rate_for_date(&self, _date: Date) -> f64 {
        self.rate
    }

    /// Get rates for multiple dates.
    pub fn get_rates_for_dates(&self, dates: &[Date]) -> Vec<f64> {
        vec![self.rate; dates.len()]
    }
}

// impl<C> Curves<C> for FlatCurve<C>
// where
//     C: Calendar,
// {
//     /// NOT TO BE USED. Prefer the `new_flat_curve()` method.
//     fn new(dates: &[Date], rates: &[f64]) -> Self {
//         unimplemented!("FlatCurve does not support this method. Use `new_flat_curve()` instead.")
//     }

//     fn initial_date(&self) -> Date {
//         Date::MIN
//     }

//     fn terminal_date(&self) -> Date {
//         Date::MAX
//     }

//     fn get_rate(&mut self, date: Date) -> f64 {
//         self.rate
//     }

//     fn get_rates(&mut self, dates: &[Date]) -> Vec<f64> {
//         vec![self.rate; dates.len()]
//     }

//     fn insert_rate(&mut self, date: Date, rate: f64) {
//         todo!()
//     }

//     fn fit(&mut self) -> Result<(), argmin::core::Error> {
//         unimplemented!()
//     }

//     fn plot(&self) {
//         unimplemented!()
//     }
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Base Curve Trait
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// /// Base trait for all curves to implement.
// pub trait Curve {
//     /// Initial date of the curve.
//     fn initial_date(&self) -> Date;

//     /// Final date of the curve.
//     fn terminal_date(&self) -> Date;

//     /// Updates the rate for the given date.
//     fn update_rate(&mut self, date: Date, rate: f64);

//     /// Create a new curve from a set of dates and rates.
//     fn from_dates_and_rates(dates: &[Date], rates: &[f64]) -> Self;

//     /// Create a new curve from an initial date, a set of rates, and a set of
//     /// durations.
//     /// The dates are calculated as the initial date plus the duration, thus
//     /// there must be:
//     /// - One initial date
//     /// - n rates
//     /// - n-1 durations
//     fn from_initial_date_rates_and_durations(
//         initial_date: Date,
//         rates: &[f64],
//         durations: &[Duration],
//     ) -> Self;

//     /// Function to find the interval of dates that contains the given date.
//     /// The interval is defined by the two dates that are closest to the given
//     /// date, just before and just after.
//     fn find_date_interval(&self, date: Date) -> (Date, Date);

//     /// Returns the rate for the given date, using linear interpolation for
//     /// dates between the curve's initial and terminal dates.
//     /// If the date is outside the curve's range, we panic.
//     ///
//     /// We use the following formula for the interpolation:
//     ///
//     /// $$
//     /// y = \frac{y_0 (x_1 - x) + y_1 (x - x_0)}{x_1 - x_0}
//     /// $$
//     ///
//     /// Note: there must be at least two points in the curve, otherwise
//     /// we consider the curve to be a flat rate, and return the same rate
//     /// for all dates.
//     fn rate(&self, date: Date) -> f64;

//     /// Returns the discount factor for the given date.
//     /// This is a convenience function that calls [`rate`](Curve::rate) to get the rate for
//     /// the given date, and then calculates the discount factor using the
//     /// formula:
//     /// $$
//     /// p(t) = e^{- r \cdot t}
//     /// $$
//     fn discount_factor(&self, date: Date) -> f64 {
//         let t = DayCountConvention::default().day_count_factor(self.initial_date(), date);

//         f64::exp(-self.rate(date) * t)
//     }

//     /// Returns multiple discount factors for the given dates.
//     /// This is a convenience function that calls [`discount_factor`](Curve::discount_factor) for each
//     /// date.
//     fn discount_factors(&self, dates: &[Date]) -> Vec<f64> {
//         dates
//             .iter()
//             .map(|date| self.discount_factor(*date))
//             .collect::<Vec<f64>>()
//     }
// }

// #[allow(clippy::module_name_repetitions)]
// /// Yield curve struct.
// pub struct YieldCurve {
//     /// Map of dates and rates.
//     /// The dates are the keys and the rates are the values.
//     /// The reason for using a [BTreeMap] is that it is sorted by date,
//     /// which makes sense for a term structure.
//     pub rates: BTreeMap<Date, f64>,
//     // /// A model for the curve.
//     // pub model: Option<M>,
// }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_curves {
    // use super::*;
    // use crate::time::today;
    // use time::Duration;
    // use time::OffsetDateTime;

    // #[test]
    // fn test_discount_curve_creation() {
    //     let dates = [today() + Duration::days(30), today() + Duration::days(60)];
    //     let rates = [0.025, 0.03];

    //     let discount_curve = DiscountCurve::new(&dates, &rates);

    //     assert_eq!(discount_curve.rates, rates);
    // }

    // #[test]
    // fn test_discount_curve_initial_date() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = DiscountCurve::new(&dates, &rates);
    //     let initial_date = discount_curve.initial_date();

    //     assert_eq!(
    //         initial_date,
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30)
    //     );
    // }

    // #[test]
    // fn test_discount_curve_final_date() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = DiscountCurve::new(&dates, &rates);
    //     let final_date = discount_curve.terminal_date();

    //     assert_eq!(
    //         final_date,
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60)
    //     );
    // }

    // #[test]
    // fn test_find_date_interval() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = DiscountCurve::new(&dates, &rates);

    //     let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30);
    //     let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
    //     let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60);

    //     let interval1 = discount_curve.find_date_interval(date1);
    //     let interval2 = discount_curve.find_date_interval(date2);
    //     let interval3 = discount_curve.find_date_interval(date3);

    //     assert_eq!(interval1, (date1, date1));
    //     assert_eq!(interval2, (date1, date3));
    //     assert_eq!(interval3, (date3, date3));
    // }

    // #[allow(clippy::similar_names)]
    // #[test]
    // fn test_discount_curve_discount_factor() {
    //     // Initial date of the curve.
    //     let t0 = OffsetDateTime::UNIX_EPOCH.date();

    //     // Create a discount curve with 8 points.
    //     let rate_vec = vec![0.025, 0.03, 0.035, 0.04, 0.045, 0.05, 0.055, 0.06];
    //     let date_vec = vec![
    //         t0 + Duration::days(30),
    //         t0 + Duration::days(60),
    //         t0 + Duration::days(90),
    //         t0 + Duration::days(120),
    //         t0 + Duration::days(150),
    //         t0 + Duration::days(180),
    //         t0 + Duration::days(210),
    //         t0 + Duration::days(360),
    //     ];

    //     let discount_curve = DiscountCurve::from_dates_and_rates(&date_vec, &rate_vec);

    //     println!("Curve: {:?}", discount_curve.rates);

    //     // Test the discount factor for a dates inside the curve's range.
    //     let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
    //     let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(80);
    //     let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(250);

    //     let df1 = discount_curve.discount_factor(date1);
    //     let df2 = discount_curve.discount_factor(date2);
    //     let df3 = discount_curve.discount_factor(date3);

    //     println!("df1: {:?}", df1);
    //     println!("df2: {:?}", df2);
    //     println!("df3: {:?}", df3);

    //     assert!(df1 > 0.0 && df1 < 1.0 && df2 > 0.0 && df2 < 1.0 && df3 > 0.0 && df3 < 1.0);

    //     assert!(df1 > df2 && df2 > df3);
    // }

    // #[test]
    // fn test_discount_curve_creation() {
    //     let dates = [today() + Duration::days(30), today() + Duration::days(60)];
    //     let rates = [0.025, 0.03];

    //     let discount_curve = ForwardCurve::new(&dates, &rates);

    //     assert_eq!(discount_curve.rates, rates);
    // }

    // #[test]
    // fn test_discount_curve_initial_date() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = ForwardCurve::new(&dates, &rates);
    //     let initial_date = discount_curve.initial_date();

    //     assert_eq!(
    //         initial_date,
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30)
    //     );
    // }

    // #[test]
    // fn test_discount_curve_final_date() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = ForwardCurve::new(&dates, &rates);
    //     let final_date = discount_curve.terminal_date();

    //     assert_eq!(
    //         final_date,
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60)
    //     );
    // }

    // #[test]
    // fn test_find_date_interval() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = ForwardCurve::new(&dates, &rates);

    //     let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30);
    //     let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
    //     let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60);

    //     let interval1 = discount_curve.find_date_interval(date1);
    //     let interval2 = discount_curve.find_date_interval(date2);
    //     let interval3 = discount_curve.find_date_interval(date3);

    //     assert_eq!(interval1, (date1, date1));
    //     assert_eq!(interval2, (date1, date3));
    //     assert_eq!(interval3, (date3, date3));
    // }

    // #[allow(clippy::similar_names)]
    // #[test]
    // fn test_discount_curve_discount_factor() {
    //     // Initial date of the curve.
    //     let t0 = OffsetDateTime::UNIX_EPOCH.date();

    //     // Create a discount curve with 8 points.
    //     let rate_vec = vec![0.025, 0.03, 0.035, 0.04, 0.045, 0.05, 0.055, 0.06];
    //     let date_vec = vec![
    //         t0 + Duration::days(30),
    //         t0 + Duration::days(60),
    //         t0 + Duration::days(90),
    //         t0 + Duration::days(120),
    //         t0 + Duration::days(150),
    //         t0 + Duration::days(180),
    //         t0 + Duration::days(210),
    //         t0 + Duration::days(360),
    //     ];

    //     let discount_curve = ForwardCurve::from_dates_and_rates(&date_vec, &rate_vec);

    //     println!("Curve: {:?}", discount_curve.rates);

    //     // Test the discount factor for a dates inside the curve's range.
    //     let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
    //     let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(80);
    //     let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(250);

    //     let df1 = discount_curve.discount_factor(date1);
    //     let df2 = discount_curve.discount_factor(date2);
    //     let df3 = discount_curve.discount_factor(date3);

    //     println!("df1: {:?}", df1);
    //     println!("df2: {:?}", df2);
    //     println!("df3: {:?}", df3);

    //     assert!(df1 > 0.0 && df1 < 1.0 && df2 > 0.0 && df2 < 1.0 && df3 > 0.0 && df3 < 1.0);

    //     assert!(df1 > df2 && df2 > df3);
    // }

    // #[test]
    // fn test_discount_curve_creation() {
    //     let dates = [today() + Duration::days(30), today() + Duration::days(60)];
    //     let rates = [0.025, 0.03];

    //     let discount_curve = SpotCurve::new(&dates, &rates);

    //     assert_eq!(discount_curve.rates, rates);
    // }

    // #[test]
    // fn test_discount_curve_initial_date() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = SpotCurve::new(&dates, &rates);
    //     let initial_date = discount_curve.initial_date();

    //     assert_eq!(
    //         initial_date,
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30)
    //     );
    // }

    // #[test]
    // fn test_discount_curve_final_date() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = SpotCurve::new(&dates, &rates);
    //     let final_date = discount_curve.terminal_date();

    //     assert_eq!(
    //         final_date,
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60)
    //     );
    // }

    // #[test]
    // fn test_find_date_interval() {
    //     let dates = [
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30),
    //         OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60),
    //     ];

    //     let rates = [0.025, 0.03];

    //     let discount_curve = SpotCurve::new(&dates, &rates);

    //     let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(30);
    //     let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
    //     let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(60);

    //     let interval1 = discount_curve.find_date_interval(date1);
    //     let interval2 = discount_curve.find_date_interval(date2);
    //     let interval3 = discount_curve.find_date_interval(date3);

    //     assert_eq!(interval1, (date1, date1));
    //     assert_eq!(interval2, (date1, date3));
    //     assert_eq!(interval3, (date3, date3));
    // }

    // #[allow(clippy::similar_names)]
    // #[test]
    // fn test_discount_curve_discount_factor() {
    //     // Initial date of the curve.
    //     let t0 = OffsetDateTime::UNIX_EPOCH.date();

    //     // Create a discount curve with 8 points.
    //     let rate_vec = vec![0.025, 0.03, 0.035, 0.04, 0.045, 0.05, 0.055, 0.06];
    //     let date_vec = vec![
    //         t0 + Duration::days(30),
    //         t0 + Duration::days(60),
    //         t0 + Duration::days(90),
    //         t0 + Duration::days(120),
    //         t0 + Duration::days(150),
    //         t0 + Duration::days(180),
    //         t0 + Duration::days(210),
    //         t0 + Duration::days(360),
    //     ];

    //     let discount_curve = SpotCurve::new(&date_vec, &rate_vec);

    //     println!("Curve: {:?}", discount_curve.rates);

    //     // Test the discount factor for a dates inside the curve's range.
    //     let date1 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(45);
    //     let date2 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(80);
    //     let date3 = OffsetDateTime::UNIX_EPOCH.date() + Duration::days(250);

    //     let df1 = discount_curve.discount_factor(date1);
    //     let df2 = discount_curve.discount_factor(date2);
    //     let df3 = discount_curve.discount_factor(date3);

    //     println!("df1: {:?}", df1);
    //     println!("df2: {:?}", df2);
    //     println!("df3: {:?}", df3);

    //     assert!(df1 > 0.0 && df1 < 1.0 && df2 > 0.0 && df2 < 1.0 && df3 > 0.0 && df3 < 1.0);

    //     assert!(df1 > df2 && df2 > df3);
    // }
}
