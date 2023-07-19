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
    let s = 5.;
    let t = 1.;
    let k = 5.;
    let r = 0.1;
    let v = 0.5;
    let n_sims = 500;
    let n_assets = 500;

    println!("millis, spot, time, price, delta, vega, theta, rho");

    // Vary over time and underlying.
    // for t in (1..=10).map(|x| x as f64 / 10.) {
    //     for s in (1..=10).step_by(1) {
    //         test_basket_option(s as f64, k, t as f64, r, v, n_sims, n_assets);
    //     }
    // }

    // // Just increase the number of simulations.
    for i in (10..=n_sims).step_by(10) {
        test_basket_option(s, k, t, r, v, i, 100);
    }

    // // Just increase the number of assets.
    for j in (10..=n_assets).step_by(10) {
        test_basket_option(s, k, t, r, v, 100, j);
    }

    // println!("millis, n_sims, n_assets, price, delta, vega, theta, rho");

    // Increase both the number of simulations and the number of assets.
    // for i in (10..=n_sims).step_by(10) {
    //     for j in (10..=n_assets).step_by(10) {
    //         test_basket_option(s, k, t, r, v, i, j);
    //     }
    // }
}

fn test_basket_option(s: f64, k: f64, t: f64, r: f64, v: f64, n_sims: usize, n_assets: usize) {
    let mut basket_option = BasketOption::new();

    // Allocate a new graph.
    let g = Graph::new();

    // Allocate variables.
    let strike = k; // We don't differentiate with respect to the strike.
    let spot = g.var(s);
    let time = g.var(t);
    let drift = g.var(r);
    let diffusion = g.var(v);

    // Allocate a vector for the basket option.
    let mut basket: Vec<Variable> = Vec::with_capacity(2 * n_assets);

    let start = std::time::Instant::now();
    for _ in 0..n_sims {
        basket.clear();

        // Generate paths of the underlying assets.
        for _ in 0..n_assets {
            basket.push(Pathwise::gbm(spot, time, drift, diffusion));
        }

        // Compute the basket (equally) weighted sum.
        let s = basket.iter().copied().sum::<Variable>() / n_assets as f64;

        // Compute the payoff and discount it.
        let discount = Pathwise::df(drift, time);
        let payoff = Pathwise::payoff(s, strike, TypeFlag::Call);
        let discounted_payoff = discount * payoff;

        basket_option.price += discounted_payoff.value();

        // Accumulate the gradient.
        let gradient = discounted_payoff.accumulate();

        // Differentiate with respect to the parameters.
        basket_option.delta += gradient.wrt(&spot);
        basket_option.vega += gradient.wrt(&diffusion);
        basket_option.theta += gradient.wrt(&time);
        basket_option.rho += gradient.wrt(&drift);
    }
    let end = start.elapsed();

    print!("{:?},", end.as_millis());

    // println!(
    //     "{:.4},{:.1},{:.1},{:.4},{:.4},{:.4},{:.4},{:.4}",
    //     end.as_millis(),
    //     s,
    //     t,
    //     basket_option.price / n_sims as f64,
    //     basket_option.delta / n_sims as f64,
    //     basket_option.vega / n_sims as f64,
    //     basket_option.theta / n_sims as f64,
    //     basket_option.rho / n_sims as f64
    // );

    // println!(
    //     "{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
    //     end.as_millis(),
    //     n_sims,
    //     n_assets,
    //     basket_option.price / n_sims as f64,
    //     basket_option.delta / n_sims as f64,
    //     basket_option.vega / n_sims as f64,
    //     basket_option.theta / n_sims as f64,
    //     basket_option.rho / n_sims as f64
    // );
}

struct BasketOption {
    price: f64,
    delta: f64,
    vega: f64,
    theta: f64,
    rho: f64,
}

impl BasketOption {
    fn new() -> Self {
        Self {
            price: 0.,
            delta: 0.,
            vega: 0.,
            theta: 0.,
            rho: 0.,
        }
    }
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
