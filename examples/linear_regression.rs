use nalgebra::{DMatrix, DVector};

use RustQuant::ml::*;

fn main() {
    // Test data generated from the following R code:
    //
    // set.seed(2023)
    // x <- matrix(rnorm(12), 4, 3)
    // y <- rexp(4)
    // lm(y ~ x)

    // Create the design matrix.
    #[rustfmt::skip]
    let x = DMatrix::from_row_slice(
        4, // rows
        3, // columns
        &[-0.08378436, -0.6334857, -0.3992666, 
          -0.98294375, 1.0907975, -0.4681231,
          -1.87506732, -0.9137273, 0.3269621, 
          -0.18614466, 1.0016397, -0.4127469],
    );

    // Create the response vector.
    let y = DVector::from_row_slice(&[0.4259088, 0.2617037, 0.4928989, 2.1477291]);

    // Create the linear regression model.
    let linear_regression = LinearRegression {};
    let input = LinearRegressionInput { x, y };
    let output = linear_regression.fit(input);

    // Output from R:
    //
    // Call:
    // lm(formula = y ~ x)
    //
    // Coefficients:
    // (Intercept)           x1           x2           x3
    //       3.682        2.105        1.232        5.759

    println!("Intercept: {:?}", output.intercept);
    println!("Coefficients: {:?}", output.coefficients);
}
