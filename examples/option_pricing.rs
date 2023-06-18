use RustQuant::options::*;

fn main() {
    let vanilla_option = EuropeanOption {
        initial_price: 100.0,
        strike_price: 110.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        dividend_rate: 0.02,
        time_to_maturity: 0.5,
    };

    let prices = vanilla_option.price();

    println!("Call price = \t {}", prices.0);
    println!("Put price = \t {}", prices.1);
}
