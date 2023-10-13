// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! A Rust library for quantitative finance.
//!
//! Contact: <RustQuantContact@gmail.com>
//!
//! Any contributions are greatly appreciated. Make a PR or open an issue !

// Strictly enforce documentation.
#![forbid(missing_docs)]
// Allow snake case.
// This is because much of this library is based on mathematics, so I
// want to adhere to the standard mathematical notation.
#![allow(non_snake_case)]
// Strictly enforce SAFETY comments.
// There is no unsafe code currently, but for anyone to add any, it must be
// documented with a SAFETY comment.
#![forbid(clippy::undocumented_unsafe_blocks)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RUSTQUANT MISCELLANEOUS MODULES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub use {error::*, macros::*, portfolio::*};

/// Macros module.
#[macro_use]
pub mod macros;

/// RustQuant error module.
pub mod error;

/// Portfolio module.
pub mod portfolio;

/// Curves module.
/// Curves (in the financial sense) are functions that map
/// a time to a value, such as a yield curve or a swap curve.
/// They may also be known as term structures.
pub mod curves {
    pub use crate::curves::{curve::*, models::*};

    /// Base curve trait.
    pub mod curve;

    /// Curve models.
    pub mod models;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// AUTOMATIC DIFFERENTIATION MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Reverse mode automatic differentation.
/// Currently only gradients can be computed.
/// Suggestions on how to extend the functionality to Hessian matrices are
/// definitely welcome.
///
/// Additionally, only functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$
/// (scalar output) are supported. However, you can manually apply the
/// differentiation to multiple functions that could represent a vector output.
///
/// - [x] Reverse (Adjoint) Mode
///   - Implementation via Operator and Function Overloading.
///   - Useful when number of outputs is *smaller* than number of inputs.
///     - i.e for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \ll n$
/// - [ ] Forward (Tangent) Mode
///   - Implementation via Dual Numbers.
///   - Useful when number of outputs is *larger* than number of inputs.
///     - i.e. for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \gg n$
///
/// ```rust
/// use RustQuant::autodiff::*;
///
/// fn main() {
///     // Create a new Graph to store the computations.
///     let g = Graph::new();
///
///     // Assign variables.
///     let x = g.var(69.);
///     let y = g.var(420.);
///
///     // Define a function.
///     let f = {
///       let a = x.powi(2);
///       let b = y.powi(2);
///
///       a + b + (x * y).exp()
///     };
///
///     // Accumulate the gradient.
///     let gradient = f.accumulate();
///
///     println!("Function = {}", f);
///     println!("Gradient = {:?}", gradient.wrt([x, y]));
/// }
/// ```
///
/// You can also generate Graphviz (dot) code to visualize the computation graphs:
///
/// ```rust
/// println!("{}", graphviz(&graph, &variables));
/// ```  
///
/// The computation graph from computing Black-Scholes Greeks is:
///
/// ![Black-Scholes Greeks tape.](./images/black_scholes_tape.png)
///
/// It is clearly a work in progress, but gives a general idea of how the
/// computation graph is structured.
///
/// If you want to improve the visualization, please feel free to submit a PR!
pub mod autodiff {
    pub use crate::autodiff::{
        accumulate::*,
        gradient::*,
        graph::*,
        graphviz::*,
        overloading::*,
        overloading::{
            add::*, div::*, f64::*, iter::*, log::*, minmax::*, mul::*, pow::*, statrs::*, sub::*,
        },
        variables::{nalgebra::*, ndarray::*, variable::*},
        vertex::*,
    };

    /// [`Accumulate`] trait.
    pub mod accumulate;
    /// Implements the gradient computation.
    pub mod gradient;
    /// The Graph (aka. tape or Wengert List).
    pub mod graph;
    /// Visualisation of the [`Graph`].
    pub mod graphviz;
    /// Implements [`Vertex`] (nodes) for the `Graph`.
    pub mod vertex;

    /// Operator/function overloading.
    /// This module contains the overloaded operators and primitive functions.
    /// In Griewank and Walther - Evaluating Derivatives, they refer to this
    /// as the "elemental library".
    /// Operations such as `+` and `*` are redefined, along with primitive
    /// functions such as `sin`, `exp`, and `log`.
    /// Each overload has an associated test to ensure functionality.
    pub mod overloading {
        /// Overload the standard addition operator (`+`).
        pub mod add;
        /// Overload the standard division operator (`/`).
        pub mod div;
        /// Overload the standard f64 type methods.
        pub mod f64;
        /// Overload the iterator traits.
        pub mod iter;
        /// Overload the standard logarithm function (`log`).
        pub mod log;
        /// Overload the standard min/max functions (`min` and `max`).
        pub mod minmax;
        /// Overload the standard multiplication operator (`*`).
        pub mod mul;
        /// Overload the power functions.
        pub mod pow;
        /// Overloading functions from `statrs`.
        pub mod statrs;
        /// Overload the standard subtraction operator (`-`).
        pub mod sub;
    }

