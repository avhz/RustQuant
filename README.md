
![](./images/logo.png)

<p align="center">
    <a href="#license" alt="license">
        <img alt="License" src="https://img.shields.io/badge/Dual_License-MIT_and_Apache_2.0-black?logo=apache">
    <a href="#version" alt="version">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/RustQuant?logo=rust&color=black">
    <a href="#downloads" alt="downloads">
        <img alt="Crates.io" src="https://img.shields.io/crates/d/RustQuant?logo=rust&color=black">
    <a href="#stars" alt="stars">
        <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/avhz/RustQuant?logo=github&color=black">
</p>

<p align="center">
    <a href="#build" alt="build">
        <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/avhz/RustQuant/build.yml">
    <a href="#codecov" alt="codecov">
        <img alt="Codecov" src="https://img.shields.io/codecov/c/gh/avhz/RustQuant">
    <a href="#deps" alt="deps">
        <img alt="Dependencies" src="https://deps.rs/repo/github/avhz/RustQuant/status.svg">
</p>

<p align="center">
    <a href="#discord" alt="discord">
        <img alt="Discord" src="https://img.shields.io/discord/1146771658082881636?logo=discord">
</p>

A Rust library for quantitative finance tools. Also the largest option pricing library in Rust.

:dart: I want to hit a stable `v0.1.0` by the end of 2023, so any feedback, suggestions, or contributions are strongly welcomed!

<div align="center">

| Email                        | Discord                         | Latest Changes              |
|:----------------------------:|:-------------------------------:|:---------------------------:|
| <RustQuantContact@gmail.com> | <https://discord.gg/tQcM77h8vr> | [Changelog](./CHANGELOG.md) |

</div>


## Modules

**[`autodiff`](./src/autodiff/README.md)** - Algorithmic adjoint differentiation for efficiently computing gradients of scalar output functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$.

> [`curves`](./src/curves/README.md) - Curves and surfaces, such as the yield curve and volatility surface. 

> [`data`](./src/data/README.md) - Methods for reading and writing data from/to various sources (CSV, JSON, Parquet). Can also download data from Yahoo! Finance.

> [`error`](./src/error.rs) - RustQuant error module.

> [`instruments`](./src/instruments/README.md) - Various implementations for instruments like `Bonds` and `Options`, and the pricing of them. Others coming in the future (swaps, futures, CDSs, etc).

> [`math`](./src/math/README.md)

> [`ml`](./src/ml/README.md)

> [`macros`](./src/macros/README.md)

> [`money`](./src/money/README.md)

> [`portfolio`](./src/portfolio/README.md)

> [`statistics`](./src/statistics/README.md)

> [`stochastics`](./src/stochastics/README.md)

> [`time`](./src/time/README.md)

> [`trading`](./src/trading/README.md)


## Features

<details>
<summary>
<h4>:bar_chart: Distributions <a name="distributions"></a></h4>
<br>PDFs, CDFs, MGFs, CFs, and other ditrubution related functions for common distributions.<br>
</summary>

Probability density/mass functions, distribution functions, characteristic functions, etc.

- [x] Gaussian
- [x] Bernoulli
- [x] Binomial
- [x] Poisson
- [x] Uniform (discrete & continuous)
- [x] Chi-Squared
- [x] Gamma
- [x] Exponential

</details>

<details>
<summary>
<h4> :chart_with_upwards_trend: Instruments <a name="instruments"></a></h4>
<br><br>
</summary>


</details>

<details>
<summary>
<h4> :triangular_ruler: Mathematics <a name="maths"></a></h4>
<br>Fast Fourier Transform (FFT), numerical integration (double-exponential quadrature), optimisation/root-finding (gradient descent, Newton-Raphson), and risk-reward metrics. <br>
</summary>

### Optimization and Root Finding

- [x] Gradient Descent
- [x] Newton-Raphson

Note: the reason you need to specify the lifetimes and use the type `Variable` is because the gradient descent optimiser uses the `RustQuant::autodiff` module to compute the gradients. This is a slight inconvenience, but the speed-up is enormous when working with functions with many inputs (when compared with using finite-difference quotients).

```rust
use RustQuant::optimisation::GradientDescent;

// Define the objective function.
fn himmelblau<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
    let x = variables[0];
    let y = variables[1];

    ((x.powf(2.0) + y - 11.0).powf(2.0) + (x + y.powf(2.0) - 7.0).powf(2.0))
}

fn main() {
    // Create a new GradientDescent object with:
    //      - Step size: 0.005 
    //      - Iterations: 10000
    //      - Tolerance: sqrt(machine epsilon)
    let gd = GradientDescent::new(0.005, 10000, std::f64::EPSILON.sqrt() );

    // Perform the optimisation with:
    //      - Initial guess (10.0, 10.0),
    //      - Verbose output.
    let result = gd.optimize(&himmelblau, &vec![10.0, 10.0], true);
    
    // Print the result.
    println!("{:?}", result.minimizer);
}
```

### Integration

