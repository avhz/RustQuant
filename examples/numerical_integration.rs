use RustQuant::math::*;

fn main() {
    // Define a function to integrate: e^(sin(x))
    fn f(x: f64) -> f64 {
        (x.sin()).exp()
    }

    // Integrate from 0 to 5.
    let integral = integrate(f, 0.0, 5.0);

    // ~ 7.18911925
    println!("Integral = {}", integral);
}
