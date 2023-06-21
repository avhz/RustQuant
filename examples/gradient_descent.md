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

To do this, we can use `GradientDescent` from the `optimisation` module.

See [this example](./examples/gradient_descent.rs) for more details.
