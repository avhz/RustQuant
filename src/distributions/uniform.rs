use num_complex::Complex;

/// Uniform distribution: X ~ Uni(a, b)
pub struct Uniform<'a> {
    /// Lower bound.
    a: f64,
    /// Upper bound.
    b: f64,
    /// Continuous or discrete ?
    class: &'a str,
}

impl Uniform<'_> {
    /// New instance of a Uniform distribution.
    pub fn new(a: f64, b: f64, class: &str) -> Uniform {
        assert!(a <= b);
        assert!(class == "discrete" || class == "continuous");

        if class == "discrete" {
            Uniform {
                a: a.round(),
                b: b.round(),
                class: class,
            }
        } else if class == "continuous" {
            Uniform {
                a: a,
                b: b,
                class: class,
            }
        } else {
            panic!("Class should be either 'discrete' or 'continuous'.")
        }
    }

    /// Uniform characteristic function.
    pub fn cf(&self, t: f64) -> Complex<f64> {
        let i: Complex<f64> = Complex::i();

        if self.class == "discrete" {
            ((i * t * self.a).exp() - (i * t * (self.b + 1_f64)).exp())
                / ((1_f64 - (i * t).exp()) * (self.b - self.a + 1_f64))
        } else if self.class == "continuous" {
            ((i * t * self.b).exp() - (i * t * self.a).exp()) / (i * t * (self.b - self.a))
        } else {
            panic!("Class should be either 'discrete' or 'continuous'.")
        }
    }

    /// Uniform mass function.
    pub fn pmf(&self, x: f64) -> f64 {
        if self.class == "discrete" {
            1_f64 / self.b - self.a + 1_f64
        } else if self.class == "continuous" {
            if x >= self.a && x <= self.b {
                (self.b - self.a).recip()
            } else {
                0.0
            }
        } else {
            panic!("Class should be either 'discrete' or 'continuous'.")
        }
    }

    /// Uniform distribution function.
    pub fn cdf(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_equal;

    #[test]
    fn test_poisson_distribution() {
        let dist: Uniform = Uniform::new(0.0, 1.0, "discrete");

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.42079361743, 1e-10);
        assert_approx_equal!(cf.im, 0.47084264330, 1e-10);

        // Probability mass function
        let pmf = dist.pmf(1);
        assert_approx_equal!(pmf, 0.367879441171, 1e-10);

        // Distribution function
        let cdf = dist.cdf(1);
        assert_approx_equal!(cdf, 0.640859085770, 1e-10);
    }
}
