
Currently only gradients can be computed. Suggestions on how to extend the functionality to Hessian matrices are definitely welcome.

Additionally, only functions $f: \mathbb{R}^n \rightarrow \mathbb{R}$ (scalar output) are supported. However, you can manually apply the differentiation to multiple functions that could represent a vector output.

- [x] Reverse (Adjoint) Mode
  - Implementation via Operator and Function Overloading.
  - Useful when number of outputs is *smaller* than number of inputs.
    - i.e for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \ll n$
- [ ] Forward (Tangent) Mode
  - Implementation via Dual Numbers.
  - Useful when number of outputs is *larger* than number of inputs.
    - i.e. for functions $f:\mathbb{R}^n \rightarrow \mathbb{R}^m$, where $m \gg n$

```rust
use RustQuant::autodiff::*;

fn main() {
    // Create a new Graph to store the computations.
    let g = Graph::new();

    // Assign variables.
    let x = g.var(69.);
    let y = g.var(420.);

    // Define a function.
    let f = {
      let a = x.powi(2);
      let b = y.powi(2);

      a + b + (x * y).exp()
    };

    // Accumulate the gradient.
    let gradient = f.accumulate();

    println!("Function = {}", f);
    println!("Gradient = {:?}", gradient.wrt([x, y]));
}
```

You can also generate Graphviz (dot) code to visualize the computation graphs:

```rust
println!("{}", graphviz(&graph, &variables));
```  

The computation graph from computing Black-Scholes Greeks is:

![Black-Scholes Greeks tape.](./images/black_scholes_tape.png)

It is clearly a work in progress, but gives a general idea of how the computation graph is structured.

If you want to improve the visualization, please feel free to submit a PR!