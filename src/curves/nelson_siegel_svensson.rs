// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::curves::{Curve, CurveModel};
use crate::time::{DayCountConvention, DayCounter};
use time::OffsetDateTime;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Nelson-Siegel-Svensson (1994) model parameters.
pub struct NelsonSiegelSvensson {
    beta0: f64,
    beta1: f64,
    beta2: f64,
    beta3: f64,
    lambda1: f64,
    lambda2: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl NelsonSiegelSvensson {
    /// Create a new Nelson-Siegel model.
    pub fn new(beta0: f64, beta1: f64, beta2: f64, beta3: f64, lambda1: f64, lambda2: f64) -> Self {
        Self {
            beta0,
            beta1,
            beta2,
            beta3,
            lambda1,
            lambda2,
        }
    }
}

impl CurveModel for NelsonSiegelSvensson {
    /// Returns the forward rate for a given date.
    fn forward_rate(&self, date: OffsetDateTime) -> f64 {
        assert!(
            date > OffsetDateTime::now_utc(),
            "Date must be in the future."
        );

        let tau = DayCounter::day_count_factor(
            OffsetDateTime::now_utc(),
            date,
            &DayCountConvention::Actual365,
        );

        let term1 = f64::exp(-tau / self.lambda1);
        let term2 = (tau / self.lambda1) * term1;
        let term3 = (tau / self.lambda2) * f64::exp(-tau / self.lambda2);

        self.beta0 + self.beta1 * term1 + self.beta2 * term2 + self.beta3 * term3
    }

    /// Returns the spot rate for a given date.
    fn spot_rate(&self, date: OffsetDateTime) -> f64 {
        assert!(
            date > OffsetDateTime::now_utc(),
            "Date must be in the future."
        );

        let tau = DayCounter::day_count_factor(
            OffsetDateTime::now_utc(),
            date,
            &DayCountConvention::Actual365,
        );

        let term1 = self.lambda1 * (1. - f64::exp(-tau / self.lambda1)) / tau;
        let term2 = term1 - f64::exp(-tau / self.lambda1);
        let term3 = self.lambda2 * (1. - f64::exp(-tau / self.lambda2)) / tau
            - f64::exp(-tau / self.lambda2);

        self.beta0 + self.beta1 * term1 + self.beta2 * term2 + self.beta3 * term3
    }

    fn discount_factor(&self, date: OffsetDateTime) -> f64 {
        let tau = DayCounter::day_count_factor(
            OffsetDateTime::now_utc(),
            date,
            &DayCountConvention::Actual365,
        );

        f64::exp(-self.spot_rate(date) * tau / 100.)
    }

    fn calibrate<C: Curve>(&self, curve: C) -> Self {
        unimplemented!()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_nelson_siegel_svensson {
    use super::*;
    use crate::plot_vector;
    use time::Duration;

    #[test]
    fn test_nelson_siegel_svensson() {
        let nss = NelsonSiegelSvensson {
            beta0: 0.0806,
            beta1: -0.0031,
            beta2: -0.0625,
            beta3: -0.0198,
            lambda1: 1.58,
            lambda2: 0.15,
        };

        let dates = (2..365 * 30)
            .map(|i| OffsetDateTime::now_utc() + Duration::days(i))
            .collect::<Vec<OffsetDateTime>>();

        let forward_curve = dates
            .iter()
            .map(|date| nss.forward_rate(*date))
            .collect::<Vec<_>>();

        let discount_curve = dates
            .iter()
            .map(|date| nss.discount_factor(*date))
            .collect::<Vec<_>>();

        plot_vector!(forward_curve, "./images/nelson_siegel_svensson_forward.png");
        plot_vector!(
            discount_curve,
            "./images/nelson_siegel_svensson_discount.png"
        );
    }
}
