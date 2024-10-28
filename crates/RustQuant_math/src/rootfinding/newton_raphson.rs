// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::rootfinder::{Rootfinder, RootfinderData};

/// Newton-Raphson root-finding algorithm.
pub struct NewtonRaphson<F, G>
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    function: F,
    derivative: G,
    guess: f64,
    data: RootfinderData,
}

impl<F, G> NewtonRaphson<F, G>
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    /// Create a new Newton-Raphson solver.
    pub fn new(function: F, derivative: G, guess: f64, data: RootfinderData) -> Self {
        Self {
            function,
            derivative,
            guess,
            data,
        }
    }
}

impl<F, G> Rootfinder<F> for NewtonRaphson<F, G>
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    fn value(&self, x: f64) -> f64 {
        (self.function)(x)
    }

    fn derivative(&self, x: f64) -> f64 {
        (self.derivative)(x)
    }

    fn solve_impl(&mut self) -> f64 {
        let mut froot: f64;
        let mut dfroot: f64;
        let mut dx: f64;
        let mut dxold: f64;
        let mut xh: f64;
        let mut xl: f64;

        // Orient the search so that f(xl) < 0
        if self.data.y_min < 0.0 {
            xl = self.data.x_min;
            xh = self.data.x_max;
        } else {
            xl = self.data.x_max;
            xh = self.data.x_min;
        }

        dxold = self.data.x_max - self.data.x_min;
        dx = dxold;

        froot = self.value(self.data.root);
        dfroot = self.derivative(self.data.root);

        self.data.increment_evaluation_count();

        while self.data.iteration_count <= Self::MAX_ITERATIONS {
            let check_1 = (self.data.root - xh) * dfroot - froot;
            let check_2 = (self.data.root - xl) * dfroot - froot;
            let check_3 = (2.0 * froot).abs() > (dxold * dfroot).abs();

            // Bisect if (out of range || not decreasing fast enough)
            if (check_1 * check_2 > 0.0) || check_3 {
                dxold = dx;
                dx = (xh - xl) / 2.0;
                self.data.root = xl + dx;
            } else {
                dxold = dx;
                dx = froot / dfroot;
                self.data.root -= dx;
            }

            if dx.abs() < self.data.accuracy {
                return self.data.root;
            }

            froot = self.value(self.data.root);
            dfroot = self.derivative(self.data.root);
            self.data.increment_evaluation_count();

            if froot < 0.0 {
                xl = self.data.root;
            } else {
                xh = self.data.root;
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
mod TESTS_newton_raphson_solver {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_newton_raphson_solver() {
        // f(x) = x^2 - 2
        let f = |x: f64| x.powi(2) - 2.0;
        // f'(x) = 2x
        let df = |y: f64| 2.0 * y;

        let data = RootfinderData::new(1e-15, 1e-5, 0.0, 2.0, true);
        let mut solver = NewtonRaphson::new(f, df, 1.0, data);
        let root = solver.solve();
        assert!((root - SQRT_2) < 1e-15);

        // // 1 million iterations
        // let n = 1_000_000;

        // let start = std::time::Instant::now();
        // for _ in 0..n {
        //     solver.solve();
        // }
        // let duration = start.elapsed();

        // // Takes about 1.235926167s on MacBook Air M2
        // println!("Newton-Raphson: {} solutions took: {:?}", n, duration);
        // println!("Solution: {}", solver.data.root);
        // println!("Expected: {}", SQRT_2);
    }
}
