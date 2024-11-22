use RustQuant::data::*;
use RustQuant::time::*;

fn main() {
    let market_data: MarketData<AustraliaCalendar> = MarketDataBuilder::default()
        .underlying_price(Some(100.0))
        .volatility(Some(0.2))
        .dividend_yield(Some(0.0))
        .build()
        .unwrap();

    println!("{:?}", market_data);
}
