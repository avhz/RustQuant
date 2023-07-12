// This example compares the performance of the using finite difference to
// compute the gradient of the Black-Scholes model to the performance of
// using autodiff to compute the gradient.
//
// There are only 6 parameters in the Black-Scholes model, so the
// difference is only about 2x better using autodiff. However, the
// difference will be much more significant for models with more
// parameters.
//
// Run:  cargo run --release --example black_scholes

use finitediff::*;
use std::time::Instant;
use RustQuant::autodiff::*;

fn main() {
    test_black_scholes();
}

#[allow(non_snake_case)]
fn test_black_scholes() {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // BLACK SCHOLES ( PRICES ONLY )
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline]
    fn test_black_scholes_price(maturity: &usize) {
        #[inline]
        fn normcdf<'v>(x: Variable<'v>) -> Variable<'v> {
            0.5 * (-x / core::f64::consts::SQRT_2).erfc()
        }

        #[inline]
        fn black_scholes<'v>(
            S: Variable<'v>,
            K: Variable<'v>,
            T: Variable<'v>,
            r: Variable<'v>,
            v: Variable<'v>,
            d: Variable<'v>,
        ) -> Variable<'v> {
            let d1 = ((S / K).ln() + (r - d + v * v / 2.0) * T) / (v * T.sqrt());
            let d2 = d1 - v * T.sqrt();

            S * (-d * T).exp() * normcdf(d1) - K * (-r * T).exp() * normcdf(d2)
        }

        for price in 1..=100 {
            for time in 1..=*maturity {
                let graph = Graph::new();

                let s = graph.var(price as f64);
                let k = graph.var(50.0);
                let t = graph.var(time as f64 / *maturity as f64);
                let r = graph.var(0.05);
                let v = graph.var(0.25);
                let d = graph.var(0.02);

                // Compute price only
                let _call = black_scholes(s, k, t, r, v, d);
            }
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // BLACK SCHOLES AUTODIFF
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline]
    fn test_black_scholes_autodiff(maturity: &usize) {
        //let maturity = 365 * 10;

        #[inline]
        fn normcdf<'v>(x: Variable<'v>) -> Variable<'v> {
            0.5 * (-x / core::f64::consts::SQRT_2).erfc()
        }

        #[inline]
        fn black_scholes<'v>(
            S: Variable<'v>,
            K: Variable<'v>,
            T: Variable<'v>,
            r: Variable<'v>,
            v: Variable<'v>,
            d: Variable<'v>,
        ) -> Variable<'v> {
            let d1 = ((S / K).ln() + (r - d + v * v / 2.0) * T) / (v * T.sqrt());
            let d2 = d1 - v * T.sqrt();

            S * (-d * T).exp() * normcdf(d1) - K * (-r * T).exp() * normcdf(d2)
        }

        for price in 1..=100 {
            for time in 1..=*maturity {
                let graph = Graph::new();

                let s = graph.var(price as f64);
                let k = graph.var(50.0);
                let t = graph.var(time as f64 / *maturity as f64);
                let r = graph.var(0.05);
                let v = graph.var(0.25);
                let d = graph.var(0.02);

                // Compute price and Greeks
                let call = black_scholes(s, k, t, r, v, d);
                call.accumulate();
            }
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // BLACK SCHOLES FINITEDIFF
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    fn test_black_scholes_finitediff(maturity: &usize) {
        use ::statrs::function::erf::erfc;

        fn normcdf(x: f64) -> f64 {
            0.5 * erfc(-x / core::f64::consts::SQRT_2)
        }

        let black_scholes = |x: &Vec<f64>| -> f64 {
            let S = x[0];
            let K = x[1];
            let T = x[2];
            let r = x[3];
            let v = x[4];
            let d = x[5];

            let d1 = ((S / K).ln() + (r - d + v * v / 2.0) * T) / (v * T.sqrt());
            let d2 = d1 - v * T.sqrt();

            S * (-d * T).exp() * normcdf(d1) - K * (-r * T).exp() * normcdf(d2)
        };

        for price in 1..=100 {
            for time in 1..=*maturity {
                let s = price as f64;
                let k = 50.0;
                let t = time as f64 / *maturity as f64;
                let r = 0.05;
                let v = 0.25;
                let d = 0.02;

                let x = vec![s, k, t, r, v, d];

                // Compute price and Greeks
                black_scholes(&x);
                x.central_diff(&black_scholes);
            }
        }
    }

    let mat = 365 * 10;

    let start_price = Instant::now();
    test_black_scholes_price(&mat);
    let duration_price = start_price.elapsed();

    let start_autodiff = Instant::now();
    test_black_scholes_autodiff(&mat);
    let duration_autodiff = start_autodiff.elapsed();

    let start_finitediff = Instant::now();
    test_black_scholes_finitediff(&mat);
    let duration_finitediff = start_finitediff.elapsed();

    println!("TIME (prices only) \t {:?}", duration_price);
    println!("TIME (auto-diff) \t {:?}", duration_autodiff);
    println!("TIME (finite-diff) \t {:?}", duration_finitediff);
}
