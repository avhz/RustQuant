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

use crate::instruments::options::TypeFlag;
use crate::statistics::distributions::{Distribution, Gaussian};
use crate::time::{DayCountConvention, DayCounter};

use time::{Duration, OffsetDateTime};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Generalised Black-Scholes-Merton European Option pricing model.
pub struct BlackScholesMerton {
    /// The cost of carry factor.
    /// For the generalised Black-Scholes-Merton model,
    /// there are five possibilities for the cost of carry factor:
    ///     1. b = r:           Black-Scholes 1973 stock option model.
    ///     2. b = r - q:       Merton 1973 stock option model with continuous dividend yield.
    ///     3. b = 0:           Black 1976 futures option model.
    ///     4. b = 0, r = 0:    Asay 1982 margined futures option model.
    ///     5. b = r_d - r_f:   Garman and Kohlhagen 1983 currency option model.
    pub cost_of_carry: f64,
    /// The underlying asset price.
    pub underlying_price: f64,
    /// The options strike price.
    pub strike_price: f64,
    /// The underlying asset's volatility.
    pub volatility: f64,
    /// The risk-free interest rate.
    pub risk_free_rate: f64,
    /// Evaluation date (optional, defaults to today t = 0).
    pub evaluation_date: Option<OffsetDateTime>,
    /// The options expiration date.
    pub expiration_date: OffsetDateTime,
    /// Call or put flag.
    pub option_type: TypeFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl BlackScholesMerton {
    /// New European Option
    pub fn new(
        cost_of_carry: f64,
        underlying_price: f64,
        strike_price: f64,
        volatility: f64,
        risk_free_rate: f64,
        evaluation_date: Option<OffsetDateTime>,
        expiration_date: OffsetDateTime,
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
    pub fn price(&self) -> f64 {
        let (S, K, v, r, b) = self.unpack();

        // Compute time to maturity.
        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => S * ((b - r) * T).exp() * n.cdf(d1) - K * (-r * T).exp() * n.cdf(d2),
            TypeFlag::Put => {
                -S * ((b - r) * T).exp() * n.cdf(-d1) + K * (-r * T).exp() * n.cdf(-d2)
            }
        }
    }

    /// Compute d1 and d2.
    pub fn d1_d2(&self) -> (f64, f64) {
        let (S, K, v, r, b) = self.unpack();

        // Compute time to maturity.
        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let d1 = (1.0 / (v * T.sqrt())) * ((S / K).ln() + (b + 0.5 * v.powi(2)) * T);
        let d2 = d1 - v * T.sqrt();

        (d1, d2)
    }

    // Unpack struct to get option parameters.
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
    pub fn delta(&self) -> f64 {
        let (S, K, v, r, b) = self.unpack();

        // Compute time to maturity.
        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let d1 = self.d1_d2().0;

        let n = Gaussian::default();

        match self.option_type {
            TypeFlag::Call => ((b - r) * T).exp() * n.cdf(d1),
            TypeFlag::Put => ((b - r) * T).exp() * (n.cdf(d1) - 1.0),
        }
    }

    /// Vanna of generalised Black-Scholes European Option.
    /// Also known as DdeltaDvol.
    pub fn vanna(&self) -> f64 {
        let (S, K, v, r, b) = self.unpack();

        // Compute time to maturity.
        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        let n = Gaussian::default();

        -((b - r) * T).exp() * n.pdf(d1) * d2 / v
    }

    /// Charm of generalised Black-Scholes European Option.
    /// Also known as DdeltaDtime, delta decay or delta bleed.
    pub fn charm(&self) -> f64 {
        let (S, K, v, r, b) = self.unpack();

        // Compute time to maturity.
        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

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
    pub fn lambda(&self) -> f64 {
        let n = Gaussian::default();

        let (S, K, v, r, b) = self.unpack();

        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        self.delta() * S / self.price()
    }

    /// Gamma of generalised Black-Scholes European Option.
    /// Also known as convexity.
    pub fn gamma(&self) -> f64 {
        let n = Gaussian::default();

        let (S, K, v, r, b) = self.unpack();

        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        ((b - r) * T).exp() * n.pdf(d1) / (S * v * T.sqrt())
    }

    /// Gamma percent of generalised Black-Scholes European Option.
    pub fn gamma_percent(&self) -> f64 {
        self.gamma() * self.underlying_price / 100.0
    }

    /// Zomma of generalised Black-Scholes European Option.
    /// Also known as DgammaDvol.
    pub fn zomma(&self) -> f64 {
        let n = Gaussian::default();

        let (S, K, v, r, b) = self.unpack();

        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        let gamma = self.gamma();

        gamma * ((d1 * d2 - 1.0) / v)
    }

    /// Zomma percent of generalised Black-Scholes European Option.
    pub fn zomma_percent(&self) -> f64 {
        self.zomma() * self.underlying_price / 100.0
    }

    /// Speed of generalised Black-Scholes European Option.
    /// Also known as DgammaDspot.
    pub fn speed(&self) -> f64 {
        let n = Gaussian::default();

        let (S, K, v, r, b) = self.unpack();

        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        let gamma = self.gamma();

        -gamma * (1.0 + d1 / (v * T.sqrt())) / S
    }

    /// Colour of generalised Black-Scholes European Option.
    /// Also known as DgammaDtime.
    pub fn colour(&self) -> f64 {
        let n = Gaussian::default();

        let (S, K, v, r, b) = self.unpack();

        let T = DayCounter::day_count_factor(
            self.evaluation_date.unwrap_or(OffsetDateTime::now_utc()),
            self.expiration_date,
            &DayCountConvention::Actual365,
        );

        let (d1, d2) = self.d1_d2();

        let gamma = self.gamma();

        gamma * (r - b + b * d1 / (v * T.sqrt()) + (1.0 - d1 * d2) / (2.0 * T))
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_black_scholes_merton {
    use super::*;

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
            OffsetDateTime::now_utc() + Duration::days(91),
            TypeFlag::Call,
        );
        assert_approx_equal!(bsm.price(), 2.1044558953508385, 1e-10);
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
            OffsetDateTime::now_utc() + Duration::days(182),
            TypeFlag::Put,
        );
        assert_approx_equal!(bsm.price(), 2.4524152213972776, 1e-10);
    }
}
