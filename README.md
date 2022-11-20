
# `RustQuant`

Rust library for quantitative finance tools.

**Disclaimer**: This is currently a free-time project and not a professional financial software library. Nothing in this library should be taken as financial advice, and I do not recommend you to use it for trading or making financial decisions. 

# Features

Below is a checklist of features that are:
+ [x] currently implemented, or
+ [ ] I would like to implement in the future.

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

The following is a list of stochastic processes that can be generated, along with notable models they are used in.

+ [x] Brownian Motion
+ [x] Geometric Brownian Motion
    + $dX_t = \mu X_t dt + \sigma X_t dW_t$
    + Models: Black-Scholes (1973), Rendleman-Bartter (1980)
+ [x] Cox-Ingersoll-Ross (1985)
    + $dr_t = (\theta - \alpha r_t)dt + \sqrt{r_t} \sigma dW_t$
+ [x] Ornstein-Uhlenbeck process
    + $dx_t = \theta(\mu - x_t)dt + \sigma dW_t$
    + Models: Vasicek (1977)
+ [ ] Ho-Lee (1986)
    + $dr_t = \theta_t dt + \sigma dW_t$
+ [ ] Hull-White (1990)
    + $dr_t = (\theta - \alpha r_t)dt + \sigma_t dW_t$
+ [ ] Black-Derman-Toy (1990)
    + $d\ln(r) = \left[ \theta_t + \frac{\sigma_t'}{\sigma_t}\ln(r) \right]dt + \sigma_t dW_t$
    + $d\ln(r) = \theta_t dt + \sigma dW_t$
+ [ ] Merton's model (1973)
    + $r_t = r_0 + at + \sigma W_t^*$

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
