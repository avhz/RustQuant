# `RustQuant`: Gradient Descent

## Gradient Descent Primer  

We want to implement an algorithm for solving uncostrained optimisation problems of the form:

$$
\min_{x \in \mathbb{R}^n} f(x) \qquad f(x) \in \mathcal{C}^1
$$

when the objective function $f(x)$ and its gradient, $\nabla f(x)$, are known.

We start with an initial guess, $x_0$, and perform the iteration:

$$
x_{k+1} = x_k + \alpha_k d_k = x_k - \alpha_k \nabla f(x_k)
$$

Where:

$$
\begin{aligned}
& d_k = -\nabla f(x_k) && \text{is the descent direction} \\
&\alpha_k && \text{is the step size in iteration $k$} \\
\end{aligned}
$$

This iteration gives us a monotonic sequence which converges to a local minimum, $f(x^*)$, if it exists:

$$
f(x_0) \geq f(x_1) \geq f(x_2) \geq \cdots \geq f(x^*)
$$

The algorithm is repeated until the stationarity condition is fulfilled:

$$
\nabla f(x) = 0
$$

Numerically, this condition is fulfilled if:

$$
\| \nabla f(x_{k+1}) \| \leq \epsilon
$$

Where $\|\cdot\|$ denotes the Euclidean norm:

$$
\|x\| = \sqrt{\langle x,x \rangle}
$$

Or in Rust, something like:

```rust
gradient.iter().map(|&x| x * x).sum::<f64>().sqrt() < std::f64::EPSILON.sqrt()
```

## Himmelblau's Function

We will use the following function as an example:

$$
f(x,y) = (x^2 + y - 11)^2 + (x + y^2 - 7)^2
$$

Which has four local minima:

$$
\begin{aligned}
&f(3.0, 2.0) = 0.0 \\
&f(-2.81, 3.13) = 0.0 \\
&f(-3.78, -3.28) = 0.0 \\
&f(3.58, -1.85) = 0.0 \\
\end{aligned}
$$

To do this, we can use `GradientDescent` from the `optimisation` module:

```rust
use RustQuant::optimisation::GradientDescent;

// Define the objective function
// The reason you need to specify the type of the variables
// is because the function is evaluated using automatic differentiation 
// from the `autodiff` module.
// This may be a slight inconvenience, but it is a big benefit when 
// dealing with large, complex functions with many inputs.
fn himmelblau<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
    let x1 = variables[0];
    let y1 = variables[1];

    ((x1.powf(2.0) + y1 - 11.0).powf(2.0) + (x1 + y1.powf(2.0) - 7.0).powf(2.0))
}

fn main() {
    // Create a new GradientDescent object,
    // with a step size of 0.005, a maximum of 10000 iterations,
    // and a tolerance of sqrt(machine epsilon).
    let gd = GradientDescent::new(0.005, 10000, std::f64::EPSILON.sqrt() );

    // Perform the optimisation,
    // starting from the initial guess (10.0, 10.0),
    // with verbose output.
    let result = gd.optimize_aad(&himmelblau, &vec![10.0, 10.0], true);
    
    // Print the result.
    println!("{:?}", result.minimizer);
}
```
