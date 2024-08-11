// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Analytic pricer trait.

use std::any::TypeId;

use crate::{
    instruments::VanillaOption,
    models::{ArithmeticBrownianMotion, GeometricBrownianMotion},
    stochastics::{process, StochasticProcess},
};

/// Analytic pricer trait.
pub trait AnalyticPricer<S>
where
    S: StochasticProcess,
{
    /// Price the instrument using an analytic solution.
    ///
    /// # Arguments
    ///
    /// * `process` - The [StochasticProcess] model to use for the price.
    fn price_analytic(&self, process: S) -> f64;
}

impl<S> AnalyticPricer<S> for VanillaOption
where
    S: StochasticProcess + 'static,
{
    fn price_analytic(&self, process: S) -> f64 {
        let process_id = TypeId::of::<S>();

        if process_id == TypeId::of::<GeometricBrownianMotion>() {
            let params = process.parameters();
            let mu = params[0];
            let sigma = params[1];

            println!(
                "Geometric Brownian Motion detected. mu: {}, sigma: {}",
                mu, sigma
            );

            // Implement the specific analytic pricing for Geometric Brownian Motion
            // Example: return some calculation
            return 2.0; // Placeholder value for demonstration
        } else if process_id == TypeId::of::<ArithmeticBrownianMotion>() {
            // Add another specific process handling here
            println!("Another process detected.");

            // Implement another pricing logic for `AnotherProcess`
            return 3.0; // Placeholder value
        } else {
            println!("Unknown process.");
            // Handle the case where the process type is unknown or not handled
            return 0.0;
        }
    }
}

#[cfg(test)]
mod tests_analytic_pricer {
    use time::macros::date;

    use super::*;
    use crate::{
        instruments::{ExerciseFlag, OptionContract, TypeFlag},
        models::model_parameter::ModelParameter,
    };

    #[test]
    fn test_price_analytic() {
        let contract = OptionContract {
            type_flag: TypeFlag::Call,
            exercise_flag: ExerciseFlag::European {
                expiry: date!(2025 - 01 - 01),
            },
            strike_flag: None,
            settlement_flag: None,
        };
        let option = VanillaOption::new(contract, 1.0);
        let process = ArithmeticBrownianMotion::new(0.05, 0.2);

        let price = option.price_analytic(process);

        assert_eq!(price, 0.0);
    }
}
