// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::math::rootfinder::{Rootfinder, RootfinderData};

/// Brent root-finding algorithm.
pub struct Brent<F>
where
    F: Fn(f64) -> f64,
{
    function: F,
    guess: f64,
    data: RootfinderData,
}

impl<F> Brent<F>
where
    F: Fn(f64) -> f64,
{
    /// Create a new Brent solver.
    pub fn new(function: F, guess: f64, data: RootfinderData) -> Self {
        Self {
            function,
            guess,
            data,
        }
    }
}

impl<F> Rootfinder<F> for Brent<F>
where
    F: Fn(f64) -> f64,
{
    fn value(&self, x: f64) -> f64 {
        (self.function)(x)
    }

    fn derivative(&self, _: f64) -> f64 {
        0.0
    }

    fn solve_impl(&mut self) -> f64 {
        let mut min1: f64;
        let mut min2: f64;

        let mut p: f64;
        let mut q: f64;
        let mut r: f64;
        let mut s: f64;
        let mut x_acc1: f64;
        let mut x_mid: f64;

        let mut d: f64 = 0.0;
        let mut e: f64 = 0.0;

        // let mut root = self.data.x_max;
        self.data.root = self.data.x_max;
        let mut froot = self.data.y_max;

        while self.data.iteration_count <= Self::MAX_ITERATIONS {
            if (froot > 0.0 && self.data.y_max > 0.0) || (froot < 0.0 && self.data.y_max < 0.0) {
                self.data.x_max = self.data.x_min;
                self.data.y_max = self.data.y_min;
                e = self.data.root - self.data.x_min;
                d = e;
            }

            if self.data.y_max.abs() < froot.abs() {
                // Adjust x's
                self.data.x_min = self.data.root;
                self.data.root = self.data.x_max;
                self.data.x_max = self.data.x_min;

                // Adjust f(x)'s
                self.data.y_min = froot;
                froot = self.data.y_max;
                self.data.y_max = self.data.y_min;
            }

            x_acc1 = 2.0 * f64::EPSILON * self.data.root.abs() + 0.5 * self.data.accuracy;
            x_mid = 0.5 * (self.data.x_max - self.data.root);

            // if x_mid.abs() <= x_acc1 || close(froot, 0.0) {
            if x_mid.abs() <= x_acc1 || RootfinderData::close(froot, 0.0) {
                return self.data.root;
            }

            if e.abs() >= x_acc1 && self.data.y_min > froot.abs() {
                s = froot / self.data.y_min;

                if RootfinderData::close(self.data.x_min, self.data.x_max) {
                    p = 2.0 * x_mid * s;
                    q = 1.0 - s;
                } else {
                    q = self.data.y_min / self.data.y_max;
                    r = froot / self.data.y_max;
                    p = s
                        * (2.0 * x_mid * q * (q - r)
                            - (self.data.root - self.data.x_min) * (r - 1.0));
                    q = (q - 1.0) * (r - 1.0) * (s - 1.0);
                }

                if p > 0.0 {
                    q = -q;
                }

                p = p.abs();

                min1 = 3.0 * x_mid * q - (x_acc1 * q).abs();
                min2 = (e * q).abs();

                let ternary = if min1 < min2 { min1 } else { min2 };

                if 2.0 * p < ternary {
                    e = d;
                    d = p / q;
                } else {
                    d = x_mid;
                    e = d;
                }
            } else {
                d = x_mid;
                e = d;
            }

            self.data.x_min = self.data.root;
            self.data.y_min = froot;

            if d.abs() > x_acc1 {
                self.data.root += d;
            } else {
                self.data.root += RootfinderData::nrsign(x_acc1, x_mid); // x_acc1.abs() * x_mid.signum();
            }
            froot = self.value(self.data.root);
            self.data.increment_evaluation_count();
        }

        0.0
    }

    fn solve(&mut self) -> f64 {
        assert!(self.data.accuracy > 0., "accuracy must be positive");

        self.data.accuracy = f64::max(self.data.accuracy, f64::EPSILON);

        let growth_factor = 1.6;
        let mut flipflop = -1;

        self.data.root = self.guess;
        self.data.y_max = self.value(self.data.root);

        if RootfinderData::close(self.data.y_max, 0.0) {
            return self.data.root;
        } else if self.data.y_max > 0.0 {
            self.data.x_min = self
                .data
                .enforce_bounds(self.data.root - self.data.stepsize);
            self.data.y_min = self.value(self.data.x_min);
            self.data.x_max = self.data.root;
        } else {
            self.data.x_min = self.data.root;
            self.data.y_min = self.data.y_max;
            self.data.x_max = self
                .data
                .enforce_bounds(self.data.root + self.data.stepsize);
            self.data.y_max = self.value(self.data.x_max);
        }

        self.data.iteration_count = 2;

        while self.data.iteration_count <= Self::MAX_ITERATIONS {
            // Check if we can solve.
            if self.data.y_min * self.data.y_max <= 0.0 {
                if RootfinderData::close(self.data.y_min, 0.0) {
                    return self.data.x_min;
                }
                if RootfinderData::close(self.data.y_max, 0.0) {
                    return self.data.x_max;
                }
                self.data.root = 0.5 * (self.data.x_max + self.data.x_min);

                return self.solve_impl();
            }

            // If we can't solve, adjust.
            if self.data.y_min.abs() < self.data.y_max.abs() {
                self.data.x_min = self.data.enforce_bounds(
                    self.data.x_min + growth_factor * (self.data.x_min - self.data.x_max),
                );
                self.data.y_min = self.value(self.data.x_min);
            } else if self.data.y_min.abs() > self.data.y_max.abs() {
                self.data.x_max = self.data.enforce_bounds(
                    self.data.x_max + growth_factor * (self.data.x_max - self.data.x_min),
                );
                self.data.y_max = self.value(self.data.x_max);
            } else if flipflop == -1 {
                self.data.x_min = self.data.enforce_bounds(
                    self.data.x_min + growth_factor * (self.data.x_min - self.data.x_max),
                );
                self.data.y_min = self.value(self.data.x_min);
                self.data.increment_evaluation_count();
                flipflop = 1;
            } else if flipflop == 1 {
                self.data.x_max = self.data.enforce_bounds(
                    self.data.x_max + growth_factor * (self.data.x_max - self.data.x_min),
                );
                self.data.y_max = self.value(self.data.x_max);
                flipflop = -1;
            }

            self.data.increment_evaluation_count();
        }

        0.0
    }
}

