use time::macros::date;
use RustQuant::instruments::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the option contract.
    let option = EuropeanVanillaOptionBuilder::default()
        .strike(100.0)
        .expiry(date!(2025 - 12 - 31))
        .type_flag(TypeFlag::Call)
        .build()?;

    // Define some models to price the option.
    let bs73 = BlackScholes73::new(100., 0.05, 0.2);
    let m73 = Merton73::new(100., 0.05, 0.03, 0.2);
    let b76 = Black76::new(100., 0.05, 0.2);
    let a82 = Asay82::new(100., 0.2);
    let gk83 = GarmanKohlhagen83::new(100., 0.05, 0.03, 0.2);

    // Print the option price and greeks.
    // There are more greeks available, but these are the most common.
    AnalyticOptionPricer::new(option.clone(), bs73.clone()).report();
    AnalyticOptionPricer::new(option.clone(), m73.clone()).report();
    AnalyticOptionPricer::new(option.clone(), b76.clone()).report();
    AnalyticOptionPricer::new(option.clone(), a82.clone()).report();
    AnalyticOptionPricer::new(option.clone(), gk83.clone()).report();

    Ok(())
}