    /// `Variable`s for `autodiff`.
    pub mod variables {
        /// Implements `Variable`s for `nalgebra`.
        pub mod nalgebra;
        /// Implements `Variable`s for `ndarray`.
        pub mod ndarray;
        /// Base trait for all `Variable`s.
        pub mod variable;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// INSTRUMENTS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Financial instrument types and modules (bonds, options, etc).
/// ### :chart_with_downwards_trend: Bonds <a name="bonds"></a>
///
/// - Prices:
///   - [x] The Vasicek Model
///   - [x] The Cox, Ingersoll, and Ross Model
///   - [x] The Hull–White (One-Factor) Model
///   - [ ] The Rendleman and Bartter Model
///   - [ ] The Ho–Lee Model
///   - [ ] The Black–Derman–Toy Model
///   - [ ] The Black–Karasinski Model
/// - [ ] Duration
/// - [ ] Convexity
///
/// ### :money_with_wings: Option Pricing <a name="options"></a>
///
/// - Closed-form price solutions:
///   - [x] Heston Model
///   - [x] Barrier
///   - [x] European
///   - [x] Greeks/Sensitivities
///   - [x] Lookback
///   - [x] Asian: Continuous Geometric Average
///   - [x] Forward Start
///   - [x] Bachelier and Modified Bachelier
///   - [x] Generalised Black-Scholes-Merton
///   - [ ] Basket
///   - [ ] Rainbow
///   - [ ] American
///
/// - Lattice models:
///   - [x] Binomial Tree (Cox-Ross-Rubinstein)
///
/// The stochastic process generators can be used to price path-dependent options via Monte-Carlo.
///
/// - Monte Carlo pricing:
///   - [x] Lookback
///   - [ ] Asian
///   - [ ] Chooser
///   - [ ] Barrier
///
/// ```rust
/// use RustQuant::options::*;
///
/// fn main() {
///     let VanillaOption = EuropeanOption {
///         initial_price: 100.0,
///         strike_price: 110.0,
///         risk_free_rate: 0.05,
///         volatility: 0.2,
///         dividend_rate: 0.02,
///         time_to_maturity: 0.5,
///     };
///
///     let prices = VanillaOption.price();
///
///     println!("Call price = {}", prices.0);
///     println!("Put price = {}", prices.1);
/// }
/// ```
pub mod instruments {
    pub use crate::instruments::instrument::*;

    /// Base trait for all instruments.
    pub mod instrument;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // BONDS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Bond pricing models.
    pub mod bonds {
        pub use crate::instruments::bonds::{bond::*, cox_ingersoll_ross::*, vasicek::*};

        /// Base bond traits.
        pub mod bond;
        /// Cox-Ingersoll-Ross bond pricing model.
        pub mod cox_ingersoll_ross;
        /// One-factor Hull-White bond pricing model.
        pub mod hull_white;
        /// Vasicek bond pricing model.
        pub mod vasicek;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // OPTION PRICING MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Option pricers and sensitivity functions.
    pub mod options {
        pub use crate::instruments::options::{
            american::*, asian::*, bachelier::*, barrier::*, binary::*, binomial::*,
            black_scholes_merton::*, european::*, forward_start::*, greeks::*, heston::*,
            lookback::*, option::*, power::*,
        };

        /// American option pricers.
        pub mod american;
        /// Asian option pricers.
        pub mod asian;
        /// Bachelier option pricer.
        pub mod bachelier;
        /// Barrier option pricers.
        pub mod barrier;
        /// Binary option pricers.
        pub mod binary;
        /// Binomial option pricers.
        pub mod binomial;
        /// Generalised Black-Scholes-Merton option pricer.
        pub mod black_scholes_merton;
        /// European option pricers.
        pub mod european;
        /// Forward start options pricers.
        pub mod forward_start;
        /// European option Greeks/sensitivities.
        pub mod greeks;
        /// Heston model option pricer.
        pub mod heston;
        /// Lookback option pricers.
        pub mod lookback;
        /// Base option traits.
        pub mod option;
        /// Power option pricers.
        pub mod power;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONEY RELATED ITEMS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Module containing all money related items.
/// This includes currencies, cashflows, exchange rates, and money types,
/// among other things.
pub mod money {
    pub use crate::money::{cashflows::*, currency::*, exchange::*, iso_currencies::*, quotes::*};

