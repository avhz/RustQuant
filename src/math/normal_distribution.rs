// #![deny(missing_docs)]

// use {
//     rand::thread_rng,
//     rand_distr::{Distribution, Normal},
//     statrs::function::erf,
//     std::f64::consts::{PI, SQRT_2},
// };

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // STANDARD NORMAL DISTRIBUTION STRUCT
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// /// Base struct for a Standard Normal distribution.
// /// Gaussian with mean = 0, variance = 1.
// #[derive(Debug)]
// pub struct StandardNormal {}

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // FUNCTIONS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// impl Default for StandardNormal {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl StandardNormal {
//     /// New standard normal distribution.
//     pub fn new() -> Self {
//         Self {}
//     }

//     /// Standard Normal Density Function
//     pub fn pdf(&self, x: f64) -> f64 {
//         (-x * x / 2.0).exp() / (2.0 * PI).sqrt()
//     }

//     /// Standard Normal Distribution Function
//     ///
//     /// I used `erfc` (complementary error function) instead of `erf` to avoid
//     /// subtractive cancellation that leads to inaccuracy in the tails.
//     pub fn cdf(&self, x: f64) -> f64 {
//         0.5 * erf::erfc(-x / SQRT_2)
//     }

//     /// Standard Normal Random Variates Generator
//     pub fn variates(&self, n: usize) -> Vec<f64> {
//         let mut rng = thread_rng();
//         let normal = Normal::new(0.0, 1.0).unwrap();
//         let mut variates: Vec<f64> = Vec::with_capacity(n);

//         for _ in 0..variates.capacity() {
//             variates.push(normal.sample(&mut rng));
//         }

//         variates
//     }
// }

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // TESTS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::assert_approx_equal;

//     #[test]
//     fn test_pnorm() {
//         let normal = StandardNormal::new();

//         // Values from WolframAlpha
//         assert_approx_equal!(normal.cdf(-4.0), 0.00003167, 1e-8);
//         assert_approx_equal!(normal.cdf(-3.0), 0.00134990, 1e-8);
//         assert_approx_equal!(normal.cdf(-2.0), 0.02275013, 1e-8);
//         assert_approx_equal!(normal.cdf(-1.0), 0.15865525, 1e-8);
//         assert_approx_equal!(normal.cdf(0.0), 0.5, 1e-8);
//         assert_approx_equal!(normal.cdf(1.0), 0.84134475, 1e-8);
//         assert_approx_equal!(normal.cdf(2.0), 0.97724987, 1e-8);
//         assert_approx_equal!(normal.cdf(3.0), 0.99865010, 1e-8);
//         assert_approx_equal!(normal.cdf(4.0), 0.99996833, 1e-8);
//     }

//     #[test]
//     fn test_dnorm() {
//         let normal = StandardNormal::new();

//         // Values from WolframAlpha
//         assert_approx_equal!(normal.pdf(-4.0), 0.00013383, 1e-8);
//         assert_approx_equal!(normal.pdf(-3.0), 0.00443185, 1e-8);
//         assert_approx_equal!(normal.pdf(-2.0), 0.05399097, 1e-8);
//         assert_approx_equal!(normal.pdf(-1.0), 0.24197072, 1e-8);
//         assert_approx_equal!(normal.pdf(0.0), 0.39894228, 1e-8);
//         assert_approx_equal!(normal.pdf(1.0), 0.24197072, 1e-8);
//         assert_approx_equal!(normal.pdf(2.0), 0.05399097, 1e-8);
//         assert_approx_equal!(normal.pdf(3.0), 0.00443185, 1e-8);
//         assert_approx_equal!(normal.pdf(4.0), 0.00013383, 1e-8);
//     }

//     #[test]
//     fn test_rnorm() {
//         let normal = StandardNormal::new();

//         let v = normal.variates(1000);

//         println!("{:?}", v);
//         let mean = (v.iter().sum::<f64>()) / (v.len() as f64);
//         println!("MEAN = {}", mean);
//         // assert!(5 == 6);
//     }
// }
