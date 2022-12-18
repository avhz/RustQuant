
# `RustQuant`

Rust library for quantitative finance tools. 

Contact: rustquantcontact@gmail.com 

## Release notes:

### `v0.0.9`

+ Basic numerical integrators (midpoint, trapezoid, Simpson's 3/8).
+ Density functions for common distributions:
    + Gaussian, Bernoulli, Binomial, Poisson.

### `v0.0.8`

+ Characteristic functions for common distributions:
    + Gaussian, Bernoulli, Binomial, Poisson, Uniform, Chi-Squared, Gamma, and Exponential.

**Disclaimer**: This is currently a free-time project and not a professional financial software library. Nothing in this library should be taken as financial advice, and I do not recommend you to use it for trading or making financial decisions. 

Some references used:

+ *Options, Futures, and Other Derivatives* - John C. Hull 
+ *Interest Rate Models - Theory and Practice (With Smile, Inflation and Credit)* - Damiano Brigo & Fabio Mercurio
+ *Monte Carlo Methods in Financial Engineering* - Paul Glasserman
+ *Evaluating Derivatives - Principles and Techniques of Algorithmic Differentiation* - Andreas Griewank & Andrea Walther
+ *Stochastic Calculus for Finance II: Continuous-Time Models* - Steven E. Shreve
+ *Option Pricing Formulas* - Espen Gaarder Haug
+ *Modern Computational Finance: AAD and Parallel Simulations* - Antoine Savine

# Table of Contents

1. [Automatic Differentiation](#autodiff)
2. [Option Pricers](#options)
3. [Stochastic Processes and Short Rate Models](#stochastics)
4. [Bonds](#bonds)
5. [Random](#random)
5. [Mathematics](#maths)
6. [Helper Functions and Macros](#helpers)
7. [How-tos](#howto)

## Automatic Differentiation <a name="autodiff"></a>

Currently only gradients can be computed. Suggestions on how to extend the functionality to Hessian matrices are definitely welcome. 

+ [ ] Forward (Tangent) Mode
    + Implementation via Dual Numbers.
    + Useful when number of outputs is *larger* than number of inputs. 
        + i.e. for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \gg n$
+ [x] Reverse (Adjoint) Mode
    + Implementation via Operator and Function Overloading.
    + Useful when number of outputs is *smaller* than number of inputs. 
        + i.e for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \ll n$

## Option Pricers <a name="options"></a>

+ Closed-form price solutions:
    + [x] Barrier
    + [x] European Options
    + [x] Greeks/Sensitivities
    + [x] Lookback 
    + [ ] Heston Model
    + [ ] Basket
    + [ ] Rainbow
    + [ ] American
    + [ ] Heston Model 

+ Lattice models:
    + [x] Binomial Tree (Cox-Ross-Rubinstein)

The stochastic process generators can be used to price path-dependent options via Monte-Carlo.

+ Monte Carlo pricing:
    + [x] Lookback
    + [ ] Asian
    + [ ] Chooser
    + [ ] Barrier

## Stochastic Processes and Short Rate Models <a name="stochastics"></a>

The following is a list of stochastic processes that can be generated.

+ [x] Brownian Motion
+ [x] Geometric Brownian Motion
    + $dX_t = \mu X_t dt + \sigma X_t dW_t$
    + Models: Black-Scholes (1973), Rendleman-Bartter (1980)
+ [x] Cox-Ingersoll-Ross (1985)
    + $dX_t = (\theta - \alpha X_t)dt + \sqrt{r_t} \sigma dW_t$
+ [x] Ornstein-Uhlenbeck process
    + $dX_t = \theta(\mu - X_t)dt + \sigma dW_t$
    + Models: Vasicek (1977)
+ [ ] Ho-Lee (1986)
    + $dX_t = \theta_t dt + \sigma dW_t$
+ [ ] Hull-White (1990)
    + $dX_t = (\theta - \alpha X_t)dt + \sigma_t dW_t$
+ [ ] Black-Derman-Toy (1990)
    + $d\ln(X) = \left[ \theta_t + \frac{\sigma_t'}{\sigma_t}\ln(X) \right]dt + \sigma_t dW_t$
    + $d\ln(X) = \theta_t dt + \sigma dW_t$
+ [ ] Merton's model (1973)
    + $X_t = X_0 + at + \sigma W_t^*$
    + $dX_t = adt + \sigma dW_t^*$

## Bonds <a name="bonds"></a>

Most will follow the notation and formulas in John C. Hull's *Options, Futures, and Other Derivatives*.

+ [ ] Prices:
    + [X] The Vasicek Model
    + [x] The Cox, Ingersoll, and Ross Model
    + [ ] The Rendleman and Bartter Model
    + [ ] The Ho–Lee Model
    + [ ] The Hull–White (One-Factor) Model
    + [ ] The Black–Derman–Toy Model
    + [ ] The Black–Karasinski Model
+ [ ] Duration
+ [ ] Convexity

## Random <a name="random"></a>

+ Characteristic functions:
    + [x] Gaussian
    + [x] Bernoulli
    + [x] Binomial
    + [x] Poisson
    + [x] Uniform (discrete & continuous)
    + [x] Chi-Squared
    + [x] Gamma
    + [x] Exponential
+ Density/mass functions:
    + [x] Bernoulli
    + [x] Binomial
    + [x] Poisson
    + [ ] Gaussian
    + [ ] Uniform (discrete & continuous)
    + [ ] Chi-Squared
    + [ ] Gamma
    + [ ] Exponential
+ Distribution functions:
    + [ ] Gaussian
    + [ ] Bernoulli
    + [ ] Binomial
    + [ ] Poisson
    + [ ] Uniform (discrete & continuous)
    + [ ] Chi-Squared
    + [ ] Gamma
    + [ ] Exponential

## Mathematics <a name="maths"></a>

+ Numerical Integration (needed for Heston model, for example):
    + [x] Composite Midpoint Rule
    + [x] Composite Trapezoidal Rule
    + [x] Composite Simpson's 3/8 Rule
    + [ ] Tanh-Sinh (double exponential) quadrature 
+ [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)
+ [x] Newton-Raphson
+ [x] Standard Normal Distribution (Distribution/Density functions, and generation of variates)
+ [ ] Interpolation

## Helper Functions and Macros <a name="helpers"></a>

A collection of utility functions and macros. 

+ [x] Plot a vector.
+ [x] Write vector to file.
+ [x] Cumulative sum of vector.
+ [x] Linearly spaced sequence.
+ [x] `assert_approx_equal!`

# How-tos <a name="howto"></a>

## Compute gradients:

```rust
use RustQuant::autodiff::*;

fn main() {
    // Create a new Tape.
    let t = Tape::new();

    // Assign variables.
    let x = t.var(0.5);
    let y = t.var(4.2);

    // Define a function.
    let z = x * y + x.sin();

    // Accumulate the gradient.
    let grad = z.accumulate();

    println!("Function = {}", z);
    println!("Gradient = {:?}", grad.wrt([x, y]));
}
```

## Price options:

```rust
use RustQuant::options::*;

fn main() {
    let VanillaOption = EuropeanOption {
        initial_price: 100.0,
        strike_price: 110.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        dividend_rate: 0.02,
        time_to_maturity: 0.5,
    };

    let prices = VanillaOption.price();

    println!("Call price = {}", prices.0);
    println!("Put price = {}", prices.1);
}
```

## Generate stochastic processes:

```rust
use RustQuant::stochastics::*;

fn main() {
    // Create new GBM with mu and sigma.
    let gbm = GeometricBrownianMotion::new(0.05, 0.9);

    // Generate path using Euler-Maruyama scheme.
    // Parameters: x_0, t_0, t_n, n, sims, parallel.
    let output = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 10, 1, false);

    println!("GBM = {:?}", output.trajectories);
}
```