    /// Cashflow definitions.
    pub mod cashflows;
    /// Currency data struct.
    pub mod currency;
    /// Currency exchange rate helpers.
    pub mod exchange;
    /// Global currencies defined by ISO 4217.
    pub mod iso_currencies;
    /// Legs (sequence of cashflows).
    pub mod legs;
    /// Quotes (price, yield, etc).
    pub mod quotes;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STATISTICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Statistics related items.
///
/// Probability density/mass functions, distribution functions, characteristic functions, etc.
///
/// - [x] Gaussian
/// - [x] Bernoulli
/// - [x] Binomial
/// - [x] Poisson
/// - [x] Uniform (discrete & continuous)
/// - [x] Chi-Squared
/// - [x] Gamma
/// - [x] Exponential
///
pub mod statistics {
    pub use crate::statistics::{copulas::*, distributions::*, statistic::*};

    /// Base trait for statistics of a collection of data.
    pub mod statistic;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // COPULAS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Copula implementations.
    pub mod copulas {}

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // DISTRIBUTIONS MODULE
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Random variable distributions (PDFs, CDFs, CFs, etc).
    pub mod distributions {
        pub use crate::statistics::distributions::{
            bernoulli::*, binomial::*, chi_squared::*, distribution::*, exponential::*, gamma::*,
            gaussian::*, poisson::*, uniform::*,
        };

