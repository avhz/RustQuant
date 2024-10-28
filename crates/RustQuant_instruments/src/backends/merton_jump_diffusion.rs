// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Merton (1976) jump diffusion model
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::TypeFlag;
use crate::instruments::BlackScholesMerton;
use crate::time::{today, DayCountConvention};
use time::Date;

/// Merton (1976) jump diffusion model parameters.
#[allow(clippy::module_name_repetitions)]
#[derive(derive_builder::Builder, Debug)]
pub struct Merton1976 {
    /// `underlying_price` - Initial price of the underlying.
    pub underlying_price: f64,

    /// `strike_price` - The option's strike price.
    pub strike_price: f64,

    /// `risk_free_rate` - Risk-free rate parameter.
    pub risk_free_rate: f64,

    /// `volatility` - Volatility parameter.
    pub volatility: f64,

    /// `lambda` - The expected number of jumps per year.
    pub lambda: f64,

    /// `gamma` - The percentage of the total volatility explained by jumps.
    pub gamma: f64,

    /// `type_flag` - The option direction (call or put).
    pub type_flag: TypeFlag,

    /// `evaluation_date` - Valuation date.
    #[builder(default = "None")]
    pub evaluation_date: Option<Date>,

    /// `expiration_date` - Valuation date.
    pub expiration_date: Date,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MERTON (1976) JUMP DIFFUSION OPTION PRICING
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Merton1976 {
    /// Merton (1976) Jump Diffusion Option Price formula.
    #[must_use]
    pub fn price(&self) -> f64 {
        let mut bsm = BlackScholesMerton::new(
            self.risk_free_rate,
            self.underlying_price,
            self.strike_price,
            self.volatility,
            self.risk_free_rate,
            self.evaluation_date,
            self.expiration_date,
            self.type_flag,
        );

        let tau = DayCountConvention::default().day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        );

        let mut price = 0_f64;

        for i in 0..20 {
            bsm.volatility = Self::sigma(self, i, tau);

            let factorial: usize = (1..=i).product();
            let numerator = f64::exp(-self.lambda * tau) * f64::powi(self.lambda * tau, i as i32);

            price += bsm.price() * numerator / factorial as f64;
        }

        price
    }

    fn sigma(&self, i: usize, tau: f64) -> f64 {
        f64::sqrt(f64::powi(Self::z(self), 2) + f64::powi(Self::delta(self), 2) * i as f64 / tau)
    }

    fn delta(&self) -> f64 {
        f64::sqrt(f64::powi(self.volatility, 2) * self.gamma / self.lambda)
    }

    fn z(&self) -> f64 {
        f64::sqrt(f64::powi(self.volatility, 2) - self.lambda * f64::powi(Self::delta(self), 2))
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_merton_1976 {
    use crate::{assert_approx_equal, instruments::TypeFlag};

    use super::*;

    #[test]
    fn test_merton_1976() {
        let merton76 = Merton1976 {
            underlying_price: 100.,
            strike_price: 80.,
            risk_free_rate: 0.08,
            volatility: 0.25,
            lambda: 1.,
            gamma: 0.25,
            type_flag: TypeFlag::Call,
            evaluation_date: None,
            expiration_date: today() + time::Duration::days(36),
        };

        // Price example from Haug's book.
        // Result is slightly off due to using Dates instead of floats for T.
        assert_approx_equal!(merton76.price(), 20.67, 0.1);
    }
}
