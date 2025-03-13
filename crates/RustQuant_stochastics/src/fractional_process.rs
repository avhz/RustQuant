use super::process::*;
use crate::monte_carlo::run_monte_carlo;
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::StandardNormal;
use nalgebra::{DMatrix, DVector, Dim, Dyn, RowDVector};
use ndrustfft::{ndfft_par, FftHandler};
use num::{complex::ComplexDistribution, Complex};
use ndarray::{concatenate, prelude::*};
use ndarray_rand::RandomExt;
use rand::Rng;

/// Method used to generate the Fractional Brownian Motion.
#[derive(Debug)]
pub enum FractionalProcessGeneratorMethod {
    /// Chooses the Cholesky decomposition method.
    CHOLESKY,
    /// Chooses the Davies-Harte method.
    FFT,
}

/// Function to run the monte carlo method for the fractional stochastic process.
pub(crate) fn fractional_monte_carlo<T: StochasticProcess>(
    stochastic_process: &T, 
    config: &StochasticProcessConfig, 
    method: &FractionalProcessGeneratorMethod, 
    hurst: f64
) -> Trajectories {
    let fgn = match method {
        FractionalProcessGeneratorMethod::CHOLESKY => fgn_cholesky,
        FractionalProcessGeneratorMethod::FFT => fgn_fft
    };
    run_monte_carlo(stochastic_process, &config, None,Some((fgn, hurst)))
}

/// Autocovariance function (ACF).
fn acf_vector(hurst: f64, n: usize) -> RowDVector<f64> {
    let h = hurst;

    let mut v = RowDVector::<f64>::zeros(n);
    v[0] = 1.0;

    for i in 1..n {
        let idx = i as f64;

        v[i] = 0.5
            * ((idx + 1.0).powf(2.0 * h) - 2.0 * idx.powf(2.0 * h) + (idx - 1.0).powf(2.0 * h));
    }

    v
}

/// Autocovariance matrix.
fn acf_matrix_sqrt(hurst: f64, n: usize) -> DMatrix<f64> {
    let acf_vector = acf_vector(hurst, n);

    let mut m = DMatrix::<f64>::from_diagonal_element_generic(
        Dyn::from_usize(n),
        Dyn::from_usize(n),
        acf_vector[0],
    );

    for i in 1..n {
        for j in 0..i {
            m[(i, j)] = acf_vector[i - j];
        }
    }

    m.cholesky().unwrap().l()
}

/// Fractional Gaussian noise.
pub fn fgn_cholesky(hurst: f64, n: usize, t_n: f64, seed: Option<u64>) -> Vec<f64> {
    let acf_sqrt = acf_matrix_sqrt(hurst, n);
    let noise = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy()
    };

    let noise = noise.sample_iter::<f64, StandardNormal>(StandardNormal)
    .take(n)
    .collect();
    let noise = DVector::<f64>::from_vec(noise);
    let noise = (acf_sqrt * noise).transpose() * (1.0 * t_n / n as f64).powf(hurst);

    noise.data.as_vec().clone()
}

/// Fractional Gaussian noise via FFT.
pub fn fgn_fft(hurst: f64, n: usize, t_n: f64, _: Option<u64>) -> Vec<f64> {
    if !(0.0..=1.0).contains(&hurst) {
        panic!("Hurst parameter must be between 0 and 1");
    }
    let mut r = Array1::linspace(0.0, n as f64, n + 1);
    r.par_mapv_inplace(|x| {
        if x == 0.0 {
            1.0
        } else {
            0.5 * ((x + 1.0).powf(2.0 * hurst) - 2.0 * (x).powf(2.0 * hurst)
                + (x - 1.0).powf(2.0 * hurst))
        }
    });
    let r = concatenate(
        Axis(0),
        #[allow(clippy::reversed_empty_ranges)]
        &[r.view(), r.slice(s![..;-1]).slice(s![1..-1]).view()],
    )
    .unwrap();
    let mut data = Array1::<Complex<f64>>::zeros(r.len());
    for (i, v) in r.iter().enumerate() {
        data[i] = Complex::new(*v, 0.0);
    }
    let r_fft = FftHandler::new(r.len());
    let mut sqrt_eigenvalues = Array1::<Complex<f64>>::zeros(r.len());

    ndfft_par(&data, &mut sqrt_eigenvalues, &r_fft, 0);

    sqrt_eigenvalues.par_mapv_inplace(|x| Complex::new((x.re / (2.0 * n as f64)).sqrt(), x.im));

    let rnd = Array1::<Complex<f64>>::random(
        2 * n,
        ComplexDistribution::new(StandardNormal, StandardNormal),
    );
    let fgn = &sqrt_eigenvalues * &rnd;
    let fft_handler = FftHandler::new(2 * n);
    let mut fgn_fft = Array1::<Complex<f64>>::zeros(2 * n);

    ndfft_par(&fgn, &mut fgn_fft, &fft_handler, 0);

    let fgn = fgn_fft
        .slice(s![1..n + 1])
        .mapv(|x: Complex<f64>| (x.re * (n as f64).powf(-hurst)) * t_n.powf(hurst));
    fgn.to_vec()
}

#[cfg(test)]
mod test_fractional_brownian_motion {
    use super::*;
    use RustQuant_ml::{Decomposition, LinearRegressionInput};
    use nalgebra::{DMatrix, DVector};

    fn higuchi_fd(x: &Vec<f64>, kmax: usize) -> f64 {
        let n_times = x.len();

        let mut lk = vec![0.0; kmax];
        let mut x_reg = vec![0.0; kmax];
        let mut y_reg = vec![0.0; kmax];

        for k in 1..=kmax {
            let mut lm = vec![0.0; k];

            for m in 0..k {
                let mut ll = 0.0;
                let n_max = ((n_times - m - 1) as f64 / k as f64).floor() as usize;

                for j in 1..n_max {
                    ll += (x[m + j * k] - x[m + (j - 1) * k]).abs();
                }

                ll /= k as f64;
                ll *= (n_times - 1) as f64 / (k * n_max) as f64;
                lm[m] = ll;
            }

            lk[k - 1] = lm.iter().sum::<f64>() / k as f64;
            x_reg[k - 1] = (1.0 / k as f64).ln();
            y_reg[k - 1] = lk[k - 1].ln();
        }

        let x_reg = DMatrix::from_vec(kmax, 1, x_reg);
        let y_reg = DVector::from_vec(y_reg);
        let linear_regression = LinearRegressionInput::new(x_reg, y_reg);
        let regression_result = linear_regression.fit(Decomposition::None).unwrap();

        regression_result.coefficients[0]
    }

    #[test]
    fn test_chol() {
        let hursts = vec![0.1, 0.3, 0.5, 0.7, 0.9];

        for hurst in hursts {
            let fbm = fgn_cholesky(0.7, 1000, 1.0, None);
            let higuchi_fd = higuchi_fd(&fbm.to_vec(), 10);
            let est_hurst = 2.0 - higuchi_fd;
            print!("hurst: {}, higuchi_fd: {}\n", hurst, est_hurst);
            assert!(est_hurst - hurst < 0.05);
        }
    }

}
