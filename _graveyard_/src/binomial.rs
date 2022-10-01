/// # Arguments:
///
/// * `S` - Initial underlying price.
/// * `K` - Strike price.
/// * `T` - Time to expiry.
/// * `r` - Risk-free rate.
/// * `v` - Volatility.
/// * `q` - Dividend yield.
/// * `n` - Height of the binomial tree.
pub fn AmericanCallBinomialTree(S: f64, K: f64, T: f64, r: f64, v: f64, q: f64, n: usize) -> f64 {
    let deltaT: f64 = T / (n as f64);
    let up: f64 = (v * deltaT.sqrt()).exp();
    let p0: f64 = (up * (-q * deltaT).exp() - (-r * deltaT).exp()) / (up * up - 1.);
    let p1: f64 = (-r * deltaT).exp() - p0;

    let mut p: Vec<f64> = vec![0.0; n + 1];

    // Initial values at time T
    for i in 0..=n {
        p[i] = S * up.powi(2 * i as i32 - n as i32) - K;
        if p[i] < 0.0 {
            p[i] = 0.0;
        };
    }
    // Move to earlier times
    for j in (0..(n - 1)).rev() {
        for i in 0..=j {
            // Binomial value
            p[i] = p0 * p[i + 1] + p1 * p[i];
            // Exercise value
            let exercise: f64 = S * up.powi(2 * i as i32 - j as i32) - K;
            if p[i] < exercise {
                p[i] = exercise;
            };
        }
    }
    return p[0];
}

/// # Arguments:
///
/// * `S` - Initial underlying price.
/// * `K` - Strike price.
/// * `T` - Time to expiry.
/// * `r` - Risk-free rate.
/// * `v` - Volatility.
/// * `q` - Dividend yield.
/// * `n` - Height of the binomial tree.
pub fn AmericanPutBinomialTree(S: f64, K: f64, T: f64, r: f64, v: f64, q: f64, n: usize) -> f64 {
    let deltaT: f64 = T / (n as f64);
    let up: f64 = (v * deltaT.sqrt()).exp();
    let p0: f64 = (up * (-q * deltaT).exp() - (-r * deltaT).exp()) / (up * up - 1.);
    let p1: f64 = (-r * deltaT).exp() - p0;

    let mut p: Vec<f64> = vec![0.0; n + 1];

    // Initial values at time T
    for i in 0..=n {
        p[i] = K - S * up.powi(2 * i as i32 - n as i32);
        if p[i] < 0.0 {
            p[i] = 0.0;
        };
    }
    // Move to earlier times
    for j in (0..(n - 1)).rev() {
        for i in 0..=j {
            // Binomial value
            p[i] = p0 * p[i + 1] + p1 * p[i];
            // Exercise value
            let exercise: f64 = K - S * up.powi(2 * i as i32 - j as i32);
            if p[i] < exercise {
                p[i] = exercise;
            };
        }
    }
    return p[0];
}

// #[test]
// fn binomial_parity() {
//     let c = AmericanCallBinomialTree(100.0, 110.0, 1.0, 0.05, 0.1, 0.0, 1000);
//     let p = AmericanPutBinomialTree(100.0, 110.0, 1.0, 0.05, 0.1, 0.0, 1000);
//     println!("Call: {}, Put {}", c, p);
//     let parity = c - p - 100.0 + 110.0 * (-0.05_f64).exp();
//     println!("parity = {}", parity);
//     assert_approx_equal(parity, 0.0, 1e-4);
// }

// #[test]
// fn AmericanCallBinomialTree_test() {
//     let call = AmericanCallBinomialTree(100.0, 110.0, 1.0, 0.05, 0.2, 0.0, 2000);
//     println!("American Call Price: {}\n\n", call);
//     assert!((call - 6.0401).abs() < 0.0000000001);
// }

// #[test]
// fn AmericanPutBinomialTree_test() {
//     let put = AmericanPutBinomialTree(100.0, 110.0, 1.0, 0.05, 0.2, 0.0, 2000);
//     println!("American Put Price: {}\n\n", put);
//     assert!((put - 11.9051).abs() < 0.1);
// }
