/// Autonomous stochastic process.
///
/// Autonomous refers to a process where the drift and diffusion
/// do not explicitly depend on time `t`.
pub trait AutonomousStochasticProcess: StochasticProcess {}
