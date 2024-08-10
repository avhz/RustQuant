// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Asian option.
#[derive(Debug, Clone)]
pub struct AsianOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Averging method (arithmetic or geometric).
    pub averaging_method: AveragingMethod,

    /// Strike price of the option.
    /// Required for fixed strike Asian options.
    pub strike: Option<f64>,
}

impl Payoff for AsianOption {
    type Underlying = Vec<f64>;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        let n = underlying.len();
        let path = underlying.iter();
        let terminal = underlying[n - 1];

        let average = match self.averaging_method {
            AveragingMethod::ArithmeticDiscrete => path.sum::<f64>() / n as f64,
            AveragingMethod::GeometricDiscrete => path.product::<f64>().powf(1.0 / n as f64),

            // Continuous averaging (i.e. integral of the path).
            _ => panic!("Continuous averaging not implemented."),
        };

        match self.contract.strike_flag {
            StrikeFlag::Fixed => match self.contract.type_flag {
                TypeFlag::Call => (average - self.strike.unwrap_or_default()).max(0.0),
                TypeFlag::Put => (self.strike.unwrap_or_default() - average).max(0.0),
            },
            StrikeFlag::Floating => match self.contract.type_flag {
                TypeFlag::Call => (terminal - average).max(0.0),
                TypeFlag::Put => (average - terminal).max(0.0),
            },
        }
    }
}
