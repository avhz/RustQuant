#![allow(non_snake_case)]
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
pub fn Treynors_Ratio(r_p: f64, r: f64, beta_p: f64) -> f64 {
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
pub fn Sharpe_Ratio(r_p: f64, r: f64, sigma_p: f64) -> f64 {
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
pub fn Sortino_Ratio(r_p: f64, r: f64, sigma_down: f64) -> f64 {
    (r_p - r) / sigma_down
}

/// Burke's (1994) risk-reward ratio.
///
/// `Burke ratio = (r_p - r) / SSDrawdowns`
///
/// Where:
///
/// * `r_p` is the average return of the portfolio.
/// * `r` is the risk-free return over the same period.
/// * `SSDrawdowns` is the sum of the squared drawdowns.
pub fn Burke_Ratio(r_p: f64, r: f64, drawdowns: &mut Vec<f64>) -> f64 {
    let SSDrawdowns = drawdowns.iter().map(|x| x.powi(2)).sum::<f64>();

    (r_p - r) / SSDrawdowns
}

/// Return on VaR.
///
/// `Return on VaR = r_p / VaR`
///
/// Where:
///
/// * `r_p` is the average return of the portfolio.
/// * `VaR` is the Value-at-Risk.
pub fn Return_on_VaR(r_p: f64, VaR: f64) -> f64 {
    r_p / VaR
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
pub fn Jensens_Alpha(r_p: f64, r_m: f64, r: f64, beta_p: f64) -> f64 {
    r_p - (r + beta_p * (r_m - r))
}

// ############################################################################
// TESTS
// ############################################################################

// NEED TESTS HERE
