
### :chart_with_downwards_trend: Bonds <a name="bonds"></a>

- Prices:
  - [x] The Vasicek Model
  - [x] The Cox, Ingersoll, and Ross Model
  - [x] The Hull–White (One-Factor) Model
  - [ ] The Rendleman and Bartter Model
  - [ ] The Ho–Lee Model
  - [ ] The Black–Derman–Toy Model
  - [ ] The Black–Karasinski Model
- [ ] Duration
- [ ] Convexity

### :money_with_wings: Option Pricing <a name="options"></a>

- Closed-form price solutions:
  - [x] Heston Model
  - [x] Barrier
  - [x] European
  - [x] Greeks/Sensitivities
  - [x] Lookback
  - [x] Asian: Continuous Geometric Average
  - [x] Forward Start
  - [x] Bachelier and Modified Bachelier
  - [x] Generalised Black-Scholes-Merton
  - [ ] Basket
  - [ ] Rainbow
  - [ ] American

- Lattice models:
  - [x] Binomial Tree (Cox-Ross-Rubinstein)

The stochastic process generators can be used to price path-dependent options via Monte-Carlo.

- Monte Carlo pricing:
  - [x] Lookback
  - [ ] Asian
  - [ ] Chooser
  - [ ] Barrier

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