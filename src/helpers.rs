#![allow(non_snake_case)]
#![deny(missing_docs)]

use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Helper macro for tests.
///
/// Checks whether the absolute difference between two floats is within a specified tolerance.
macro_rules! assert_approx_equal {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y <= $d) && ($y - $x <= $d),
            "\nLeft: \t{}, \nRight: \t{}, \nDelta: \t{}\n",
            $x,
            $y,
            $d
        )
    };
}

/// Linspace helper function.
///
/// Generates a sequence from `a` to `b` with `n` elements.
pub fn linspace(a: f64, b: f64, n: usize) -> Vec<f64> {
    // let mut v: Vec<f64> = vec![0.0; num];
    let mut v: Vec<f64> = Vec::with_capacity(n);

    for i in 0..n {
        // v[i] = a + i as f64 * ((b - a) / num as f64);
        v.push(a + i as f64 * ((b - a) / n as f64));
    }

    v
}

/// Cumulative sum helper function.
///
/// Performs a cumulative sum of a vector.
pub fn cumsum(v1: &[f64]) -> Vec<f64> {
    let v2: Vec<f64> = v1
        .iter()
        .scan(0.0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    v2
}

/// Write vector to file.
pub fn write_vector(v: &[f64]) -> Result<(), Box<dyn Error>> {
    let strings: Vec<String> = v.iter().map(|n| n.to_string()).collect();

    let mut file = File::create("vector.dat")?;
    writeln!(file, "{}", strings.join(", "))?;
    Ok(())
}

/// Compute the mean of a vector.
pub fn mean(v: &Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// Prepare vector for plotting in `plot_vector()`.
fn prepare_vec(vals: Vec<f64>) -> (Vec<(f64, f64)>, f64, f64) {
    let mut out = vec![(0.0, 0.0); vals.len()];
    let mut min = vals[0];
    let mut max = vals[0];

    for i in 0..vals.len() {
        out[i] = (i as f64, vals[i]);
        if vals[i] > max {
            max = vals[i]
        } else if vals[i] < min {
            min = vals[i]
        }
    }
    (out, min, max)
}

/// Plot a vector of values.
pub fn plot_vector(v: Vec<f64>, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (vec2d, min, max) = prepare_vec(v);

    let root = BitMapBackend::new(file, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(file, ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..vec2d.len() as f64, min..max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(vec2d, RED))?.label(file);

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

// ############################################################################
// TESTS
// ############################################################################

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::helpers::assert_approx_equal;

    #[test]
    fn TEST_cumsum() {
        let v: Vec<f64> = vec![0.0, 1.0, 2.0, 11.5];
        assert!(cumsum(&v) == vec![0.0, 1.0, 3.0, 14.5]);
    }
}
