// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS AND ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Portfolio struct for risk-reward measures.
pub struct Portfolio {
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

impl Portfolio {
    /// Treynor's (1965) risk-reward ratio.
    ///
    /// `Treynor ratio = (r_p - r) / beta_p`
    ///
    /// Where:
    ///
    /// * `r_p` is the average return of the portfolio.
    /// * `r` is the risk-free return over the same period.
    /// * `beta_p` is the beta of the portfolio.
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
    pub fn burke_ratio(&self, drawdowns: &mut [f64]) -> f64 {
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

    static PORTFOLIO: Portfolio = Portfolio {
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
        assert_eq!(PORTFOLIO.treynors_ratio(), (0.12 - 0.05) / 1.2);
    }

    #[test]
    fn test_sharpe_ratio() {
        assert_eq!(PORTFOLIO.sharpe_ratio(), (0.12 - 0.05) / 0.2);
    }

    #[test]
    fn test_sortino_ratio() {
        assert_eq!(PORTFOLIO.sortino_ratio(), (0.12 - 0.05) / 0.1);
    }

    #[test]
    fn test_burke_ratio() {
        let mut drawdowns = vec![0.05, 0.10, 0.20];
        let ss_drawdowns = drawdowns.iter().map(|x| x * x).sum::<f64>();
        assert_eq!(
            PORTFOLIO.burke_ratio(&mut drawdowns),
            (0.12 - 0.05) / ss_drawdowns
        );
    }

    #[test]
    fn test_return_on_var() {
        assert_eq!(PORTFOLIO.return_on_var(), 0.12 / 0.15);
    }

    #[test]
    fn test_jensens_alpha() {
        assert_eq!(
            PORTFOLIO.jensens_alpha(),
            0.12 - (0.05 + 1.2 * (0.1 - 0.05))
        );
    }
}