        /// Bernoulli distribution.
        pub mod bernoulli;
        /// Binomial distribution.
        pub mod binomial;
        /// Chi-Squared distribution.
        pub mod chi_squared;
        /// Base trait for all distributions.
        pub mod distribution;
        /// Exponential distribution.
        pub mod exponential;
        /// Gamma distribution.
        pub mod gamma;
        /// Gaussian (normal) distribution.
        pub mod gaussian;
        /// Poisson distribution.
        pub mod poisson;
        /// Uniform distribution.
        pub mod uniform;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// DATA MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Data reading and writing utilities.
/// Disabled by default, due to Polars increasing compile times.
///
/// You can:
///
/// - Download data from Yahoo! Finance into a Polars `DataFrame`.
/// - Compute returns on the `DataFrame` you just downloaded.
///
/// ```rust
/// use RustQuant::data::*;
/// use time::macros::date;
///
/// fn main() {
///     // New YahooFinanceData instance.
///     // By default, date range is: 1970-01-01 to present.
///     let mut yfd = YahooFinanceData::new("AAPL".to_string());
///
///     // Can specify custom dates (optional).
///     yfd.set_start_date(time::macros::datetime!(2019 - 01 - 01 0:00 UTC));
///     yfd.set_end_date(time::macros::datetime!(2020 - 01 - 01 0:00 UTC));
///
///     // Download the historical data.
///     yfd.get_price_history();
///
///     // Compute the returns.
///     // Specify the type of returns to compute (Simple, Logarithmic, Absolute)
///     // You don't need to run .get_price_history() first, .compute_returns()
///     // will do it for you if necessary.
///     yfd.compute_returns(ReturnsType::Logarithmic);
///
///     println!("Apple's quotes: {:?}", yfd.price_history);
///     println!("Apple's returns: {:?}", yfd.returns);
/// }
/// ```
///
/// ```bash
/// Apple's quotes: Some(shape: (252, 7)
/// ┌────────────┬───────────┬───────────┬───────────┬───────────┬────────────┬───────────┐
/// │ date       ┆ open      ┆ high      ┆ low       ┆ close     ┆ volume     ┆ adjusted  │
/// │ ---        ┆ ---       ┆ ---       ┆ ---       ┆ ---       ┆ ---        ┆ ---       │
/// │ date       ┆ f64       ┆ f64       ┆ f64       ┆ f64       ┆ f64        ┆ f64       │
/// ╞════════════╪═══════════╪═══════════╪═══════════╪═══════════╪════════════╪═══════════╡
/// │ 2019-01-02 ┆ 38.7225   ┆ 39.712502 ┆ 38.557499 ┆ 39.48     ┆ 1.481588e8 ┆ 37.994499 │
/// │ 2019-01-03 ┆ 35.994999 ┆ 36.43     ┆ 35.5      ┆ 35.547501 ┆ 3.652488e8 ┆ 34.209969 │
/// │ 2019-01-04 ┆ 36.1325   ┆ 37.137501 ┆ 35.950001 ┆ 37.064999 ┆ 2.344284e8 ┆ 35.670372 │
/// │ 2019-01-07 ┆ 37.174999 ┆ 37.2075   ┆ 36.474998 ┆ 36.982498 ┆ 2.191112e8 ┆ 35.590965 │
/// │ …          ┆ …         ┆ …         ┆ …         ┆ …         ┆ …          ┆ …         │
/// │ 2019-12-26 ┆ 71.205002 ┆ 72.495003 ┆ 71.175003 ┆ 72.477501 ┆ 9.31212e7  ┆ 70.798401 │
/// │ 2019-12-27 ┆ 72.779999 ┆ 73.4925   ┆ 72.029999 ┆ 72.449997 ┆ 1.46266e8  ┆ 70.771545 │
/// │ 2019-12-30 ┆ 72.364998 ┆ 73.172501 ┆ 71.305    ┆ 72.879997 ┆ 1.441144e8 ┆ 71.191582 │
/// │ 2019-12-31 ┆ 72.482498 ┆ 73.419998 ┆ 72.379997 ┆ 73.412498 ┆ 1.008056e8 ┆ 71.711739 │
/// └────────────┴───────────┴───────────┴───────────┴───────────┴────────────┴───────────┘)
/// ```
///
/// ```bash
/// Apple's returns: Some(shape: (252, 7)
/// ┌────────────┬────────────┬───────────────┬───────────────┬───────────────┬──────────────┬──────────────┐
/// │ date       ┆ volume     ┆ open_logarith ┆ high_logarith ┆ low_logarithm ┆ close_logari ┆ adjusted_log │
/// │ ---        ┆ ---        ┆ mic           ┆ mic           ┆ ic            ┆ thmic        ┆ arithmic     │
/// │ date       ┆ f64        ┆ ---           ┆ ---           ┆ ---           ┆ ---          ┆ ---          │
/// │            ┆            ┆ f64           ┆ f64           ┆ f64           ┆ f64          ┆ f64          │
/// ╞════════════╪════════════╪═══════════════╪═══════════════╪═══════════════╪══════════════╪══════════════╡
/// │ 2019-01-02 ┆ 1.481588e8 ┆ null          ┆ null          ┆ null          ┆ null         ┆ null         │
/// │ 2019-01-03 ┆ 3.652488e8 ┆ -0.073041     ┆ -0.086273     ┆ -0.082618     ┆ -0.104924    ┆ -0.104925    │
/// │ 2019-01-04 ┆ 2.344284e8 ┆ 0.003813      ┆ 0.019235      ┆ 0.012596      ┆ 0.041803     ┆ 0.041803     │
/// │ 2019-01-07 ┆ 2.191112e8 ┆ 0.028444      ┆ 0.001883      ┆ 0.014498      ┆ -0.002228    ┆ -0.002229    │
/// │ …          ┆ …          ┆ …             ┆ …             ┆ …             ┆ …            ┆ …            │
/// │ 2019-12-26 ┆ 9.31212e7  ┆ 0.000457      ┆ 0.017709      ┆ 0.006272      ┆ 0.019646     ┆ 0.019646     │
/// │ 2019-12-27 ┆ 1.46266e8  ┆ 0.021878      ┆ 0.013666      ┆ 0.011941      ┆ -0.00038     ┆ -0.00038     │
/// │ 2019-12-30 ┆ 1.441144e8 ┆ -0.005718     ┆ -0.004364     ┆ -0.010116     ┆ 0.005918     ┆ 0.005918     │
/// │ 2019-12-31 ┆ 1.008056e8 ┆ 0.001622      ┆ 0.003377      ┆ 0.014964      ┆ 0.00728      ┆ 0.00728      │
/// └────────────┴────────────┴───────────────┴───────────────┴───────────────┴──────────────┴──────────────┘)
/// ```
///
/// ### Read/write data
///
/// ```rust
/// use RustQuant::data::*;
///
/// fn main() {
///     // New `Data` instance.
///     let mut data = Data::new(
///         format: DataFormat::CSV, // Can also be JSON or PARQUET.
///         path: String::from("./file/path/read.csv")
///     )
///
///     // Read from the given file.
///     data.read().unwrap();
///
///     // New path to write the data to.
///     data.path = String::from("./file/path/write.csv")
///     data.write().unwrap();
///
///     println!("{:?}", data.data)
/// }
/// ```
///
#[cfg(feature = "data")]
pub mod data {
    pub use crate::data::{io::*, yahoo::*};

