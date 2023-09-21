use RustQuant::time::{Duration, OffsetDateTime};
use RustQuant::instruments::options::*;

fn main() {
    // Also has a new() method.
    let vanilla_option = EuropeanOption {
        initial_price: 100.0,
        strike_price: 110.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        dividend_rate: 0.02,
        evaluation_date: None, // Optional field. Defaults to now.
        expiration_date: OffsetDateTime::now_utc() + Duration::days(365),
    };

    let prices = vanilla_option.price();

    println!("Call price = \t {}", prices.0);
    println!("Put price = \t {}", prices.1);
}
