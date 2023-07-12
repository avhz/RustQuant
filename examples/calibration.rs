// This is a rough example of how to use the autodiff crate to calibrate a model to market data.
// The model is the Black-Scholes model, and the market data is the prices of call options on AAPL.
// We are simply calibrating the volatility parameter of the model.
// We get a volatility of 18.013%.
//
// Run: cargo run --release --example calibration

use std::time::Instant;
use RustQuant::autodiff::*;
use RustQuant::math::optimization::gradient_descent::GradientDescent;

fn main() {
    calibrate();
}

#[inline]
pub(crate) fn calibrate() {
    let graph = Graph::new();

    let gd = GradientDescent::new(0.01, 1000, std::f64::EPSILON.sqrt());

    let start = Instant::now();
    let result = gd.optimize(&mse, &vec![0.1], true);
    let duration = start.elapsed();

    println!("MSE  = \t {}", mse(&graph.vars(&[0.1])));
    println!("Vol  = \t {:?}", result.minimizer.first().unwrap());
    println!("Time = \t {:?}", duration);
}

fn mse<'v>(v: &[Variable<'v>]) -> Variable<'v> {
    // HARD CODED DATA FROM YAHOO FINANCE
    // These are June 23, 2023 call options for AAPL
    let strikes: &[f64] = &[
        70.0, 100.0, 110.0, 115.0, 120.0, 125.0, 130.0, 135.0, 140.0, 145.0, 148.0, 149.0, 150.0,
        152.5, 155.0, 157.5, 160.0, 162.5, 165.0, 167.5, 170.0, 172.5, 175.0, 177.5, 180.0, 182.5,
        185.0, 187.5, 190.0, 192.5, 195.0, 197.5, 200.0, 202.5, 205.0, 207.5, 210.0, 215.0, 220.0,
        225.0, 230.0, 235.0, 240.0, 245.0, 250.0, 255.0,
    ];
    let prices: &[f64] = &[
        108.57, 79.80, 75.93, 70.92, 65.89, 60.20, 54.00, 50.73, 45.80, 40.65, 34.75, 33.80, 35.05,
        33.24, 30.01, 27.73, 24.90, 23.21, 19.90, 17.57, 14.87, 12.40, 10.25, 7.81, 5.27, 3.26,
        1.69, 0.67, 0.24, 0.08, 0.03, 0.03, 0.02, 0.02, 0.02, 0.01, 0.02, 0.01, 0.02, 0.02, 0.01,
        0.01, 0.01, 0.01, 0.01, 0.01,
    ];

    // HARD CODED PARAMETERS
    let s = 184.92;
    let t = 4. / 250.;
    let r = 0.05;
    let d = 0.01;

    // Compute the squared errors:
    // MSE(Model - Market)
    let se = strikes
        .iter()
        .copied()
        .zip(prices.iter().copied())
        .map(|(strike, price)| {
            (black_scholes(s, strike, t, r, v[0], d, TypeFlag::CALL) - price).powf(2.)
        })
        .sum::<Variable>();

    // Return the MSE/Variable<'v>,
    se / (strikes.len() as f64)
}

#[allow(non_snake_case)]
#[inline]
fn N<'v>(x: Variable<'v>) -> Variable<'v> {
    0.5 * (-x / core::f64::consts::SQRT_2).erfc()
}

#[allow(non_snake_case)]
#[inline]
fn black_scholes<'v>(
    S: f64,
    K: f64,
    T: f64,
    r: f64,
    v: Variable<'v>,
    d: f64,
    type_flag: TypeFlag,
) -> Variable<'v> {
    let d1 = ((S / K).ln() + (r - d + v * v / 2.0) * T) / (v * T.sqrt());
    let d2 = d1 - v * T.sqrt();

    let Nd1 = N(d1);
    let Nd2 = N(d2);
    let Nd1_ = N(-d1);
    let Nd2_ = N(-d2);

    match type_flag {
        TypeFlag::CALL => S * (-d * T).exp() * Nd1 - K * (-r * T).exp() * Nd2,
        TypeFlag::PUT => -S * (-d * T).exp() * Nd1_ + K * (-r * T).exp() * Nd2_,
    }
}

#[allow(dead_code)]
enum TypeFlag {
    CALL,
    PUT,
}
