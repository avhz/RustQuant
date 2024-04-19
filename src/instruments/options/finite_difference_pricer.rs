// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::f64::consts::E;
use time::Date;
use nalgebra::{DMatrix, DVector};
use crate::instruments::options::option::{TypeFlag, ExerciseFlag};
use crate::time::{today, DayCountConvention};

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

impl FiniteDifferencePricer {

        /// Constructor for FiniteDifferencePricer
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
            exercise_flag: ExerciseFlag
        ) -> Self {
    
            assert!(initial_price > 0.0, "initial_price must be greater than 0!");
            assert!(strike_price > 0.0, "strike_price must be greater than 0!");
            assert!(risk_free_rate > 0.0, "risk_free_rate must be greater than 0!");
            assert!(volatility > 0.0, "volatility must be greater than 0!");
            assert!(time_steps > 0, "time_steps must be greater than 0!");
            assert!(price_steps > 0, "price_steps must be greater than 0!");
    
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
                exercise_flag
            }
        }

    fn create_tridiagonal_matrix<A, B, C>(&self, sub_diagonal: A, diagonal: B, super_diagonal: C, price_steps: u32) -> DMatrix<f64> 
    where
        A: Fn(f64) -> f64,
        B: Fn(f64) -> f64,
        C: Fn(f64) -> f64
    {
        let tridiagonal_matrix = DMatrix::from_fn(
            (price_steps-1) as usize, (price_steps-1) as usize, 
            |i, j| {
                if i == j {
                    diagonal((i + 1) as f64)
                } else if i as isize == j as isize - 1 {
                    super_diagonal((i + 1) as f64)
                } else if i as isize == j as isize + 1 {
                    sub_diagonal((i + 1) as f64)
                } else {
                    0.0
                }
            }
        );
        tridiagonal_matrix  
    }

    fn sub_diagonal(&self, scaler: f64) -> Box<dyn Fn(f64) -> f64 + '_> {
        let function = move |m: f64| scaler * ((self.volatility.powi(2) * m.powi(2)) - (self.risk_free_rate * m));
        Box::new(function)
    }

    fn diagonal(&self, scaler: f64) -> Box<dyn Fn(f64) -> f64 + '_> {
        let function = move |m: f64| 1.0 + scaler * ((self.volatility.powi(2) * m.powi(2)) + self.risk_free_rate);
        Box::new(function)
    }

    fn super_diagonal(&self, scaler: f64) -> Box<dyn Fn(f64) -> f64 + '_> {
        let function = move |m: f64| scaler * ((self.volatility.powi(2)) * m.powi(2) + (self.risk_free_rate * m));
        Box::new(function)
    }

    fn year_fraction(&self) -> f64 {
        DayCountConvention::default().day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        )
    }

    fn payoff(&self, s: f64) -> f64 {
        match self.type_flag {
            TypeFlag::Call => (s - self.strike_price).max(0.0),
            TypeFlag::Put => (self.strike_price - s).max(0.0)
        }
    }

    fn american_time_stop_step(&self, u: nalgebra::Matrix<f64, nalgebra::Dyn, nalgebra::Const<1>, nalgebra::VecStorage<f64, nalgebra::Dyn, nalgebra::Const<1>>>, price_steps: u32) -> DVector<f64> {
        DVector::<f64>::from_fn((price_steps-1) as usize,|i, _| {
            u[i].max(self.payoff((i + 1) as f64 * (2.0 * self.initial_price) / (price_steps as f64)))
        })
    }

    fn boundary_condition_at_time_n(&self, price_steps: u32) -> DVector<f64> {
        DVector::<f64>::from_fn((price_steps-1) as usize,|i, _| {
            self.payoff(((i + 1) as f64) * (2.0 * self.initial_price / (price_steps as f64)))
        })
    }

    fn call_boundary(&self, t: u32, T: f64, delta_t: f64) -> f64 {
        2.0 * self.initial_price - self.strike_price * E.powf(-self.risk_free_rate * (((- self.risk_free_rate * T) / 365.0) - (t as f64 * delta_t)))
    }

    fn put_boundary(&self, t: u32, T: f64, delta_t: f64) -> f64 {
        self.strike_price * E.powf(-self.risk_free_rate * (-(self.risk_free_rate * T)) / 365.0 - (t as f64 * delta_t))
    }

    /// Explicit method
    pub fn explicit(&self) -> f64 {
        let price_steps: u32 = self.price_steps;
        let time_steps: u32 = self.time_steps;
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (time_steps as f64);

        let tridiagonal_matrix: DMatrix<f64> = self.create_tridiagonal_matrix(
            self.sub_diagonal(delta_t / 2.0), 
            self.diagonal(- delta_t), 
            self.super_diagonal(delta_t / 2.0), 
            price_steps
        );

        let mut u = self.boundary_condition_at_time_n(price_steps);

        for time_step in (1..(time_steps)).rev() {
            u = &tridiagonal_matrix * u;
            
            match self.type_flag {
                TypeFlag::Call => {
                    u[(price_steps-2) as usize] += self.super_diagonal(delta_t / 2.0)((price_steps-1) as f64) * self.call_boundary(time_step, T, delta_t);
                },
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 2.0)(1.0) * self.put_boundary(time_step, T, delta_t);
                }
            }

            match self.exercise_flag {
                ExerciseFlag::American => {u = self.american_time_stop_step(u, price_steps)}
                _ => {}
            }
        }
        (u[((price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)
    }

    /// Implicit method
    pub fn implicit(&self) -> f64 {
        let price_steps: u32 = self.price_steps;
        let time_steps: u32 = self.time_steps;
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (time_steps as f64);

        let inverse_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(- delta_t / 2.0), 
            self.diagonal(delta_t), 
            self.super_diagonal(- delta_t / 2.0), 
            price_steps
        )
        .try_inverse()
        .unwrap();

        let mut u: DVector<f64> = self.boundary_condition_at_time_n(price_steps);
        
        for time_step in (1..(time_steps)).rev() {
                match self.type_flag {
                    TypeFlag::Call => {
                        u[(price_steps-2) as usize] -= self.super_diagonal(- delta_t / 2.0)((price_steps - 1) as f64) * self.call_boundary(time_step, T, delta_t);
                    },
                    TypeFlag::Put => {
                        u[0] -= self.sub_diagonal(- delta_t / 2.0)(1.0) * self.put_boundary(time_step, T, delta_t);
                    }
                }
                u = &inverse_matrix * u;

                match self.exercise_flag {
                    ExerciseFlag::American => {u = self.american_time_stop_step(u, price_steps)}
                    _ => {}
                }
        }

        (u[((price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)

    }

    /// Crank-Nicolson method
    pub fn crank_nicolson(&self) -> f64 {
        let price_steps: u32 = self.price_steps;
        let time_steps: u32 = self.time_steps;
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (time_steps as f64);

        let inverse_past_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(- delta_t / 4.0), 
            self.diagonal(delta_t / 2.0), 
            self.super_diagonal(- delta_t / 4.0),
            price_steps
        )
        .try_inverse().
        unwrap();
        
        let tridiagonal_future_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(delta_t / 4.0), 
            self.diagonal(- delta_t / 2.0), 
            self.super_diagonal(delta_t / 4.0),
            price_steps);

        let mut u = self.boundary_condition_at_time_n(price_steps);

        for t in (1..(time_steps)).rev() {  
            u = &tridiagonal_future_matrix * u;

            match self.type_flag {
                TypeFlag::Call => {
                    u[(price_steps-2) as usize] += self.super_diagonal(delta_t / 4.0)((price_steps-1) as f64) * (self.call_boundary(t + 1, T, delta_t) - self.call_boundary(t, T, delta_t))
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 4.0)(1.0) * (self.put_boundary(t + 1, T, delta_t) - self.put_boundary(t, T, delta_t))
                }
            }
            u = &inverse_past_matrix * u;

            match self.exercise_flag {
                ExerciseFlag::American => {u = self.american_time_stop_step(u, price_steps)}
                _ => {}
            }
        }
        (u[((price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)
    }
}

#[cfg(test)]
mod tests_finite_difference_pricer {
    use super::*;
    use crate::assert_approx_equal;
    use crate::RUSTQUANT_EPSILON;

    #[test]
    fn european_call_option() {

        let finite_difference_obj = FiniteDifferencePricer::new(
            10.11,
            5.43,
            0.1,
            0.3,
            50,
            TypeFlag::Call,
            ExerciseFlag::European,
        );

        let answer = 4.75;
        assert_approx_equal!(finite_difference_obj.explicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.implicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.crank_nicolson(), answer, RUSTQUANT_EPSILON);
    }

    #[test]
    fn european_put_option() {

        let finite_difference_obj = FiniteDifferencePricer::new(
            101.22,
            137.89,
            0.12,
            0.25,
            14,
            TypeFlag::Put,
            ExerciseFlag::European,
        );

        let answer = 36.04;
        assert_approx_equal!(finite_difference_obj.explicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.implicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.crank_nicolson(), answer, RUSTQUANT_EPSILON);
    }

    #[test]
    fn american_call_option() {

        let finite_difference_obj = FiniteDifferencePricer::new(
            150.66,
            133.4, 
            0.01, 
            0.2, 
            365, 
            TypeFlag::Call,
            ExerciseFlag::American,
        );

        let answer = 22.9;
        assert_approx_equal!(finite_difference_obj.explicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.implicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.crank_nicolson(), answer, RUSTQUANT_EPSILON);
    }

    #[test]
    fn american_put_option() {

        let finite_difference_obj = FiniteDifferencePricer::new(
            3.22,
            12.87, 
            0.02, 
            0.2, 
            365, 
            TypeFlag::Put,
            ExerciseFlag::American,
        );

        let answer = 9.65;
        assert_approx_equal!(finite_difference_obj.explicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.implicit(), answer, RUSTQUANT_EPSILON);
        assert_approx_equal!(finite_difference_obj.crank_nicolson(), answer, RUSTQUANT_EPSILON);
    }
}
