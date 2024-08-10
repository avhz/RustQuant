// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use super::option_flags::*;
use time::Date;

#[allow(dead_code)]
trait Payoff<U, S> {
    fn payoff(&self, underlying: U, strike: S) -> f64;
}

impl Payoff<f64, f64> for OptionContract {
    fn payoff(&self, underlying: f64, strike: f64) -> f64 {
        match self.type_flag {
            TypeFlag::Call => (underlying - strike).max(0.0),
            TypeFlag::Put => (strike - underlying).max(0.0),
        }
    }
}

impl Payoff<Vec<f64>, f64> for OptionContract {
    fn payoff(&self, underlying: Vec<f64>, strike: f64) -> f64 {
        let mut payoff = 0.0;

        for &spot in underlying.iter() {
            payoff += match self.type_flag {
                TypeFlag::Call => (spot - strike).max(0.0),
                TypeFlag::Put => (strike - spot).max(0.0),
            };
        }

        payoff / underlying.len() as f64
    }
}

// trait Payoff {
//     type Underlying;
//     type Strike;

//     fn call_payoff(&self, underlying: Self::Underlying, strike: Self::Strike) -> f64;
//     fn put_payoff(&self, underlying: Self::Underlying, strike: Self::Strike) -> f64;
// }

// impl Payoff for OptionContract {
//     type Underlying = f64;
//     type Strike = f64;

//     #[inline]
//     fn call_payoff(&self, underlying: f64, strike: f64) -> f64 {
//         f64::max(underlying - strike, 0.0)
//     }

//     #[inline]
//     fn put_payoff(&self, underlying: f64, strike: f64) -> f64 {
//         f64::max(strike - underlying, 0.0)
//     }
// }

// impl Payoff for OptionContract {
//     type Underlying = Vec<f64>;
//     type Strike = f64;

//     #[inline]
//     fn call_payoff(&self, underlying: Vec<f64>, strike: f64) -> f64 {
//         let mut payoff = 0.0;
//         for &spot in underlying.iter() {
//             payoff += f64::max(spot - strike, 0.0);
//         }
//         payoff / underlying.len() as f64
//     }

//     #[inline]
//     fn put_payoff(&self, underlying: Vec<f64>, strike: f64) -> f64 {
//         let mut payoff = 0.0;
//         for &spot in underlying.iter() {
//             payoff += f64::max(strike - spot, 0.0);
//         }
//         payoff / underlying.len() as f64
//     }
// }

// trait Payoff<UNDERLYING, STRIKE> {
//     fn call_payoff(&self, underlying: UNDERLYING, strike: STRIKE) -> f64;
//     fn put_payoff(&self, underlying: UNDERLYING, strike: STRIKE) -> f64;
// }

// impl Payoff<f64, f64> for f64 {
//     fn call_payoff(&self, underlying: f64, strike: f64) -> f64 {
//         (underlying - strike).max(0.0)
//     }

//     fn put_payoff(&self, underlying: f64, strike: f64) -> f64 {
//         (strike - underlying).max(0.0)
//     }
// }

// impl Payoff<Vec<f64>, f64> for Vec<f64> {
//     fn call_payoff(&self, underlying: Vec<f64>, strike: f64) -> f64 {
//         let mut payoff = 0.0;
//         for (i, &spot) in underlying.iter().enumerate() {
//             payoff += (spot - strike).max(0.0);
//         }
//         payoff / underlying.len() as f64
//     }

//     fn put_payoff(&self, underlying: Vec<f64>, strike: f64) -> f64 {
//         let mut payoff = 0.0;
//         for (i, &spot) in underlying.iter().enumerate() {
//             payoff += (strike - spot).max(0.0);
//         }
//         payoff / underlying.len() as f64
//     }
// }

// impl Payoff<f64> for f64 {
//     fn payoff(&self, spot: f64) -> f64 {
//         self.max(spot)
//     }
// }

// impl Payoff<Vec<f64>> for Vec< {
//     fn payoff(&self, spot: Decimal) -> Decimal {
//         self.max(spot)
//     }
// }
