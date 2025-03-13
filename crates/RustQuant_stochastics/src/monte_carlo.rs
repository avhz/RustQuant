use rayon::prelude::*;
use rand::prelude::Distribution;
use rand::{rngs::StdRng, SeedableRng};
use crate::process::{StochasticProcess, Trajectories, StochasticProcessConfig, StochasticScheme};
use RustQuant_math::{Distribution as LocalDistribution, Poisson};


enum NoiseGenerator {
    Dynamic(StdRng),
    Fractional((fn(f64, usize, f64, Option<u64>) -> Vec<f64>, f64)),
}

pub(crate) fn run_monte_carlo<T: StochasticProcess>(
        stochastic_process: &T,
        config: &StochasticProcessConfig,
        jump_config: Option<f64>,
        fractional_config: Option<(fn(f64, usize, f64, Option<u64>) -> Vec<f64>, f64)>
    ) -> Trajectories {
    assert!(config.t_0 < config.t_n);

    let dt: f64 = (config.t_n - config.t_0) / (config.n_steps as f64);

    let times: Vec<f64> = (0..=config.n_steps)
        .map(|t| config.t_0 + dt * (t as f64))
        .collect();

    let times_ref: &[f64] = &times;
    let normal_dist: rand_distr::Normal<f64> = rand_distr::Normal::new(0.0, 1.0).unwrap();
    let diffusion_scale: f64 = dt.sqrt();

    let jumps: &[f64] = match jump_config {
        Some(lambda) => {
            &Poisson::new(lambda * dt).sample(config.n_steps).unwrap()
        }
        None => &vec![],
    };

    let scheme = match config.scheme {
        StochasticScheme::EulerMaruyama => Box::new({
            |path: &mut Vec<f64>, mut noise_gen: NoiseGenerator| {

                let fraction_noise: Vec<f64> = match noise_gen {
                    NoiseGenerator::Fractional(fractional_config) => {
                        fractional_config.0(fractional_config.1, config.n_steps, config.t_n, config.seed)
                    },
                    NoiseGenerator::Dynamic(_) => vec![],
                };

                for t in 0..config.n_steps {                    
                    path.push(
                        path[t]
                        + stochastic_process.drift(path[t], times_ref[t]) * dt
                        + stochastic_process.diffusion(path[t], times_ref[t]) * diffusion_scale * match noise_gen {
                            NoiseGenerator::Dynamic(ref mut rng) => normal_dist.sample(rng),
                            NoiseGenerator::Fractional(_) => fraction_noise[t],
                        } + match !jumps.is_empty() {
                            true => calculate_jump(stochastic_process, path[t], times_ref[t], jumps[t]),
                            false => 0.0,
                        }
                    );
                }
            }
        }) as Box<dyn Fn(&mut Vec<f64>, NoiseGenerator) + Send + Sync>,
        StochasticScheme::Milstein => Box::new({
            |path: &mut Vec<f64>, mut noise_gen: NoiseGenerator| {

                let fraction_noise: Vec<f64> = match noise_gen {
                    NoiseGenerator::Fractional(fractional_config) => {
                        fractional_config.0(fractional_config.1, config.n_steps, config.t_n, config.seed)
                    },
                    NoiseGenerator::Dynamic(_) => vec![],
                };

                let mut dw: f64;
                for t in 0..config.n_steps {
                    dw = match noise_gen {
                        NoiseGenerator::Dynamic(ref mut rng) => normal_dist.sample(rng),
                        NoiseGenerator::Fractional(_) => fraction_noise[t],
                    };
                    path.push(
                        path[t]
                        + stochastic_process.drift(path[t], times_ref[t]) * dt
                        + stochastic_process.diffusion(path[t], times_ref[t]) * dw
                        + 0.5
                        * (stochastic_process.diffusion(path[t], times_ref[t])
                        * ((stochastic_process.diffusion(path[t] + 1e-5, times_ref[t])
                                    - stochastic_process.diffusion(path[t] - 1e-5, times_ref[t]))
                                    / 2.0
                                    * 1e-5)
                        * ((dw * dw) - dt))
                        + match !jumps.is_empty() {
                            true => calculate_jump(stochastic_process, path[t], times_ref[t], jumps[t]),
                            false => 0.0,
                        }
                    );
                }
            }
        }) as Box<dyn Fn(&mut Vec<f64>, NoiseGenerator) + Send + Sync>,
        StochasticScheme::StrangSplitting => Box::new({
            |path: &mut Vec<f64>, mut noise_gen: NoiseGenerator| {


                let fraction_noise: Vec<f64> = match noise_gen {
                    NoiseGenerator::Fractional(fractional_config) => {
                        fractional_config.0(fractional_config.1, config.n_steps, config.t_n, config.seed)
                    },
                    NoiseGenerator::Dynamic(_) => vec![],
                };

                for t in 0..config.n_steps {
                    path.push(
                        path[t]
                        + 0.5 * stochastic_process.drift(path[t], times_ref[t]) * dt
                        + stochastic_process.diffusion(
                            path[t] + 0.5 * stochastic_process.drift(path[t], times[t]) * dt,
                            times[t] + 0.5 * dt,
                        ) * match noise_gen {
                            NoiseGenerator::Dynamic(ref mut rng) => normal_dist.sample(rng),
                            NoiseGenerator::Fractional(_) => fraction_noise[t],
                        } + 0.5 * stochastic_process.drift(path[t], times_ref[t]) * dt
                        + match !jumps.is_empty() {
                            true => calculate_jump(stochastic_process, path[t], times_ref[t], jumps[t]),
                            false => 0.0,
                        }
                    );
                }
            }
        }) as Box<dyn Fn(&mut Vec<f64>, NoiseGenerator) + Send + Sync>,
    };

    let mut paths: Vec<Vec<f64>> = vec![vec![config.x_0]; config.m_paths];

    let base_seed: u64 = config.seed.unwrap_or_else(rand::random);
    if config.parallel {
        paths.par_iter_mut().enumerate().for_each(|(i, path)| {
            let noise_gen = match fractional_config {
                Some(fractional_config) => NoiseGenerator::Fractional(fractional_config),
                None => NoiseGenerator::Dynamic(StdRng::seed_from_u64(base_seed.wrapping_add(i as u64))),
            };
            scheme(path, noise_gen);
        });
    } else {
        paths.iter_mut().enumerate().for_each(|(i, path)| {
            let noise_gen = match fractional_config {
                Some(fractional_config) => NoiseGenerator::Fractional(fractional_config),
                None => NoiseGenerator::Dynamic(StdRng::seed_from_u64(base_seed.wrapping_add(i as u64))),
            };
            scheme(path, noise_gen);
        });
    }

    Trajectories {
        times: times.to_vec(),
        paths,
    }
}

