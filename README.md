
# `RustQuant`

Rust library for quantitative finance tools.

# Features

Below is a checklist of features that are currently implemented (&#9745;) and that I would like to implement in the future (&#9744;).

## Automatic Differentiation

+ [ ] Forward Mode (using Dual Numbers)
    + Useful when number of outputs is *larger* than number of inputs. 
        + i.e. for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \gg n$
+ [ ] Reverse Mode (using Operator/Function Overloading)
    + Useful when number of outputs is *smaller* than number of inputs. 
        + i.e for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \ll n$

## Bonds

+ [ ] Price
+ [ ] Duration
+ [ ] Convexity

## Mathematics & Statistics

+ [x] Risk-Reward Measures (Sharpe, Treynor, Sortino, etc)
+ [x] Standard Normal Distribution (Distribution/Density functions, and generation of variates)
+ [ ] Interpolation
+ [ ] Newton-Raphson

## Option Pricers

+ [x] Barrier
+ [x] Binomial Option Pricing Model (CRR)
+ [x] European Options
+ [x] Greeks/Sensitivities
+ [ ] Basket
+ [ ] Lookback
+ [ ] Rainbow
+ [ ] American
+ [ ] Asian
+ [ ] Heston Model 

## Stochastic Processes and Short Rate Models

+ Autonomous (time invariant) processes:
    + [x] Brownian Motion
    + [x] Geometric Brownian Motion
    + [x] Cox-Ingersoll-Ross (1985)
    + [x] Ornstein-Uhlenbeck process
    + [ ] Vasicek (1977)
    + [ ] Rendleman-Bartter (1980)
    + [ ] Ho-Lee (1986)
    + [ ] Hull-White (1990)
    + [ ] Black-Derman-Toy (1990)

+ Non-autonomous (time variant) processes:
    + [ ] Merton's (1973)

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
