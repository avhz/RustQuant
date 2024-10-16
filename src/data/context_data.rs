// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Contextual (reference) data container.

use crate::instruments::Currency;
use crate::time::{
    Calendar, DateGenerationConvention, DateRollingConvention, DayCountConvention, Frequency,
    Schedule,
};
use ::time::Date;
use derive_builder::Builder;

/// Contextual (reference) data.
#[derive(Builder, Clone)]
pub struct ContextData<C>
where
    C: Calendar,
{
    /// Calendar object.
    #[builder(default)]
    pub calendar: Option<C>,

    /// Evaluation date.
    #[builder(default)]
    pub evaluation_date: Option<Date>,

    /// Currency.
    #[builder(default)]
    pub currency: Option<Currency>,

    /// Frequency.
    #[builder(default)]
    pub frequency: Option<Frequency>,

    /// Schedule.
    #[builder(default)]
    pub schedule: Option<Schedule>,

    /// Day count convention.
    #[builder(default)]
    pub day_count_convention: Option<DayCountConvention>,

    /// Date rolling convention.
    #[builder(default)]
    pub date_rolling_convention: Option<DateRollingConvention>,

    /// Date generation convention.
    #[builder(default)]
    pub date_generation_convention: Option<DateGenerationConvention>,
}
