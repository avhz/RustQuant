// This example is a demonstration of implementing a custom stochastic process
// and using it to generate a path.
// You can create your own stochastic process by creating a struct that
// implements the `StochasticProcess` trait.
//
// The demonstration uses the Rickter Wavelet function in the diffusion
// coefficient.
//
// From Wikipedia: https://en.wikipedia.org/wiki/Ricker_wavelet
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

use std::f64::consts::PI;
use RustQuant::{
    math::Sequence, prelude::StochasticScheme, stochastics::{process::StochasticProcess, StochasticProcessConfig}, utils::plot_vector
};

fn main() {
    // Create an x-axis.
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

    // Generate a path and plot it.
    let config = StochasticProcessConfig::new(
        0.01, 0.0, 10.0, 500, StochasticScheme::EulerMaruyama, 1, false, None
    );
    let output = custom_process.generate(&config);
    plot_vector!(output.paths[0], "./images/ricker_wavelet_process.png");
}

// Your custom stochastic process parameters.
// In theory you can have as many parameters as you want,
// but you will need to implement the `StochasticProcess` trait.
struct CustomItoProcess {
    /// The drift coefficient ($\mu$).
    pub mu: f64,

    /// The volatility coefficient ($\sigma$).
    pub sigma: f64,
}

// dX(t)
// = μ(X, t) dt + σ(X, t) dW(t)
// = μ * X dt + φ(t - 5, σ) * X dW(t)
impl StochasticProcess for CustomItoProcess {
    // μ(X, t) = μ * X
    fn drift(&self, x: f64, _t: f64) -> f64 {
        self.mu * x
    }

    // σ(X, t) = φ(t - 5, σ) * X
    fn diffusion(&self, x: f64, t: f64) -> f64 {
        x * ricker_wavelet(t - 5., self.sigma)
    }

    fn jump(&self, _x: f64, _t: f64) -> Option<f64> {
        None
    }
}

// The Ricker Wavelet.
fn ricker_wavelet(t: f64, sigma: f64) -> f64 {
    2. * f64::sqrt(3. * sigma)
        * f64::powf(PI, 0.25)
        * (1. - f64::powi(t / sigma, 2))
        * f64::exp(-0.5 * f64::powi(t / sigma, 2))
}