fn calculate_jump<T: StochasticProcess>(
    stochastic_process: &T,
    x: f64,
    time_ref: f64,
    jump: f64,
) -> f64 {
    match stochastic_process.jump(x, time_ref) {
        Some(jump_size) => {
            if jump > 0.0 {
                jump_size
            } else {
                0.0
            }
        },
        None => 0.0,
    }
}

#[cfg(test)]
mod test_process {
    use crate::geometric_brownian_motion::GeometricBrownianMotion;
    use crate::{StochasticScheme, StochasticProcessConfig, StochasticProcess};
    use std::time::Instant;
    use super::run_monte_carlo;

    #[test]
    fn test_run_monte_carlo() {

        struct CustomProcess {
            pub mu: f64,
        
            pub sigma: f64,
        }
        
        impl StochasticProcess for CustomProcess {

            fn drift(&self, x: f64, _t: f64) -> f64 {
                self.mu * x
            }
        
            fn diffusion(&self, x: f64, _t: f64) -> f64 {
                self.sigma * x
            }
        
            fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
                Some(1.0)
            }
        }
        
        let config = StochasticProcessConfig::new(
            10.0,
            0.0,
            1.0,
            10,
            StochasticScheme::EulerMaruyama,
            3,
            false,
            Some(9999),
        );

        let stochastic_process = CustomProcess { mu: 0.1, sigma: 0.2 };

        run_monte_carlo(&stochastic_process, &config, Some(1.0), None);
    }
}
