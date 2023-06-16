use RustQuant::autodiff::*;

fn main() {
    // Function to differentiate.
    fn f<'v>(variables: &[Variable<'v>], constants: &[f64]) -> Variable<'v> {
        variables[0].powf(variables[1]) + constants[0].sin() - variables[2].asinh() / constants[1]
    }

    // New tape.
    let tape = Tape::new();

    // Variables and constants.
    let variables = tape.vars(&[3.0, 2.0, 1.0]);
    let constants = [1., 2.];

    // Evaluate and differentiate the function.
    let result = f(&variables, &constants);
    let gradient = result.accumulate();

    println!("{:?}", gradient.wrt(&variables));
}
