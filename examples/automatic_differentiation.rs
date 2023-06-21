use RustQuant::autodiff::*;

// The general workflow for using the `autodiff` module is as follows:
//
// 1. Create a new tape.
// 2. Assign variables onto the tape.
// 3. Define an expression using the variables.
// 4. Accumulate (differentiate) the expression.
// 5. Profit.

fn main() {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // SIMPLE EXPRESSIONS
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let t = Tape::new();

    let x = t.var(69.);
    let y = t.var(420.);

    let z = x * y + x.sin();

    let grad = z.accumulate();

    println!("z = {}", z.value);
    println!("dz/dx = {}", grad.wrt(&x));
    println!("dz/dy = {}", grad.wrt(&y));
    println!("grad = {:?}", grad.wrt(&[x, y]));

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // BLOCK EXPRESSIONS
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let t = Tape::new();

    let x = t.var(69.);
    let y = t.var(420.);

    let block = {
        let z = x.sin() + y.tan();
        z.exp()
    };

    let grad = block.accumulate();

    println!("f = {}", block.value);
    println!("df/dx = {}", grad.wrt(&x));
    println!("df/dy = {}", grad.wrt(&y));
    println!("grad = {:?}", grad.wrt(&[x, y]));

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // CLOSURES
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let t = Tape::new();

    let x = t.var(1.);
    let y = t.var(2.);

    let closure = || (x * y).cosh() / (x.tanh() * y.sinh());

    let grad = closure().accumulate();

    println!("z = {}", closure().value);
    println!("dz/dx = {}", grad.wrt(&x));
    println!("dz/dy = {}", grad.wrt(&y));
    println!("grad = {:?}", grad.wrt(&[x, y]));

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // PROPER FUNCTIONS.
    //
    // Note that you can also add many variables via vectors, slices, arrays, etc.
    // This is where the `autodiff` crate really shines, as it allows
    // you to differentiate functions of any number of variables and
    // computing gradients for large functions using AD rather than
    // finite-difference quotients is significantly faster and has no error.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Function to differentiate.
    // f = x^y + sin(1) - asinh(z) / 2
    // at x = 3, y = 2, z = 1.
    #[rustfmt::skip]
    fn function<'v>(variables: &[Variable<'v>], constants: &[f64]) -> Variable<'v> {
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
    let result = function(&variables, &constants);
    let gradient = result.accumulate();

    println!("{:?}", gradient.wrt(&variables));
}
