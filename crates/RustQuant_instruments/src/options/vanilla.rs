// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::{
    Asay82, Black76, BlackScholes73, GarmanKohlhagen83, GeneralisedBlackScholesMerton, Merton73,
    TypeFlag,
};
use super::{Bachelier, Heston93};
use crate::AnalyticOptionPricer;
use crate::Payoff;
use derive_builder::Builder;
use time::Date;
use RustQuant_time::{today, year_fraction};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// European vanilla option.
#[derive(Debug, Clone, Builder, Copy)]
pub struct EuropeanVanillaOption {
    /// The strike price of the option.
    pub strike: f64,

    /// The expiry date of the option.
    pub expiry: Date,

    /// The type of the option (call or put).
    pub type_flag: TypeFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Payoff for EuropeanVanillaOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match self.type_flag {
            TypeFlag::Call => (underlying - self.strike).max(0.0),
            TypeFlag::Put => (self.strike - underlying).max(0.0),
        }
    }
}

impl EuropeanVanillaOption {
    /// Create a new vanilla option.
    pub fn new(strike: f64, expiry: Date, type_flag: TypeFlag) -> Self {
        Self {
            strike,
            expiry,
            type_flag,
        }
    }
}

macro_rules! european_vanilla_option_gbsm {
    ($gbsm_variant:ident) => {
        impl AnalyticOptionPricer<EuropeanVanillaOption, $gbsm_variant> {
            /// Print a report of the option price and greeks.
            pub fn report(&self) {
                use std::collections::HashMap;

                let map = HashMap::from([
                    ("price", self.price()),
                    ("delta", self.delta()),
                    ("gamma", self.gamma()),
                    ("theta", self.theta()),
                    ("vega", self.vega()),
                    ("rho", self.rho()),
                    ("vanna", self.vanna()),
                    ("charm", self.charm()),
                    ("lambda", self.lambda()),
                    ("zomma", self.zomma()),
                    ("speed", self.speed()),
                    ("color", self.color()),
                    ("vomma", self.vomma()),
                    ("ultima", self.ultima()),
                ]);

                println!("Model: {:?}", self.model);
                println!("Option: {:?}", self.option);
                println!("{:#?}", map);
                println!();
            }

            /// Calculate the price of the option.
            pub fn price(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.price(k, t, f)
            }

            /// Calculate the delta of the option.
            pub fn delta(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.delta(k, t, f)
            }

            /// Calculate the gamma of the option.
            pub fn gamma(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.gamma(k, t, f)
            }

            /// Calculate the theta of the option.
            pub fn theta(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.theta(k, t, f)
            }

            /// Calculate the vega of the option.
            pub fn vega(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.vega(k, t, f)
            }

            /// Calculate the rho of the option.
            pub fn rho(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.rho(k, t, f)
            }

            /// Calculate the vanna of the option.
            pub fn vanna(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.vanna(k, t, f)
            }

            /// Calculate the charm of the option.
            pub fn charm(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.charm(k, t, f)
            }

            /// Calculate the lambda of the option.
            pub fn lambda(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.lambda(k, t, f)
            }

            /// Calculate the zomma of the option.
            pub fn zomma(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.zomma(k, t, f)
            }

            /// Calculate the speed of the option.
            pub fn speed(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.speed(k, t, f)
            }

            /// Calculate the color of the option.
            pub fn color(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.color(k, t, f)
            }

            /// Calculate the vomma of the option.
            pub fn vomma(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.vomma(k, t, f)
            }

            /// Calculate the ultima of the option.
            pub fn ultima(&self) -> f64 {
                let k = self.option.strike;
                let t = year_fraction(today(), self.option.expiry);
                let f = self.option.type_flag;

                self.model.ultima(k, t, f)
            }
        }
    };
}

european_vanilla_option_gbsm!(BlackScholes73);
european_vanilla_option_gbsm!(Merton73);
european_vanilla_option_gbsm!(Black76);
european_vanilla_option_gbsm!(Asay82);
european_vanilla_option_gbsm!(GarmanKohlhagen83);

impl AnalyticOptionPricer<EuropeanVanillaOption, Heston93> {
    /// Calculate the price of the option.
    pub fn price(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.price(k, t, f)
    }

    /// Calculate the delta of the option.
    pub fn delta(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.delta(k, t, f)
    }

    /// Calculate the gamma of the option.
    pub fn gamma(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.gamma(k, t, f)
    }

    /// Calculate the rho of the option.
    pub fn rho(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.rho(k, t, f)
    }

