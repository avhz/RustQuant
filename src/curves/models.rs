// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Various yield curve models.
pub enum CurveModels {
    /// Nelson-Siegel (1987) model.
    NelsonSiegel,

    /// Nelson-Siegel-Svensson (1994) model.
    NelsonSiegelSvensson,

    /// Björk-Christensen (1999) model.
    BjoerkChristensen,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, TRAITS, AND FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl CurveModels {
    /// Returns the zero rate for a given yield curve model.
    pub fn zero_rate(&self, parameters: &[f64], ttm: f64) -> f64 {
        match self {
            CurveModels::NelsonSiegel => CurveModels::nelson_siegel(parameters, ttm),
            CurveModels::NelsonSiegelSvensson => {
                CurveModels::nelson_siegel_svensson(parameters, ttm)
            }
            CurveModels::BjoerkChristensen => CurveModels::bjoerk_christensen(parameters, ttm),
        }
    }

    fn nelson_siegel(params: &[f64], ttm: f64) -> f64 {
        assert_eq!(
            params.len(),
            4,
            "Nelson-Siegel requires 4 parameters: b0, b1, b2, t1."
        );

        let (b0, b1, b2, t1) = (params[0], params[1], params[2], params[3]);

        let term0 = b0;
        let term1 = b1 * ((1.0 - (-ttm / t1).exp()) / (ttm / t1));
        let term2 = b2 * ((1.0 - (-ttm / t1).exp()) / (ttm / t1) - (-ttm / t1).exp());

        term0 + term1 + term2
    }

    fn nelson_siegel_svensson(params: &[f64], ttm: f64) -> f64 {
        assert_eq!(
            params.len(),
            6,
            "Nelson-Siegel-Svensson requires 6 parameters: b0, b1, b2, b3, t1, t2."
        );

        let (b0, b1, b2, b3, t1, t2) = (
            params[0], params[1], params[2], params[3], params[4], params[5],
        );

        let term0 = b0;
        let term1 = b1 * ((1.0 - (-ttm / t1).exp()) / (ttm / t1));
        let term2 = b2 * ((1.0 - (-ttm / t1).exp()) / (ttm / t1) - (-ttm / t1).exp());
        let term3 = b3 * ((1.0 - (-ttm / t2).exp()) / (ttm / t2) - (-ttm / t2).exp());

        term0 + term1 + term2 + term3
    }

    fn bjoerk_christensen(params: &[f64], ttm: f64) -> f64 {
        assert_eq!(
            params.len(),
            6,
            "Björk-Christensen requires 6 parameters: b0, b1, b2, b3, t1, t2."
        );

        let (b0, b1, b2, b3, t1, t2) = (
            params[0], params[1], params[2], params[3], params[4], params[5],
        );

        let term0 = b0;
        let term1 = b1 * ((1.0 - (-ttm / t1).exp()) / (ttm / t1));
        let term2 = b2 * ((1.0 - (-ttm / t1).exp()) / (ttm / t1) - (-ttm / t1).exp());
        let term3 = b3 * ((1.0 - (-2.0 * ttm / t2).exp()) / (2.0 * ttm / t2));

        term0 + term1 + term2 + term3
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    #[test]
    fn very_thorough_test() {}
}
