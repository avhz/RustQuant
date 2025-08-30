// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! The generalised Black-Scholes-Merton European Option pricing model.
//!
//! The differing cost of carry factor allows for the following models:
//! - b = r
//!     - Black-Scholes 1973 stock option model.
//! - b = r - q
//!     - Merton 1973 stock option model with continuous dividend yield.
//! - b = 0
//!     - Black 1976 futures option model.
//! - b = 0, r = 0
//!     - Asay 1982 margined futures option model.
//! - b = r_d - r_f
//!     - Garman and Kohlhagen 1983 currency option model.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::options::TypeFlag;
use crate::Instrument;
use time::Date;
use RustQuant_math::distributions::{Distribution, Gaussian};
use RustQuant_time::{utilities::today, DayCountConvention};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Generalised Black-Scholes-Merton European Option pricing model.
#[derive(derive_builder::Builder)]
pub struct BlackScholesMerton {
    /// The cost of carry factor.
    /// For the generalised Black-Scholes-Merton model there are five options:
    /// - b = r
    ///     - Black-Scholes 1973 stock option model.
    /// - b = r - q
    ///     - Merton 1973 stock option model with continuous dividend yield.
    /// - b = 0
    ///     - Black 1976 futures option model.
    /// - b = 0, r = 0
    ///     - Asay 1982 margined futures option model.
    /// - b = r_d - r_f
    ///     - Garman and Kohlhagen 1983 currency option model.
    pub cost_of_carry: f64,
    /// S - The underlying asset price.
    pub underlying_price: f64,
    /// K - The options strike price.
    pub strike_price: f64,
    /// sigma - The underlying asset's volatility.
    pub volatility: f64,
    /// r - The risk-free interest rate.
    pub risk_free_rate: f64,

    /// Evaluation date (optional, defaults to today t = 0).
    #[builder(default = "None")]
    pub evaluation_date: Option<Date>,
    /// The options expiration date.
    pub expiration_date: Date,

    /// Call or put flag.
    pub option_type: TypeFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Instrument for BlackScholesMerton {
    /// Returns the price (net present value) of the instrument.
    fn price(&self) -> f64 {
        self.price()
    }

    /// Returns the error on the NPV in case the pricing engine can
    /// provide it (e.g. Monte Carlo pricing engine).
    fn error(&self) -> Option<f64> {
        None
    }

    /// Returns the date at which the NPV is calculated.
    fn valuation_date(&self) -> Date {
        self.evaluation_date.unwrap_or(today())
    }

    /// Instrument type.
    fn instrument_type(&self) -> &'static str {
        "Black-Scholes-Merton European Option"
    }
}

