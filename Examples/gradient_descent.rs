use RustQuant::autodiff::*;
use RustQuant::math::optimization::gradient_descent::*;

// Define the objective function
// The reason you need to specify the type of the variables
// is because the function is evaluated using automatic differentiation
// from the `autodiff` module.
// This may be a slight inconvenience, but it is a big benefit when
// dealing with large, complex functions with many inputs.
fn himmelblau<'v>(variables: &[Variable<'v>]) -> Variable<'v> {
    let x = variables[0];
    let y = variables[1];

    (x.powf(2.0) + y - 11.0).powf(2.0) + (x + y.powf(2.0) - 7.0).powf(2.0)
}

fn main() {
    // Create a new GradientDescent object,
    // with a step size of 0.1, a maximum of 100 iterations,
    // and a tolerance of sqrt(machine epsilon).
    let gd = GradientDescent::new(0.1, 100, std::f64::EPSILON.cbrt());

    // Perform the optimisation,
    // starting from the initial guess (5.0, 5.0),
    // with verbose output.
    let result = gd.optimize(&himmelblau, &vec![5.0, 5.0], true);

    // Print the result.
    println!("{:?}", result.minimizer);
}
