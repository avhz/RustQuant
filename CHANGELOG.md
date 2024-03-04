# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/avhz/RustQuant/compare/v0.1.0...v0.1.1) - 2024-03-04

### Other
- clippy lints.

## [0.0.49](https://github.com/avhz/RustQuant/compare/v0.0.48...v0.0.49) - 2024-02-25

### Other
- Merge branch 'main' of github.com:avhz/RustQuant
- remove more unused dependencies.

## [0.0.48](https://github.com/avhz/RustQuant/compare/v0.0.47...v0.0.48) - 2024-02-24

### Other
- remove unused dependencies.

## [0.0.46](https://github.com/avhz/RustQuant/compare/v0.0.45...v0.0.46) - 2024-02-24

### Added
- added `derive_builder` crate.

### Other
- remove feature flags, move `curves` into `data` module.
- day counting submodule split into files.
- `OffsetDateTime` refactored to `Date`
- Merge branch 'main' of github.com:avhz/RustQuant
- working progress: exponential interpolator

## [0.0.45](https://github.com/avhz/RustQuant/compare/v0.0.44...v0.0.45) - 2024-02-04

### Other
- moved references to separate .md file
- Merge pull request [#181](https://github.com/avhz/RustQuant/pull/181) from maacl/patch-1
- moved stochastic process models to `models` module.

## [0.0.44](https://github.com/avhz/RustQuant/compare/v0.0.43...v0.0.44) - 2024-02-01

### Added
- Implied Volatility implementation (Let's Be Rational - Peter Jaeckel)

### Other
- Merge remote-tracking branch 'refs/remotes/origin/main'

## [0.0.43](https://github.com/avhz/RustQuant/compare/v0.0.42...v0.0.43) - 2024-01-31

### Added
- improve fft fbm generator

### Fixed
- fix naming THRESHOLD
- fix Gaussian cdf test, values as sympy and mpmath fix
- missing docs
- build error
- resolve timedependent issues

### Other
- Slight re-factor of IV module.
- Merge pull request [#174](https://github.com/avhz/RustQuant/pull/174) from lukaskiss222/feature/implied_volatility
- separators
- remove neg before comparator + new test case
- solve most clippy issues
- errorfunctions erfc for cdf
- small fixes
- must use
- apply clippy
- remove dbg
- Add middle box test
- documentation fix
- documentation
- extreme case precision 1e-13
- ITM + extreme case price ~ 1e-19
- OTM passed
- sync merton jump
- Merge pull request [#171](https://github.com/avhz/RustQuant/pull/171) from lukaskiss222/fix-inverse-gaussian-cdf
- Merge pull request [#146](https://github.com/avhz/RustQuant/pull/146) from dancixx/main
- Merge remote-tracking branch 'upstream'

## [0.0.42](https://github.com/avhz/RustQuant/compare/v0.0.41...v0.0.42) - 2023-12-21

### Fixed
- update barrier pricing

## [0.0.41](https://github.com/avhz/RustQuant/compare/v0.0.40...v0.0.41) - 2023-12-16

### Other
- implemented get quote for yahoo finance api

## [0.0.40](https://github.com/avhz/RustQuant/compare/v0.0.39...v0.0.40) - 2023-12-13

### Other
- Merge branch 'main' into interpolation

## [0.0.39](https://github.com/avhz/RustQuant/compare/v0.0.38...v0.0.39) - 2023-11-26

### Fixed
- denmark calender: general prayer day is no longer a public holiday from 2024

### Other
- Merge pull request [#159](https://github.com/avhz/RustQuant/pull/159) from robertchristensen/docs/fix-warnings
- [#142](https://github.com/avhz/RustQuant/pull/142) - tests for denmark calendar
- [#142](https://github.com/avhz/RustQuant/pull/142) - tests for calendars & update Hong Kong calendar
- [#142](https://github.com/avhz/RustQuant/pull/142) - tests for calendars

## [0.0.38](https://github.com/avhz/RustQuant/compare/v0.0.37...v0.0.38) - 2023-11-14

### Other
- [#142](https://github.com/avhz/RustQuant/pull/142) - Add more calendars (Czech Rep., Denmark, Finland, France, Germany, Hong Kong)
- Fix some clippy lints.
- Add ISO code implementations (ISO-4217, ISO-3166, ISO-10383)
- [#142](https://github.com/avhz/RustQuant/pull/142) - Add calendar boilerplate
- [#142](https://github.com/avhz/RustQuant/pull/142) - Add calendars (Argentina, Botswana, Brazil, Chile, China)

## [0.0.37](https://github.com/avhz/RustQuant/compare/v0.0.36...v0.0.37) - 2023-11-12

### Other
- *(https://github.com/avhz/RustQuant/pull/150)* Fix gitignore problem.
- *(https://github.com/avhz/RustQuant/pull/150)* Re-added correct logo.
- *(https://github.com/avhz/RustQuant/pull/150)* Added `Release-plz` workflow.
- *(https://github.com/avhz/RustQuant/pull/150)* Added `Release-plz` workflow.
# Change Log

## October 26 2023

- @avhz: Added Constant Elasticity of Variance (CEV) model process generator.

## October 24 2023

- @kinrezC: Added Merton Jump Diffusion and Brownian Bridge process generators.

## October 23 2023

- @autoparallel: Added `TimeDependent` parameters for all stochastic processes.

## October 17 2023

- @dancixx: Fractional Ornstein-Uhlenbeck process generator.

## October 9 2023

- @aatmunbaxi: KNN classifier.

## 4 October 2023

- @avhz: Added 5% padding to the y-axis in the `plot_vector` macro.
- @avhz: Updated the `curves` module.
- @avhz: Included a yield curve interpolation example [here](./examples/yield_curve_interpolation.rs) with a plot output [here](./images/interpolated_yield_curve.png)

## 3 October 2023

- @avhz: Added `curves` module for rate curves (work in progress).
  - Currently only supports linear interpolation, but will add more interpolation methods soon.

## 1 October 2023

- @avhz: Added basic `Portfolio` interface. Work in progress.
- @avhz: Added power option contract pricer.

## 17 September 2023

- @avhz: More Greeks for BSM model: theta, rho, phi, zeta, strike-delta, strike-gamma.

## 17 September 2023

- @avhz: More Greeks for BSM model: vega, vomma, ultima, vega bleed (adding more soon).

## 16 September 2023

- @avhz: Greeks for BSM model: delta, vanna, charm, lambda, gamma, zomma, speed, colour (adding more soon).
- @avhz: Clean up year fraction computation for some options.

## 15 September 2023

- Generalised Black-Scholes-Merton option pricer by @avhz.
- Bachelier and Modified Bachelier option pricers by @avhz.

## 14 September 2023

- Added `Result` wrapping for `statistics` module.

## 13 September 2023

- Re-licensing from GPL3 to dual Apache2/MIT licenses.

## 29 August 2023

- `plotting` mod deprecated, `plot_vector` is now a macro.
- `Statistic` trait for computing statistics on `Vec<f64>` objects.
- Remove `utilities` module (macros and plotting are in root module now).

## 28 August 2023

- `Cashflow`s can now be added, subtracted, multiplied, and divided.
- `Leg`s are now available, which are collections of `Cashflow`s.

## 27 August 2023

- 150+ currencies added (definitions according to [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217)).
- Basic arithmetic operations (addition, subtraction, multiplication, division) on `Money` objects.
- Fractional Brownian Motion generator.

## 6 July 2023

- Compute returns (simple, arithmetic, absolute) on Yahoo! Finance timeseries downloaded into Polars `DataFrame`s.

## 27 June 2023

- Moved `options` and `bonds` modules to the parent module `instruments`.
- Moved `cashflows`, `quotes` and `currencies` modules to the parent module `money`.

## 21 June 2023

- Updated regression to use QR or SVD decomposition.

## 20 June 2023

- Simple linear regression using `nalgebra`.

## 12 June 2023

- Gradient descent optimizer for functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$.

## 5 June 2023

- Additional stochastic process generators
  - Ho-Lee model
  - Hull-White model
  - Black-Derman-Toy model

## 26 May 2023

- Download time series data from [Yahoo! Finance](https://finance.yahoo.com/).

## 25 May 2023

- Read (write) from (to) `.csv`, `.json`, and `.parquet` files, using [Polars `DataFrames`](https://pola-rs.github.io/polars-book/).

## Older

- Arithmetic Brownian Motion generator.
- Gamma, exponential, and chi-squared distributions.
- Forward start option pricer (Rubinstein 1990 formula).
- Gap option and cash-or-nothing option pricers (currently adding more binary options).
- Asian option pricer (closed-form solution for continuous geometric average).
- Heston Model option pricer (uses the tanh-sinh quadrature numerical integrator).
- Tanh-sinh (double exponential) quadrature for evaluating integrals.
  - Plus other basic numerical integrators (midpoint, trapezoid, Simpson's 3/8).
- Characteristic functions and density functions for common distributions:
  - Gaussian, Bernoulli, Binomial, Poisson, Uniform, Chi-Squared, Gamma, and Exponential.
