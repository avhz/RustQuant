use RustQuant::stochastics::*;

const INITIAL_VALUE: f64 = 10.0;
const START_TIME: f64 = 0.0;
const END_TIME: f64 = 1.0;
const NUM_STEPS: usize = 252;
const NUM_SIMS: usize = 1;
const PARALLEL: bool = false;

enum OptionType {
    Call = 1,
    Put = -1,
}

enum BarrierType {
    UpAndOut,
    DownAndOut,
    UpAndIn,
    DownAndIn,
}

#[rustfmt::skip]
fn barrier_option_payoff(
    path: &[f64],
    strike: f64,
    barrier: f64,
    option_type: OptionType,
    barrier_type: BarrierType,
) -> f64 {
    match (option_type, barrier_type) {
        (OptionType::Call, BarrierType::UpAndOut) => {
            if path.iter().any(|&x| x >= barrier) { 0.0 } else { (path.last().unwrap() - strike).max(0.0) }
        }
        (OptionType::Call, BarrierType::DownAndOut) => {
            if path.iter().any(|&x| x <= barrier) { 0.0 } else { (path.last().unwrap() - strike).max(0.0) }
        }
        (OptionType::Call, BarrierType::UpAndIn) => {
            if path.iter().any(|&x| x >= barrier) { (path.last().unwrap() - strike).max(0.0) } else { 0.0 }
        }
        (OptionType::Call, BarrierType::DownAndIn) => {
            if path.iter().any(|&x| x <= barrier) { (path.last().unwrap() - strike).max(0.0) } else { 0.0 }
        }
        (OptionType::Put, BarrierType::UpAndOut) => {
            if path.iter().any(|&x| x >= barrier) { (strike - path.last().unwrap()).max(0.0) } else { 0.0 }
        }
        (OptionType::Put, BarrierType::DownAndOut) => {
            if path.iter().any(|&x| x <= barrier) { (strike - path.last().unwrap()).max(0.0) } else { 0.0 }
        }
        (OptionType::Put, BarrierType::UpAndIn) => {
            if path.iter().any(|&x| x >= barrier) { 0.0 } else { (strike - path.last().unwrap()).max(0.0) }
        }
        (OptionType::Put, BarrierType::DownAndIn) => {
            if path.iter().any(|&x| x <= barrier) { 0.0 } else { (strike - path.last().unwrap()).max(0.0) }
        }
    }
}

#[rustfmt::skip]
fn main() {
    // Create new stochastic processes.
    let gbm = GeometricBrownianMotion::new(0.05, 0.9);

    // Generate path using Euler-Maruyama scheme.
    // Parameters: x_0, t_0, t_n, n, sims, parallel.
    let config = StochasticProcessConfig::new(
        INITIAL_VALUE, START_TIME, END_TIME, NUM_STEPS, StochasticScheme::EulerMaruyama, NUM_SIMS, PARALLEL, None
    );
    let gbm_out = gbm.generate(&config);

    // Price the options. 
    println!("Up-and-out call: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 12.0, OptionType::Call, BarrierType::UpAndOut));
    println!("Down-and-out call: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 8.0, OptionType::Call, BarrierType::DownAndOut));
    println!("Up-and-in call: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 12.0, OptionType::Call, BarrierType::UpAndIn));
    println!("Down-and-in call: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 8.0, OptionType::Call, BarrierType::DownAndIn));
    println!("Up-and-out put: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 12.0, OptionType::Put, BarrierType::UpAndOut));
    println!("Down-and-out put: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 8.0, OptionType::Put, BarrierType::DownAndOut));
    println!("Up-and-in put: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 12.0, OptionType::Put, BarrierType::UpAndIn));
    println!("Down-and-in put: {}", barrier_option_payoff(&gbm_out.paths[0], 10.0, 8.0, OptionType::Put, BarrierType::DownAndIn));
}
