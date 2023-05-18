// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// pub struct Bond {
//     price: f64,
//     duration: f64,
//     convexity: f64,
// }

/// Trait for zero-coupon bond pricing.
pub trait ZeroCouponBond {
    /// Price method for the zero-coupon bond trait.
    fn price(&self) -> f64;
    // fn duration(&self) -> f64;
    // fn convexity(&self) -> f64;
}

// pub trait CouponBond {
//     fn price(&self) -> f64;
//     fn duration(&self) -> f64;
//     fn convexity(&self) -> f64;
// }
