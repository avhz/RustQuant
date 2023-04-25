//! Vasicek's model for zero-coupon bond prices.
//!
//! The risk-neutral short rate follows the process:
//!
//! dr = a(b-r)dt + sigma * dW
//!
//! It incorporates a mean-reversion factor into the drift term:
//!
//! - `a`: is the rate at which it gets pulled.
//! - `b`: is the level to which it gets pulled.
//! - `sigma`: is the diffusion coefficient.

use crate::bonds::*;

/// Struct containing the Vasicek model parameters.
pub struct Vasicek {
    a: f64,
    b: f64,
    sigma: f64,
    r: f64,
    t: f64,
    maturity: f64,
}

impl ZeroCouponBond for Vasicek {
    fn price(&self) -> f64 {
        let a = self.a;
        let b = self.b;
        let sigma = self.sigma;
        let r = self.r;
        let t = self.t;
        let maturity = self.maturity;

        let tau = maturity - t;

        let b_t = (1_f64 - (-a * tau).exp()) / a;
        let a_t = (((b_t - tau) * (a * a * b - sigma * sigma / 2_f64)) / (a * a)
            - (sigma * sigma * b_t * b_t) / (4_f64 * a))
            .exp();

        // Price:
        a_t * (-b_t * r).exp()
    }

    // fn duration(&self) -> f64 {}
    // fn convexity(&self) -> f64 {}
}

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_vasicek_zero_coupon_bond() {
        let vasicek = Vasicek {
            a: 0.3,
            b: 0.1,
            sigma: 0.03,
            r: 0.03,
            t: 0.0,
            maturity: 1.0,
        };

        let vasicek_price = vasicek.price();

        assert_approx_equal!(vasicek_price, 0.96136248, 1e-8);
    }
}
