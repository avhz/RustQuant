// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Pricer module.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MODULES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub mod monte_carlo_pricer;
pub use monte_carlo_pricer::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// PRICER STRUCT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Pricing engine for instruments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PricingMethod {
    /// Analytic pricing method (e.g. closed-form solution).
    Analytic,

    /// Simulation pricing method (e.g. Monte Carlo).
    Simulation,

    /// Numerical method (e.g. PDE, lattice, finite differences).
    Numerical,
}

// /// Pricer type.
// /// This is the main struct for pricing financial instruments.
// #[derive(Builder)]
// pub struct Pricer<C, P>
// where
//     C: Calendar,
//     P: Priceable<C>,
// {
//     /// The instrument to be priced.
//     pub instrument: P,

//     /// Contextual (reference) data for the pricing.
//     pub context_data: Option<ContextData<C>>,

//     /// Market data for the pricing.
//     pub market_data: Option<MarketData<C>>,

//     /// Pricing engine.
//     pub method: Option<PricingMethod>,
//     // /// The model to be used to price the instrument.
//     // pub model: Option<S>,
// }

// impl<C, P> Pricer<C, P>
// where
//     C: Calendar,
//     P: Priceable<C>,
// {
//     /// Compute the Net Present Value (NPV) of the instrument.
//     pub fn npv(&mut self) -> f64 {
//         match self.method {
//             Some(PricingMethod::Analytic) => self.instrument.price_analytic_impl(),
//             Some(PricingMethod::Simulation) => self.instrument.price_simulation_impl(),
//             Some(PricingMethod::Numerical) => self.instrument.price_numerical_impl(),
//             None => {
//                 // Default to analytic pricing.
//                 self.instrument.price_analytic_impl()
//             }
//         }
//     }

//     // self.instrument.pricer_impl(
//     //     &self.context_data,
//     //     &mut self.market_data,
//     //     // &self.model,
//     //     // &self.engine,
//     // )
//     // }
// }