#[cfg(test)]
mod TESTS_brent_solver {

    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_brent_solver() {
        // Define the objective function:
        // f(x) = x^2 - 2
        let f = |x: f64| x.powi(2) - 2.0;

        // Create a new Brent solver with:
        //      - Objective function: f(x) = x^2 - 2
        //      - Initial guess: 1.0
        //      - Root-finder data:
        //          - Accuracy: 1e-15
        //          - Step size: 1e-5
        //          - Lower bound: 0.0
        //          - Upper bound: 2.0
        //          - Interval enforced: true
        let data = RootfinderData::new(1e-15, 1e-5, 0.0, 2.0, true);
        let mut solver = Brent::new(f, 1.0, data);
        let root = solver.solve();
        assert!((root - SQRT_2) < 1e-15);

        // let n = 1_000_000;
        // let start = std::time::Instant::now();
        // for _ in 0..n {
        //     solver.solve();
        // }
        // let duration = start.elapsed();
        // // Takes about 1.235926167s on MacBook Air M2
        // println!("Brent: {} solutions took: {:?}", n, duration);
        // println!("Solution: {}", solver.data.root);
        // println!("Expected: {}", SQRT_2);
    }

    #[test]
    fn test_implied_volatility() {
        use errorfunctions::RealErrorFunctions;

        let phi = |x: f64| 0.5 * (1.0 + (x / SQRT_2).erf());

        let d1 = |s: f64, k: f64, v: f64, r: f64, q: f64, t: f64| {
            ((s / k).ln() + (r - q + (v * v) / 2.0) * t) / (v * t.sqrt())
        };

        let d2 =
            |s: f64, k: f64, v: f64, r: f64, q: f64, t: f64| d1(s, k, v, r, q, t) - v * t.sqrt();

        let black_scholes_call = |s: f64, k: f64, v: f64, r: f64, q: f64, t: f64| {
            let d1 = d1(s, k, v, r, q, t);
            let d2 = d2(s, k, v, r, q, t);

            s * (-q * t).exp() * phi(d1) - k * (-r * t).exp() * phi(d2)
        };

        println!(
            "Black-Scholes Call: {}",
            black_scholes_call(100.0, 100.0, 0.2, 0.05, 0.0, 1.0)
        );

        let price = 10.4505835721856;
        let expected_vol = 0.2;

        // f(x) = Black-Scholes-Call(vol) - price
        let f = |v: f64| black_scholes_call(100.0, 100.0, v, 0.05, 0.0, 1.0) - price;

        let mut solver = Brent::new(f, 0.5, RootfinderData::default());
        let root = solver.solve();
        assert!((root - expected_vol).abs() < 1e-10, "Impl. Vol.: {}", root);

        // println!("Implied Volatility: {}", root);
        // println!("Expected: {}", expected_vol);
        // assert!(false)
    }
}
