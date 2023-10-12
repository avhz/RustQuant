
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

A Rust library for quantitative finance tools.

:dart: I want to hit a stable `v0.1.0` by the end of 2023, so any feedback, suggestions, or contributions are strongly welcomed!

<div align="center">

| Email                        | Discord                         | Latest Changes              |
|:----------------------------:|:-------------------------------:|:---------------------------:|
| <RustQuantContact@gmail.com> | <https://discord.gg/tQcM77h8vr> | [Changelog](./CHANGELOG.md) |

</div>

## Modules

| Module | Description |
|--------|-------------|
| [`autodiff`](https://docs.rs/RustQuant/latest/RustQuant/autodiff/index.html) | Algorithmic adjoint differentiation for efficiently computing gradients of scalar output functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$. |
| [`curves`](https://docs.rs/RustQuant/latest/RustQuant/curves/index.html) | Curves and surfaces, such as the yield curve and volatility surface. |
| [`data`](https://docs.rs/RustQuant/latest/RustQuant/data/index.html) | Methods for reading and writing data from/to various sources (CSV, JSON, Parquet). Can also download data from Yahoo! Finance. |
| [`error`](https://docs.rs/RustQuant/latest/RustQuant/error/index.html) | RustQuant error handling module. |
| [`instruments`](https://docs.rs/RustQuant/latest/RustQuant/instruments/index.html) | Various implementations for instruments like `Bonds` and `Options`, and the pricing of them. Others coming in the future (swaps, futures, CDSs, etc). |
| [`math`](https://docs.rs/RustQuant/latest/RustQuant/math/index.html) | Fast Fourier Transform (FFT), numerical integration (double-exponential quadrature), optimisation/root-finding (gradient descent, Newton-Raphson), and risk-reward metrics. Also some sequence methods such as `linspace` and `cumsum`. |
| [`ml`](https://docs.rs/RustQuant/latest/RustQuant/ml/index.html) | Currently only linear and logistic regression, along with k-nearest neighbours classification are implemented. More to come in the future. |
| [`macros`](https://docs.rs/RustQuant/latest/RustQuant/macros/index.html) | Currently only `plot_vector!()` and `assert_approx_equal!()`. |
| [`money`](https://docs.rs/RustQuant/latest/RustQuant/money/index.html) | Implementations for `Cashflows`, `Currencies`, and `Quotes`, and similar objects. |
| [`portfolio`](https://docs.rs/RustQuant/latest/RustQuant/portfolio/index.html) | |
| [`statistics`](https://docs.rs/RustQuant/latest/RustQuant/statistics/index.html) | PDFs, CDFs, MGFs, CFs, and other distribution related functions for common distributions. |
| [`stochastics`](https://docs.rs/RustQuant/latest/RustQuant/stochastics/index.html) | Stochastic process generators for Brownian Motion (standard, arithmetic, fractional, and geometric) and various short-rate models (CIR, OU, Vasicek, Hull-White, etc). Multi-factor processes coming shortly. |
| [`time`](https://docs.rs/RustQuant/latest/RustQuant/time/index.html) | Time and date functionality, such as `DayCounter`, calendars, constants, conventions, schedules, etc. |
| [`trading`](https://docs.rs/RustQuant/latest/RustQuant/trading/index.html) | |

<!-- 
#### [`autodiff`](https://docs.rs/RustQuant/latest/RustQuant/autodiff/index.html)

Algorithmic adjoint differentiation for efficiently computing gradients of scalar output functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$.

#### [`curves`](https://docs.rs/RustQuant/latest/RustQuant/curves/index.html)

Curves and surfaces, such as the yield curve and volatility surface.

#### [`data`](https://docs.rs/RustQuant/latest/RustQuant/data/index.html)

Methods for reading and writing data from/to various sources (CSV, JSON, Parquet). Can also download data from Yahoo! Finance.

#### [`error`](https://docs.rs/RustQuant/latest/RustQuant/error/index.html)

RustQuant error handling module.

#### [`instruments`](https://docs.rs/RustQuant/latest/RustQuant/instruments/index.html)

Various implementations for instruments like `Bonds` and `Options`, and the pricing of them. Others coming in the future (swaps, futures, CDSs, etc).

#### [`math`](https://docs.rs/RustQuant/latest/RustQuant/math/index.html)

Fast Fourier Transform (FFT), numerical integration (double-exponential quadrature), optimisation/root-finding (gradient descent, Newton-Raphson), and risk-reward metrics. Also some sequence methods such as `linspace` and `cumsum`.

#### [`ml`](https://docs.rs/RustQuant/latest/RustQuant/ml/index.html)

Currently only linear and logistic regression, along with k-nearest neighbours classification are implemented. More to come in the future.

#### [`macros`](https://docs.rs/RustQuant/latest/RustQuant/macros/index.html)

Currently only `plot_vector!()` and `assert_approx_equal!()`.

#### [`money`](https://docs.rs/RustQuant/latest/RustQuant/money/index.html)

Implementations for `Cashflows`, `Currencies`, and `Quotes`, and similar objects.

#### [`portfolio`](https://docs.rs/RustQuant/latest/RustQuant/portfolio/index.html)

#### [`statistics`](https://docs.rs/RustQuant/latest/RustQuant/statistics/index.html)

PDFs, CDFs, MGFs, CFs, and other distribution related functions for common distributions.

#### [`stochastics`](https://docs.rs/RustQuant/latest/RustQuant/stochastics/index.html)

Stochastic process generators for Brownian Motion (standard, arithmetic, fractional, and geometric) and various short-rate models (CIR, OU, Vasicek, Hull-White, etc). Multi-factor processes coming shortly.

#### [`time`](https://docs.rs/RustQuant/latest/RustQuant/time/index.html)

Time and date functionality, such as `DayCounter`, calendars, constants, conventions, schedules, etc.

#### [`trading`](https://docs.rs/RustQuant/latest/RustQuant/trading/index.html) -->

## Examples

See [/examples](./examples) for more details. Run them with:

```bash
cargo run --example <example>
```

## :book: References

- John C. Hull - *Options, Futures, and Other Derivatives*
- Damiano Brigo & Fabio Mercurio - *Interest Rate Models - Theory and Practice (With Smile, Inflation and Credit)*
- Paul Glasserman - *Monte Carlo Methods in Financial Engineering*
- Andreas Griewank & Andrea Walther - *Evaluating Derivatives - Principles and Techniques of Algorithmic Differentiation*
- Steven E. Shreve - *Stochastic Calculus for Finance II: Continuous-Time Models*
- Espen Gaarder Haug - *Option Pricing Formulas*
- Antoine Savine - *Modern Computational Finance: AAD and Parallel Simulations*

> [!NOTE]  
> Disclaimer: This is currently a free-time project and not a professional financial software library. Nothing in this library should be taken as financial advice, and I do not recommend you to use it for trading or making financial decisions.
