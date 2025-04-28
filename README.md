![](./images/logo.png)

<p align="center">
    <a href="#license" alt="license">
        <img alt="License" src="https://img.shields.io/badge/Dual_License-MIT_and_Apache_2.0-black?logo=apache"></a>
    <a href="#version" alt="version">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/RustQuant?logo=rust&color=black"></a>
    <a href="#downloads" alt="downloads">
        <!-- <img alt="Crates.io" src="https://img.shields.io/crates/d/RustQuant?logo=rust&color=black"></a> -->
        <img alt="Crates.io User Total Downloads" src="https://img.shields.io/crates/udt/178069">
    <a href="#stars" alt="stars">
        <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/avhz/RustQuant?logo=github&color=black"></a>
</p>

<p align="center">
    <a href="#build" alt="build">
        <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/avhz/RustQuant/cargo_build.yml"></a>
    <a href="#codecov" alt="codecov">
        <img alt="Codecov" src="https://img.shields.io/codecov/c/gh/avhz/RustQuant"></a>
    <a href="#deps" alt="deps">
        <img alt="Dependencies" src="https://deps.rs/repo/github/avhz/RustQuant/status.svg"></a>
    <a href="https://app.fossa.com/projects/git%2Bgithub.com%2Favhz%2FRustQuant?ref=badge_shield" alt="FOSSA Status"><img src="https://app.fossa.com/api/projects/git%2Bgithub.com%2Favhz%2FRustQuant.svg?type=shield"/></a>
</p>

<p align="center">
    <a href="https://discord.com/invite/gMdv8Hpuwr" alt="discord">
        <img alt="Discord" src="https://img.shields.io/discord/1146771658082881636?logo=discord"></a>
</p>

A Rust library for quantitative finance.

:dart: If you are an experienced quant developer in any language and would like to help out, feel free to contact me!

<div align="center">

| Email                        | Discord                         | Latest Changes              |
|:----------------------------:|:-------------------------------:|:---------------------------:|
| <RustQuantContact@gmail.com> | <https://discord.gg/gMdv8Hpuwr> | [Changelog](./CHANGELOG.md) |

</div>



## Modules

| Module | Description |
|--------|-------------|
| [`autodiff`](https://docs.rs/RustQuant/latest/RustQuant/autodiff/index.html) | Algorithmic adjoint differentiation (AAD) for efficiently computing gradients of scalar output functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$. |
| [`cashflows`](https://docs.rs/RustQuant/latest/RustQuant/cashflows/index.html) | Implementations for `Cashflows` and `Quotes`, and similar types. |
| [`data`](https://docs.rs/RustQuant/latest/RustQuant/data/index.html) | Data types that can be used for pricing and similar tasks (curves, term-structures, surfaces, etc). Methods for reading and writing data from/to various sources (CSV, JSON, Parquet). Can also download data from Yahoo! Finance. |
| [`error`](https://docs.rs/RustQuant/latest/RustQuant/error/index.html) | RustQuant error handling module. |
| [`instruments`](https://docs.rs/RustQuant/latest/RustQuant/instruments/index.html) | Implementations for financial instruments like `Bonds`, `Options`, and `Money`, including their pricing. Future additions will include swaps, futures, CDSs, etc. |
| [`iso`](https://docs.rs/RustQuant/latest/RustQuant/iso/index.html) | A few ISO code implementations: [ISO-4217](https://www.iso.org/iso-4217-currency-codes.html) (currency codes), [ISO-3166](https://www.iso.org/iso-3166-country-codes.html) (country codes), [ISO-10383](https://www.iso20022.org/market-identifier-codes) (market identifier codes). |
| [`math`](https://docs.rs/RustQuant/latest/RustQuant/math/index.html) | Statistical distributions and their related functions (PDF, CDF, CF, etc), Fast Fourier Transform (FFT), numerical integration (double-exponential quadrature), optimisation/root-finding (gradient descent, Newton-Raphson), and risk-reward metrics. Also some sequence methods such as `linspace` and `cumsum`. |
| [`ml`](https://docs.rs/RustQuant/latest/RustQuant/ml/index.html) | Currently only linear and logistic regression, along with k-nearest neighbours classification are implemented. More to come in the future. |
| [`macros`](https://docs.rs/RustQuant/latest/RustQuant/macros/index.html) | Currently only `plot_vector!()` and `assert_approx_equal!()`. |
| [`models`](https://docs.rs/RustQuant/latest/RustQuant/models/index.html) | Various models commonly used in quantitative finance, such as the various forms of Brownian Motion, short rate models, curve models, etc. |
| [`portfolio`](https://docs.rs/RustQuant/latest/RustQuant/portfolio/index.html) | Implementation of a portfolio type, which is a collection (`HashMap`) of `Position`s. |
| [`stochastics`](https://docs.rs/RustQuant/latest/RustQuant/stochastics/index.html) | Stochastic process generators for Brownian Motion (standard, arithmetic, fractional, and geometric) and various short-rate models (CIR, OU, Vasicek, Hull-White, etc). |
| [`time`](https://docs.rs/RustQuant/latest/RustQuant/time/index.html) | Time and date functionality, such as `DayCounter`, calendars, constants, conventions, schedules, etc. |
| [`trading`](https://docs.rs/RustQuant/latest/RustQuant/trading/index.html) | Currently only a basic limit order book (LOB). Hopefully adding additional trading tools in the future. |

## Examples

See [/examples](./examples) for various uses of RustQuant. You can run them with:

```bash
cargo run --example <example>
```

> [!NOTE]  
> Disclaimer: This is currently a free-time project and not a professional financial software library. Nothing in this library should be taken as financial advice, and I do not recommend you to use it for trading or making financial decisions.

<div align="center">
    
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Favhz%2FRustQuant.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Favhz%2FRustQuant?ref=badge_large)

</div>
