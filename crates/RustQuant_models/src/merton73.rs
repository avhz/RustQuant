use serde::{Deserialize, Serialize};

/// Meron (1973) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merton73 {
    s: f64,
    r: f64,
    q: f64,
    v: f64,
}

impl Merton73 {
    /// Create a new Merton (1973) option pricing parameters.
    pub fn new(s: f64, r: f64, q: f64, v: f64) -> Self {
        Self { s, r, q, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.s
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r
    }

    #[inline]
    fn b(&self) -> f64 {
        self.r - self.q
    }
}