impl BlackScholesMerton {
    /// New European Option
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        cost_of_carry: f64,
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        risk_free_rate: f64,
        evaluation_date: Option<Date>,
        expiration_date: Date,
        option_type: TypeFlag,
    ) -> Self {
        Self {
            cost_of_carry,
            underlying_price,
            strike_price,
            volatility,
            risk_free_rate,
            evaluation_date,
            expiration_date,
            option_type,
        }
    }

    /// Generalised Black-Scholes European Option Price.
    #[must_use]
    pub fn price(&self) -> f64 {
        let (S, K, _, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, d2) = self.d1_d2();
        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => S * ((b - r) * T).exp() * n.cdf(d1) - K * (-r * T).exp() * n.cdf(d2),
            TypeFlag::Put => {
                -S * ((b - r) * T).exp() * n.cdf(-d1) + K * (-r * T).exp() * n.cdf(-d2)
            }
        }
    }

    /// Implied volatility.
    pub fn implied_volatility(&self, price: f64) -> f64 {
        crate::options::implied_volatility(
            price,
            self.underlying_price,
            self.strike_price,
            self.year_fraction(),
            self.risk_free_rate,
            self.option_type,
        )
    }

    /// Compute the year fraction between two dates.
    #[must_use]
    pub fn year_fraction(&self) -> f64 {
        DayCountConvention::Actual_365_25.day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        )
    }

    // Compute d1 and d2.
    #[must_use]
    fn d1_d2(&self) -> (f64, f64) {
        let (S, K, v, _, b) = self.unpack();

        // Compute time to maturity.
        let T = self.year_fraction();

        let d1 = (1.0 / (v * T.sqrt())) * ((S / K).ln() + (b + 0.5 * v.powi(2)) * T);
        let d2 = d1 - v * T.sqrt();

        (d1, d2)
    }

    // Unpack struct to get option parameters.
    #[must_use]
    fn unpack(&self) -> (f64, f64, f64, f64, f64) {
        (
            self.underlying_price,
            self.strike_price,
            self.volatility,
            self.risk_free_rate,
            self.cost_of_carry,
        )
    }

    /// Delta of generalised Black-Scholes European Option.
    #[must_use]
    pub fn delta(&self) -> f64 {
        let (_, _, _, r, b) = self.unpack();
        let T = self.year_fraction();
        let d1 = self.d1_d2().0;
        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => ((b - r) * T).exp() * n.cdf(d1),
            TypeFlag::Put => ((b - r) * T).exp() * (n.cdf(d1) - 1.0),
        }
    }

    /// Vanna of generalised Black-Scholes European Option.
    /// Also known as DdeltaDvol.
    #[must_use]
    pub fn vanna(&self) -> f64 {
        let (_, _, v, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, d2) = self.d1_d2();
        let n = Gaussian::default();

        -((b - r) * T).exp() * n.pdf(d1) * d2 / v
    }

    /// Charm of generalised Black-Scholes European Option.
    /// Also known as DdeltaDtime, delta decay or delta bleed.
    #[must_use]
    pub fn charm(&self) -> f64 {
        let (_, _, v, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, d2) = self.d1_d2();
        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => {
                ((b - r) * T).exp()
                    * (n.pdf(d1) * ((b / (v * T.sqrt())) - (d2 / (2.0 * T))) + (b - r) * n.cdf(d1))
            }
            TypeFlag::Put => {
                ((b - r) * T).exp()
                    * (n.pdf(d1) * ((b / (v * T.sqrt())) - (d2 / (2.0 * T))) - (b - r) * n.cdf(-d1))
            }
        }
    }

    /// Lambda of generalised Black-Scholes European Option.
    /// Also known as elasticity or leverage.
    #[must_use]
    pub fn lambda(&self) -> f64 {
        self.delta() * self.underlying_price / self.price()
    }

    /// Gamma of generalised Black-Scholes European Option.
    /// Also known as convexity.
    #[must_use]
    pub fn gamma(&self) -> f64 {
        let n = Gaussian::default();
        let (S, _, v, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, _) = self.d1_d2();

        ((b - r) * T).exp() * n.pdf(d1) / (S * v * T.sqrt())
    }

    /// Gamma percent of generalised Black-Scholes European Option.
    #[must_use]
    pub fn gamma_percent(&self) -> f64 {
        self.gamma() * self.underlying_price / 100.0
    }

    /// Zomma of generalised Black-Scholes European Option.
    /// Also known as DgammaDvol.
    #[must_use]
    pub fn zomma(&self) -> f64 {
        let (d1, d2) = self.d1_d2();
        self.gamma() * ((d1 * d2 - 1.0) / self.volatility)
    }

    /// Zomma percent of generalised Black-Scholes European Option.
    #[must_use]
    pub fn zomma_percent(&self) -> f64 {
        self.zomma() * self.underlying_price / 100.0
    }

    /// Speed of generalised Black-Scholes European Option.
    /// Also known as DgammaDspot.
    #[must_use]
    pub fn speed(&self) -> f64 {
        let (S, _, v, _, _) = self.unpack();
        let T = self.year_fraction();
        let (d1, _) = self.d1_d2();

        let gamma = self.gamma();

        -gamma * (1.0 + d1 / (v * T.sqrt())) / S
    }

    /// Colour of generalised Black-Scholes European Option.
    /// Also known as DgammaDtime.
    #[must_use]
    pub fn colour(&self) -> f64 {
        let (_, _, v, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, d2) = self.d1_d2();

        let gamma = self.gamma();

        gamma * (r - b + b * d1 / (v * T.sqrt()) + (1.0 - d1 * d2) / (2.0 * T))
    }

    /// Vega of generalised Black-Scholes European Option.
    /// Also known as zeta.
    #[must_use]
    pub fn vega(&self) -> f64 {
        let (S, _, _, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, _) = self.d1_d2();

        let n = Gaussian::default();

        S * ((b - r) * T).exp() * n.pdf(d1) * T.sqrt()
    }

    /// Vomma of generalised Black-Scholes European Option.
    /// Also known as DvegaDvol.
    #[must_use]
    pub fn vomma(&self) -> f64 {
        let (d1, d2) = self.d1_d2();

        self.vega() * d1 * d2 / self.volatility
    }

    /// Ultima of generalised Black-Scholes European Option.
    /// Also known as DvommaDvol.
    #[must_use]
    pub fn ultima(&self) -> f64 {
        let (d1, d2) = self.d1_d2();

        (self.vomma() / self.volatility) * (d1 * d2 - d1 / d2 + d2 / d1 - 1.0)
    }

    /// Vega Bleed of the generalised Black-Scholes European option.
    /// Also known as DvegaDtime.
    #[must_use]
    pub fn vega_bleed(&self) -> f64 {
        let (_, _, v, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, d2) = self.d1_d2();

        self.vega() * (r - b + b * d1 / (v * T.sqrt()) - (d1 * d2 + 1.0) / (2.0 * T))
    }

    /// Theta of the generalised Black-Scholes European option.
    /// Also known as Expected Bleed.
    #[must_use]
    pub fn theta(&self) -> f64 {
        let (S, K, v, r, b) = self.unpack();
        let T = self.year_fraction();
        let (d1, d2) = self.d1_d2();

        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => {
                -S * ((b - r) * T).exp() * n.pdf(d1) * v / (2.0 * T.sqrt())
                    - (b - r) * S * ((b - r) * T).exp() * n.cdf(d1)
                    - r * K * (-r * T).exp() * n.cdf(d2)
            }
            TypeFlag::Put => {
                -S * ((b - r) * T).exp() * n.pdf(d1) * v / (2.0 * T.sqrt())
                    + (b - r) * S * ((b - r) * T).exp() * n.cdf(-d1)
                    + r * K * (-r * T).exp() * n.cdf(-d2)
            }
        }
    }

    /// Rho of the generalised Black-Scholes European option.
    #[must_use]
    pub fn rho(&self) -> f64 {
        let T = self.year_fraction();

        match self.option_type {
            TypeFlag::Call => {
                self.strike_price
                    * T
                    * (-self.risk_free_rate * T).exp()
                    * Gaussian::default().cdf(self.d1_d2().1)
            }
            TypeFlag::Put => {
                -self.strike_price
                    * T
                    * (-self.risk_free_rate * T).exp()
                    * Gaussian::default().cdf(-self.d1_d2().1)
            }
        }
    }

    /// Phi of the generalised Black-Scholes European option.
    /// Also known as Rho-2.
    #[must_use]
    pub fn phi(&self) -> f64 {
        let (S, _, _, r, b) = self.unpack();
        let T = self.year_fraction();

        let (d1, _) = self.d1_d2();

        match self.option_type {
            TypeFlag::Call => -T * S * ((b - r) * T).exp() * Gaussian::default().cdf(d1),
            TypeFlag::Put => T * S * ((b - r) * T).exp() * Gaussian::default().cdf(-d1),
        }
    }

    /// Zeta of the generalised Black-Scholes European option.
    /// Also known as the in-the-money probability.
    #[must_use]
    pub fn zeta(&self) -> f64 {
        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => n.cdf(self.d1_d2().1),
            TypeFlag::Put => n.cdf(-self.d1_d2().1),
        }
    }

    /// Strike Delta of the generalised Black-Scholes European option.
    /// Also known as Dual Delta or Discounted Probability.
    #[must_use]
    pub fn strike_delta(&self) -> f64 {
        let n = Gaussian::default();

        let T = self.year_fraction();

        match self.option_type {
            TypeFlag::Call => -(-self.risk_free_rate * T).exp() * n.cdf(self.d1_d2().1),
            TypeFlag::Put => (-self.risk_free_rate * T).exp() * n.cdf(-self.d1_d2().1),
        }
    }

    /// Strike Gamma of the generalised Black-Scholes European option.
    #[must_use]
    pub fn strike_gamma(&self) -> f64 {
        let n = Gaussian::default();
        let T = self.year_fraction();

        n.pdf(self.d1_d2().1) * (-self.risk_free_rate * T).exp()
            / (self.strike_price * self.volatility * T.sqrt())
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_black_scholes_merton {
    use super::*;
    use time::Duration;
    use RustQuant_utils::assert_approx_equal;

    #[test]
    fn black_scholes_1973() {
        // Values from Haug
        let bsm = BlackScholesMerton::new(
            0.08,
            60.0,
            65.0,
            0.3,
            0.08,
            None,
            today() + Duration::days(91),
            TypeFlag::Call,
        );
        assert_approx_equal!(bsm.price(), 2.121846776001, 1e-2);
    }

    #[test]
    fn merton_1973() {
        // Values from Haug
        let bsm = BlackScholesMerton::new(
            0.1 - 0.05,
            100.0,
            95.0,
            0.2,
            0.1,
            None,
            today() + Duration::days(182),
            TypeFlag::Put,
        );
        assert_approx_equal!(bsm.price(), 2.456571166461579, 1e-2);
    }
}
