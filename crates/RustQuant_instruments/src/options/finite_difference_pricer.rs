// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::option_flags::*;
use std::cmp::Ordering;
use time::Date;
use RustQuant_time::{today, DayCountConvention};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Finite difference object
pub struct FiniteDifferencePricer {
    /// Spot Price
    pub initial_price: f64,
    /// Strike price
    pub strike_price: f64,
    /// Risk free rate
    pub risk_free_rate: f64,
    /// Volatility
    pub volatility: f64,

    /// Evaluation date
    pub evaluation_date: Option<Date>,
    /// Maturity date
    pub expiration_date: Date,

    /// Time steps
    pub time_steps: u32,
    /// Price steps
    pub price_steps: u32,

    /// Option Type
    pub type_flag: TypeFlag,
    /// Option Style
    pub exercise_flag: ExerciseFlag,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl FiniteDifferencePricer {
    /// Constructor for FiniteDifferencePricer
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        evaluation_date: Option<Date>,
        expiration_date: Date,
        time_steps: u32,
        price_steps: u32,
        type_flag: TypeFlag,
        exercise_flag: ExerciseFlag,
    ) -> Self {
        assert!(initial_price > 0.0, "initial_price must be positive!");
        assert!(strike_price > 0.0, "strike_price must be positive!");
        assert!(risk_free_rate > 0.0, "risk_free_rate must be positive!");
        assert!(volatility > 0.0, "volatility must be positive!");
        assert!(time_steps > 0, "time_steps must be positive!");
        assert!(price_steps > 0, "price_steps must be positive!");

        Self {
            initial_price,
            strike_price,
            risk_free_rate,
            volatility,
            evaluation_date,
            expiration_date,
            time_steps,
            price_steps,
            type_flag,
            exercise_flag,
        }
    }

    fn tridiagonal_matrix_multiply_vector(
        &self,
        sub_diagonal: f64,
        diagonal: f64,
        super_diagonal: f64,
        v: Vec<f64>,
    ) -> Vec<f64> {
        let mut Av: Vec<f64> = Vec::new();

        Av.push(diagonal * v[0] + super_diagonal * v[1]);

        for i in 1..(v.len() - 1) {
            Av.push(sub_diagonal * v[i - 1] + diagonal * v[i] + super_diagonal * v[i + 1])
        }

        Av.push(sub_diagonal * v[v.len() - 2] + diagonal * v[v.len() - 1]);

        Av
    }

    fn general_matrix_multiply_vector(&self, A: &[Vec<f64>], v: Vec<f64>) -> Vec<f64> {
        let mut Av: Vec<f64> = Vec::new();
        let mut value: f64;

        for row in A {
            value = 0.0;
            for (a, b) in row.iter().zip(&v) {
                value += a * b;
            }
            Av.push(value);
        }

        Av
    }

    fn invert_tridiagonal_matrix(
        &self,
        sub_diagonal: f64,
        diagonal: f64,
        super_diagonal: f64,
    ) -> Vec<Vec<f64>> {
        let mut theta: Vec<f64> = Vec::new();
        let system_size: usize = (self.price_steps - 1) as usize;

        theta.push(1.0);
        theta.push(diagonal);
        theta.push(diagonal * diagonal - super_diagonal * sub_diagonal);

        for i in 2..system_size {
            theta.push(diagonal * theta[i] - super_diagonal * sub_diagonal * theta[i - 1])
        }

        let mut phi: Vec<f64> = Vec::new();
        phi.push(1.0);
        phi.push(diagonal);

        for i in 1..(system_size) {
            phi.push(diagonal * phi[i] - super_diagonal * sub_diagonal * phi[i - 1])
        }

        let theta_n = theta.pop().unwrap();
        phi.pop();
        phi.reverse();

        let mut value: f64;
        let mut inverse_matrix: Vec<Vec<f64>> = Vec::new();
        let mut matrix_row: Vec<f64> = Vec::new();

        for i in 0..system_size {
            for j in 0..system_size {
                value = (-1.0_f64).powi((i + j) as i32);

                match i.cmp(&j) {
                    Ordering::Less => {
                        for k in i..j {
                            value *= match k {
                                k if k == system_size - 1 => diagonal,
                                _ => super_diagonal,
                            }
                        }
                        value *= theta[i] * phi[j] / theta_n;
                    }
                    Ordering::Equal => value *= theta[i] * phi[i] / theta_n,
                    Ordering::Greater => {
                        for _k in (j + 1)..(i + 1) {
                            value *= sub_diagonal
                        }
                        value *= theta[j] * phi[i] / theta_n
                    }
                }
                matrix_row.push(value);
            }
            inverse_matrix.push(matrix_row.clone());
            matrix_row.clear()
        }

        inverse_matrix
    }

