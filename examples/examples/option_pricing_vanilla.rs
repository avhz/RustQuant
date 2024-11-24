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

// ANCHOR: output
// Model: BlackScholes73 { s: 100.0, r: 0.05, v: 0.2 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "theta": -6.21680948799122,
//     "vega": 39.12469842218203,
//     "charm": 0.06220900042967281,
//     "zomma": -0.0837348839459069,
//     "price": 11.08579175284256,
//     "lambda": 5.802560832438351,
//     "rho": 58.59700502681849,
//     "delta": 0.6432598102161233,
//     "vanna": -0.2934352381663651,
//     "speed": -0.0004887850033760005,
//     "color": 0.009163226593140452,
//     "gamma": 0.01777400012276366,
//     "vomma": 11.303583301850122,
//     "ultima": -160.90535329601363,
// }

// Model: Merton73 { s: 100.0, r: 0.05, q: 0.03, v: 0.2 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "theta": -4.272756285502488,
//     "gamma": 0.017995444677726104,
//     "zomma": -0.08997722338863051,
//     "speed": -0.0003599088935545221,
//     "color": 0.009074941597741354,
//     "delta": 0.5641575465829798,
//     "ultima": -43.59776593173644,
//     "price": 9.092954864173855,
//     "rho": 52.084231395586315,
//     "vanna": 5.2399933461312134e-17,
//     "charm": 0.01906616295796282,
//     "vega": 39.6121492700665,
//     "vomma": -1.1534441205446944e-15,
//     "lambda": 6.204336819109864,
// }

// Model: Black76 { f: 100.0, r: 0.05, v: 0.2 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "zomma": -0.09046830043585345,
//     "speed": -0.00026845029437156125,
//     "vomma": -2.1679251440048266,
//     "ultima": 10.958928376819475,
//     "color": 0.009114622059396617,
//     "gamma": 0.017896686291437418,
//     "vanna": 0.19697379573327434,
//     "theta": -3.183941196979144,
//     "delta": 0.5127676040720752,
//     "vega": 39.394759146654856,
//     "rho": 47.7324390165048,
//     "price": 7.907921226166792,
//     "charm": -0.007741693912166335,
//     "lambda": 6.4842275157643305,
// }

// Model: Asay82 { f: 100.0, v: 0.2 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "ultima": 11.578909557862003,
//     "theta": -3.7818316687297053,
//     "rho": 50.4328138066247,
//     "vega": 41.62344506035159,
//     "vanna": 0.208117225301758,
//     "gamma": 0.018909158343648525,
//     "charm": 0.018909158343648536,
//     "lambda": 6.48422751576433,
//     "zomma": -0.0955863778447514,
//     "speed": -0.0002836373751547279,
//     "color": 0.008684807091491406,
//     "price": 8.355297299155879,
//     "delta": 0.5417764864957794,
//     "vomma": -2.290571514614832,
// }

// Model: GarmanKohlhagen83 { s: 100.0, r_d: 0.05, r_f: 0.03, v: 0.2 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "ultima": -43.59776593173644,
//     "vega": 39.6121492700665,
//     "theta": -4.272756285502488,
//     "lambda": 6.204336819109864,
//     "color": 0.009074941597741354,
//     "charm": 0.01906616295796282,
//     "rho": 52.084231395586315,
//     "vanna": 5.2399933461312134e-17,
//     "zomma": -0.08997722338863051,
//     "price": 9.092954864173855,
//     "gamma": 0.017995444677726104,
//     "delta": 0.5641575465829798,
//     "speed": -0.0003599088935545221,
//     "vomma": -1.1534441205446944e-15,
// }

// Model: Heston93 { s: 100.0, v: 0.05, r: 0.03, q: 0.2, rho: 0.1, kappa: 0.1, theta: 0.1, sigma: 0.1 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "delta": 0.2008452387306263,
//     "gamma": 0.010457662539600598,
//     "price": 2.683032821893125,
//     "rho": 19.152359760630095,
// }

// Model: Bachelier { f: 100.0, r: 0.05, v: 0.2 }
// Option: EuropeanVanillaOption { strike: 100.0, expiry: 2025-12-31, type_flag: Call }
// {
//     "delta": 0.5,
//     "vega": 0.4185313362945294,
//     "gamma": 1.9013503803678706,
//     "atm_price": 0.08370626725890587,
//     "theta": -0.03802700760735742,
//     "price": 0.079224298540133,
// }
// ANCHOR_END: output
