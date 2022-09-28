#![allow(non_snake_case)]

use super::*;

/// Adapted from closed-form solutions provided in Haug's Option Pricing Formulas.

// ############################################################################
// FUNCTIONS
// ############################################################################

/// # Arguments:
///
/// * `S` - Initial underlying price.
/// * `X` - Strike price.
/// * `H` - Barrier.
/// * `t` - Time to expiry.
/// * `r` - Risk-free rate.
/// * `v` - Volatility.
/// * `K` - Rebate (paid if the option is not able to be exercised).
/// * `q` - Dividend yield.
/// * `type_flag` - One of: `cui`, `cuo`, `pui`, `puo`, `cdi`, `cdo`, `pdi`, `pdo`.
///
/// # Note:
///
/// * `b = r - q` - The cost of carry.
pub fn BarrierOptionClosedForm(
    S: f64,
    X: f64,
    H: f64,
    t: f64,
    r: f64,
    v: f64,
    K: f64,
    q: f64,
    type_flag: &str,
) -> f64 {
    // Cost of carry = risk-free rate - dividend yield
    let b: f64 = r - q;

    // Common terms:
    let mu: f64 = (b - v * v / 2.) / (v * v);
    let lambda: f64 = (mu * mu + 2. * r / (v * v)).sqrt();
    let z: f64 = (H / S).ln() / (v * t.sqrt()) + lambda * v * t.sqrt();

    let x1: f64 = (S / X).ln() / v * t.sqrt() + (1. + mu) * v * t.sqrt();
    let x2: f64 = (S / H).ln() / v * t.sqrt() + (1. + mu) * v * t.sqrt();

    let y1: f64 = (H * H / (S * X)).ln() / (v * t.sqrt()) + (1. + mu) * v * t.sqrt();
    let y2: f64 = (H / S).ln() / (v * t.sqrt()) + (1. + mu) * v * t.sqrt();

    // Common functions:
    let AA = |phi: f64| -> f64 {
        let term1: f64 = phi * S * ((b - r) * t).exp() * pnorm(phi * x1);
        let term2: f64 = phi * X * (-r * t).exp() * pnorm(phi * x1 - phi * v * (t).sqrt());
        return term1 - term2;
    };

    let BB = |phi: f64| -> f64 {
        let term1: f64 = phi * S * ((b - r) * t).exp() * pnorm(phi * x2);
        let term2: f64 = phi * X * (-r * t).exp() * pnorm(phi * x2 - phi * v * (t).sqrt());
        return term1 - term2;
    };

    let CC = |phi: f64, eta: f64| -> f64 {
        let term1: f64 =
            phi * S * ((b - r) * t).exp() * (H / S).powf(2. * (mu + 1.)) * pnorm(eta * y1);
        let term2: f64 =
            phi * X * (-r * t).exp() * (H / S).powf(2. * mu) * pnorm(eta * y1 - eta * v * t.sqrt());
        return term1 - term2;
    };

    let DD = |phi: f64, eta: f64| -> f64 {
        let term1: f64 =
            phi * S * ((b - r) * t).exp() * (H / S).powf(2. * (mu + 1.)) * pnorm(eta * y2);
        let term2: f64 = phi
            * X
            * (-r * t).exp()
            * (H / S).powf(2. * mu)
            * pnorm(eta * y2 - eta * v * (t).sqrt());
        return term1 - term2;
    };

    let EE = |eta: f64| -> f64 {
        let term1: f64 = pnorm(eta * x2 - eta * v * (t).sqrt());
        let term2: f64 = (H / S).powf(2. * mu) * pnorm(eta * y2 - eta * v * t.sqrt());
        return K * (-r * t).exp() * (term1 - term2);
    };

    let FF = |eta: f64| -> f64 {
        let term1: f64 = (H / S).powf(mu + lambda) * pnorm(eta * z);
        let term2: f64 =
            (H / S).powf(mu - lambda) * pnorm(eta * z - 2. * eta * lambda * v * t.sqrt());
        return K * (term1 + term2);
    };

    // Strike above barrier (X >= H):
    if X >= H {
        match type_flag {
            // Knock-In calls:
            "cdi" if S >= H => CC(1., 1.) + EE(1.),
            "cui" if S <= H => AA(1.) + EE(-1.),
            // Knock-In puts:
            "pdi" if S >= H => BB(-1.) - CC(-1., 1.) + DD(-1., 1.) + EE(1.),
            "pui" if S <= H => AA(-1.) - BB(-1.) + DD(-1., -1.) + EE(-1.),
            // Knock-Out calls:
            "cdo" if S >= H => AA(1.) - CC(1., 1.) + FF(1.),
            "cuo" if S <= H => FF(-1.),
            // Knock-Out puts:
            "pdo" if S >= H => AA(-1.) - BB(-1.) + CC(-1., 1.) - DD(-1., 1.) + FF(1.),
            "puo" if S <= H => BB(-1.) - DD(-1., -1.) + FF(-1.),

            _ => panic!("Barrier touched - check barrier and type flag."),
        }
    }
    // Strike below barrier (X < H):
    else {
        match type_flag {
            // Knock-In calls:
            "cdi" if S >= H => AA(1.) - BB(1.) + DD(1., 1.) + EE(1.),
            "cui" if S <= H => BB(1.) - CC(1., -1.) + DD(1., -1.) + EE(-1.),
            // Knock-In puts:
            "pdi" if S >= H => AA(-1.) + EE(1.),
            "pui" if S <= H => CC(-1., -1.) + EE(-1.),
            // Knock-Out calls:
            "cdo" if S >= H => BB(1.) - DD(1., 1.) + FF(1.),
            "cuo" if S <= H => AA(1.) - BB(1.) + CC(1., -1.) - DD(1., -1.) + FF(-1.),
            // Knock-Out puts:
            "pdo" if S >= H => FF(1.),
            "puo" if S <= H => AA(-1.) - CC(-1., -1.) + FF(-1.),

            _ => panic!("Barrier touched - check barrier and type flag."),
        }
    }
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;

    // // Function arguments:
    // S: f64,            // Underlying price
    // X: f64,            // Strike price
    // H: f64,            // Barrier
    // t: f64,            // Time to expiry
    // r: f64,            // Risk-free rate
    // v: f64,            // Volatility
    // K: f64,            // Rebate
    // q: f64,            // Dividend yield
    // type_flag: &str,   // One of: cui, cuo, pui, puo, cdi, cdo, pdi, pdo

    // ########################################################################
    // Down-and-In Call
    // ########################################################################

    #[test]
    fn cdi() {
        let price = BarrierOptionClosedForm(110.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cdi");
        assert!(price - 9.5048 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cdi_panic() {
        BarrierOptionClosedForm(90.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cdi");
    }

    // ########################################################################
    // Up-and-In Call
    // ########################################################################

    #[test]
    fn cui() {
        let price = BarrierOptionClosedForm(90.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cui");
        assert!(price - 4.6926 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cui_panic() {
        BarrierOptionClosedForm(110.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cui");
    }

    // ########################################################################
    // Down-and-In Put
    // ########################################################################

    #[test]
    fn pdi() {
        let price = BarrierOptionClosedForm(110.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "pdi");
        assert!(price - 3.0173 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pdi_panic() {
        BarrierOptionClosedForm(90.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "pdi");
    }

    // ########################################################################
    // Up-and-In Put
    // ########################################################################

    #[test]
    fn pui() {
        let price = BarrierOptionClosedForm(90.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "pui");
        assert!(price - 1.3596 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pui_panic() {
        BarrierOptionClosedForm(110.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "pui");
    }

    // ########################################################################
    // Down-and-Out Call
    // ########################################################################

    #[test]
    fn cdo() {
        let price = BarrierOptionClosedForm(110.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cdo");
        assert!(price - 7.295 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cdo_panic() {
        BarrierOptionClosedForm(90.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cdo");
    }

    // ########################################################################
    // Up-and-Out Call
    // ########################################################################

    #[test]
    fn cuo() {
        let price = BarrierOptionClosedForm(90.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cuo");
        assert!(price - 0.0224 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn cuo_panic() {
        BarrierOptionClosedForm(110.0, 100.0, 105.0, 1.0, 0.05, 0.2, 0.0, 0.01, "cuo");
    }

    // ########################################################################
    // Down-and-Out Put
    // ########################################################################

    // und, str, bar

    #[test]
    fn pdo() {
        let price = BarrierOptionClosedForm(150.0, 100.0, 40.0, 1.0, 0.05, 0.2, 0.0, 0.01, "pdo");
        assert!(price - 0.107 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn pdo_panic() {
        BarrierOptionClosedForm(30.0, 100.0, 40.0, 1.0, 0.05, 0.2, 0.0, 0.01, "pdo");
    }

    // ########################################################################
    // Up-and-Out Put
    // ########################################################################

    #[test]
    fn puo() {
        let price = BarrierOptionClosedForm(30.0, 80.0, 100.0, 1.0, 0.05, 0.2, 0.0, 0.01, "puo");
        assert!(price - 46.3969 < 0.0001);
    }

    #[test]
    #[should_panic(expected = "Barrier touched - check barrier and type flag.")]
    fn puo_panic() {
        BarrierOptionClosedForm(110.0, 80.0, 100.0, 1.0, 0.05, 0.2, 0.0, 0.01, "puo");
    }
}
