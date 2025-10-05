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
use plotly::{common::Mode, Plot, Scatter};
use pyo3::{pyclass, pymethods, PyResult};
use std::collections::BTreeMap;
use std::sync::Arc;
use ordered_float::OrderedFloat;
use RustQuant_math::interpolation::{ExponentialInterpolator, Interpolator, LinearInterpolator};
use RustQuant_stochastics::{CurveModel, NelsonSiegelSvensson};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BASE CURVE DATA STRUCTURE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Curve type enum.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[pyclass]
pub enum CurveType {
    /// Flat curve (constant for all dates).
    #[default]
    Flat,

    /// Spot (zero) curve.
    Spot,

    /// Forward curve.
    Forward,

    /// Discount curve.
    Discount,
}

/// Interpolation method enum.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[pyclass]
pub enum InterpolationMethod {
    /// Linear interpolation.
    #[default]
    Linear,

    /// Exponential interpolation.
    Exponential,

    /// Cubic-spline interpolation.
    CubicSpline,

    /// Lagrange interpolation.
    Lagrange,
}

/// Curve data structure.
#[derive(Clone, Builder)]
#[pyclass]
pub struct Curve {
    /// The nodes of the curve.
    nodes: BTreeMap<OrderedFloat<f64>, f64>,

    /// The type of the curve.
    curve_type: CurveType,

    /// The interpolation method used by the curve.
    interpolation_method: InterpolationMethod,

    /// Interpolator backend.
    interpolator: Arc<dyn Interpolator<f64, f64>>,

    /// Nelson-Siegel-Svensson curve parameters.
    nss: Option<NelsonSiegelSvensson>,
}

#[pymethods]
impl Curve {
    /// Create a new curve.
    #[new]
    pub fn new(
        dates: Vec<f64>,
        rates: Vec<f64>,
        curve_type: CurveType,
        interpolation_method: InterpolationMethod,
    ) -> PyResult<Self> {
        let interpolator: Arc<dyn Interpolator<f64, f64>> = match interpolation_method {
            InterpolationMethod::Linear => {
                Arc::new(LinearInterpolator::new(dates.clone(), rates.clone())?)
            }
            InterpolationMethod::Exponential => {
                Arc::new(ExponentialInterpolator::new(dates.clone(), rates.clone())?)
            }
            InterpolationMethod::CubicSpline => {
                todo!("Implement CubicSplineInterpolator")
            }
            InterpolationMethod::Lagrange => {
                todo!("Implement LagrangeInterpolator")
            }
        };            

        Ok(Self {
            nodes: dates.into_iter().zip(rates.into_iter()).map(|(a, b)| (OrderedFloat(a), b)).collect(),
            curve_type,
            interpolation_method,
            interpolator,
            nss: None,
        })
    }


    // /// Create a new Curve from a list of nodes.
    // #[staticmethod]
    // pub fn from_nodes(
    //     nodes: BTreeMap<Date, f64>,
    //     curve_type: CurveType,
    //     interpolation_method: InterpolationMethod,
    // ) -> PyResult<Self> {
    //     let interpolator: Arc<dyn Interpolator<Date, f64>> = match interpolation_method {
    //         InterpolationMethod::Linear => Arc::new(LinearInterpolator::new(
    //             nodes.keys().cloned().collect(),
    //             nodes.values().cloned().collect(),
    //         )?),
    //         InterpolationMethod::Exponential => Arc::new(ExponentialInterpolator::new(
    //             nodes.keys().cloned().collect(),
    //             nodes.values().cloned().collect(),
    //         )?),
    //         InterpolationMethod::CubicSpline => {
    //             todo!("Implement CubicSplineInterpolator")
    //         }
    //         InterpolationMethod::Lagrange => {
    //             todo!("Implement LagrangeInterpolator")
    //         }
    //     };

    //     Ok(Self {
    //         nodes,
    //         curve_type,
    //         interpolation_method,
    //         interpolator,
    //         nss: None,
    //     })
    // }

    /// Get the interpolation method used by the curve.
    pub fn interpolation_method(&self) -> InterpolationMethod {
        self.interpolation_method
    }

    /// Get a rate from the curve.
    pub fn get_rate(&self, date: f64) -> Option<f64> {
        match self.nodes.get(&OrderedFloat(date)) {
            Some(rate) => Some(*rate),
            None => self.interpolator.interpolate(date).ok(),
        }
    }

    /// Get a rate, and simultaneously add it to the nodes.
    pub fn get_rate_and_insert(&mut self, date: Date) -> Option<f64> {
        match self.nodes.get(&date) {
            Some(rate) => Some(*rate),
            None => {
                let rate = self.interpolator.interpolate(date).ok()?;
                self.nodes.insert(date, rate);
                Some(rate)
            }
        }
    }

    /// Get multiple rates from the curve.
    pub fn get_rates(&self, dates: Vec<f64>) -> Vec<Option<f64>> {
        dates.iter().map(|date| self.get_rate(*date)).collect()
    }

    /// Get multiple rates from the curve, and simultaneously add them to the nodes.
    pub fn get_rates_and_insert(&mut self, dates: Vec<Date>) -> Vec<Option<f64>> {
        dates
            .iter()
            .map(|date| self.get_rate_and_insert(*date))
            .collect()
    }

    /// Set a rate in the curve.
    pub fn set_rate(&mut self, date: f64, rate: f64) {
        self.nodes.insert(OrderedFloat(date), rate);
    }

