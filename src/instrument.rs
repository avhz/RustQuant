pub trait Instrument {
    fn NPV(&self) -> f64;
    fn error(&self) -> f64;
    fn valuationDate(&self) -> f64;
}

pub enum PricingEngine {
    Analytic,
    Simulation,
}
