// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::macros::date;
use RustQuant::instruments::*;
use RustQuant::stochastics::*;

fn main() {
    // Set up the parameters.
    let underlying = 100.0;
    let strike = 100.0;
    let rate = 0.05;
    let time = 1.0;
    let volatility = 0.2;
    let expiry = date!(2025 - 01 - 01);

    // Create the stochastic process.
    let process = GeometricBrownianMotion::new(rate, volatility);
    let config = StochasticProcessConfig::new(
        underlying,
        0.0,
        time,
        365,
        StochasticScheme::EulerMaruyama,
        100_000,
        true,
        None
    );

    // Create the option contract.
    let direction = TypeFlag::Call;

    let exercise = ExerciseFlag::European { expiry };

    let contract = OptionContractBuilder::default()
        .type_flag(direction)
        .exercise_flag(exercise)
        .strike_flag(Some(StrikeFlag::Fixed))
        .build()
        .unwrap();

    // VANILLA
    let vanilla = EuropeanVanillaOption::new(strike, expiry, direction);
    let asian = AsianOption::new(
        contract.clone(),
        AveragingMethod::ArithmeticDiscrete,
        Some(strike),
    );
    let power = PowerOption::new(contract.clone(), strike, 2.0);

    println!(
        "Vanilla: {:?}",
        vanilla.price_monte_carlo(&process, &config, rate)
    );
    println!(
        "Asian: {:?}",
        asian.price_monte_carlo(&process, &config, rate)
    );
    println!(
        "Power: {:?}",
        power.price_monte_carlo(&process, &config, rate)
    );

    // let start = Instant::now();
    // let price = option.price_monte_carlo(process, &config, interest_rate);
    // println!("Elapsed time: {:?}", start.elapsed());

    // println!("Price: {}", price);

    // let underlying = 100.0;
    // let strike = 100.0;
    // let interest_rate = 0.05;
    // let time_to_maturity = 1.0;
    // let volatility = 0.2;

    // let contract = OptionContractBuilder::default()
    //     .type_flag(TypeFlag::Call)
    //     .exercise_flag(ExerciseFlag::European {
    //         expiry: date!(2025 - 01 - 01),
    //     })
    //     .strike_flag(Some(StrikeFlag::Fixed))
    //     .build()
    //     .unwrap();

    // let option = AsianOption::new(contract, AveragingMethod::ArithmeticDiscrete, Some(strike));
    // let process = GeometricBrownianMotion::new(interest_rate, volatility);

    // let config = StochasticProcessConfig::new(underlying, 0.0, time_to_maturity, 1000, 1000, true);

    // let start = Instant::now();
    // let price = option.price_monte_carlo(process, &config, interest_rate);
    // println!("Elapsed time: {:?}", start.elapsed());

    // println!("Price: {}", price);
}