- Numerical Integration (needed for Heston model, for example):
  - [x] Tanh-Sinh (double exponential) quadrature
  - [x] Composite Midpoint Rule
  - [x] Composite Trapezoidal Rule
  - [x] Composite Simpson's 3/8 Rule

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

### Risk-Reward Metrics

- [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)

</details>

<details>
<summary>
<h4>:crystal_ball: Machine Learning <a name="ml"></a></h4>
<br>Currently only linear regression is implemented (and working on logistic regression). More to come in the future.<br>
</summary>

### Regression

- [x] Linear (using QR or SVD decomposition)
- [x] Logistic (via IRLS, adding MLE in the future).

</details>

<details>
<summary>
<h4> :moneybag: Money <a name="money"></a></h4>
<br>Implementations for `Cashflows`, `Currencies`, and `Quotes`, and similar objects.<br>
</summary>

- `Cashflow`
- `Currency`
- `Money`
- `Quote`
- `Leg`

</details>

<details>
<summary>
<h4>:chart_with_upwards_trend: Stochastic Processes and Short Rate Models <a name="stochastics"></a></h4>
<br> Can generate Brownian Motion (standard, arithmetic and geometric) and various short-rate models (CIR, OU, Vasicek, Hull-White, etc). <br>
</summary>

The following is a list of stochastic processes that can be generated.

- Brownian Motions:
  - Standard Brownian Motion
    - $dX(t) = dW(t)$
  - Arithmetic Brownian Motion
    - $dX(t) = \mu dt + \sigma dW(t)$
  - Geometric Brownian Motion
    - $dX(t) = \mu X(t) dt + \sigma X(t) dW(t)$
  - Fractional Brownian Motion
- Cox-Ingersoll-Ross (1985)
  - $dX(t) = \left[ \theta - \alpha X(t) \right] dt + \sigma \sqrt{r_t} dW(t)$
- Ornstein-Uhlenbeck process
  - $dX(t) = \theta \left[ \mu - X(t) \right] dt + \sigma dW(t)$
- Ho-Lee (1986)
  - $dX(t) = \theta(t) dt + \sigma dW(t)$
- Hull-White (1990)
  - $dX(t) = \left[ \theta(t) - \alpha X(t) \right]dt + \sigma dW(t)$
- Extended Vasicek (1990)
  - $dX(t) = \left[ \theta(t) - \alpha(t) X(t) \right] dt + \sigma dW(t)$
- Black-Derman-Toy (1990)
  - $d\ln[X(t)] = \left[ \theta(t) + \frac{\sigma'(t)}{\sigma(t)}\ln[X(t)] \right]dt + \sigma_t dW(t)$

```rust
use RustQuant::stochastics::*;

fn main() {
    // Create new GBM with mu and sigma.
    let gbm = GeometricBrownianMotion::new(0.05, 0.9);

    // Generate path using Euler-Maruyama scheme.
    // Parameters: x_0, t_0, t_n, n, sims, parallel.
    let output = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 10, 1, false);

    println!("GBM = {:?}", output.paths);
}
```

</details>

<details>
<summary>
<h4>:calendar: Time and Date <a name="time"></a></h4>
<br>Time and date functionality. Mostly the `DayCounter` for pricing options and bonds. <br>
</summary>

- `DayCounter`

</details>

<details>
<summary>
<h4>:handshake: Miscellaneous Functions and Macros <a name="helpers"></a></h4>
<br>Various helper functions and macros.<br>
</summary>

A collection of utility functions and macros.

- [x] Plot a vector.
- [x] Write vector to file.
- [x] Cumulative sum of vector.
- [x] Linearly spaced sequence.
- [x] `assert_approx_equal!`

</details>

<details>
<summary>
<h4>:heavy_check_mark: How-tos <a name="howto"></a></h4>
<br>Guides for using RustQuant.<br>
</summary>

See [/examples](./examples) for more details. Run them with:

```bash
cargo run --example automatic_differentiation
```

I would not recommend using RustQuant within any other libraries for some time, as it will most likely go through many breaking changes as I learn more Rust and settle on a decent structure for the library.

:pray: I would greatly appreciate contributions so it can get to the `v1.0.0` mark ASAP.

</details>

<details>
<summary>
<h4>:book: References <a name="references"></a></h4>
<br>References and resources used for this project.<br>
</summary>

- John C. Hull - *Options, Futures, and Other Derivatives*
- Damiano Brigo & Fabio Mercurio - *Interest Rate Models - Theory and Practice (With Smile, Inflation and Credit)*
- Paul Glasserman - *Monte Carlo Methods in Financial Engineering*
- Andreas Griewank & Andrea Walther - *Evaluating Derivatives - Principles and Techniques of Algorithmic Differentiation*
- Steven E. Shreve - *Stochastic Calculus for Finance II: Continuous-Time Models*
- Espen Gaarder Haug - *Option Pricing Formulas*
- Antoine Savine - *Modern Computational Finance: AAD and Parallel Simulations*

</details>


> [!NOTE]  
> Disclaimer: This is currently a free-time project and not a professional financial software library. Nothing in this library should be taken as financial advice, and I do not recommend you to use it for trading or making financial decisions.
