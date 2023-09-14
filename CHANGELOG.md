# Change Log

## 14 September 2023
- Added `Result` wrapping for `statistics` module.

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
