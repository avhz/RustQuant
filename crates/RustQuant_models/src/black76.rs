use serde::{Deserialize, Serialize};

/// Black (1976) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Black76 {
    f: f64,
    r: f64,
    v: f64,
}

impl Black76 {
    /// Create a new Black (1976) option pricing parameters.
    pub fn new(f: f64, r: f64, v: f64) -> Self {
        Self { f, r, v }
    }

    #[inline]
    fn s(&self) -> f64 {
        self.f
    }

    #[inline]
    fn r(&self) -> f64 {
        self.r
    }

    #[inline]
    fn b(&self) -> f64 {
        0.0
    }
}
