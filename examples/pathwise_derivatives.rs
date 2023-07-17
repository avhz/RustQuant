// A simple pathwise derivative example.
// We compute the Black-Scholes Greeks for a call option,
// using the pathwise method.
//
// Basically, we compute the derivative of the payoff function
// with respect to the parameters of the model many times via
// Monte Carlo simulation, and then average the results.

use rand_distr::{Distribution, Normal};
use RustQuant::{autodiff::*, instruments::options::TypeFlag};

fn main() {
    let mut greeks = Greeks {
        delta: 0.,
        vega: 0.,
        theta: 0.,
        rho: 0.,
    };

    // Allocate a new graph.
    let g = Graph::new();
    // Allocate variables.
    let spot = g.var(150.);
    let time = g.var(1.);
    let drift = g.var(0.1);
    let diffusion = g.var(0.2);

    let n = 100;

    // Allocate a vector for the basket option.
    let mut basket: Vec<Variable> = Vec::with_capacity(100);

    let start = std::time::Instant::now();
    for _ in 0..n {
        // Generate paths of the underlying assets.
        for _ in 0..100 {
            let path = Pathwise::gbm(spot, time, drift, diffusion);
            basket.push(path);
        }

        // Compute the basket sum.
        let s = basket.iter().copied().sum::<Variable>();

        // let s = Pathwise::gbm(spot, time, drift, diffusion);

        // Compute the payoff and discount it.
        let discount = Pathwise::df(drift, time);
        let payoff = discount * Pathwise::payoff(s, 120., TypeFlag::Call);

        // Accumulate the gradient.
        let gradient = payoff.accumulate();

        // Differentiate with respect to the parameters.
        greeks.delta += gradient.wrt(&spot);
        greeks.vega += gradient.wrt(&diffusion);
        greeks.theta += gradient.wrt(&time);
        greeks.rho += gradient.wrt(&drift);
    }
    let end = start.elapsed();

    println!("Delta \t= {}", greeks.delta / n as f64);
    println!("Vega \t= {}", greeks.vega / n as f64);
    println!("Theta \t= {}", greeks.theta / n as f64);
    println!("Rho \t= {}", greeks.rho / n as f64);

    println!("Computation time: {:?}", end);
}

struct Pathwise {}

struct Greeks {
    delta: f64,
    vega: f64,
    theta: f64,
    rho: f64,
}

impl Pathwise {
    // Discount factor.
    fn df<'v>(rate: Variable<'v>, time: Variable<'v>) -> Variable<'v> {
        (-rate * time).exp()
    }

    // Payoff function.
    fn payoff<'v>(spot: Variable<'v>, strike: f64, flag: TypeFlag) -> Variable<'v> {
        match flag {
            TypeFlag::Call => RustQuant::autodiff::Max::max(&(spot - strike), 0.),
            TypeFlag::Put => RustQuant::autodiff::Max::max(&(strike - spot), 0.),
        }
    }

    // Closed-form solution for Geometric Brownian Motion.
    fn gbm<'v>(
        spot: Variable<'v>,
        time: Variable<'v>,
        drift: Variable<'v>,
        diffusion: Variable<'v>,
    ) -> Variable<'v> {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0., 1.).unwrap();
        let z = normal.sample(&mut rng);
        let w = z * time.sqrt();

        // ST = S0 * exp((mu - sigma^2 / 2) * t + sigma * Wt)
        spot * ((drift - diffusion.powi(2) / 2.) * time + diffusion * w).exp()
    }
}
