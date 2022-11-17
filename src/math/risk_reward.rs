#![deny(missing_docs)]

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Treynor's (1965) risk-reward ratio.
///
/// `Treynor ratio = (r_p - r) / beta_p`
///
/// Where:
///
/// * `r_p` is the average return of the portfolio.
/// * `r` is the risk-free return over the same period.
/// * `beta_p` is the beta of the portfolio.
pub fn treynors_ratio(r_p: f64, r: f64, beta_p: f64) -> f64 {
    (r_p - r) / beta_p
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
pub fn sharpe_ratio(r_p: f64, r: f64, sigma_p: f64) -> f64 {
    (r_p - r) / sigma_p
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
pub fn sortino_ratio(r_p: f64, r: f64, sigma_down: f64) -> f64 {
    (r_p - r) / sigma_down
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
pub fn burke_ratio(r_p: f64, r: f64, drawdowns: &mut [f64]) -> f64 {
    let ss_drawdowns = drawdowns.iter().map(|x| x.powi(2)).sum::<f64>();

    (r_p - r) / ss_drawdowns
}

/// Return on VaR (value-at-risk).
///
/// `Return on var = r_p / var`
///
/// Where:
///
/// * `r_p` is the average return of the portfolio.
/// * `var` is the Value-at-Risk.
pub fn return_on_var(r_p: f64, var: f64) -> f64 {
    r_p / var
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
pub fn jensens_alpha(r_p: f64, r_m: f64, r: f64, beta_p: f64) -> f64 {
    r_p - (r + beta_p * (r_m - r))
}

// ############################################################################
// TESTS
// ############################################################################

// NEED TESTS HERE
