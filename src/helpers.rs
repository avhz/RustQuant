use std::error::Error;
use std::fs::File;
use std::io::Write;
// use std::process;

// ############################################################################
// FUNCTIONS
// ############################################################################

/// Helper function for tests.
///
/// Checks whether the absolute difference between two floats is within a specified tolerance.
pub fn assert_approx_equal(x: f64, y: f64, tol: f64) {
    assert!((x - y).abs() < tol);
}

/// Linspace helper function.
///
/// Generates a sequence from `a` to `b` with `num` elements.
pub fn linspace(a: f64, b: f64, num: usize) -> Vec<f64> {
    let mut v: Vec<f64> = vec![0.0; num];

    for i in 0..num {
        v[i] = a + i as f64 * ((b - a) / num as f64);
    }

    return v;
}

/// Cumulative sum helper function.
///
/// Performs a cumulative sum of a vector.
pub fn cumsum(v1: &Vec<f64>) -> Vec<f64> {
    let v2: Vec<f64> = v1
        .iter()
        .scan(0.0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();

    return v2;
}

/// Write vector to file.
pub fn write_vector(v: &Vec<f64>) -> Result<(), Box<dyn Error>> {
    let strings: Vec<String> = v.iter().map(|n| n.to_string()).collect();

    let mut file = File::create("/tmp/foobar")?;
    writeln!(file, "{}", strings.join(", "))?;
    Ok(())
}

/// Compute the mean of a vector.
pub fn mean(v: &Vec<f64>) -> f64 {
    v.iter().sum::<f64>() as f64 / v.len() as f64
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
