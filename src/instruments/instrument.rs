// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::OffsetDateTime;

/// Instrument trait
/// The trait provides a common interface for all instruments.
/// All instruments can be queried for their net present value (NPV) and
/// error (if available).
/// The valuation date is the date at which the instrument's NPV is
/// being calculated; for most instruments it is the trade date, for
/// some exotic products it might be the exercise date.
pub trait Instrument {
    /// Returns the price (net present value) of the instrument.
    fn price(&self) -> f64;

    /// Returns the error on the NPV in case the pricing engine can
    /// provide it (e.g. Monte Carlo pricing engine).
    fn error(&self) -> Option<f64>;

    /// Returns the date at which the NPV is calculated.
    fn valuation_date(&self) -> OffsetDateTime;

    /// Instrument type.
    fn instrument_type(&self) -> &'static str;
}

// /// Price structure.
// pub struct Price {
//     /// Price of the instrument.
//     pub price: f64,

//     /// Error on the price of the instrument.
//     pub error: Option<f64>,
// }

// /// Pricing engine for instruments.
// pub enum PricingEngine {
//     /// Analytic pricing method (e.g. closed-form solution).
//     Analytic,

//     /// Simulation pricing method (e.g. Monte Carlo).
//     Simulation,

//     /// Numerical method (e.g. PDE, lattice, finite differences).
//     Numerical,
// }

// /// Base instrument type enum.
// pub enum InstrumentType {
//     /// A stock instrument.
//     Stock,
//     /// A bond instrument.
//     Bond,
//     /// An option contract.
//     Option,
//     /// A future contract.
//     Future,
//     /// A swap contract.
//     Swap,
//     /// Commodities.
//     Commodity,
//     /// A hybrid instrument (e.g. callable bond).
//     Hybrid,
// }
