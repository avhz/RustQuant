use serde::{Deserialize, Serialize};

/// Asay (1982) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asay82 {
    f: f64,
    v: f64,
}

impl Asay82 {
    /// Create a new Asay (1982) option pricing parameters.
    pub fn new(f: f64, v: f64) -> Self {
        Self { f, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.f
    }

    #[inline]
    fn r(&self) -> f64 {
        0.0
    }

    #[inline]
    fn b(&self) -> f64 {
        0.0
    }
}
