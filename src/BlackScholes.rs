
use statrs::function::erf



fn main() {
    println!("Standard normal cdf at 0.3 = {}", normalCDF(0.3));
}

struct BlackScholes {
    volatility: f64,
    risk_free_rate: f64,
    time_to_expiry: f64,
    underlying_price: f64,
    strike_price: f64,
    dividend_yield: f64,
}

fn normalCDF(x: f64) -> f64 {
    0.5 + 0.5 * erf( x / SQRT_2 )
}

// Black-Scholes European Call Option Price
fn BlackScholesCall(
    const underlying_price: f64,
    const strike_price: f64,
    const volatility: f64,
    const risk_free_rate: f64,
    const time_to_expiry: f64,
    const dividend_yield: f64,
) -> f64 {
    const df: f64   = exp( - risk_free_rate * time_to_expiry );
    const Ff: f64   = underlying_price * exp((risk_free_rate-dividend_yield)*time_to_expiry);
    const std: f64  = volatility * sqrt(time_to_expiry);
    const d: f64    = log(Ff/strike_price) / std;
    const d1: f64   = d + 0.5 * std, 
    const d2: f64   = d1 - std;
    const nd1: f64  = normalCdf(d1), nd2 = normalCdf(d2);
    const c: f64    = df * (Ff * nd1 - K * nd2);

    return c;
}

// Black-Scholes European Put Option Price