    fn payoff(&self, s: f64) -> f64 {
        match self.type_flag {
            TypeFlag::Call => (s - self.strike_price).max(0.0),
            TypeFlag::Put => (self.strike_price - s).max(0.0),
        }
    }

    fn american_time_stop_step(&self, v: Vec<f64>, tau: f64, x_min: f64, delta_x: f64) -> Vec<f64> {
        (1..self.price_steps)
            .map(|i: u32| {
                v[(i - 1) as usize].max(
                    f64::exp(self.risk_free_rate * tau)
                        * self.payoff(f64::exp(x_min + (i as f64) * delta_x)),
                )
            })
            .collect()
    }

    fn initial_condition(&self, x_min: f64, delta_x: f64) -> Vec<f64> {
        (1..self.price_steps)
            .map(|i: u32| self.payoff(f64::exp(x_min + (i as f64) * delta_x)))
            .collect()
    }

    fn call_boundary(&self, tau: f64, x_max: f64) -> f64 {
        f64::exp(x_max) - self.strike_price * f64::exp(-self.risk_free_rate * tau)
    }

    fn put_boundary(&self, tau: f64, x_min: f64) -> f64 {
        self.strike_price * f64::exp(-self.risk_free_rate * tau) - f64::exp(x_min)
    }

    fn year_fraction(&self) -> f64 {
        DayCountConvention::Actual_365_25.day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        )
    }

    fn return_price(&self, u: Vec<f64>) -> f64 {
        match self.price_steps % 2 {
            0 => u[((self.price_steps - 1) / 2) as usize],
            _ => {
                (u[((self.price_steps - 1) / 2) as usize]
                    + u[((self.price_steps - 3) / 2) as usize])
                    / 2.0
            }
        }
    }

    fn grid(&self) -> (f64, f64, f64, f64) {
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (self.time_steps as f64);
        let x_min: f64 = self.initial_price.ln() - 5.0 * self.volatility * T.sqrt();
        let delta_x: f64 = (self.initial_price.ln() + 5.0 * self.volatility * T.sqrt() - x_min)
            / self.price_steps as f64;

        (T, delta_t, delta_x, x_min)
    }

    fn coefficients(&self, delta_t: f64, delta_x: f64) -> (f64, f64) {
        (
            0.5 * delta_t * self.volatility.powi(2) / delta_x.powi(2),
            delta_t * (self.risk_free_rate - 0.5 * self.volatility.powi(2)) / (2.0 * delta_x),
        )
    }

    /// Explicit method
    pub fn explicit(&self) -> f64 {
        let (T, delta_t, delta_x, x_min) = self.grid();
        let (x, y) = self.coefficients(delta_t, delta_x);
        let sub_diagonal: f64 = x - y;
        let diagonal: f64 = 1.0 - 2.0 * x;
        let super_diagonal: f64 = x + y;

        let mut v: Vec<f64> = self.initial_condition(x_min, delta_x);

        for t in 1..(self.time_steps + 1) {
            v = self.tridiagonal_matrix_multiply_vector(sub_diagonal, diagonal, super_diagonal, v);

            match self.type_flag {
                TypeFlag::Call => {
                    v[(self.price_steps - 2) as usize] += super_diagonal
                        * self.call_boundary(
                            (t as f64) * delta_t,
                            self.initial_price.ln() + 5.0 * self.volatility * T.sqrt(),
                        );
                }
                TypeFlag::Put => {
                    v[0] += sub_diagonal * self.put_boundary((t as f64) * delta_t, x_min);
                }
            }

            if let ExerciseFlag::American {
                start: Date::MIN,
                end: Date::MAX,
            } = self.exercise_flag
            {
                v = self.american_time_stop_step(v, (t as f64) * delta_t, x_min, delta_x);
            }
        }

        f64::exp(-self.risk_free_rate * T) * self.return_price(v)
    }

    ///Implicit method
    pub fn implicit(&self) -> f64 {
        let (T, delta_t, delta_x, x_min) = self.grid();
        let (x, y) = self.coefficients(delta_t, delta_x);

        let inverse_matrix: Vec<Vec<f64>> =
            self.invert_tridiagonal_matrix(-x + y, 1.0 + 2.0 * x, -x - y);

        let mut v: Vec<f64> = self.initial_condition(x_min, delta_x);

        for t in 1..(self.time_steps + 1) {
            match self.type_flag {
                TypeFlag::Call => {
                    v[(self.price_steps - 2) as usize] -= (-x - y)
                        * self.call_boundary(
                            (t as f64) * delta_t,
                            self.initial_price.ln() + 5.0 * self.volatility * T.sqrt(),
                        );
                }
                TypeFlag::Put => {
                    v[0] -= (-x + y) * self.put_boundary((t as f64) * delta_t, x_min);
                }
            }

            v = self.general_matrix_multiply_vector(&inverse_matrix, v);

            if let ExerciseFlag::American {
                start: Date::MIN,
                end: Date::MAX,
            } = self.exercise_flag
            {
                v = self.american_time_stop_step(v, (t as f64) * delta_t, x_min, delta_x);
            }
        }

        f64::exp(-self.risk_free_rate * T) * self.return_price(v)
    }

    /// Crank-Nicolson method
    pub fn crank_nicolson(&self) -> f64 {
        let (T, delta_t, delta_x, x_min) = self.grid();
        let (x, y) = self.coefficients(delta_t, delta_x);
        let sub_diagonal: f64 = 0.5 * (x - y);
        let diagonal: f64 = 1.0 - x;
        let super_diagonal: f64 = 0.5 * (x + y);

        let inverse_future_matrix =
            self.invert_tridiagonal_matrix(-sub_diagonal, 1.0 + x, -super_diagonal);

        let mut v: Vec<f64> = self.initial_condition(x_min, delta_x);

        for t in 1..(self.time_steps + 1) {
            v = self.tridiagonal_matrix_multiply_vector(sub_diagonal, diagonal, super_diagonal, v);

            match self.type_flag {
                TypeFlag::Call => {
                    v[(self.price_steps - 2) as usize] += 2.0
                        * super_diagonal
                        * self.call_boundary(
                            (t as f64) * delta_t,
                            self.initial_price.ln() + 5.0 * self.volatility * T.sqrt(),
                        );
                }
                TypeFlag::Put => {
                    v[0] += 2.0 * sub_diagonal * self.put_boundary((t as f64) * delta_t, x_min);
                }
            }

            v = self.general_matrix_multiply_vector(&inverse_future_matrix, v);

            if let ExerciseFlag::American {
                start: Date::MIN,
                end: Date::MAX,
            } = self.exercise_flag
            {
                v = self.american_time_stop_step(v, (t as f64) * delta_t, x_min, delta_x);
            }
        }

        f64::exp(-self.risk_free_rate * T) * self.return_price(v)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS: AT THE MONEY
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_finite_difference_pricer_at_the_money {
    use super::*;
    use time::macros::date;
    use RustQuant_utils::assert_approx_equal;

    const EPS: f64 = 1e-4;
    const EUROPEAN_CALL: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 250,
        type_flag: TypeFlag::Call,
        exercise_flag: ExerciseFlag::European {
            expiry: date!(2025 - 01 - 01),
        },
    };

    const EUROPEAN_PUT: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 250,
        type_flag: TypeFlag::Put,
        exercise_flag: ExerciseFlag::European {
            expiry: date!(2025 - 01 - 01),
        },
    };

    const AMERICAN_CALL: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 250,
        type_flag: TypeFlag::Call,
        exercise_flag: ExerciseFlag::American {
            start: date!(2024 - 01 - 01),
            end: date!(2025 - 01 - 01),
        },
    };

    const AMERICAN_PUT: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 250,
        type_flag: TypeFlag::Put,
        exercise_flag: ExerciseFlag::American {
            start: date!(2024 - 01 - 01),
            end: date!(2025 - 01 - 01),
        },
    };

    const EXPECT_A_CALL: f64 = 0.680_478_009_892_241;
    const EXPECT_A_PUT: f64 = 0.243_630_311_556;
    const EXPECT_E_CALL: f64 = 0.680_495_770_882_215;
    const EXPECT_E_PUT: f64 = 0.192_790_015_889_355;

    #[test]
    fn american_call_explicit() {
        assert_approx_equal!(AMERICAN_CALL.explicit(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_call_implicit() {
        assert_approx_equal!(AMERICAN_CALL.implicit(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_call_crank_nicolson() {
        assert_approx_equal!(AMERICAN_CALL.crank_nicolson(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_put_explicit() {
        assert_approx_equal!(AMERICAN_PUT.explicit(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn american_put_implicit() {
        assert_approx_equal!(AMERICAN_PUT.implicit(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn american_put_crank_nicolson() {
        assert_approx_equal!(AMERICAN_PUT.crank_nicolson(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn european_call_explicit() {
        assert_approx_equal!(EUROPEAN_CALL.explicit(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_call_implicit() {
        assert_approx_equal!(EUROPEAN_CALL.implicit(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_call_crank_nicolson() {
        assert_approx_equal!(EUROPEAN_CALL.crank_nicolson(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_put_explicit() {
        assert_approx_equal!(EUROPEAN_PUT.explicit(), EXPECT_E_PUT, EPS);
    }

    #[test]
    fn european_put_implicit() {
        assert_approx_equal!(EUROPEAN_PUT.implicit(), EXPECT_E_PUT, EPS);
    }

    #[test]
    fn european_put_crank_nicolson() {
        assert_approx_equal!(EUROPEAN_PUT.crank_nicolson(), EXPECT_E_PUT, EPS);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS: IN THE MONEY
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_finite_difference_pricer_in_the_money {
    use super::*;
    use time::macros::date;
    use RustQuant_utils::assert_approx_equal;

    const EPS: f64 = 1e-5;
    const EUROPEAN_CALL: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 15.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Call,
        exercise_flag: ExerciseFlag::European {
            expiry: date!(2025 - 01 - 01),
        },
    };

    const EUROPEAN_PUT: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 15.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Put,
        exercise_flag: ExerciseFlag::European {
            expiry: date!(2025 - 01 - 01),
        },
    };

    const AMERICAN_CALL: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 15.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Call,
        exercise_flag: ExerciseFlag::American {
            start: date!(2024 - 01 - 01),
            end: date!(2025 - 01 - 01),
        },
    };

    const AMERICAN_PUT: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 15.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Put,
        exercise_flag: ExerciseFlag::American {
            start: date!(2024 - 01 - 01),
            end: date!(2025 - 01 - 01),
        },
    };

    const EXPECT_A_CALL: f64 = 5.487_706_388_002_172;
    const EXPECT_A_PUT: f64 = 4.999_999_999_999_999;
    const EXPECT_E_CALL: f64 = 5.487_706_388_002_172;
    const EXPECT_E_PUT: f64 = 4.268_497_436_100_947;

    #[test]
    fn american_call_explicit() {
        assert_approx_equal!(AMERICAN_CALL.explicit(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_call_implicit() {
        assert_approx_equal!(AMERICAN_CALL.implicit(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_call_crank_nicolson() {
        assert_approx_equal!(AMERICAN_CALL.crank_nicolson(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_put_explicit() {
        assert_approx_equal!(AMERICAN_PUT.explicit(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn american_put_implicit() {
        assert_approx_equal!(AMERICAN_PUT.implicit(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn american_put_crank_nicolson() {
        assert_approx_equal!(AMERICAN_PUT.crank_nicolson(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn european_call_explicit() {
        assert_approx_equal!(EUROPEAN_CALL.explicit(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_call_implicit() {
        assert_approx_equal!(EUROPEAN_CALL.implicit(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_call_crank_nicolson() {
        assert_approx_equal!(EUROPEAN_CALL.crank_nicolson(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_put_explicit() {
        assert_approx_equal!(EUROPEAN_PUT.explicit(), EXPECT_E_PUT, EPS);
    }

    #[test]
    fn european_put_implicit() {
        assert_approx_equal!(EUROPEAN_PUT.implicit(), EXPECT_E_PUT, EPS);
    }

    #[test]
    fn european_put_crank_nicolson() {
        assert_approx_equal!(EUROPEAN_PUT.crank_nicolson(), EXPECT_E_PUT, EPS);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS: OUT OF THE MONEY
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_finite_difference_pricer_out_of_the_money {
    use super::*;
    use time::macros::date;
    use RustQuant_utils::assert_approx_equal;

    const EPS: f64 = 1e-5;
    const EUROPEAN_CALL: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 15.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Call,
        exercise_flag: ExerciseFlag::European {
            expiry: date!(2025 - 01 - 01),
        },
    };

    const EUROPEAN_PUT: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 15.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Put,
        exercise_flag: ExerciseFlag::European {
            expiry: date!(2025 - 01 - 01),
        },
    };

    const AMERICAN_CALL: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 10.0,
        strike_price: 15.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Call,
        exercise_flag: ExerciseFlag::American {
            start: date!(2024 - 01 - 01),
            end: date!(2025 - 01 - 01),
        },
    };

    const AMERICAN_PUT: FiniteDifferencePricer = FiniteDifferencePricer {
        initial_price: 15.0,
        strike_price: 10.0,
        risk_free_rate: 0.05,
        volatility: 0.1,
        evaluation_date: Some(date!(2024 - 01 - 01)),
        expiration_date: date!(2025 - 01 - 01),
        time_steps: 10000,
        price_steps: 200,
        type_flag: TypeFlag::Put,
        exercise_flag: ExerciseFlag::American {
            start: date!(2024 - 01 - 01),
            end: date!(2025 - 01 - 01),
        },
    };

    const EXPECT_A_CALL: f64 = 0.000_059_393_327_777_911;
    const EXPECT_A_PUT: f64 = 0.000_000_693_279_415_654_018;
    const EXPECT_E_CALL: f64 = 0.000_056_068_590_237_768;
    const EXPECT_E_PUT: f64 = 0.000_000_633_009_309_923_640;

    #[test]
    fn american_call_explicit() {
        assert_approx_equal!(AMERICAN_CALL.explicit(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_call_implicit() {
        assert_approx_equal!(AMERICAN_CALL.implicit(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_call_crank_nicolson() {
        assert_approx_equal!(AMERICAN_CALL.crank_nicolson(), EXPECT_A_CALL, EPS);
    }

    #[test]
    fn american_put_explicit() {
        assert_approx_equal!(AMERICAN_PUT.explicit(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn american_put_implicit() {
        assert_approx_equal!(AMERICAN_PUT.implicit(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn american_put_crank_nicolson() {
        assert_approx_equal!(AMERICAN_PUT.crank_nicolson(), EXPECT_A_PUT, EPS);
    }

    #[test]
    fn european_call_explicit() {
        assert_approx_equal!(EUROPEAN_CALL.explicit(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_call_implicit() {
        assert_approx_equal!(EUROPEAN_CALL.implicit(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_call_crank_nicolson() {
        assert_approx_equal!(EUROPEAN_CALL.crank_nicolson(), EXPECT_E_CALL, EPS);
    }

    #[test]
    fn european_put_explicit() {
        assert_approx_equal!(EUROPEAN_PUT.explicit(), EXPECT_E_PUT, EPS);
    }

    #[test]
    fn european_put_implicit() {
        assert_approx_equal!(EUROPEAN_PUT.implicit(), EXPECT_E_PUT, EPS);
    }

    #[test]
    fn european_put_crank_nicolson() {
        assert_approx_equal!(EUROPEAN_PUT.crank_nicolson(), EXPECT_E_PUT, EPS);
    }
}
