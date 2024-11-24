# `autodiff`

The `autodiff` module can be used to differentiate scalar output functions of the form:

\\[f : \mathbb{R}^n \rightarrow \mathbb{R} \\]


Simple expressions are differentiated as follows:

```rust,noplayground
{{#include ../../../examples/examples/autodiff.rs:simple_expressions}}
```

Block expressions are also supported:

```rust,noplayground
{{#include ../../../examples/examples/autodiff.rs:block_expressions}}
```

Closures and functions are also supported:

```rust,noplayground
{{#include ../../../examples/examples/autodiff.rs:closures}}
```

```rust,noplayground
{{#include ../../../examples/examples/autodiff.rs:functions}}
```

