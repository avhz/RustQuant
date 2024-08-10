// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Priceable trait.

use super::{ContextData, MarketData};
use crate::{
    instruments::{Instrument, Payoff},
    stochastics::process::StochasticProcess,
    time::Calendar,
};

/// Priceable trait.
pub trait Priceable<C, S, P>: Payoff + Instrument
where
    C: Calendar,
    S: StochasticProcess,
    P: Payoff,
{
    /// Function to prepare the data for the specific instrument.
    fn prepare_data(&self) -> ();

    /// Analytic pricer implementation.
    fn price_analytic_impl(
        &self,
        context_data: &Option<ContextData<C>>,
        market_data: &mut Option<MarketData<C>>,
        model: &Option<S>,
        // engine: &Option<PricingEngine>,
    ) -> f64;

    /// Simulation pricer implementation.
    fn price_simulation_impl(
        &self,
        context_data: &Option<ContextData<C>>,
        market_data: &mut Option<MarketData<C>>,
        model: &Option<S>,
        // engine: &Option<PricingEngine>,
    ) -> f64;

    /// Numerical pricer implementation.
    fn price_numerical_impl(
        &self,
        context_data: &Option<ContextData<C>>,
        market_data: &mut Option<MarketData<C>>,
        model: &Option<S>,
        // engine: &Option<PricingEngine>,
    ) -> f64;
}
