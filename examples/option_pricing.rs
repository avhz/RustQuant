use time::macros::date;
use RustQuant::instruments::options::*;

fn main() {
    let option = BlackScholesMerton {
        cost_of_carry: 0.05,
        underlying_price: 100.0,
        strike_price: 100.0,
        volatility: 0.3,
        risk_free_rate: 0.03,
        evaluation_date: None,
        expiration_date: date!(2024 - 12 - 31),
        option_type: TypeFlag::Call,
    };

    // Print the option price and greeks.
    // There are more greeks available, but these are the most common.
    println!("Call price = \t {}", option.price());
    println!("Call delta = \t {}", option.delta());
    println!("Call gamma = \t {}", option.gamma());
    println!("Call theta = \t {}", option.theta());
    println!("Call vega = \t {}", option.vega());
    println!("Call rho = \t {}", option.rho());
}