    /// File reading and writing.
    pub mod io;
    /// Yahoo! Finance data reader.
    pub mod yahoo;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MATHEMATICS MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Mathematics related items.
///
/// ### Optimization and Root Finding
///
/// - [x] Gradient Descent
/// - [x] Newton-Raphson
///
/// Note: the reason you need to specify the lifetimes and use the type `Variable` is because the gradient descent optimiser uses the `RustQuant::autodiff` module to compute the gradients. This is a slight inconvenience, but the speed-up is enormous when working with functions with many inputs (when compared with using finite-difference quotients).
///
/// ```rust
/// use RustQuant::optimisation::GradientDescent;
///
/// // Define the objective function.
/// fn himmelblau<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
///     let x = variables[0];
///     let y = variables[1];
///
///     ((x.powf(2.0) + y - 11.0).powf(2.0) + (x + y.powf(2.0) - 7.0).powf(2.0))
/// }
///
/// fn main() {
///     // Create a new GradientDescent object with:
///     //      - Step size: 0.005
///     //      - Iterations: 10000
///     //      - Tolerance: sqrt(machine epsilon)
///     let gd = GradientDescent::new(0.005, 10000, std::f64::EPSILON.sqrt() );
///
///     // Perform the optimisation with:
///     //      - Initial guess (10.0, 10.0),
///     //      - Verbose output.
///     let result = gd.optimize(&himmelblau, &vec![10.0, 10.0], true);
///     
///     // Print the result.
///     println!("{:?}", result.minimizer);
/// }
/// ```
///
/// ### Integration
///
/// - Numerical Integration (needed for Heston model, for example):
///   - [x] Tanh-Sinh (double exponential) quadrature
///
/// ```rust
/// use RustQuant::math::*;
///
/// fn main() {
///     // Define a function to integrate: e^(sin(x))
///     fn f(x: f64) -> f64 {
///         (x.sin()).exp()
///     }
///
///     // Integrate from 0 to 5.
///     let integral = integrate(f, 0.0, 5.0);
///
///     // ~ 7.18911925
///     println!("Integral = {}", integral);
/// }
/// ```
///
/// ### Risk-Reward Metrics
///
/// - [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)
///
pub mod math {
    pub use crate::math::{
        fft::*, integration::*, interpolation::*, optimization::gradient_descent::*,
        optimization::newton_raphson::*, risk_reward::*, sequences::*,
    };

    /// Numerical integration routines.
    /// The primary (useful) integrator is the Tanh-Sinh (double exponential) implementation.
    pub mod integration;

    /// Numerical optimization and root-finding routines.
    pub mod optimization {
        /// Gradient descent optimization.
        pub mod gradient_descent;
        /// Newton-Raphson method.
        pub mod newton_raphson;
    }

