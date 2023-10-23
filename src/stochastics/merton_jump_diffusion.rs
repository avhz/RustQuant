use crate::stochastics::*;

pub struct MertonJumpDiffusion {
    mu: TimeDependent,
    sigma: TimeDependent,
    lam: TimeDependent,
    m: f64,
    v: f64,
}

impl MertonJumpDiffusion {
    pub fn new(mu: impl Into<TimeDependent>, sigma: impl Into<TimeDependent>, lam: impl Into<TimeDependent>, m: f64, v: f64) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            lam: lam.into(),
            m,
            v,
        }
    }
}

impl StochasticProcess for MertonJumpDiffusion {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        self.sigma.0(t) * x
    }

    fn jump(&self, x: f64, t: f64) -> Option<f64> {
        let lam_t = self.lam.0(t);
        if lam_t > 0.0 {
            let jump_size = self.m + self.v * rand::random::<f64>();
            Some(x * (1.0 + jump_size))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests_gbm_bridge {
    use super::*;
    use crate::{assert_approx_equal, statistics::*};

    #[test]
    fn test_geometric_brownian_motion_bridge() -> Result<(), Box<dyn std::error::Error>> {
        let mjd = MertonJumpDiffusion::new(0.05, 0.8, 1.0, 0.0, 0.3);

        let output = mjd.euler_maruyama(10.0, 0.0, 0.5, 125, 10000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        println!("X_T = {:?}", X_T);

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(E_XT, 10.0, 0.5);
        // V[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_approx_equal!(
            V_XT,
            0.0,
            0.5
        );

        std::result::Result::Ok(())
    }
}