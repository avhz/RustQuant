pub trait PathDependentOption {
    // fn payoff(&self) -> f64;
    fn price(&self) -> f64;
}
