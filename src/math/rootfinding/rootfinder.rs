// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Root-finding base trait.
pub trait Rootfinder<F>
where
    F: Fn(f64) -> f64,
{
    // const STEP_SIZE: f64 = 1e-6;
    // const ACCURACY: f64 = 0.000_000_014_901_161_193_847_656;

    /// Maximum number of iterations.
    const MAX_ITERATIONS: i32 = 1000;

    /// Function value at x.
    fn value(&self, x: f64) -> f64;

    /// Derivative value at x.
    fn derivative(&self, x: f64) -> f64;

    /// Solve the root-finding problem (back-end implementation).
    fn solve_impl(&mut self) -> f64;

    /// Solve the root-finding problem (front-end, performs mostly checks).
    fn solve(&mut self) -> f64;
}

/// Root-finder data.
#[derive(Debug, Clone, Copy)]
pub struct RootfinderData {
    /// Root value.
    pub root: f64,

    /// Step size.
    pub stepsize: f64,

    /// Accuracy.
    pub accuracy: f64,

    /// Interval to search.
    pub interval: (f64, f64),

    /// Whether the interval is strictly enforced.
    pub interval_enforced: bool,

    // Private fields
    pub(crate) x_min: f64,
    pub(crate) x_max: f64,
    pub(crate) y_min: f64,
    pub(crate) y_max: f64,
    pub(crate) iteration_count: i32,
}

impl Default for RootfinderData {
    fn default() -> Self {
        Self {
            interval: (f64::MIN, f64::MAX),
            interval_enforced: true,
            stepsize: 1e-6,
            accuracy: f64::EPSILON.sqrt(),
            root: 0.0,
            x_min: f64::MIN,
            x_max: f64::MAX,
            y_min: f64::MIN,
            y_max: f64::MAX,
            iteration_count: 0,
        }
    }
}

impl RootfinderData {
    /// Create a new root-finder data.
    pub fn new(
        accuracy: f64,
        stepsize: f64,
        lower_bound: f64,
        upper_bound: f64,
        interval_enforced: bool,
    ) -> Self {
        Self {
            interval: (lower_bound, upper_bound),
            interval_enforced,
            stepsize,
            accuracy,
            root: 0.0,
            x_min: f64::MIN,
            x_max: f64::MAX,
            y_min: f64::MIN,
            y_max: f64::MAX,
            iteration_count: 0,
        }
    }

    pub(crate) fn enforce_bounds(&self, x: f64) -> f64 {
        if self.interval_enforced && x < self.interval.0 {
            return self.interval.0;
        }
        if self.interval_enforced && x > self.interval.1 {
            return self.interval.1;
        }

        x
    }

    #[inline]
    pub(crate) fn increment_evaluation_count(&mut self) {
        self.iteration_count += 1;
    }

    #[inline]
    pub(crate) fn nrsign(a: f64, b: f64) -> f64 {
        let t1 = if a >= 0.0 { a } else { -a };
        let t2 = if a >= 0.0 { -a } else { a };

        if b >= 0.0 {
            t1
        } else {
            t2
        }
    }

    pub(crate) fn close(x: f64, y: f64) -> bool {
        if x == y {
            return true;
        }

        let n = 42;
        let diff = f64::abs(x - y);
        let tolerance = n as f64 * f64::EPSILON;
        // let tolerance = f64::sqrt(f64::EPSILON);

        if x * y == 0.0 {
            return diff < f64::powi(tolerance, 2);
        }

        diff <= tolerance * f64::abs(x) && diff <= tolerance * f64::abs(y)
    }

    pub(crate) fn close_enough(x: f64, y: f64) -> bool {
        if x == y {
            return true;
        }

        let n = 42;
        let diff = f64::abs(x - y);
        let tolerance = n as f64 * f64::EPSILON;

        if x * y == 0.0 {
            return diff < f64::powi(tolerance, 2);
        }

        diff <= tolerance * f64::abs(x) || diff <= tolerance * f64::abs(y)
    }
}
