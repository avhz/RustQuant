// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Monte-Carlo pricer trait.

use crate::Payoff;
use RustQuant_stochastics::{StochasticProcess, StochasticProcessConfig};

/// Monte-Carlo pricer trait.
pub trait MonteCarloPricer<S>: Payoff
where
    S: StochasticProcess,
{
    /// Price the instrument using a Monte-Carlo method.
    ///
    /// # Arguments
    ///
    /// * `process` - The [StochasticProcess] to use for the sample paths.
    /// * `config` - The [StochasticProcessConfig] for the simulation.
    /// * `rate` - The interest rate used to discount the payoff.
    fn price_monte_carlo(&self, process: &S, config: &StochasticProcessConfig, rate: f64) -> f64;
}

/// Macro to implement `MonteCarloPricer` for a given instrument type.
#[macro_export]
macro_rules! impl_monte_carlo_pricer {
    ($type:ty, $underlying:expr) => {
        impl<S> MonteCarloPricer<S> for $type
        where
            S: StochasticProcess,
        {
            fn price_monte_carlo(
                &self,
                process: &S,
                config: &StochasticProcessConfig,
                rate: f64,
            ) -> f64 {
                let out = process.generate(&config);

                let n = out.paths.len();

                let df = (-rate * (config.t_n - config.t_0)).exp();

                let payoffs = out.paths.iter().fold(0.0, |acc, path| {
                    let underlying = $underlying(&*path);
                    let payoff = self.payoff(underlying);

                    acc + payoff
                });

                df * payoffs / n as f64
            }
        }
    };
}

fn path_independent(path: &[f64]) -> f64 {
    path.last().cloned().unwrap_or(0.0)
}

fn path_dependent(path: &[f64]) -> Vec<f64> {
    path.to_vec()
}

impl_monte_carlo_pricer!(crate::AsianOption, path_dependent);
impl_monte_carlo_pricer!(crate::BinaryOption, path_independent);
impl_monte_carlo_pricer!(crate::EuropeanVanillaOption, path_independent);
impl_monte_carlo_pricer!(crate::PowerContract, path_independent);
impl_monte_carlo_pricer!(crate::PowerOption, path_independent);
impl_monte_carlo_pricer!(crate::SupershareOption, path_independent);
impl_monte_carlo_pricer!(crate::BarrierOption, path_dependent);
impl_monte_carlo_pricer!(crate::CappedPowerOption, path_independent);
impl_monte_carlo_pricer!(crate::PoweredOption, path_independent);
impl_monte_carlo_pricer!(crate::LogMoneynessContract, path_independent);
impl_monte_carlo_pricer!(crate::LogUnderlyingContract, path_independent);
impl_monte_carlo_pricer!(crate::LogOption, path_independent);
