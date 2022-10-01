use crate::{dnorm, pnorm};

pub struct Greeks {
    Delta: (f64, f64),
    // Lambda: (f64, f64),
    Gamma: (f64, f64),
    Vega: (f64, f64),
    // Theta: (f64, f64),
    Rho: (f64, f64),
    Phi: (f64, f64),
    Zeta: (f64, f64),
}

pub fn Greeks(S: f64, K: f64, v: f64, r: f64, T: f64, q: f64) -> Greeks {
    let sqrtT: f64 = T.sqrt();
    let df: f64 = (-r * T).exp();
    let b: f64 = r - q;
    let ebrT: f64 = ((b - r) * T).exp();
    let Fp: f64 = S * (b * T).exp();
    let std: f64 = v * sqrtT;
    let d: f64 = (Fp / K).ln() / std;
    let d1: f64 = d + 0.5 * std;
    let d2: f64 = d1 - std;

    let nd1: f64 = dnorm(d1);
    let nd2: f64 = dnorm(d2);
    let Nd1: f64 = pnorm(d1);
    let Nd2: f64 = pnorm(d2);

    let nd1_: f64 = dnorm(-d1);
    let nd2_: f64 = dnorm(-d2);
    let Nd1_: f64 = pnorm(-d1);
    let Nd2_: f64 = pnorm(-d2);

    Greeks {
        Delta: (ebrT * Nd1, ebrT * (Nd1 - 1.0)),
        // Lambda: (),
        Gamma: (
            (nd1 * ebrT) / (S * v * sqrtT),
            (nd1 * ebrT) / (S * v * sqrtT),
        ),
        Vega: (S * ebrT * nd1 * sqrtT, S * ebrT * nd1 * sqrtT),
        // Theta: (),
        Rho: (T * K * df * Nd2, -T * K * df * Nd2_),
        Phi: (-T * S * ebrT * Nd1, T * S * ebrT * Nd1_),
        Zeta: (Nd2, Nd2_),
    }
}
