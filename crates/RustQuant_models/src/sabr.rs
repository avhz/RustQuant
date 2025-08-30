use argmin::{core::CostFunction, solver::particleswarm::ParticleSwarm};
use serde::{Deserialize, Serialize};

/// SABR (2002) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sabr02 {
    f: f64,
    alpha: f64,
    beta: f64,
    rho: f64,
    nu: f64,
}

impl Sabr02 {
    /// Create a new SABR (2002) option pricing parameters.
    pub fn new(f: f64, alpha: f64, beta: f64, rho: f64, nu: f64) -> Self {
        Self {
            f,
            alpha,
            beta,
            rho,
            nu,
        }
    }
}

pub(crate) fn sabr_volatility(
    f: f64,
    k: f64,
    t: f64,
    alpha: f64,
    beta: f64,
    rho: f64,
    nu: f64,
) -> f64 {
    coefficient(f, k, beta, rho, nu) * numerator(f, k, t, alpha, beta, rho, nu)
        / denominator(f, k, beta, rho, nu)
}

fn coefficient(f: f64, k: f64, beta: f64, rho: f64, _: f64) -> f64 {
    z(f, k, beta) / chi(f, k, beta, rho)
}

fn numerator(f: f64, k: f64, t: f64, alpha: f64, beta: f64, rho: f64, nu: f64) -> f64 {
    let term1 = (1.0 - beta).powi(2) * alpha.powi(2) / (24.0 * fk_power(f, k, beta).powi(2));
    let term2 = 0.25 * rho * beta * alpha * nu / fk_power(f, k, beta);
    let term3 = (2.0 - 3.0 * rho.powi(2)) * nu.powi(2) / 24.0;
    alpha * (1.0 + (term1 + term2 + term3) * t)
}

fn denominator(f: f64, k: f64, beta: f64, _: f64, _: f64) -> f64 {
    let term1 = fk_power(f, k, beta);
    let term2 = (1.0 - beta).powi(2) * (f / k).ln().powi(2) / 24.0;
    let term3 = (1.0 - beta).powi(4) * (f / k).ln().powi(4) / 1920.0;
    term1 * (1.0 + term2 + term3)
}

fn z(f: f64, k: f64, beta: f64) -> f64 {
    (f / k).ln() * fk_power(f, k, beta)
}

fn chi(f: f64, k: f64, beta: f64, rho: f64) -> f64 {
    let numerator =
        ((1.0 - 2.0 * rho * z(f, k, beta) + z(f, k, beta).powi(2)).sqrt() + z(f, k, beta) - rho)
            .ln();
    let denominator = 1.0 - rho;
    numerator / denominator
}

fn fk_power(f: f64, k: f64, beta: f64) -> f64 {
    (f * k).powf((1.0 - beta) / 2.0)
}

pub(crate) struct Sabr02Calibrator {
    pub(crate) beta: f64,
    pub(crate) f: f64,
    pub(crate) t: f64,
    pub(crate) ks: Vec<f64>,
    pub(crate) vs: Vec<f64>,
}

impl CostFunction for Sabr02Calibrator {
    type Output = f64;
    type Param = Vec<f64>;

    fn cost(&self, params: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        let alpha = params[0];
        let rho = params[1];
        let nu = params[2];

        let mut cost = 0.0;

        let data = self.ks.iter().zip(self.vs.iter());

        for (strike, vol) in data {
            let model_vol = sabr_volatility(self.f, *strike, self.t, alpha, self.beta, rho, nu);
            cost += (model_vol - vol).powi(2);
        }

        Ok(cost.sqrt())
    }
}

impl Sabr02 {
    /// Calculate the SABR volatility for input to the Black (76) model.
    pub fn volatility(&self, k: f64, t: f64) -> f64 {
        sabr_volatility(self.f, k, t, self.alpha, self.beta, self.rho, self.nu)
    }

    /// Fit the SABR model to a set of market data (volatilities).
    ///
    /// Note: Beta ($\beta$) is assumed to be fixed and is not optimized.
    /// It can be any value between 0 and 1, but:
    /// * $\beta = 0$ corresponds to the stochastic normal model.
    /// * $\beta = 1$ corresponds to the stochastic lognormal model.
    /// * $\beta = 0.5$ corresponds to the stochastic CIR model.
    pub fn fit(
        &mut self,
        volatilities: &[f64],
        strikes: &[f64],
        t: f64,
    ) -> Result<(), argmin::core::Error> {
        use argmin::core::{Executor, State};

        let calibrator = Sabr02Calibrator {
            beta: self.beta,
            f: self.f,
            t,
            ks: strikes.to_vec(),
            vs: volatilities.to_vec(),
        };

        let zero = f64::EPSILON;

        let bounds = [
            (zero, f32::MAX as f64), // Alpha
            (-1.0, 1.0),             // Rho (correlation)
            (-1.0, f32::MAX as f64), // Nu
        ]
        .to_vec()
        .into_iter()
        .map(|(a, b)| (a, b))
        .collect();

        let model = calibrator;

        let solver = ParticleSwarm::new(bounds, 100); //-0.3593 -0.7238 2.0289
                                                      // .with_inertia_factor(0.1)? // Inertia factor (w)
                                                      // .with_cognitive_factor(2.)? // Cognitive (personal) factor
                                                      // .with_social_factor(2.)?; // Social (global) factor

        let executor = Executor::new(model, solver).configure(|state| state.max_iters(1000));

        let result = executor.run()?;
        let params = result.state().get_best_param().unwrap().position.to_vec();

        self.alpha = params[0];
        self.rho = params[1];
        self.nu = params[2];

        println!("TIME: {:?}", result.state().get_time());

        Ok(())
    }
}

#[cfg(test)]
mod tests_sabr {
    use super::*;

    #[test]
    fn test_sabr_volatility() {
        let f = 100.0;
        let k = 100.0;
        let t = 1.0;
        let alpha = 0.2;
        let beta = 0.5;
        let rho = 0.0;
        let nu = 0.4;

        let vol = sabr::sabr_volatility(f, k, t, alpha, beta, rho, nu);
        assert!((vol - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_sabr_calibrator() {
        let f = 100.0;
        let t = 1.0;
        let beta = 0.5;
        let alpha = 0.2;
        let rho = 0.0;
        let nu = 0.4;

        let ks = vec![90.0, 95.0, 100.0, 105.0, 110.0];
        let vs = vec![0.25, 0.22, 0.2, 0.18, 0.16];

        let mut sabr = Sabr02::new(f, alpha, beta, rho, nu);

        sabr.fit(&vs, &ks, t).unwrap();

        assert!((sabr.alpha - 0.2).abs() < 1e-10);
        assert!((sabr.rho - 0.0).abs() < 1e-10);
        assert!((sabr.nu - 0.4).abs() < 1e-10);
    }
}
