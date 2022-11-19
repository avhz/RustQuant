
# `RustQuant`

Rust library for quantitative finance tools.

# Features

Below is a checklist of features that are currently implemented ([x]) and that I would like to implement in the future ([ ]).

## Automatic Differentiation

+ [ ] Forward Mode 
+ [ ] Reverse Mode

## Bonds

+ [ ] Price
+ [ ] Duration
+ [ ] Convexity

## Mathematics & Statistics

+ [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)
+ [x] Standard Normal Distribution (Distribution and Density functions)
+ [ ] Interpolation
+ [ ] Newton-Raphson

## Option Pricers

+ [x] Barrier
+ [x] Binomial Option Pricing Model (CRR)
+ [x] European Options
+ [x] Greeks/Sensitivities
+ [ ] American
+ [ ] Asian
+ [ ] Heston Model 

## Stochastic Processes

+ [x] Brownian Motion
+ [x] Geometric Brownian Motion
+ [x] Cox-Ingersoll-Ross
+ [x] Ornstein-Uhlenbeck process

## Short-Rate Models

+ [ ] Merton's (1973)
+ [ ] Vasicek (1977)
+ [ ] Rendleman-Bartter (1980)
+ [ ] Cox-Ingersoll-Ross (1985)
+ [ ] Ho-Lee (1986)
+ [ ] Hull-White (1990)
+ [ ] Black-Derman-Toy (1990)

## Helper Functions/Macros

+ [x] Plot a vector.
+ [x] Write vector to file.
+ [x] Cumulative sum of vector.
+ [x] Linearly spaced sequence.
+ [x] `assert_approx_equal!`


# How-tos:

## Compute gradients:

```rust
todo!()
```

## Price options:

```rust
todo!()
```

## Generate stochastic processes:

```rust
todo!()
```
