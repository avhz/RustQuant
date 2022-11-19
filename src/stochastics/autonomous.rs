/// Autonomous stochastic process.
///
/// Autonomous refers to a process where the drift and diffusion
/// do not depend on time `t`
pub trait AutonomousStochasticProcess: StochasticProcess {
    fn drift(&self, x: f32) -> f32;

    fn diffusion(&self, x: f32) -> f32;

    fn pdv_diffusion_x(&self, x: f32) -> Option<f32> {
        None
    }

    fn euler_maruyama(&self, x_0: f32, t_0: f32, t_n: f32, n: usize) -> crate::SimulatedPath {
        use rand::prelude::Distribution;

        let dt: f32 = (t_n - t_0) / (n as f32);

        // Set up rng.
        let mut rng = rand::thread_rng();
        let increments: Vec<f32> = match statrs::distribution::Normal::new(0.0, 1.0) {
            Ok(dist) => dist,
            Err(_) => panic!("Bad parameters."),
        }
        .sample_iter(&mut rng)
        .take(n)
        .map(|x| (x as f32) * dt.sqrt())
        .collect();

        // Construct empty path.
        let mut path = crate::SimulatedPath::zeros_generic(
            nalgebra::Dim::from_usize(n + 1),
            nalgebra::Dim::from_usize(2),
        );

        // Fill in t.
        path[(0, 0)] = t_0;
        path[(n, 0)] = t_n;
        for t in 1..(n + 1) {
            path[(t, 0)] = t_0 + dt * (t as f32);
        }

        // Fill in x.
        path[(0, 1)] = x_0;
        for t in 0..n {
            path[(t + 1, 1)] = path[(t, 1)]
                + self.drift(path[(t, 1)]) * (path[(t + 1, 0)] - path[(t, 0)])
                + self.diffusion(path[(t, 1)]) * increments[t];
        }
        path
    }

    fn run_milstein(&self, x_0: f32, t_0: f32, t_n: f32, n: usize) -> crate::SimulatedPath {
        if self.pdv_diffusion_x(x_0) == None {
            panic!("Cannot run Milstein without defining first-derivative of drift.");
        }

        use rand::prelude::Distribution;

        let dt: f32 = (t_n - t_0) / (n as f32);

        // Set up rng.
        let mut rng = rand::thread_rng();
        let increments: Vec<f32> = match statrs::distribution::Normal::new(0.0, 1.0) {
            Ok(dist) => dist,
            Err(_) => panic!("Bad parameters."),
        }
        .sample_iter(&mut rng)
        .take(n)
        .map(|x| (x as f32) * dt.sqrt())
        .collect();

        // Construct empty path.
        let mut path = crate::SimulatedPath::zeros_generic(
            nalgebra::Dim::from_usize(n + 1),
            nalgebra::Dim::from_usize(2),
        );

        // Fill in t.
        path[(0, 0)] = t_0;
        path[(n, 0)] = t_n;
        for t in 1..(n + 1) {
            path[(t, 0)] = t_0 + dt * (t as f32);
        }

        // Fill in x.
        path[(0, 1)] = x_0;
        for t in 0..n {
            path[(t + 1, 1)] = path[(t, 1)]
                + self.drift(path[(t, 1)]) * (path[(t + 1, 0)] - path[(t, 0)])
                + self.diffusion(path[(t, 1)]) * increments[t]
                + 0.5
                    * self.diffusion(path[(t, 1)])
                    * self.pdv_diffusion_x(path[(t, 1)]).unwrap()
                    * (increments[t].powi(2) - (path[(t + 1, 0)] - path[(t, 0)]));
        }
        path
    }
}
