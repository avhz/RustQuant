// Only run test when 'cargo test' not 'cargo build/run'.
// #[cfg(test)]
// mod tests;

mod BlackScholes;
mod normalDistribution;

pub use normalDistribution::*;
pub use BlackScholes::*;
