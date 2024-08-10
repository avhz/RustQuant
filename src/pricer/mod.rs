// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Pricer module.

use crate::data::CurveIndex;
use crate::instruments::PricingEngine;
use crate::pricer::priceable::Priceable;
use crate::stochastics::StochasticProcess;
use crate::time::Calendar;
use derive_builder::Builder;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MODULES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub mod context_data;
pub use context_data::*;

pub mod market_data;
pub use market_data::*;

pub mod priceable;
pub use priceable::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// PRICER STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Pricer type.
/// This is the main struct for pricing financial instruments.
#[derive(Builder)]
pub struct Pricer<C, P>
where
    C: Calendar,
    P: Priceable<C>,
{
    /// The instrument to be priced.
    pub instrument: P,

    /// Contextual (reference) data for the pricing.
    pub context_data: Option<ContextData<C>>,

    /// Market data for the pricing.
    pub market_data: Option<MarketData<C>>,
    // /// The model to be used to price the instrument.
    // pub model: Option<S>,
    // /// Pricing engine.
    // pub engine: Option<PricingEngine>,
}

impl<C, P> Pricer<C, P>
where
    C: Calendar,
    P: Priceable<C>,
{
    /// Compute the Net Present Value (NPV) of the instrument.
    pub fn npv(&mut self) -> f64 {
        self.instrument.pricer_impl(
            &self.context_data,
            &mut self.market_data,
            // &self.model,
            // &self.engine,
        )
    }
}
