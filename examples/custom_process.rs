use std::f64::consts::PI;
use RustQuant::{plot_vector, stochastics::*, Sequence};

fn main() {
    // Create an x axis.
    let x = f64::linspace(-5., 5., 1000);

    // Create a y axis.
    let y = x.iter().map(|&t| ricker_wavelet(t, 1.)).collect::<Vec<_>>();

    // Plot the wavelet.
    plot_vector!(y, "./images/ricker_wavelet.png");

    // Create a custom Ito process instance.
    let custom_process = CustomItoProcess {
        mu: 0.0,
        sigma: 1.0,
    };

    // Generate a path.
    let output = custom_process.euler_maruyama(0.01, 0.0, 10.0, 100, 1, false);

    // Plot the path.
    println!("output.paths.len() = {}", output.paths.len());
    println!("output.paths[0] = {:?}", output.paths[0]);
    plot_vector!(output.paths[0], "./images/ricker_wavelet_process.png");
}

struct CustomItoProcess {
    /// The drift coefficient ($\mu$).
    pub mu: f64,

    /// The volatility coefficient ($\sigma$).
    pub sigma: f64,
}

impl StochasticProcess for CustomItoProcess {
    fn drift(&self, x: f64, _t: f64) -> f64 {
        self.mu * x
    }

    fn diffusion(&self, x: f64, t: f64) -> f64 {
        // sigma X_t dW_t
        // self.sigma * x
        x * ricker_wavelet(t - 5., self.sigma)
    }

    fn jump(&self, _x: f64, _t: f64) -> f64 {
        0.0
    }
}

// The Ricker Wavelet.
// From Wikipedia: https://en.wikipedia.org/wiki/Mexican_hat_wavelet
//
// """
// The Ricker Wavelet is the negative normalized second derivative of a Gaussian
// function, i.e., up to scale and normalization, the second Hermite function.
// It is a special case of the family of continuous wavelets (wavelets used in a
// continuous wavelet transform) known as Hermitian wavelets.
// The Ricker wavelet is frequently employed to model seismic data, and as a
// broad spectrum source term in computational electrodynamics. It is usually
// only referred to as the Mexican hat wavelet in the Americas, due to taking
// the shape of a sombrero when used as a 2D image processing kernel.
// It is also known as the Marr wavelet for David Marr.
// """
fn ricker_wavelet(t: f64, sigma: f64) -> f64 {
    let coef = 2. * f64::sqrt(3. * sigma) * f64::powf(PI, 0.25);
    let term = 1. - f64::powi(t / sigma, 2);
    let expo = f64::exp(-0.5 * f64::powi(t / sigma, 2));

    coef * term * expo
}
