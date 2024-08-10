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
use crate::data::curves::Curves;
use crate::{
    data::CurveIndex,
    instruments::{BlackScholesMerton, ExerciseFlag, Instrument, PricingEngine, VanillaOption},
    stochastics::StochasticProcess,
    time::{Calendar, DayCounter},
};

/// Priceable trait.
pub trait Priceable<C>
where
    C: Calendar,
{
    /// Backend method for pricing the instrument.
    fn pricer_impl(
        &self,
        context_data: &Option<ContextData<C>>,
        market_data: &mut Option<MarketData<C>>,
        // model: &Option<S>,
        // engine: &Option<PricingEngine>,
    ) -> f64;
}

impl<C> Priceable<C> for VanillaOption
where
    C: Calendar + Clone,
{
    /// VanillaOption pricer implementation.
    ///
    /// This aksjdfoasj ofdjsod
    fn pricer_impl(
        &self,
        context_data: &Option<ContextData<C>>,
        market_data: &mut Option<MarketData<C>>,
        // model: &Option<S>,
        // engine: &Option<PricingEngine>,
    ) -> f64 {
        let cal = context_data.as_ref().unwrap().calendar.as_ref().unwrap();
        let eval = context_data.as_ref().unwrap().evaluation_date.unwrap();

        let s = market_data.as_ref().unwrap().underlying_price.unwrap();
        let k = self.strike;
        let t = match self.contract.exercise_flag {
            ExerciseFlag::European { expiry } => expiry,
            ExerciseFlag::American { .. } => todo!(),
            ExerciseFlag::Bermudan { .. } => todo!(),
        };
        let tau = DayCounter::day_count_factor(
            cal,
            eval,
            t,
            &context_data.as_ref().unwrap().day_count_convention.unwrap(),
        );
        let r = market_data
            .as_mut()
            .unwrap()
            .spot_curve
            .as_mut()
            .unwrap()
            .get_rate(t);
        let v = market_data.as_ref().unwrap().volatility.unwrap();

        let bsm = BlackScholesMerton {
            cost_of_carry: r,
            underlying_price: s,
            strike_price: k,
            volatility: v,
            risk_free_rate: r,
            evaluation_date: Some(eval),
            expiration_date: t,
            option_type: self.contract.type_flag,
        };

        bsm.price()
    }
}

#[cfg(test)]
mod test_pricer {}
