// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Market data container.

use crate::{DiscountCurve, FlatCurve, ForwardCurve, SpotCurve};
use derive_builder::Builder;
use time::Date;
use RustQuant_instruments::ExchangeRate;
use RustQuant_time::Calendar;

/// Market data.
#[derive(Builder, Clone, Debug)]
pub struct MarketData<C>
where
    C: Calendar,
{
    /// Underlying price.
    #[builder(default)]
    pub underlying_price: Option<f64>,

    /// Exchange rate.
    #[builder(default)]
    pub exchange_rate: Option<ExchangeRate>,

    /// Dividend yield.
    #[builder(default)]
    pub dividend_yield: Option<f64>,

    /// Volatility (implied).
    #[builder(default)]
    pub volatility: Option<f64>,

    /// Spot curve.
    #[builder(default)]
    pub spot_curve: Option<SpotCurve<Date, C>>,

    /// Discount curve.
    #[builder(default)]
    pub discount_curve: Option<DiscountCurve<Date, C>>,

    /// Forward curve.
    #[builder(default)]
    pub forward_curve: Option<ForwardCurve<Date, C>>,

    /// Flat curve.
    #[builder(default)]
    pub flat_curve: Option<FlatCurve<C>>,
}
