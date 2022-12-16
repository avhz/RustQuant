/// Cumulative sum of a vector helper function.
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
