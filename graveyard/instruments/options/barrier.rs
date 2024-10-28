// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::instruments::Payoff;

use super::{BarrierType, OptionContract, TypeFlag};

/// Barrier option.
#[derive(Debug, Clone)]
pub struct BarrierOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Barrier type (up-and-out, down-and-out, up-and-in, down-and-in).
    pub barrier_type: BarrierType,

    /// Barrier level.
    pub barrier: f64,

    /// Strike price of the option.
    pub strike: f64,

    /// Rebate amount.
    pub rebate: Option<f64>,
}

impl Payoff for BarrierOption {
    type Underlying = Vec<f64>;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        let s = underlying;
        let b = self.barrier;
        let k = self.strike;

        let terminal = s.last().unwrap();

        let above = s.iter().any(|&x| x >= b);
        let below = s.iter().any(|&x| x <= b);

        match self.contract.type_flag {
            TypeFlag::Call => {
                let payoff = (terminal - k).max(0.0);

                match self.barrier_type {
                    BarrierType::UpAndOut => match above {
                        true => 0.0,
                        false => payoff,
                    },
                    BarrierType::DownAndOut => match below {
                        true => 0.0,
                        false => payoff,
                    },
                    BarrierType::UpAndIn => match above {
                        true => payoff,
                        false => 0.0,
                    },
                    BarrierType::DownAndIn => match below {
                        true => payoff,
                        false => 0.0,
                    },
                }
            }
            TypeFlag::Put => {
                let payoff = (k - terminal).max(0.0);

                match self.barrier_type {
                    BarrierType::UpAndOut => match above {
                        true => 0.0,
                        false => payoff,
                    },
                    BarrierType::DownAndOut => match below {
                        true => 0.0,
                        false => payoff,
                    },
                    BarrierType::UpAndIn => match above {
                        true => payoff,
                        false => 0.0,
                    },
                    BarrierType::DownAndIn => match below {
                        true => payoff,
                        false => 0.0,
                    },
                }
            }
        }
    }
}
