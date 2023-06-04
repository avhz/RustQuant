
![](./Images/logo.png)

<p align="center">
    <a href="#license" alt="license">
        <img alt="License" src="https://img.shields.io/github/license/avhz/RustQuant">
    <a href="#build" alt="build">
        <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/avhz/RustQuant/rust.yml">
    <a href="#downloads" alt="downloads">
        <img alt="Crates.io" src="https://img.shields.io/crates/d/RustQuant">
    <a href="#stars" alt="stars">
        <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/avhz/RustQuant">
    <a href="#version" alt="version">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/RustQuant">
    <a href="#codecov" alt="codecov">
        <img alt="Codecov" src="https://img.shields.io/codecov/c/gh/avhz/RustQuant">
    <a href="#deps" alt="deps">
        <img alt="Dependencies" src="https://deps.rs/repo/github/avhz/RustQuant/status.svg">
</p>

Rust library for quantitative finance tools. 

:dart: I want to hit a stable and legitimate `v1.0.0` by the end of 2023, so any and all feedback, suggestions, or contributions are strongly welcomed! 

Contact: rustquantcontact@gmail.com 

**Disclaimer**: This is currently a free-time project and not a professional financial software library. Nothing in this library should be taken as financial advice, and I do not recommend you to use it for trading or making financial decisions. 

## :newspaper: Latest features

