// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Supershare option.
#[derive(Debug, Clone)]
pub struct SupershareOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Lower strike price.
    pub strike_1: f64,

    /// Upper strike price.
    pub strike_2: f64,
}

impl Payoff for SupershareOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match (strike_1..=strike_2).contains(&underlying) {
            true => underlying / strike_1,
            false => 0.0,
        }
    }
}
