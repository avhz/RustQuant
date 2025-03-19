use time::Date;
use nalgebra::{DMatrix, DVector};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Normal, Distribution};
use RustQuant_time::{today, DayCountConvention};
use crate::option_flags::TypeFlag;

/// Longstaff-Schwartz Option pricing model.
pub struct LongstaffScwhartzPricer {
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
        /// Option Type
        pub type_flag: TypeFlag,
        /// Number of simulations
        pub num_simulations: u64,
        /// Seed
        pub seed: Option<u64>
}

impl LongstaffScwhartzPricer {

    /// Constructor for LongstaffScwhartz.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        evaluation_date: Option<Date>,
        expiration_date: Date,
        time_steps: u32,
        type_flag: TypeFlag,
        num_simulations: u64,
        seed: Option<u64>
    ) -> Self {
        assert!(evaluation_date.unwrap_or(today()) < expiration_date, "expiration_date must be after evaluation_date!");
        assert!(initial_price > 0.0, "initial_price must be positive!");
        assert!(strike_price > 0.0, "strike_price must be positive!");
        assert!(risk_free_rate > 0.0, "risk_free_rate must be positive!");
        assert!(volatility > 0.0, "volatility must be positive!");
        assert!(time_steps > 0, "time_steps must be positive!");
        assert!(num_simulations > 0, "num_simulations must be positive!");

        Self {
            initial_price,
            strike_price,
            risk_free_rate,
            volatility,
            evaluation_date,
            expiration_date,
            time_steps,
            type_flag,
            num_simulations,
            seed
        }
    }

    /// Run Longstaff-Schwartz pricing method for American options.
    pub fn generate_price(&self) -> f64 {
        let end_time: f64 = self.year_fraction();
        let delta_t: f64 = end_time / self.time_steps as f64;
        let mut markov_chain: Vec<f64> = self.generate_end_points(end_time);
        let mut asset_prices: Vec<f64> = self.calculate_asset_prices(&markov_chain);
        let mut payoffs: Vec<f64> = asset_prices.iter().map(|asset_price| self.calculate_payoff(asset_price)).collect();
        let mut regression_index: i32;

        for time_step in (1..self.time_steps).rev() {

            markov_chain = self.backwards_time_induction(
                markov_chain, delta_t, time_step
            );
            asset_prices = self.calculate_asset_prices(&markov_chain);
            let (in_the_money_indices, in_the_money_assets) = self.in_the_money_assets(&asset_prices);
            
            let filter_in_the_money_payoffs = payoffs.iter().enumerate().filter_map(|(i, payoff)| {
                if in_the_money_indices.contains(&i) {
                    Some(payoff.clone())
                } else {
                    None
                }
            }).collect();

            let laguerre_matrix = self.create_laguerre_matrix(&in_the_money_assets);
            let in_the_money_payoffs = self.discount(delta_t) * DVector::from_vec(filter_in_the_money_payoffs);
            let laguerre_matrix_transpose = laguerre_matrix.transpose();

            let least_squares_calculation = (&laguerre_matrix_transpose * &laguerre_matrix)
                .qr()
                .solve(&(&laguerre_matrix_transpose * in_the_money_payoffs));
                
            match least_squares_calculation {
                Some(regression_coefficients) => {
                    let continuation_value = &laguerre_matrix * regression_coefficients;
                    regression_index = -1;
                    for i in 0..self.num_simulations {
                        if in_the_money_indices.contains(&(i as usize)) {
                            regression_index += 1;
                            let payoff_at_current_time: f64 = self.calculate_payoff(&asset_prices[i as usize]);
                            payoffs[i as usize] = if payoff_at_current_time > continuation_value[regression_index as usize] {
                                payoff_at_current_time
                            } else {
                                self.discount(delta_t) * payoffs[i as usize]
                            }
                        } else {
                            payoffs[i as usize] = self.discount(delta_t) * payoffs[i as usize];
                        }
                    }
                },
                None => {
                    for i in 0..self.num_simulations {
                        payoffs[i as usize] = self.discount(delta_t) * payoffs[i as usize];
                    }
                }
            }
        }
        payoffs.iter().sum::<f64>() / self.num_simulations as f64
    }

    fn create_laguerre_matrix(&self, in_the_money_assets: &[f64]) -> DMatrix<f64> {
        let mut laguerre_matrix = DMatrix::zeros(
            in_the_money_assets.len(), 5 as usize
        );
        for i in 0..in_the_money_assets.len() {
            for j in 0..5 {
                laguerre_matrix[(i as usize, j as usize)] = match j {
                    0 => 1.0,
                    1 => 1.0 - in_the_money_assets[i as usize],
                    _ => (((2 * (j - 1)) as f64 
                        + 1.0 - in_the_money_assets[i as usize]) 
                        * laguerre_matrix[(i as usize, (j - 1) as usize)] 
                        - ((j - 1) as f64) 
                        * laguerre_matrix[(i as usize, (j - 2) as usize)]) 
                        / (j as f64),
                };
            }
        }
        laguerre_matrix
    }

    fn discount(&self, delta_t: f64) -> f64 {
        f64::exp(- self.risk_free_rate * delta_t)
    }

    fn generate_end_points(&self, end_time: f64) -> Vec<f64> {
        let mut rng = StdRng::seed_from_u64(self.seed.unwrap_or_else(rand::random));
        let normal_distribution: Normal<f64> = Normal::new(0.0, 1.0).unwrap();
        let mut markov_chain: Vec<f64> = vec![];
        for _ in 0..self.num_simulations {
            markov_chain.push(
                ((self.risk_free_rate - 0.5 * self.volatility * self.volatility) * end_time)
                    + self.volatility * end_time.sqrt() * normal_distribution.sample(&mut rng))
        }
        markov_chain
    }

    fn calculate_asset_prices(&self, markov_chain: &[f64]) -> Vec<f64> {
        let mut asset_prices = vec![]; 
        for i in 0..self.num_simulations {
            asset_prices.push(self.initial_price * f64::exp(markov_chain[i as usize]))
        }
        asset_prices
    }

    fn calculate_payoff(&self, asset_price: &f64) -> f64 {

        match self.type_flag {
            TypeFlag::Call => {
                (asset_price - self.strike_price).max(0.0)
            },
            TypeFlag::Put => {
                (self.strike_price - asset_price).max(0.0)
            }
        }
    }

    fn backwards_time_induction(&self, mut markov_chain: Vec<f64>, delta_t: f64, time_step: u32) -> Vec<f64> {

        let mut rng = match self.seed {
            Some(seed) => StdRng::seed_from_u64(seed.wrapping_add(time_step as u64)),
            None => StdRng::seed_from_u64(rand::random())
        };
        let normal_distribution: Normal<f64> = Normal::new(0.0, 1.0).unwrap();

        let current_time: f64 = (time_step as f64) * delta_t;
        for i in 0..self.num_simulations {
            markov_chain[i as usize] = (markov_chain[i as usize] * current_time / (current_time + delta_t))
                + self.volatility * (current_time * delta_t / (current_time + delta_t)).sqrt() * normal_distribution.sample(&mut rng)
        }
        markov_chain
    }

    fn year_fraction(&self) -> f64 {
        
        DayCountConvention::default().day_count_factor(
            self.evaluation_date.unwrap_or(today()),
            self.expiration_date,
        )
    }

    fn in_the_money_assets(&self, asset_prices: &[f64]) -> (Vec<usize>, Vec<f64>) {
        let mut in_the_money_indices: Vec<usize> = vec![];

        let in_the_money_assets = asset_prices.iter().enumerate().filter_map(
            |(i, asset_price)| { 
            let payoff = self.calculate_payoff(asset_price);
            if payoff > 0.0 {
                in_the_money_indices.push(i);
                Some(payoff)
            } else { None } }
        ).collect();
        (in_the_money_indices, in_the_money_assets)
    }
}

