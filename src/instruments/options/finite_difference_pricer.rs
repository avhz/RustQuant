// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::instruments::options::option::{ExerciseFlag, TypeFlag};
use std::f64::consts::E;

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
    /// Days until maturity
    pub time_to_maturity: u16, 
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
        time_to_maturity: u16, 
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

        if time_to_maturity <= 0 {
            variables_with_error.push("time_to_maturity")
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
            time_to_maturity, 
            type_flag,
            exercise_flag
        }
    }

    fn matrix_multiply_vector(&self, A: &Vec<Vec<f64>>, v: Vec<f64>) -> Vec<f64> {

        let mut Av: Vec<f64> = Vec::new();
        let mut value: f64;

        for i in 0..A.len(){
            value = 0.0;
            for j in 0..v.len() {
                value += A[i][j] * v[j]
            }
            Av.push(value)
        }
        Av
    }

    fn tridiagonal_matrix_multiply_vector(&self, A: &Vec<Vec<f64>>, v: Vec<f64>) -> Vec<f64> {
        
        let mut Av: Vec<f64> = Vec::new();
        let mut value: f64;
        let mut start: usize;
        let mut end: usize;

        for i in 0..A.len() {
            value = 0.0;
            
            match i {
                0 => {
                    start = 0;
                    end = 2;
                },
                n if n == A.len() - 1 => {
                    start = n - 1;
                    end = n + 1;
                },
                _ => {
                    start = i - 1;
                    end = i + 2;
                }
            }

            for j in start..end {
                value += A[i][j] * v[j]
            }
            Av.push(value)
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
            if i > 1 {
                for _j in 0..(i - 2) {
                    matrix_row.push(0.0)    
                }
            }
            
            if i != 1 {
                matrix_row.push(sub_diagonal(i as f64));
            } 

            matrix_row.push(diagonal(i as f64));
            
            if i != price_steps - 1 {
                matrix_row.push(super_diagonal(i as f64));
            }

            for _j in i..(price_steps-2) {
                matrix_row.push(0.0)    
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
        
        for i in 1..(tridiagonal_matrix.len()) {
            theta.push(tridiagonal_matrix[i][i] * theta[i] 
                - tridiagonal_matrix[i - 1][i] * tridiagonal_matrix[i][i - 1] * theta[i - 1]);
        }

        let mut phi: Vec<f64> = Vec::new();
        phi.push(1.0);
        phi.push(tridiagonal_matrix[last][last]);


        for i in 1..(tridiagonal_matrix.len()) {
            phi.push(
                tridiagonal_matrix[last - i][last - i] * phi[i] 
                - tridiagonal_matrix[last - i][last + 1 - i] * tridiagonal_matrix[last + 1 - i][last - i] * phi[i-1]
            )
        }

        let theta_n = theta.pop().unwrap();
        phi.pop();
        phi.reverse();

        let mut value: f64;
        let mut inverse_matrix: Vec<Vec<f64>> = Vec::new();
        let mut matrix_row: Vec<f64> = Vec::new();

        for i in 0..tridiagonal_matrix.len() {
            for j in 0..tridiagonal_matrix[0].len() {
                value = (-1.0_f64).powi((i+j) as i32);
                
                if i < j {
                    for k in i..j {
                        value *= tridiagonal_matrix[k][k+1];
                    }
                    value *= theta[i] * phi[j] / theta_n;

                } else if i == j {
                    value *= theta[i] * phi[i] / theta_n
                } else {
                    for k in j..i {
                        value *= tridiagonal_matrix[k+1][k]
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

    fn get_price_steps(&self) -> u32 {
        (self.initial_price / 100.00) as u32 * 100 + 100
    }

    fn get_delta_t(&self, time_steps: u32) -> f64 {
        self.time_to_maturity as f64 / ((365 * time_steps) as f64)
    }

    fn get_time_steps(&self) -> u32 {
        ((self.time_to_maturity / 365) + 1) as u32 * 1000
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
        let u = (1..(price_steps)).map(|i| self.payoff(((i) as f64) * (2.0 * self.initial_price / (price_steps as f64)))).collect();
        u
    }
    
    fn call_boundary(&self, t: u32, delta_t: f64) -> f64 {
        2.0 * self.initial_price - self.strike_price * E.powf(-self.risk_free_rate * (((self.time_to_maturity as f64) / 365.0) - (t as f64 * delta_t)))
    }

    fn put_boundary(&self, t: u32, delta_t: f64) -> f64 {
        self.strike_price * E.powf(-self.risk_free_rate * (self.time_to_maturity as f64) / 365.0 - (t as f64 * delta_t))
    }

    /// Explicit method
    pub fn explicit(&self) -> f64 {
        let price_steps: u32 = self.get_price_steps();
        let time_steps: u32 = self.get_time_steps();
        let delta_t: f64 = self.get_delta_t(time_steps);
    
        let tridiagonal_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(delta_t / 2.0), 
            self.diagonal(- delta_t), 
            self.super_diagonal(delta_t / 2.0), 
            price_steps
        );

        let mut u: Vec<f64> = self.boundary_condition_at_time_n(price_steps);

        for t in (1..time_steps).rev() {
            u = self.tridiagonal_matrix_multiply_vector(&tridiagonal_matrix, u);

            match self.type_flag {
                TypeFlag::Call => {
                    u[(price_steps-2) as usize] += self.super_diagonal(delta_t / 2.0)((price_steps-1) as f64) * self.call_boundary(t, delta_t);
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 2.0)(1.0) * self.put_boundary(t, delta_t);
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
        let price_steps: u32 = self.get_price_steps();
        let time_steps: u32 = self.get_time_steps();
        let delta_t: f64 = self.get_delta_t(time_steps);
        
        let inverse_matrix = self.invert_tridiagonal_matrix(
                self.create_tridiagonal_matrix(
                    self.sub_diagonal(- delta_t / 2.0), 
                    self.diagonal(delta_t), 
                    self.super_diagonal(- delta_t / 2.0), 
                    price_steps
                )
            );

        let mut u: Vec<f64> = self.boundary_condition_at_time_n(price_steps);
        
        for t in (1..time_steps).rev() {
            
            match self.type_flag {
                TypeFlag::Call => {
                    u[(price_steps-2) as usize] -= self.super_diagonal(- delta_t / 2.0)((price_steps - 1) as f64) * self.call_boundary(t, delta_t);
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 2.0)(1.0) * self.put_boundary(t, delta_t);
                }
            }

            u = self.matrix_multiply_vector(&inverse_matrix, u);

            match self.exercise_flag {
                ExerciseFlag::American => {u = self.american_time_stop_step(u, price_steps)}
                _ => {}
            }
        }

        (u[((price_steps-1) / 2) as usize] * 10.0_f64.powi(2)).round() / 10.0_f64.powi(2)
    }

    /// Crank-Nicolson method
    pub fn crank_nicolson(&self) -> f64 {
        let price_steps: u32 = self.get_price_steps();
        let time_steps: u32 = self.get_time_steps();
        let delta_t: f64 = self.get_delta_t(time_steps);

        let inverse_past_matrix = self.invert_tridiagonal_matrix(
            self.create_tridiagonal_matrix(
                self.sub_diagonal(- delta_t / 4.0), 
                self.diagonal(delta_t / 2.0), 
                self.super_diagonal(- delta_t / 4.0),
                price_steps
            )
        );
        
        let tridiagonal_future_matrix = self.create_tridiagonal_matrix(
            self.sub_diagonal(delta_t / 4.0), 
            self.diagonal(- delta_t / 2.0), 
            self.super_diagonal(delta_t / 4.0),
            price_steps
        );

        let mut u: Vec<f64> = self.boundary_condition_at_time_n(price_steps);

        for t in (1..time_steps).rev() {
            u = self.tridiagonal_matrix_multiply_vector(&tridiagonal_future_matrix, u);

            match self.type_flag {
                TypeFlag::Call => {
                    u[(price_steps-2) as usize] += self.super_diagonal(delta_t / 4.0)((price_steps-1) as f64) * (self.call_boundary(t + 1, delta_t) - self.call_boundary(t, delta_t))
                }
                TypeFlag::Put => {
                    u[0] += self.sub_diagonal(delta_t / 4.0)(1.0) * (self.put_boundary(t + 1, delta_t) - self.put_boundary(t, delta_t))
                }
            }

            u = self.matrix_multiply_vector(&inverse_past_matrix, u);

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
