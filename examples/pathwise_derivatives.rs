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
    let mut basket_option = BasketOption {
        price: 0.,
        delta: 0.,
        vega: 0.,
        theta: 0.,
        rho: 0.,
    };

    // Allocate a new graph.
    let g = Graph::new();
    // Allocate variables.
    let strike = 10.; // We don't differentiate with respect to the strike.
    let spot = g.var(10.);
    let time = g.var(1.);
    let drift = g.var(0.1);
    let diffusion = g.var(0.5);

    let n_sims = 100;
    let n_assets = 50;

    // Allocate a vector for the basket option.
    let mut basket: Vec<Variable> = Vec::with_capacity(2 * n_assets);

    let start = std::time::Instant::now();
    for _ in 0..n_sims {
        // Generate paths of the underlying assets.
        for _ in 0..n_assets {
            let path = Pathwise::gbm(spot, time, drift, diffusion);
            basket.push(path);
        }

        // Compute the basket (equally) weighted sum.
        let s = basket.iter().copied().sum::<Variable>() / 100.;

        // let s = Pathwise::gbm(spot, time, drift, diffusion);

        // Compute the payoff and discount it.
        let discount = Pathwise::df(drift, time);
        let payoff = discount * Pathwise::payoff(s, strike, TypeFlag::Call);

        basket_option.price += payoff.value();

        // Accumulate the gradient.
        let gradient = payoff.accumulate();

        // Differentiate with respect to the parameters.
        basket_option.delta += gradient.wrt(&spot);
        basket_option.vega += gradient.wrt(&diffusion);
        basket_option.theta += gradient.wrt(&time);
        basket_option.rho += gradient.wrt(&drift);
    }
    let end = start.elapsed();

    println!("Price \t= {}", basket_option.price / n_sims as f64);
    println!("Delta \t= {}", basket_option.delta / n_sims as f64);
    println!("Vega \t= {}", basket_option.vega / n_sims as f64);
    println!("Theta \t= {}", basket_option.theta / n_sims as f64);
    println!("Rho \t= {}", basket_option.rho / n_sims as f64);

    println!("Computation time: {:?}", end);
}

struct BasketOption {
    price: f64,
    delta: f64,
    vega: f64,
    theta: f64,
    rho: f64,
}

struct Pathwise {}

impl Pathwise {
    // Discount factor.
    #[inline]
    fn df<'v>(rate: Variable<'v>, time: Variable<'v>) -> Variable<'v> {
        (-rate * time).exp()
    }

    // Payoff function.
    #[inline]
    fn payoff<'v>(spot: Variable<'v>, strike: f64, flag: TypeFlag) -> Variable<'v> {
        match flag {
            TypeFlag::Call => Max::max(&(spot - strike), 0.),
            TypeFlag::Put => Max::max(&(strike - spot), 0.),
        }
    }

    // Closed-form solution for Geometric Brownian Motion (one step).
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
