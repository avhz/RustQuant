// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS AND ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Portfolio struct for risk-reward measures.
pub struct PortfolioMeasures {
    /// The average return of the portfolio.
    r_p: f64,
    /// The risk-free return over the same period.
    r: f64,
    /// The beta of the portfolio.
    beta_p: f64,
    /// The standard deviation of the portfolio returns.
    sigma_p: f64,
    /// The *downside* standard deviation of the portfolio returns, also known as semistandard deviation.
    sigma_down: f64,
    /// The Value-at-Risk.
    var: f64,
    /// The expected market return.
    r_m: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl PortfolioMeasures {
    /// Treynor's (1965) risk-reward ratio.
    ///
    /// `Treynor ratio = (r_p - r) / beta_p`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `r` is the risk-free return over the same period.
    /// * `beta_p` is the beta of the portfolio.
    #[must_use]
    pub fn treynors_ratio(&self) -> f64 {
        (self.r_p - self.r) / self.beta_p
    }

    /// Sharpe's (1966) risk-reward ratio.
    ///
    /// `Sharpe ratio = (r_p - r) / sigma_p`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `r` is the risk-free return over the same period.
    /// * `sigma_p` is the standard deviation of the portfolio returns.
    #[must_use]
    pub fn sharpe_ratio(&self) -> f64 {
        (self.r_p - self.r) / self.sigma_p
    }

    /// Sortino and Price's (1994) risk-reward ratio.
    ///
    /// `Sortino ratio = (r_p - r) / sigma_down`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `r` is the risk-free return over the same period.
    /// * `sigma_down` is the *downside* standard deviation of the portfolio returns, also known as semistandard deviation.
    #[must_use]
    pub fn sortino_ratio(&self) -> f64 {
        (self.r_p - self.r) / self.sigma_down
    }

    /// Burke's (1994) risk-reward ratio.
    ///
    /// `Burke ratio = (r_p - r) / ss_drawdowns`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `r` is the risk-free return over the same period.
    /// * `ss_drawdowns` is the sum of the squared drawdowns.
    #[must_use]
    pub fn burke_ratio(&self, drawdowns: &[f64]) -> f64 {
        let ss_drawdowns = drawdowns.iter().map(|x| x.powi(2)).sum::<f64>();

        (self.r_p - self.r) / ss_drawdowns
    }

    /// Return on VaR (value-at-risk).
    ///
    /// `Return on var = r_p / var`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `var` is the Value-at-Risk.
    #[must_use]
    pub fn return_on_var(&self) -> f64 {
        self.r_p / self.var
    }

    /// Jensen's Measure (alpha).
    ///
    /// `Jensen's Alpha = r_p - (r + beta_p * (r_m - r))`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `r` is the risk-free return over the same period.
    /// * `beta_p` is the beta of the portfolio.
    /// * `r_m` is the expected market return.
    #[must_use]
    pub fn jensens_alpha(&self) -> f64 {
        self.r_p - (self.r + self.beta_p * (self.r_m - self.r))
    }
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests_risk_reward {
    use super::*;

    use std::f64::EPSILON as EPS;

    static PORTFOLIO: PortfolioMeasures = PortfolioMeasures {
        r_p: 0.12,
        r: 0.05,
        beta_p: 1.2,
        sigma_p: 0.2,
        sigma_down: 0.1,
        var: 0.15,
        r_m: 0.1,
    };

    #[test]
    fn test_treynors_ratio() {
        assert_approx_equal!(PORTFOLIO.treynors_ratio(), (0.12 - 0.05) / 1.2, EPS);
    }

    #[test]
    fn test_sharpe_ratio() {
        assert_approx_equal!(PORTFOLIO.sharpe_ratio(), (0.12 - 0.05) / 0.2, EPS);
    }

    #[test]
    fn test_sortino_ratio() {
        assert_approx_equal!(PORTFOLIO.sortino_ratio(), (0.12 - 0.05) / 0.1, EPS);
    }

    #[test]
    fn test_burke_ratio() {
        let drawdowns = vec![0.05, 0.10, 0.20];
        let ss_drawdowns = drawdowns.iter().map(|x| x * x).sum::<f64>();
        assert_approx_equal!(
            PORTFOLIO.burke_ratio(&drawdowns),
            (0.12 - 0.05) / ss_drawdowns,
            EPS
        );
    }

    #[test]
    fn test_return_on_var() {
        assert_approx_equal!(PORTFOLIO.return_on_var(), 0.12 / 0.15, EPS);
    }

    #[test]
    fn test_jensens_alpha() {
        assert_approx_equal!(
            PORTFOLIO.jensens_alpha(),
            0.12 - (0.05 + 1.2 * (0.1 - 0.05)),
            EPS
        );
    }
}
