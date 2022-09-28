#![allow(non_snake_case)]

// use super::*;

// ############################################################################
// FUNCTIONS
// ############################################################################

pub fn AmericanPutBinomialTree(
    S0: f64,
    K: f64,
    T: f64,
    r: f64,
    sigma: f64,
    q: f64,
    n: usize,
) -> f64 {
    //   '  T... expiration time
    //   '  S... stock price
    //   '  K... strike price
    //   '  q... dividend yield
    //   '  n... height of the binomial tree
    let deltaT: f64 = T / (n as f64);
    let up: f64 = (sigma * deltaT.sqrt()).exp();
    let p0: f64 = (up * (-q * deltaT).exp() - (-r * deltaT).exp()) / (up * up - 1.);
    let p1: f64 = (-r * deltaT).exp() - p0;

    // let mut p: Vec<f64> = Vec::with_capacity(n);
    let mut p: Vec<f64> = vec![0.0; n + 1];

    //   ' initial values at time T
    for i in 0..=n {
        p[i] = K - S0 * up.powi(2 * i as i32 - n as i32);
        if p[i] < 0.0 {
            p[i] = 0.0;
        };
    }
    //   ' move to earlier times
    for j in (0..n).rev() {
        for i in 0..j {
            //   ' binomial value
            p[i] = p0 * p[i + 1] + p1 * p[i];
            //   ' exercise value
            let exercise: f64 = K - S0 * up.powi(2 * i as i32 - j as i32);
            if p[i] < exercise {
                p[i] = exercise;
            };
        }
    }
    return p[0];
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn AmericanPutBinomialTree_test() {
        let price = AmericanPutBinomialTree(100., 110., 180. / 365., 0.05, 0.2, 0.0, 100);
        println!("{}", price);
        assert!(price - 10.9546 < 0.1);
    }
}
