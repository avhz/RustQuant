use crate::stochastics::*;

pub struct GeometricBrownianBridge {
    mu: TimeDependent,
    sigma: TimeDependent,
    end_value: f64, // End value
    end_time: f64, // End time
}

impl GeometricBrownianBridge {
    pub fn new(mu: impl Into<TimeDependent>, sigma: impl Into<TimeDependent>, end_value: f64, end_time: f64) -> Self {
        Self {
            mu: mu.into(),
            sigma: sigma.into(),
            end_value,
            end_time,
        }
    }
}

impl StochasticProcess for GeometricBrownianBridge {
    fn drift(&self, x: f64, t: f64) -> f64 {
        self.mu.0(t) * x + (self.end_value.ln() - x.ln()) / (self.end_time - t) * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        assert!(self.sigma.0(t) >= 0.0);
        self.sigma.0(t) * x
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None // No jump term
    }

}

#[cfg(test)]
mod tests_gbm_bridge {
    use super::*;
    use crate::statistics::*;

    #[test]
    fn test_geometric_brownian_motion_bridge() -> Result<(), Box<dyn std::error::Error>> {
        let gbm = GeometricBrownianBridge::new(0.05, 0.9, 10.0, 0.5);

        let output = gbm.euler_maruyama(10.0, 0.0, 0.5, 125, 10000, false);

        // Test the distribution of the final values.
        let X_T: Vec<f64> = output
            .paths
            .iter()
            .filter_map(|v| v.last().cloned())
            .collect();

        let E_XT = X_T.mean();
        let V_XT = X_T.variance();
        // E[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_eq!(E_XT, 10.0);
        // V[X_T] = https://en.wikipedia.org/wiki/Geometric_Brownian_motion
        assert_eq!(
            V_XT,
            0.0
        );

        // let file1 = "./images/GBM1.png";
        // plot_vector((&output.trajectories[0]).clone(), file1).unwrap();
        // let file2 = "./images/GBM2.png";
        // plot_vector((&output.trajectories[1]).clone(), file2)

        std::result::Result::Ok(())
    }
}