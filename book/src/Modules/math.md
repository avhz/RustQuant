# `math`

## Numerical Integration

Here we integrate the Gaussian density, which integrates to 1:

\\[
\int_{\mathbb{R}} \frac{1}{\sqrt{2  \pi}} e^{\left( -\frac{x^2}{2} \right)} dx = 1    
\\]

```rust,noplayground
{{#include ../../../examples/examples/numerical_integration.rs:numerical_integration}}
```