    /// Fast fourier transform.
    pub mod fft;
    /// Interpolation routines.
    pub mod interpolation;
    /// Simple risk/reward measures.
    pub mod risk_reward;
    /// Sequences of numbers and associated functions.
    pub mod sequences;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MONTE CARLO SIMULATION AND STOCHASTIC PROCESSES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Monte Carlo engines to simulate stochastic processes.
///
/// The following is a list of stochastic processes that can be generated.
///
/// - Brownian Motions:
///   - Standard Brownian Motion
///     - $dX(t) = dW(t)$
///   - Arithmetic Brownian Motion
///     - $dX(t) = \mu dt + \sigma dW(t)$
///   - Geometric Brownian Motion
///     - $dX(t) = \mu X(t) dt + \sigma X(t) dW(t)$
///   - Fractional Brownian Motion
/// - Cox-Ingersoll-Ross (1985)
///   - $dX(t) = \left[ \theta - \alpha X(t) \right] dt + \sigma \sqrt{r_t} dW(t)$
/// - Ornstein-Uhlenbeck process
///   - $dX(t) = \theta \left[ \mu - X(t) \right] dt + \sigma dW(t)$
/// - Ho-Lee (1986)
///   - $dX(t) = \theta(t) dt + \sigma dW(t)$
/// - Hull-White (1990)
///   - $dX(t) = \left[ \theta(t) - \alpha X(t) \right]dt + \sigma dW(t)$
/// - Extended Vasicek (1990)
///   - $dX(t) = \left[ \theta(t) - \alpha(t) X(t) \right] dt + \sigma dW(t)$
/// - Black-Derman-Toy (1990)
///   - $d\ln[X(t)] = \left[ \theta(t) + \frac{\sigma'(t)}{\sigma(t)}\ln[X(t)] \right]dt + \sigma_t dW(t)$
///
/// ```rust
/// use RustQuant::stochastics::*;
///
/// fn main() {
///     // Create new GBM with mu and sigma.
///     let gbm = GeometricBrownianMotion::new(0.05, 0.9);
///
///     // Generate path using Euler-Maruyama scheme.
///     // Parameters: x_0, t_0, t_n, n, sims, parallel.
///     let output = (&gbm).euler_maruyama(10.0, 0.0, 0.5, 10, 1, false);
///
///     println!("GBM = {:?}", output.paths);
/// }
/// ```
///
pub mod stochastics {
    pub use crate::stochastics::{
        arithmetic_brownian_motion::*, black_derman_toy::*, brownian_motion::*,
        cox_ingersoll_ross::*, extended_vasicek::*, fractional_brownian_motion::*,
        geometric_brownian_motion::*, ho_lee::*, hull_white::*, ornstein_uhlenbeck::*, process::*,
    };

    /// Arithmetic Brownian Motion.
    pub mod arithmetic_brownian_motion;
    /// Black-Derman-Toy short rate model.
    pub mod black_derman_toy;
    /// Standard Brownian Motion.
    pub mod brownian_motion;
    /// Cox-Ingersoll-Ross process.
    pub mod cox_ingersoll_ross;
    /// Extended Vasicek process.
    pub mod extended_vasicek;
    /// Fractional Brownian Motion.
    pub mod fractional_brownian_motion;
    /// Geometric Brownian Motion.
    pub mod geometric_brownian_motion;
    /// Ho-Lee process.
    pub mod ho_lee;
    /// Hull-White model process.
    pub mod hull_white;
    /// Ornstein-Uhlenbeck process.
    pub mod ornstein_uhlenbeck;
    /// Defines `Trajectories` and `StochasticProcess`.
    pub mod process;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TRADING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Trading related items.
pub mod trading {
    /// Contains limit order book implementation
    pub mod limit_order_book;
    /// Order definition.
    pub mod order;
    /// Contains a limit orderbook (LOB) implementation.
    pub mod order_book;
    /// Order lifespan definitions.
    pub mod order_lifespan;
    /// Order side definitions.
    pub mod order_side;
    /// Order types definitions.
    pub mod order_type;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TIME AND DATE MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Time and date functionality.
pub mod time {
    pub use crate::time::{
        calendar::*,
        calendars::{australia::*, austria::*, canada::*, united_kingdom::*, united_states::*},
        constants::*,
        conventions::*,
        daycount::*,
        schedule::*,
    };

    /// Calendar definitions.
    pub mod calendar;
    /// Date/time constants
    pub mod constants;
    /// Day count and business day conventions.
    pub mod conventions;
    /// Daycount definitions.
    pub mod daycount;
    /// Scheduling definitions.
    pub mod schedule;

    /// Calendar definitions for settlement purposes.
    pub mod calendars {
        /// Australian settlement calendar.
        pub mod australia;
        /// Austrian settlement calendar.
        pub mod austria;
        /// Canadian settlement calendar.
        pub mod canada;
        /// UK settlement calendar.
        pub mod united_kingdom;
        /// USA settlement calendar.
        pub mod united_states;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MACHINE LEARNING MODULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Machine learning algorithms. This module relies on the [`nalgebra`] crate.
///
/// ### Regression
///
/// - [x] Linear (using QR or SVD decomposition)
/// - [x] Logistic (via IRLS, adding MLE in the future).
///
/// ### Classification
///
/// - [x] K-Nearest Neighbours
///
pub mod ml {
    pub use crate::ml::{
        activations::*, k_nearest_neighbors::*, linear_regression::*, logistic_regression::*,
    };

    /// Submodule of `ml`: activation functions.
    pub mod activations;
    /// K Nearest Neighbor classifier
    pub mod k_nearest_neighbors;
    /// Linear regression.
    pub mod linear_regression;
    /// Logistic regression.
    pub mod logistic_regression;
}