    /// Set multiple rates in the curve.
    pub fn set_rates(&mut self, rates: Vec<(f64, f64)>) {
        for (date, rate) in rates {
            self.set_rate(date, rate);
        }
    }

    /// Get the first date in the curve.
    pub fn first_date(&self) -> Option<&f64> {
        match self.nodes.keys().next() {
            Some(date) => Some(&date.0),
            None => None,
        }
    }

    /// Get the last date in the curve.
    pub fn last_date(&self) -> Option<&f64> {
        match self.nodes.keys().next_back() {
            Some(date) => Some(&date.0),
            None => None,
        }
    }

    /// Get the dates of the curve.
    pub fn dates(&self) -> Vec<f64> {
        self.nodes.keys().map(|k| k.0).collect()//.cloned().collect()
    }

    /// Get the rates of the curve.
    pub fn rates(&self) -> Vec<f64> {
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

    /// Get the first rate in the curve.
    pub fn first_rate(&self) -> Option<&f64> {
        self.nodes.values().next()
    }

    /// Get the last rate in the curve.
    pub fn last_rate(&self) -> Option<&f64> {
        self.nodes.values().next_back()
    }

    /// Get the bracketing indices for a specific index.
    pub fn get_brackets(&self, index: f64) -> (f64, f64) {
        let first = self.first_date().unwrap();
        let last = self.last_date().unwrap();

        if index <= *first {
            return (*first, *first);
        }
        if index >= *last {
            return (*last, *last);
        }

        let left = self.nodes.range(..OrderedFloat(index)).next_back().unwrap().0;
        let right = self.nodes.range(OrderedFloat(index)..).next().unwrap().0;

        return (left.0, right.0);
    }

    /// Shift the curve by a constant value.
    pub fn shift(&mut self, shift: f64) {
        for value in self.nodes.values_mut() {
            *value += shift;
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
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// CURVE RELATED CONSTANTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl CostFunction for Curve {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, params: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        let nss = RustQuant_stochastics::NelsonSiegelSvensson::new(
            params[0], params[1], params[2], params[3], params[4], params[5],
        );

        let n = self.nodes.len() as f64;
        let x = self.nodes.keys();
        let y = self.nodes.values();

        let curve_function = match self.curve_type {
            CurveType::Discount => NelsonSiegelSvensson::discount_factor,
            CurveType::Forward => NelsonSiegelSvensson::forward_rate,
            CurveType::Spot => NelsonSiegelSvensson::spot_rate,
            CurveType::Flat => panic!("Flat curve does not need fitting."),
        };

        let y_model = x
            .into_iter()
            .map(|date| curve_function(&nss, **date))
            .collect::<Vec<f64>>();

        let data = std::iter::zip(y, y_model);

        // let mae = data.map(|(o, p)| (o - p).abs()).sum::<f64>() / n;
        // let mse = data.map(|(o, p)| (o - p).powi(2)).sum::<f64>() / n;
        let log_cosh_loss = data.map(|(o, p)| (p - o).cosh().ln()).sum::<f64>() / n;

        Ok(log_cosh_loss)
    }
}

impl Curve {
    /// Fit a Nelson-Siegel-Svensson curve to the data.
    fn fit_nss(&mut self) -> Result<(), argmin::core::Error> {
        const CURVE_OPTIM_MAX_ITER: u64 = 69;
        const CURVE_OPTIM_SWARM_SIZE: usize = 1000;

        let zero = f64::EPSILON;

        let bounds = [
            (zero, 0.3), // Beta_0
            (-0.3, 0.3), // Beta_1
            (-1.0, 1.0), // Beta_2
            (-1.0, 1.0), // Beta_3
            (zero, 5.0), // Tau_1
            (zero, 5.0), // Tau_2
        ]
        .to_vec()
        .into_iter()
        .map(|(a, b)| (a, b))
        .collect();

        let model = self.clone();

        //-0.3593 -0.7238 2.0289
        // .with_inertia_factor(0.1)? // Inertia factor (w)
        // .with_cognitive_factor(2.)? // Cognitive (personal) factor
        // .with_social_factor(2.)?; // Social (global) factor
        let solver = ParticleSwarm::new(bounds, CURVE_OPTIM_SWARM_SIZE);

        // let solver = SimulatedAnnealing::new(100_f64);
        let executor =
            Executor::new(model, solver).configure(|state| state.max_iters(CURVE_OPTIM_MAX_ITER));

        let result = executor.run()?;
        let params = result.state().get_best_param().unwrap().position.to_vec();

        self.nss = Some(NelsonSiegelSvensson::new(
            params[0], params[1], params[2], params[3], params[4], params[5],
        ));

        println!("TIME: {:?}", result.state().get_time());

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// DISCOUNT CURVE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// /// Discount curve data structure.
// #[derive(Builder, Clone, Debug)]
// #[pyclass]
// pub struct DiscountCurve {
//     /// Map of dates and rates.
//     pub curve: Curve,

//     /// Calendar.
//     pub calendar: Option<Calendar>,

//     /// Day count convention.
//     pub day_count_convention: Option<DayCountConvention>,

//     /// Date rolling convention.
//     pub date_rolling_convention: Option<DateRollingConvention>,

//     /// Nelson-Siegel-Svensson parameters.
//     /// Backend for fitting the curve to interpolate missing rates.
//     #[builder(default)]
//     pub nss: NelsonSiegelSvensson,

//     /// Whether the curve has been fitted.
//     #[builder(default = "false")]
//     pub fitted: bool,

//     /// Fitted curve.
//     #[builder(default)]
//     pub fitted_curve: Option<Curve>,
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
