use time::Date;
use nalgebra::{DMatrix, DVector};
use rand::thread_rng;
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
        pub num_simulations: u64
}
