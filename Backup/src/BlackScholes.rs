// use statrs::function::erf;

// fn main() {
//     println!("Standard normal cdf at 0.3 = {}", normalCDF(0.3));
// }

// // struct BlackScholes {
// //     volatility: f64,
// //     risk_free_rate: f64,
// //     time_to_expiry: f64,
// //     underlying_price: f64,
// //     strike_price: f64,
// //     dividend_yield: f64,
// // }

// fn normalCDF(x: f64) -> f64 {
//     0.5 + 0.5 * erf(x / SQRT_2)
// }

// // Black-Scholes European Call Option Price
// fn BlackScholesCall(
//     underlying_price: f64,
//     strike_price: f64,
//     volatility: f64,
//     risk_free_rate: f64,
//     time_to_expiry: f64,
//     dividend_yield: f64,
// ) -> f64 {
//     const df: f64 = (-risk_free_rate * time_to_expiry).exp();
//     const F: f64 = underlying_price * ((risk_free_rate - dividend_yield) * time_to_expiry).exp();
//     const std: f64 = volatility * (time_to_expiry).sqrt();
//     const d: f64 = (F / strike_price).ln() / std;
//     const d1: f64 = d + 0.5 * std;
//     const d2: f64 = d1 - std;
//     const nd1: f64 = normalCdf(d1);
//     const nd2: f64 = normalCdf(d2);
//     const c: f64 = df * (F * nd1 - K * nd2);

//     return c;
// }

// // Black-Scholes European Put Option Price
