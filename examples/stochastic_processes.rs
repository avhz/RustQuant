// Run this example using:
//      cargo run --example stochastic_processes
//
// This example generates paths for each stochastic process
// and plots them using the `plotters` crate.
//
// See the ./images/ directory for the output.

use RustQuant::plot_vector;
use RustQuant::stochastics::*;

#[rustfmt::skip]
fn main() {
    // Create new stochastic processes.
    let abm = ArithmeticBrownianMotion::new(0.05, 0.9);
    let bdt = BlackDermanToy::new(0.13, theta_t);
    let bm = BrownianMotion::new();
    let cir = CoxIngersollRoss::new(0.05, 0.9, 0.1);
    let ev = ExtendedVasicek::new(alpha_t, 2.0, theta_t);
    let gbm = GeometricBrownianMotion::new(0.05, 0.9);
    let hl = HoLee::new(0.2, theta_t);
    let hw = HullWhite::new(0.1, 0.2, theta_t);
    let ou = OrnsteinUhlenbeck::new(0.05, 0.9, 0.1);
    let fbm = FractionalBrownianMotion::new(0.7);
    let mjd = MertonJumpDiffusion::new(0.05, 0.9, 10.0, 0.0, 0.3);
    let gbb = GeometricBrownianBridge::new(0.05, 0.9, 5.0, 0.5);

    // Generate path using Euler-Maruyama scheme.
    // Parameters: x_0, t_0, t_n, n, sims, parallel.
    let abm_out = abm.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let bdt_out = bdt.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let bm_out  = bm.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let cir_out = cir.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let ev_out  = ev.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let gbm_out = gbm.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let hl_out  = hl.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let hw_out  = hw.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let ou_out  = ou.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let fbm_out = fbm.euler_maruyama(0.0, 0.0, 0.5, 1000, 1, false);
    let mjd_out = mjd.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);
    let gbb_out = gbb.euler_maruyama(10.0, 0.0, 0.5, 1000, 1, false);

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
}

fn theta_t(_t: f64) -> f64 {
    0.1
}
fn alpha_t(_t: f64) -> f64 {
    0.05
}
