// ANCHOR: stochastic_processes

// Run this example using:
//      cargo run --example stochastic_processes
//
// This example generates paths for each stochastic process
// and plots them using the `plotters` crate.
//
// See the ./images/ directory for the output.

use RustQuant::stochastics::*;
use RustQuant::utils::plot_vector;

const INITIAL_VALUE: f64 = 10.0;
const START_TIME: f64 = 0.0;
const END_TIME: f64 = 1.0;
const NUM_STEPS: usize = 252;
const NUM_SIMS: usize = 1;
const PARALLEL: bool = false;

#[rustfmt::skip]
fn main() {
    // Create new stochastic processes.
    let abm = ArithmeticBrownianMotion::new(0.05, 0.9);
    let bdt = BlackDermanToy::new(0.13, 0.1);
    let bm = BrownianMotion::new();
    let cir = CoxIngersollRoss::new(0.05, 0.9, 0.1);
    let ev = ExtendedVasicek::new(0.05, 2.0, 0.1);
    let gbm = GeometricBrownianMotion::new(0.05, 0.9);
    let hl = HoLee::new(0.2, 0.1);
    let hw = HullWhite::new(0.1, 0.2, 0.1);
    let ou = OrnsteinUhlenbeck::new(0.05, 0.9, 0.1);
    let fbm = FractionalBrownianMotion::new(0.7, FractionalProcessGeneratorMethod::FFT);
    let mjd = MertonJumpDiffusion::new(0.05, 0.5, 30.0, 0.0, 5.0);
    let gbb = GeometricBrownianBridge::new(0.05, 0.9, INITIAL_VALUE, END_TIME);
    let cev = ConstantElasticityOfVariance::new(0.05, 0.9, f64::sin);

    // Generate path using Euler-Maruyama scheme.
    // Parameters: x_0, t_0, t_n, n, sims, parallel.
    let config = StochasticProcessConfig::new(
        INITIAL_VALUE, START_TIME, END_TIME, NUM_STEPS, StochasticScheme::EulerMaruyama, NUM_SIMS, PARALLEL, None
    );

    let abm_out = abm.monte_carlo(&config);
    let bdt_out = bdt.monte_carlo(&config);
    let bm_out  = bm.monte_carlo(&config);
    let cir_out = cir.monte_carlo(&config);
    let ev_out  = ev.monte_carlo(&config);
    let gbm_out = gbm.monte_carlo(&config);
    let hl_out  = hl.monte_carlo(&config);
    let hw_out  = hw.monte_carlo(&config);
    let ou_out  = ou.monte_carlo(&config);
    let fbm_out = fbm.monte_carlo(&config);
    let mjd_out = mjd.monte_carlo(&config);
    let gbb_out = gbb.monte_carlo(&config);
    let cev_out = cev.monte_carlo(&config);

    // Plot the paths.
    plot_vector!(abm_out.paths[0].clone(), "./images/arithmetic_brownian_motion.png");
    plot_vector!(bdt_out.paths[0].clone(), "./images/black_derman_toy.png");
    plot_vector!(bm_out.paths[0].clone(),  "./images/brownian_motion.png");
    plot_vector!(cir_out.paths[0].clone(), "./images/cox_ingersoll_ross.png");
    plot_vector!(ev_out.paths[0].clone(),  "./images/extended_vasicek.png");
    plot_vector!(gbm_out.paths[0].clone(), "./images/geometric_brownian_motion.png");
    plot_vector!(hl_out.paths[0].clone(),  "./images/ho_lee.png");
    plot_vector!(hw_out.paths[0].clone(),  "./images/hull_white.png");
    plot_vector!(ou_out.paths[0].clone(),  "./images/ornstein_uhlenbeck.png");
    plot_vector!(fbm_out.paths[0].clone(), "./images/fractional_brownian_motion.png");
    plot_vector!(mjd_out.paths[0].clone(), "./images/merton_jump_diffusion.png");
    plot_vector!(gbb_out.paths[0].clone(), "./images/geometric_brownian_bridge.png");
    plot_vector!(cev_out.paths[0].clone(), "./images/constant_elasticity_of_variance.png");

    plot_trajectories(&gbm_out, true);
}

// ANCHOR_END: stochastic_processes

use plotly::{common::Mode, Plot, Scatter};

fn plot_trajectories(paths: &Trajectories, show: bool) -> Plot {
    let mut plot = Plot::new();

    let xs = paths
        .times
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for (i, path) in paths.paths.iter().enumerate() {
        let ys = path.iter().cloned().collect::<Vec<f64>>();

        let trace = Scatter::new(xs.clone(), ys)
            .mode(Mode::Lines)
            .name(format!("Path {}", i + 1));

        plot.add_trace(trace);
    }

    if show {
        plot.show();
    }

    plot
}
