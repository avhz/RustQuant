use serde::{Deserialize, Serialize};

/// Black-Scholes (1973) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackScholes73 {
    s: f64,
    r: f64,
    v: f64,
}

impl BlackScholes73 {
    /// Create a new Black-Scholes (1973) option pricing parameters.
    pub fn new(s: f64, r: f64, v: f64) -> Self {
        Self { s, r, v }
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
        self.r
    }
}
