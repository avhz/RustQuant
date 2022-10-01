/// Helper function for tests.
///
/// Checks whether the absolute difference between two floats is within a specified tolerance.
pub fn assert_approx_equal(x: f64, y: f64, tol: f64) {
    assert!((x - y).abs() < tol);
}
