//! Module that contains the struct `Process`.
//! This is the return type of all the stochastic processes.

pub struct Process {
    trajectory: Vec<f64>,
}

pub trait StochasticProcess {}
