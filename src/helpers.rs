pub fn assert_approx_equal(x: f64, y: f64, tol: f64) {
    assert!((x - y).abs() < tol);
}
