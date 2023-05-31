# RustQuant: Automatic Differentiation

Automatic differentiation is a technique for evaluating derivatives of functions specified by computer programs. It is accurate to machine precision, as it does not incur truncation or round-off errors.

## Simple Expressions

```rust
use RustQuant::autodiff::*;

fn main() {
    let t = Tape::new();

    let x = t.var(0.5);
    let y = t.var(4.2);

    let z = x * y + x.sin();

    let grad = z.accumulate();

    println!("z = {}", z.value);
    println!("dz/dx = {}", grad.wrt(&x));
    println!("dz/dy = {}", grad.wrt(&y));
    println!("grad = {}", grad.wrt(&[x, y]));
}
```

## Block Assignments

```rust
use RustQuant::autodiff::*;

fn main() {
    let t = Tape::new();

    let x = t.var(0.5);
    let y = t.var(4.2);

    let f = {
        let z = x.sin() + y.tan();
        z.exp()
    };

    let grad = f.accumulate();

    println!("f = {}", f.value);
    println!("df/dx = {}", grad.wrt(&x));
    println!("df/dy = {}", grad.wrt(&y));
    println!("grad = {}", grad.wrt(&[x, y]));
}
```

## Closures

```rust
use RustQuant::autodiff::*;

fn main() {
    let t = Tape::new();

    let x = t.var(0.5);
    let y = t.var(4.2);

    let f = || (x * y).cosh() / (x.tanh() * y.sinh());

    let grad = f.accumulate();

    println!("z = {}", f.value);
    println!("dz/dx = {}", grad.wrt(&x));
    println!("dz/dy = {}", grad.wrt(&y));
    println!("grad = {}", grad.wrt(&[x, y]));
}
```

## Functions

Note that you can also add multiple variables via vectors, slices, arrays, etc.

This is where the `autodiff` crate really shines, as it allows you to differentiate functions of any number of variables and computing gradients for large functions using AD rather than finite-difference quotients is significantly faster and has no error.

```rust
use RustQuant::autodiff::*;

fn main() {
    // Function to differentiate.
    fn f<'v>(variables: &[Variable<'v>], constants: &[f64]) -> Variable<'v> {
        variables[0].powf(variables[1]) + 
        constants[0].sin() - 
        variables[2].asinh() / constants[1]
    }

    // New tape.
    let tape = Tape::new();

    // Variables and constants.
    let variables = tape.vars(&[3.0, 2.0, 1.0]);
    let constants = [1., 2.];

    // Evaluate and differentiate the function.
    let result = diff_fn(&variables, &constants);
    let gradient = result.accumulate();

    println!("{:?}", gradient.wrt(&variables));
    println!("{:?}", gradient);
}
```