    /// Print a report of the option price and greeks.
    pub fn report(&self) {
        use std::collections::HashMap;

        let map = HashMap::from([
            ("price", self.price()),
            ("delta", self.delta()),
            ("gamma", self.gamma()),
            ("rho", self.rho()),
        ]);

        println!("Model: {:?}", self.model);
        println!("Option: {:?}", self.option);
        println!("{:#?}", map);
        println!();
    }
}

impl AnalyticOptionPricer<EuropeanVanillaOption, Bachelier> {
    /// Calculate the price of the option.
    pub fn price(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.price(k, t, f)
    }

    /// Calculate the atm price of the option.
    pub fn atm_price(&self) -> f64 {
        let t = year_fraction(today(), self.option.expiry);

        self.model.atm_price(t)
    }

    /// Calculate the atm vol of the option.
    pub fn atm_vol(&self, price: f64) -> f64 {
        let t = year_fraction(today(), self.option.expiry);

        self.model.atm_vol(price, t)
    }

    /// Calculate the implied volatility of the option.
    pub fn iv(&self, price: f64) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.iv(price, k, t, f)
    }

    /// Calculate the delta of the option.
    pub fn delta(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.delta(k, t, f)
    }

    /// Calculate the gamma of the option.
    pub fn gamma(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.gamma(k, t, f)
    }

    /// Calculate the theta of the option.
    pub fn theta(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.theta(k, t, f)
    }

    /// Calculate the vega of the option.
    pub fn vega(&self) -> f64 {
        let k = self.option.strike;
        let t = year_fraction(today(), self.option.expiry);
        let f = self.option.type_flag;

        self.model.vega(k, t, f)
    }

    /// Print a report of the option price and greeks.
    pub fn report(&self) {
        use std::collections::HashMap;

        let map = HashMap::from([
            ("price", self.price()),
            ("atm_price", self.atm_price()),
            ("delta", self.delta()),
            ("gamma", self.gamma()),
            ("theta", self.theta()),
            ("vega", self.vega()),
        ]);

        println!("Model: {:?}", self.model);
        println!("Option: {:?}", self.option);
        println!("{:#?}", map);
        println!();
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_vanilla_option_monte_carlo {
    use super::*;
    use crate::AsianOption;
    use crate::AveragingMethod;
    use crate::MonteCarloPricer;
    use crate::StrikeFlag;
    use crate::{ExerciseFlag, OptionContractBuilder};
    use std::time::Instant;
    use time::macros::date;
    use RustQuant_stochastics::geometric_brownian_motion::GeometricBrownianMotion;
    use RustQuant_stochastics::StochasticProcessConfig;

    #[test]
    fn test_vanilla_option_monte_carlo() {
        let underlying = 100.0;
        let strike = 100.0;
        let interest_rate = 0.05;
        let time_to_maturity = 1.0;
        let volatility = 0.2;
        let expiry = date!(2025 - 01 - 01);

        // let contract = OptionContractBuilder::default()
        //     .type_flag(TypeFlag::Call)
        //     .exercise_flag(ExerciseFlag::European { expiry })
        //     .build()
        //     .unwrap();

        let option = EuropeanVanillaOption::new(strike, expiry, TypeFlag::Call);
        let process = GeometricBrownianMotion::new(interest_rate, volatility);

        let config =
            StochasticProcessConfig::new(underlying, 0.0, time_to_maturity, 1, 1_000_000, true);

        let start = Instant::now();
        let price = option.price_monte_carlo(&process, &config, interest_rate);
        println!("Elapsed time: {:?}", start.elapsed());

        println!("Price: {}", price);
    }

    #[test]
    fn test_asian_option_monte_carlo() {
        let underlying = 100.0;
        let strike = 100.0;
        let interest_rate = 0.05;
        let time_to_maturity = 1.0;
        let volatility = 0.2;
        let expiry = date!(2025 - 01 - 01);
        let direction = TypeFlag::Call;
        let exercise = ExerciseFlag::European { expiry };
        let strike_flag = Some(StrikeFlag::Fixed);

        let contract = OptionContractBuilder::default()
            .type_flag(direction)
            .exercise_flag(exercise)
            .strike_flag(strike_flag)
            .build()
            .unwrap();

        let option = AsianOption::new(contract, AveragingMethod::ArithmeticDiscrete, Some(strike));
        let process = GeometricBrownianMotion::new(interest_rate, volatility);

        let config =
            StochasticProcessConfig::new(underlying, 0.0, time_to_maturity, 1000, 1000, true);

        let start = Instant::now();
        let price = option.price_monte_carlo(&process, &config, interest_rate);
        println!("Elapsed time: {:?}", start.elapsed());

        println!("Price: {}", price);
    }
}
