use serde::{Deserialize, Serialize};

/// Garman-Kohlhagen (1983) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarmanKohlhagen83 {
    s: f64,
    r_d: f64,
    r_f: f64,
    v: f64,
}

impl GarmanKohlhagen83 {
    /// Create a new Garman-Kohlhagen (1983) option pricing parameters.
    pub fn new(s: f64, r_d: f64, r_f: f64, v: f64) -> Self {
        Self { s, r_d, r_f, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.s
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r_d
    }

    #[inline]
    fn b(&self) -> f64 {
        self.r_d - self.r_f
    }
}