+ Download time series data from [Yahoo! Finance](https://finance.yahoo.com/).
+ Read (write) from (to) `.csv`, `.json`, and `.parquet` files, using [Polars `DataFrames`](https://pola-rs.github.io/polars-book/).
+ Arithmetic Brownian Motion generator. 
+ Gamma, exponential, and chi-squared distributions.
+ Forward start option pricer (Rubinstein 1990 formula).
+ Gap option and cash-or-nothing option pricers (currently adding more binary options).
+ Asian option pricer (closed-form solution for continuous geometric average).
+ Heston Model option pricer (uses the tanh-sinh quadrature numerical integrator).
+ Tanh-sinh (double exponential) quadrature for evaluating integrals.
    + Plus other basic numerical integrators (midpoint, trapezoid, Simpson's 3/8).
+ Characteristic functions and density functions for common distributions:
    + Gaussian, Bernoulli, Binomial, Poisson, Uniform, Chi-Squared, Gamma, and Exponential.

# Table of Contents

1. [Automatic Differentiation](#autodiff)
2. [Option Pricers](#options)
3. [Stochastic Processes and Short Rate Models](#stochastics)
4. [Bonds](#bonds)
5. [Distributions](#distributions)
5. [Mathematics](#maths)
6. [Helper Functions and Macros](#helpers)
7. [How-tos](#howto)
8. [References](#references)


## :link: Automatic Differentiation <a name="autodiff"></a>

Currently only gradients can be computed. Suggestions on how to extend the functionality to Hessian matrices are definitely welcome. 

+ [x] Reverse (Adjoint) Mode
    + Implementation via Operator and Function Overloading.
    + Useful when number of outputs is *smaller* than number of inputs. 
        + i.e for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \ll n$
+ [ ] Forward (Tangent) Mode
    + Implementation via Dual Numbers.
    + Useful when number of outputs is *larger* than number of inputs. 
        + i.e. for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \gg n$

## :money_with_wings: Option Pricers <a name="options"></a>

+ Closed-form price solutions:
    + [x] Heston Model
    + [x] Barrier
    + [x] European
    + [x] Greeks/Sensitivities
    + [x] Lookback
    + [x] Asian: Continuous Geometric Average 
    + [x] Forward Start 
    + [ ] Basket
    + [ ] Rainbow
    + [ ] American

+ Lattice models:
    + [x] Binomial Tree (Cox-Ross-Rubinstein)

The stochastic process generators can be used to price path-dependent options via Monte-Carlo.

+ Monte Carlo pricing:
    + [x] Lookback
    + [ ] Asian
    + [ ] Chooser
    + [ ] Barrier

## :chart_with_upwards_trend: Stochastic Processes and Short Rate Models <a name="stochastics"></a>

The following is a list of stochastic processes that can be generated.

+ [x] Brownian Motion
+ [x] Arithmetic Brownian Motion
    + $dX_t = \mu dt + \sigma dW_t$
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

## :chart_with_downwards_trend: Bonds <a name="bonds"></a>

+ Prices:
    + [X] The Vasicek Model
    + [x] The Cox, Ingersoll, and Ross Model
    + [ ] The Rendleman and Bartter Model
    + [ ] The Ho–Lee Model
    + [ ] The Hull–White (One-Factor) Model
    + [ ] The Black–Derman–Toy Model
    + [ ] The Black–Karasinski Model
+ [ ] Duration
+ [ ] Convexity

## :bar_chart: Distributions <a name="distributions"></a>

Probability density/mass functions, distribution functions, characteristic functions, etc. 

+ [x] Gaussian
+ [x] Bernoulli
+ [x] Binomial
+ [x] Poisson
+ [x] Uniform (discrete & continuous)
+ [x] Chi-Squared
+ [x] Gamma
+ [x] Exponential

## :triangular_ruler: Mathematics <a name="maths"></a>

+ Numerical Integration (needed for Heston model, for example):
    + [x] Tanh-Sinh (double exponential) quadrature 
    + [x] Composite Midpoint Rule
    + [x] Composite Trapezoidal Rule
    + [x] Composite Simpson's 3/8 Rule
+ [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)
+ [x] Newton-Raphson
+ [x] Standard Normal Distribution (Distribution/Density functions, and generation of variates)
+ [ ] Interpolation

## :handshake: Helper Functions and Macros <a name="helpers"></a>

A collection of utility functions and macros. 

+ [x] Plot a vector.
+ [x] Write vector to file.
+ [x] Cumulative sum of vector.
+ [x] Linearly spaced sequence.
+ [x] `assert_approx_equal!`

## :heavy_check_mark: How-tos <a name="howto"></a>

I would not recommend using RustQuant within any other libraries for some time, as it will most likely go through many breaking changes as I learn more Rust and settle on a decent structure for the library. 

:pray: I would greatly appreciate contributions so it can get to the `v1.0.0` mark ASAP.

### Download data from Yahoo! Finance:

You can download data from Yahoo! Finance into a Polars `DataFrame`.

```rust
use RustQuant::data::*;
use time::macros::date;

fn main() {
    // New YahooFinanceData instance. 
    // By default, date range is: 1970-01-01 to present. 
    let mut yfd = YahooFinanceData::new("AAPL".to_string());

    // Can specify custom dates (optional). 
    yfd.set_start_date(time::macros::datetime!(2019 - 01 - 01 0:00 UTC));
    yfd.set_end_date(time::macros::datetime!(2020 - 01 - 01 0:00 UTC));

    // Download the historical data. 
    yfd.get_price_history();

    println!("Apple's quotes: {:?}", yfd.price_history)
}
```

```bash
Apple's quotes: Some(shape: (252, 7)
┌────────────┬───────────┬───────────┬───────────┬───────────┬────────────┬───────────┐
│ date       ┆ open      ┆ high      ┆ low       ┆ close     ┆ volume     ┆ adjusted  │
│ ---        ┆ ---       ┆ ---       ┆ ---       ┆ ---       ┆ ---        ┆ ---       │
│ date       ┆ f64       ┆ f64       ┆ f64       ┆ f64       ┆ f64        ┆ f64       │
╞════════════╪═══════════╪═══════════╪═══════════╪═══════════╪════════════╪═══════════╡
│ 2019-01-02 ┆ 38.7225   ┆ 39.712502 ┆ 38.557499 ┆ 39.48     ┆ 1.481588e8 ┆ 37.994499 │
│ 2019-01-03 ┆ 35.994999 ┆ 36.43     ┆ 35.5      ┆ 35.547501 ┆ 3.652488e8 ┆ 34.209969 │
│ 2019-01-04 ┆ 36.1325   ┆ 37.137501 ┆ 35.950001 ┆ 37.064999 ┆ 2.344284e8 ┆ 35.670372 │
│ 2019-01-07 ┆ 37.174999 ┆ 37.2075   ┆ 36.474998 ┆ 36.982498 ┆ 2.191112e8 ┆ 35.590965 │
│ …          ┆ …         ┆ …         ┆ …         ┆ …         ┆ …          ┆ …         │
│ 2019-12-26 ┆ 71.205002 ┆ 72.495003 ┆ 71.175003 ┆ 72.477501 ┆ 9.31212e7  ┆ 70.798401 │
│ 2019-12-27 ┆ 72.779999 ┆ 73.4925   ┆ 72.029999 ┆ 72.449997 ┆ 1.46266e8  ┆ 70.771545 │
│ 2019-12-30 ┆ 72.364998 ┆ 73.172501 ┆ 71.305    ┆ 72.879997 ┆ 1.441144e8 ┆ 71.191582 │
│ 2019-12-31 ┆ 72.482498 ┆ 73.419998 ┆ 72.379997 ┆ 73.412498 ┆ 1.008056e8 ┆ 71.711739 │
└────────────┴───────────┴───────────┴───────────┴───────────┴────────────┴───────────┘)
```

### Read/write data:

```rust
use RustQuant::data::*;

fn main() {
    // New `Data` instance.
    let mut data = Data::new(
        format: DataFormat::CSV, // Can also be JSON or PARQUET.
        path: String::from("./file/path/read.csv")
    )

    // Read from the given file. 
    data.read().unwrap();

    // New path to write the data to. 
    data.path = String::from("./file/path/write.csv")
    data.write().unwrap();

    println!("{:?}", data.data)
}
```

### Compute gradients:

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

### Compute integrals:

```rust
use RustQuant::math::*;

fn main() {
    // Define a function to integrate: e^(sin(x))
    fn f(x: f64) -> f64 {
        (x.sin()).exp()
    }

    // Integrate from 0 to 5.
    let integral = integrate(f, 0.0, 5.0);

    // ~ 7.18911925
    println!("Integral = {}", integral); 
}
```

### Price options:

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

### Generate stochastic processes:

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

## :book: References: <a name="references"></a>

+ John C. Hull - *Options, Futures, and Other Derivatives*
+ Damiano Brigo & Fabio Mercurio - *Interest Rate Models - Theory and Practice (With Smile, Inflation and Credit)*
+ Paul Glasserman - *Monte Carlo Methods in Financial Engineering*
+ Andreas Griewank & Andrea Walther - *Evaluating Derivatives - Principles and Techniques of Algorithmic Differentiation*
+ Steven E. Shreve - *Stochastic Calculus for Finance II: Continuous-Time Models*
+ Espen Gaarder Haug - *Option Pricing Formulas*
+ Antoine Savine - *Modern Computational Finance: AAD and Parallel Simulations*
