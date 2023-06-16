use RustQuant::{
    stochastics::{
        black_derman_toy::{BlackDermanToy, Sigma},
        *,
    },
    utilities::plot_vector,
};

fn main() {
    // Create new stochastic processes.
    let abm = ArithmeticBrownianMotion::new(0.05, 0.9);
    let bdt = BlackDermanToy::new(Sigma::Const(0.13), theta_t);
    let bm = BrownianMotion::new();
    let cir = CoxIngersollRoss::new(0.05, 0.9, 0.1);
    let ev = ExtendedVasicek::new(alpha_t, 2.0, theta_t);
    let gbm = GeometricBrownianMotion::new(0.05, 0.9);
    let hl = HoLee::new(0.2, theta_t);
    let hw = HullWhite::new(2.0, 0.2, theta_t);
    let ou = OrnsteinUhlenbeck::new(0.05, 0.9, 0.1);

    // Generate path using Euler-Maruyama scheme.
    // Parameters: x_0, t_0, t_n, n, sims, parallel.
    let abm_out = (&abm).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let bdt_out = (&bdt).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let bm_out = (&bm).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let cir_out = (&cir).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let ev_out = (&ev).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let gbm_out = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let hl_out = (&hl).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let hw_out = (&hw).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);
    let ou_out = (&ou).euler_maruyama(10.0, 0.0, 0.5, 100, 1, false);

    // Plot the paths.
    //
    // CURRENTLY PANICS.
    // Waiting on updated plotters crate (v0.0.34 -> v0.0.35).
    // See here: https://github.com/plotters-rs/plotters/issues/453
    plot_vector((&abm_out.paths[0]).clone(), "./images/abm.png").unwrap();
}

fn theta_t(_t: f64) -> f64 {
    1.5
}
fn alpha_t(_t: f64) -> f64 {
    2.0
}
