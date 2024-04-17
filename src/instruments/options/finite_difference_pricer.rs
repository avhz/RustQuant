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
use crate::time::{today, DayCountConvention};
use crate::instruments::options::option::{ExerciseFlag, TypeFlag};

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

        let mut variables_with_error: Vec<&str> = vec![];

        if initial_price <= 0.0 {
            variables_with_error.push("initial_price")
        }
        
        if strike_price <= 0.0 {
            variables_with_error.push("strike_price")
        } 
        
        if risk_free_rate <= 0.0 {
            variables_with_error.push("risk_free_rate")
        } 

        if volatility <= 0.0 {
            variables_with_error.push("volatility")
        } 

        if variables_with_error.len() > 0 {
            if variables_with_error.len() == 1 {
                panic!("{} must be greater than 0!", variables_with_error[0])
            } else if variables_with_error.len() == 2 {
                panic!("{} and {} must both be greater than 0!", variables_with_error[0], variables_with_error[1])
            } else {
                let mut error_message: String = String::from("");
                for (i, var) in variables_with_error.iter().enumerate() {
                    if i == variables_with_error.len() - 1 {
                        error_message += &format!(" and {} must all be greater than 0!", var)
                    } else if i == variables_with_error.len() - 2 {
                        error_message += &format!("{}", var)
                    } else {
                        error_message += &format!("{}, ", var)
                    }
                }
                panic!("{}", error_message)
            }
        }

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

    fn matrix_multiply_vector(&self, A: &Vec<Vec<f64>>, v: Vec<f64>) -> Vec<f64> {

        let mut Av: Vec<f64> = Vec::new();
        
        match A[0].len() {
            n if n == v.len() => {
                let mut value: f64;
                for i in 0..A.len() {
                    value = 0.0;
                    for j in 0..v.len() {
                        value += A[i][j] * v[j]
                    }
                    Av.push(value)
                }
            },
            _ => {
                for i in 0..A.len() {
                    match i {
                        0 => {
                            Av.push(A[0][0] * v[0] + A[0][1] * v[1])
                        },
                        n if n == A.len() - 1 => {
                            Av.push(A[n][0] * v[n - 1] + A[n][1] * v[n])
                        }
                        _ => {
                            Av.push(A[i][0] * v[i - 1] + A[i][1] * v[i] + A[i][2] * v[i + 1])
                        }
                    }
                }
            }
        }
        Av
    }
    
    fn create_tridiagonal_matrix<A, B, C>(&self, sub_diagonal: A, diagonal: B, super_diagonal: C, price_steps: u32) -> Vec<Vec<f64>> 
    where
        A: Fn(f64) -> f64,
        B: Fn(f64) -> f64,
        C: Fn(f64) -> f64
    {
        let mut matrix_row: Vec<f64> = Vec::new();
        let mut tridiagonal_matrix: Vec<Vec<f64>> = Vec::new();

        for i in 1..(price_steps) {
            
            if i != 1 {
                matrix_row.push(sub_diagonal(i as f64));
            } 

            matrix_row.push(diagonal(i as f64));
            
            if i != price_steps - 1 {
                matrix_row.push(super_diagonal(i as f64));
            }

            tridiagonal_matrix.push(matrix_row.clone());
            matrix_row.clear()
        }

        tridiagonal_matrix
    }

    fn invert_tridiagonal_matrix(&self, tridiagonal_matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        
        let last = tridiagonal_matrix.len() - 1;
        let mut theta: Vec<f64> = Vec::new();
        theta.push(1.0);
        theta.push(tridiagonal_matrix[0][0]);
        theta.push(
            tridiagonal_matrix[1][1] * theta[1] 
            - tridiagonal_matrix[0][1] * tridiagonal_matrix[1][0] * theta[0]
        );
        
        for i in 2..(tridiagonal_matrix.len()) {
            theta.push(
                tridiagonal_matrix[i][1] * theta[i] 
                - tridiagonal_matrix[i - 1][2] * tridiagonal_matrix[i][0] * theta[i - 1]
            )
        }

        let mut phi: Vec<f64> = Vec::new();
        phi.push(1.0);
        phi.push(tridiagonal_matrix[last][1]);

        for i in 1..(tridiagonal_matrix.len() - 1) {
            phi.push(
                tridiagonal_matrix[last - i][1]
                * phi[i] 
                - tridiagonal_matrix[last - i][2] 
                * tridiagonal_matrix[last + 1 - i][0] 
                * phi[i-1]
            )
        }

        phi.push(
            tridiagonal_matrix[0][0]
            * phi[last]
            - tridiagonal_matrix[0][1] 
            * tridiagonal_matrix[1][0] 
            * phi[last-1]
        );

        let theta_n = theta.pop().unwrap();
        phi.pop();
        phi.reverse();

        let mut value: f64;
        let mut inverse_matrix: Vec<Vec<f64>> = Vec::new();
        let mut matrix_row: Vec<f64> = Vec::new();

        for i in 0..tridiagonal_matrix.len() {
            for j in 0..tridiagonal_matrix.len() {
                value = (-1.0_f64).powi((i+j) as i32);
                
                if i < j {
                    for k in i..j {
                        value *= tridiagonal_matrix[k].last().unwrap();
                    }
                    value *= theta[i] * phi[j] / theta_n;

                } else if i == j {
                    value *= theta[i] * phi[i] / theta_n
                } else {
                    for k in j..i {
                        value *= tridiagonal_matrix[k+1].first().unwrap();
                    }
                    value *= theta[j] * phi[i] / theta_n
                }
                matrix_row.push(value);
            }
            
            inverse_matrix.push(matrix_row.clone());
            matrix_row.clear()
        }

        inverse_matrix
        
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

    fn american_time_stop_step(&self, u: Vec<f64>, price_steps: u32) -> Vec<f64> {
        (0..(price_steps - 1)).map(|i: u32| u[i as usize].max(self.payoff((i+1) as f64 * (2.0 * self.initial_price) / (price_steps as f64)))).collect()
    }

    fn boundary_condition_at_time_n(&self, price_steps: u32) -> Vec<f64> {
        (1..(price_steps)).map(|i| self.payoff(((i) as f64) * (2.0 * self.initial_price / (price_steps as f64)))).collect()
    }
    
    fn call_boundary(&self, t: u32, T: f64, delta_t: f64) -> f64 {
        2.0 * self.initial_price - self.strike_price * E.powf(-(self.risk_free_rate * T) - (t as f64 * delta_t))
    }

    fn put_boundary(&self, t: u32, T: f64, delta_t: f64) -> f64 {
        self.strike_price * E.powf(-(self.risk_free_rate * T) - (t as f64 * delta_t))
    }

    /// Explicit method
    pub fn explicit(&self) -> f64 {
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (self.time_steps as f64);
    
        let tridiagonal_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(delta_t / 2.0), 
            self.diagonal(- delta_t), 
            self.super_diagonal(delta_t / 2.0), 
            self.price_steps
        );

        let mut u: Vec<f64> = self.boundary_condition_at_time_n(self.price_steps);

        for t in (1..self.time_steps).rev() {
            u = self.matrix_multiply_vector(&tridiagonal_matrix, u);

            match self.type_flag {
                TypeFlag::Call => {
                    u[(self.price_steps-2) as usize] += self.super_diagonal(delta_t / 2.0)((self.price_steps - 1) as f64) * self.call_boundary(t, T, delta_t);
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 2.0)(1.0) * self.put_boundary(t, T, delta_t);
                }
            }

            match self.exercise_flag {
                ExerciseFlag::American => {u = self.american_time_stop_step(u, self.price_steps)}
                _ => {}
            }
        }

        (u[((self.price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)
    }

    /// Implicit method
    pub fn implicit(&self) -> f64 {
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (self.time_steps as f64);
        
        let inverse_matrix = self.invert_tridiagonal_matrix(
                self.create_tridiagonal_matrix(
                    self.sub_diagonal(- delta_t / 2.0), 
                    self.diagonal(delta_t), 
                    self.super_diagonal(- delta_t / 2.0), 
                    self.price_steps
                )
            ); 

        let mut u: Vec<f64> = self.boundary_condition_at_time_n(self.price_steps);
        
        for t in (1..self.time_steps).rev() {
            
            match self.type_flag {
                TypeFlag::Call => {
                    u[(self.price_steps-2) as usize] -= self.super_diagonal(- delta_t / 2.0)((self.price_steps - 1) as f64) * self.call_boundary(t, T, delta_t);
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 2.0)(1.0) * self.put_boundary(t, T, delta_t);
                }
            }

            u = self.matrix_multiply_vector(&inverse_matrix, u);

            match self.exercise_flag {
                ExerciseFlag::American => {u = self.american_time_stop_step(u, self.price_steps)}
                _ => {}
            }
        }

        (u[((self.price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)
    }

    /// Crank-Nicolson method
    pub fn crank_nicolson(&self) -> f64 {
        let T: f64 = self.year_fraction();
        let delta_t: f64 = T / (self.time_steps as f64);

        let inverse_past_matrix = self.invert_tridiagonal_matrix(
            self.create_tridiagonal_matrix(
                self.sub_diagonal(- delta_t / 4.0), 
                self.diagonal(delta_t / 2.0), 
                self.super_diagonal(- delta_t / 4.0),
                self.price_steps
            )
        );
        
        let tridiagonal_future_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(delta_t / 4.0), 
            self.diagonal(- delta_t / 2.0), 
            self.super_diagonal(delta_t / 4.0),
            self.price_steps
        );

        let mut u: Vec<f64> = self.boundary_condition_at_time_n(self.price_steps);

        for t in (1..self.time_steps).rev() {
            u = self.matrix_multiply_vector(&tridiagonal_future_matrix, u);

            match self.type_flag {
                TypeFlag::Call => {
                    u[(self.price_steps-2) as usize] += self.super_diagonal(delta_t / 4.0)((self.price_steps - 1) as f64) * (self.call_boundary(t + 1, T, delta_t) - self.call_boundary(t, T, delta_t))
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 4.0)(1.0) * (self.put_boundary(t + 1, T, delta_t) - self.put_boundary(t, T, delta_t))
                }
            }

            u = self.matrix_multiply_vector(&inverse_past_matrix, u);

            match self.exercise_flag {
                ExerciseFlag::American => {u = self.american_time_stop_step(u, self.price_steps)}
                _ => {}
            }
        }

        (u[((self.price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)
    }
}

#[cfg(test)]
mod tests_finite_difference_pricer {
    use super::*;
    use crate::assert_approx_equal;
    use crate::RUSTQUANT_EPSILON;
    use time::Duration;

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
