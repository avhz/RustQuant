// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Binary option.
#[derive(Debug, Clone)]
pub struct BinaryOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Type of binary option.
    pub binary_type: BinaryType,
}

impl Payoff for BinaryOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match self.binary_type {
            BinaryType::CashOrNothing => match self.contract.type_flag {
                TypeFlag::Call => match underlying > self.strike {
                    true => self.strike,
                    false => 0.0,
                },
                TypeFlag::Put => match underlying < self.strike {
                    true => self.strike,
                    false => 0.0,
                },
            },
            BinaryType::AssetOrNothing => match self.contract.type_flag {
                TypeFlag::Call => match underlying > self.strike {
                    true => underlying,
                    false => 0.0,
                },
                TypeFlag::Put => match underlying < self.strike {
                    true => underlying,
                    false => 0.0,
                },
            },
        }
    }
}
