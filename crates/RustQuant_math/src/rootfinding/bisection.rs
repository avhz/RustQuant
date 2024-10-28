// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::rootfinder::{Rootfinder, RootfinderData};

/// Bisection root-finding algorithm.
pub struct Bisection<F>
where
    F: Fn(f64) -> f64,
{
    function: F,
    guess: f64,
    data: RootfinderData,
}

impl<F> Bisection<F>
where
    F: Fn(f64) -> f64,
{
    /// Create a new Bisection solver.
    pub fn new(function: F, guess: f64, data: RootfinderData) -> Self {
        Self {
            function,
            guess,
            data,
        }
    }
}

impl<F> Rootfinder<F> for Bisection<F>
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
        let mut dx: f64;
        let mut x_mid: f64;
        let mut f_mid: f64;

        // Orient the search so that f>0 lies at self.data.root+dx
        if self.data.y_min < 0.0 {
            dx = self.data.x_max - self.data.x_min;
            self.data.root = self.data.x_min;
        } else {
            dx = self.data.x_min - self.data.x_max;
            self.data.root = self.data.x_max;
        }

        while self.data.iteration_count <= Self::MAX_ITERATIONS {
            dx /= 2.0;
            x_mid = self.data.root + dx;
            f_mid = self.value(x_mid);
            self.data.increment_evaluation_count();

            if f_mid <= 0.0 {
                self.data.root = x_mid;
            }
            if dx.abs() < self.data.accuracy || RootfinderData::close(f_mid, 0.0) {
                return self.data.root;
            }
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
mod TESTS_bisection_solver {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_bisection_solver() {
        // Define the objective function:
        // f(x) = x^2 - 2
        let f = |x: f64| x.powi(2) - 2.0;

        // Create a new Bisection solver with:
        //      - Objective function: f(x) = x^2 - 2
        //      - Initial guess: 1.0
        //      - Root-finder data:
        //          - Accuracy: 1e-15
        //          - Step size: 1e-5
        //          - Lower bound: 0.0
        //          - Upper bound: 2.0
        //          - Interval enforced: true
        let data = RootfinderData::new(1e-15, 1e-5, 0.0, 2.0, true);
        let mut solver = Bisection::new(f, 1.0, data);
        let root = solver.solve();
        assert!((root - SQRT_2) < 1e-15);

        // let n = 1_000_000;
        // let start = std::time::Instant::now();
        // for _ in 0..n {
        //     solver.solve();
        // }
        // let duration = start.elapsed();

        // // Takes about 1.235926167s on MacBook Air M2
        // println!("Bisection: {} solutions took: {:?}", n, duration);
        // println!("Solution: {}", solver.data.root);
        // println!("Expected: {}", SQRT_2);
    }
}