#[cfg(test)]
mod tests_longstaff_schwartz_pricer_at_the_money {
    use super::*;
    use time::macros::date;

    const TOLERANCE: f64 = 0.25;
    const ATM_CALL_EXPECTED_PRICE: f64 = 0.680;
    const ATM_PUT_EXPECTED_PRICE: f64 = 0.243;

    #[test]
    fn test_longstaff_schwartz_call_at_the_money() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            10.0, 
            10.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Call, 
            500,
            None
        );

        assert!(
            (longstaff_schwartz_pricer.generate_price() - ATM_CALL_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }

    #[test]
    fn test_longstaff_schwartz_put_at_the_money() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            10.0, 
            10.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Put, 
            500,
            None
        );

        assert!(
            (longstaff_schwartz_pricer.generate_price() - ATM_PUT_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS: IN THE MONEY
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_longstaff_schwartz_pricer_in_the_money {
    use super::*;
    use time::macros::date;

    const TOLERANCE: f64 = 0.25;
    const ITM_CALL_EXPECTED_PRICE: f64 = 5.4889;
    const ITM_PUT_EXPECTED_PRICE: f64 = 5.0000;

    #[test]
    fn test_longstaff_schwartz_call_in_the_money() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            15.0, 
            10.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Call, 
            500,
            None
        );
        
        assert!(
            (longstaff_schwartz_pricer.generate_price() - ITM_CALL_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }

    #[test]
    fn test_longstaff_schwartz_put_in_the_money() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            10.0, 
            15.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Put, 
            500,
            None
        );

        assert!(
            (longstaff_schwartz_pricer.generate_price() - ITM_PUT_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS: OUT OF THE MONEY
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_longstaff_schwartz_pricer_out_the_money {
    use super::*;
    use time::macros::date;

    const TOLERANCE: f64 = 0.25;
    const OTM_CALL_EXPECTED_PRICE: f64 = 0.0000;
    const OTM_PUT_EXPECTED_PRICE: f64 = 0.0000;

    #[test]
    fn test_longstaff_schwartz_call_out_the_money() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            10.0, 
            15.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Call, 
            500,
            None
        );

        assert!(
            (longstaff_schwartz_pricer.generate_price() - OTM_CALL_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }

    #[test]
    fn test_longstaff_schwartz_put_out_the_money() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            15.0, 
            10.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Put, 
            500,
            None
        );

        assert!(
            (longstaff_schwartz_pricer.generate_price() - OTM_PUT_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }
}

#[cfg(test)]
mod tests_longstaff_schwartz_pricer_seeded {
    use super::*;
    use time::macros::date;

    const TOLERANCE: f64 = 0.25;
    const CALL_SEEDED_EXPECTED_PRICE: f64 = 5.4889;
    const PUT_SEEDED_PUT_EXPECTED_PRICE: f64 = 0.0000;

    #[test]
    fn test_longstaff_schwartz_call_seeded() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            15.0, 
            10.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Call, 
            500,
            Some(1234)
        );
        
        assert!(
            (longstaff_schwartz_pricer.generate_price() - CALL_SEEDED_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }

    #[test]
    fn test_longstaff_schwartz_put_seeded() {
        let longstaff_schwartz_pricer = LongstaffScwhartzPricer::new(
            15.0, 
            10.0, 
            0.05, 
            0.1, 
            Some(date!(2024 - 01 - 01)), 
            date!(2025 - 01 - 01), 
            1000, 
            TypeFlag::Put, 
            500,
            Some(9876)
        );

        assert!(
            (longstaff_schwartz_pricer.generate_price() - PUT_SEEDED_PUT_EXPECTED_PRICE).abs() < TOLERANCE
        );
    }
}
