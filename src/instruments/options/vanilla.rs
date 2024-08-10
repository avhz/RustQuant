// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{OptionContract, TypeFlag};
use crate::{
    instruments::Payoff,
    pricer::MonteCarloPricer,
    stochastics::{StochasticProcess, StochasticProcessConfig},
};

/// Vanilla option.
#[derive(Debug, Clone)]
pub struct VanillaOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,
}

impl Payoff for VanillaOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match self.contract.type_flag {
            TypeFlag::Call => (underlying - self.strike).max(0.0),
            TypeFlag::Put => (self.strike - underlying).max(0.0),
        }
    }
}

impl VanillaOption {
    /// Create a new vanilla option.
    pub fn new(contract: OptionContract, strike: f64) -> Self {
        Self { contract, strike }
    }
}

impl<S> MonteCarloPricer<S> for VanillaOption
where
    S: StochasticProcess,
{
    fn price_monte_carlo(&self, process: S, config: StochasticProcessConfig, rate: f64) -> f64 {
        let out = process.euler_maruyama(&config);

        let n = out.paths.len();

        let df = (-rate * (config.t_n - config.t_0)).exp();

        out.paths.iter().fold(0.0, |acc, path| {
            let payoff = self.payoff(path.last().unwrap().clone());
            acc + df * payoff
        }) / n as f64
    }
}

#[cfg(test)]
mod test_vanilla_option_monte_carlo {
    use time::macros::date;

    use crate::{
        instruments::{ExerciseFlag, OptionContractBuilder},
        models::GeometricBrownianMotion,
    };

    use super::*;

    #[test]
    fn test_vanilla_option_monte_carlo() {
        let underlying = 100.0;
        let strike = 100.0;
        let interest_rate = 0.05;
        let time_to_maturity = 1.0;
        let volatility = 0.2;

        let contract = OptionContractBuilder::default()
            .type_flag(TypeFlag::Call)
            .exercise_flag(ExerciseFlag::European {
                expiry: date!(2025 - 01 - 01),
            })
            .build()
            .unwrap();

        let option = VanillaOption::new(contract, strike);
        let process = GeometricBrownianMotion::new(interest_rate, volatility);

        let config =
            StochasticProcessConfig::new(underlying, 0.0, time_to_maturity, 1000, 1000, false);

        let price = option.price_monte_carlo(process, config, interest_rate);

        println!("Price: {}", price);
    }
}

// impl Instrument for VanillaOption {
//     fn price(&self) -> f64 {
//         1.
//     }

//     fn error(&self) -> Option<f64> {
//         None
//     }

//     fn valuation_date(&self) -> Date {
//         todo!()
//     }

//     fn instrument_type(&self) -> &'static str {
//         todo!()
//     }
// }

// impl<C> Priceable<C> for VanillaOption
// where
//     C: Calendar + Clone,
// {
//     /// VanillaOption pricer implementation.
//     ///
//     /// This aksjdfoasj ofdjsod
//     fn pricer_impl(
//         &self,
//         context_data: &Option<ContextData<C>>,
//         market_data: &mut Option<MarketData<C>>,
//         // model: &Option<S>,
//         // engine: &Option<PricingEngine>,
//     ) -> f64 {
//         // let cal = context_data.as_ref().unwrap().calendar.as_ref().unwrap();
//         let eval = context_data.as_ref().unwrap().evaluation_date.unwrap();

//         let s = market_data.as_ref().unwrap().underlying_price.unwrap();
//         let k = self.strike;
//         let t = match self.contract.exercise_flag {
//             ExerciseFlag::European { expiry } => expiry,
//             ExerciseFlag::American { .. } => todo!(),
//             ExerciseFlag::Bermudan { .. } => todo!(),
//         };
//         // let tau = DayCounter::day_count_factor(
//         //     cal,
//         //     eval,
//         //     t,
//         //     &context_data.as_ref().unwrap().day_count_convention.unwrap(),
//         // );
//         let r = market_data
//             .as_mut()
//             .unwrap()
//             .spot_curve
//             .as_mut()
//             .unwrap()
//             .get_rate(t);
//         let v = market_data.as_ref().unwrap().volatility.unwrap();

//         let bsm = BlackScholesMerton {
//             cost_of_carry: r,
//             underlying_price: s,
//             strike_price: k,
//             volatility: v,
//             risk_free_rate: r,
//             evaluation_date: Some(eval),
//             expiration_date: t,
//             option_type: self.contract.type_flag,
//         };

//         bsm.price()
//     }
// }
