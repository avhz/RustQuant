/// Method of averaging (arithmetic, geometric, or harmonic).
pub enum MeanType {
    /// Arithmetic mean: `sum(x_i) / n`
    Arithmetic,
    /// Geometric mean: `prod(x_i)^(1/n)`
    Geometric,
    /// Harmonic mean: `n / sum(1/x_i)`
    Harmonic,
}

/// Mean of a vector.
pub fn mean(v: &Vec<f64>, mean_type: MeanType) -> f64 {
    assert!(!v.is_empty(), "Vector must have at least one element.");

    match mean_type {
        MeanType::Arithmetic => v.iter().sum::<f64>() / v.len() as f64,
        MeanType::Geometric => v.iter().product::<f64>().powf(1.0 / v.len() as f64),
        MeanType::Harmonic => v.len() as f64 / v.iter().map(|x| 1.0 / x).sum::<f64>(),
    }
}
