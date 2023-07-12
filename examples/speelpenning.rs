// This is "Speelpenning's Example" used in the AD literature to
// demonstrate the efficiency of Algorithmic Adjoint Differentiation.
// Speelpenning helped develop AD in the 1980s, hence the name of the
// function f(x) = x_1 * x_2 * ... * x_n.

use finitediff::*;
use std::time::Instant;
use RustQuant::autodiff::*;

fn main() {
    // 170 is the largest number of variables that can be used since
    // we'll overflow otherwise (we end up with elements > f64::MAX).
    for i in 1..=170 {
        // std::env::set_var("RUST_BACKTRACE", "full");

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Finite Difference Method
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let prod = |x: &Vec<f64>| -> f64 { x.iter().copied().product() };
        let params: Vec<f64> = (1..=i).map(|x| x as f64).collect();

        let start_finite_diff = Instant::now();
        params.central_diff(&prod);
        let duration_fdm = start_finite_diff.elapsed();

        // println!("FDM: {:?}", grad_fdm);

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Algorithmic Adjoint Differentiation
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let t = Graph::new();
        let params = (1..=i).map(|x| t.var(x as f64)).collect::<Vec<_>>();
        let prod = params.iter().copied().product::<Variable>();

        let start_aad = Instant::now();
        prod.accumulate();
        let duration_aad = start_aad.elapsed();

        // println!("AAD: {:?}", grad_aad.wrt(&params));

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // RESULTS
        //
        // AAD quickly becomes much faster than FDM.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        println!(
            "TIMINGS {{ FDM = {} \u{03BC}s, AAD = {} \u{03BC}s }}",
            duration_fdm.as_micros(),
            duration_aad.as_micros()
        )
    }
}
