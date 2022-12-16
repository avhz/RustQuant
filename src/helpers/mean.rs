/// Mean of a vector.
pub fn mean(v: &Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
