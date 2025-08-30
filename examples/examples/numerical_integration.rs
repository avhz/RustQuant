// ANCHOR: numerical_integration

use std::f64::consts::PI;
use RustQuant::math::*;

fn main() {
    // Define a function to integrate:
    // Standard Normal Distribution PDF
    fn f(x: f64) -> f64 {
        (2. * PI).sqrt().recip() * (-0.5 * x.powi(2)).exp()
    }

    // Integrate from -5 to 5.
    // This is: +/- 5 standard deviations from the mean.
    let integral = integrate(f, -5., 5.);

    // Standard Normal Distribution PDF has integral of 1.
    println!("Integral = {}", integral); // ~= 1.0
}

// ANCHOR_END: numerical_integration
