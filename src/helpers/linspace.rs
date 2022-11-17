/// Linspace helper function.
/// Generates a sequence from `a` to `b` with `n` elements.
pub fn linspace(a: f64, b: f64, n: usize) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        v.push(a + i as f64 * ((b - a) / n as f64));
    }
    v
}
