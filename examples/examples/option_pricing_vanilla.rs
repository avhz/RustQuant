use time::macros::date;
use RustQuant::instruments::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: option_definition
    // Define the option contract.
    let option = EuropeanVanillaOptionBuilder::default()
        .strike(100.0)
        .expiry(date!(2025 - 12 - 31))
        .type_flag(TypeFlag::Call)
        .build()?;
    // ANCHOR_END: option_definition

    // ANCHOR: model_definitions
    // Define some models to price the option.
    let bs73 = BlackScholes73::new(100., 0.05, 0.2);
    let m73 = Merton73::new(100., 0.05, 0.03, 0.2);
    let b76 = Black76::new(100., 0.05, 0.2);
    let a82 = Asay82::new(100., 0.2);
    let gk83 = GarmanKohlhagen83::new(100., 0.05, 0.03, 0.2);
    let h93 = Heston93::new(100., 0.05, 0.03, 0.2, 0.1, 0.1, 0.1, 0.1);
    let bch = Bachelier::new(100., 0.05, 0.2);
    // ANCHOR_END: model_definitions

    // ANCHOR: option_pricing
    // Print the option, model, price and greeks.
    AnalyticOptionPricer::new(option, bs73).report();
    AnalyticOptionPricer::new(option, m73).report();
    AnalyticOptionPricer::new(option, b76).report();
    AnalyticOptionPricer::new(option, a82).report();
    AnalyticOptionPricer::new(option, gk83).report();
    AnalyticOptionPricer::new(option, h93).report();
    AnalyticOptionPricer::new(option, bch).report();
    // ANCHOR_END: option_pricing

    Ok(())
}
