use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_PI_2, PI};
use RustQuant_math::{gaussian::N, Distribution};

/// Bachelier (1900) option pricing parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Bachelier {
    f: f64,
    r: f64,
    v: f64,
}

impl Bachelier {
    /// Create a new Bachelier (1900) option pricing parameters.
    pub fn new(f: f64, r: f64, v: f64) -> Self {
        Self { f, r, v }
    }

    /// Price a European call option using the Bacheller model.
    #[inline]
    pub(crate) fn call_price(f: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        let df = df(r, t);
        let price = (f - k) * N.cdf(d) + v * t.sqrt() * N.pdf(d);

        df * price
    }

    /// Price a European put option using the Bacheller model.
    #[inline]
    pub(crate) fn put_price(f: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        let df = df(r, t);
        let price = (k - f) * N.cdf(-d) + v * t.sqrt() * N.pdf(d);

        df * price
    }

    #[inline]
    fn d(f: f64, k: f64, t: f64, v: f64) -> f64 {
        (f - k) / (v * t.sqrt())
    }

    #[inline]
    fn df(r: f64, t: f64) -> f64 {
        (-r * t).exp()
    }

    #[inline]
    pub(crate) fn atm_price(t: f64, v: f64) -> f64 {
        v * (t / (2. * PI)).sqrt()
    }

    #[inline]
    pub(crate) fn atm_vol(price: f64, t: f64) -> f64 {
        price * (2. * PI / t).sqrt()
    }

    const A: [f64; 8] = [
        3.994_961_687_345_13e-1,
        2.100_960_795_068_49e+1,
        4.980_340_217_855_08e+1,
        5.988_761_102_690_99e+2,
        1.848_489_695_437_09e+3,
        6.106_322_407_867_05e+3,
        2.493_415_285_349_36e+4,
        1.266_458_051_348_24e+4,
    ];

    const B: [f64; 9] = [
        4.990_534_153_589_42e+1,
        3.093_573_936_743_11e+1,
        1.495_105_008_310_99e+3,
        1.323_614_537_899_73e+3,
        1.598_919_697_679_74e+4,
        2.392_008_891_720_78e+4,
        3.608_817_108_375_03e+3,
        -2.067_719_486_400_92e+2,
        1.174_240_599_306_01e+1,
    ];

    #[inline]
    pub(crate) fn call_iv(price: f64, f: f64, k: f64, t: f64) -> f64 {
        let v = (f - k).abs() / (2. * price - (f - k));
        let eta = v / v.atanh();

        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        // for k in 0..A.len() {
        for (k, v) in A.iter().enumerate() {
            sum1 += v * eta.powi(k as i32);
        }

        // for k in 0..B.len() {
        for (k, v) in B.iter().enumerate() {
            sum2 += v * eta.powi(k as i32);
        }

        let hn = eta.sqrt() * sum1 / (1. + sum2);

        (FRAC_PI_2 / t).sqrt() * (2. * price - (f - k)) * hn
    }

    #[inline]
    pub(crate) fn put_iv(price: f64, f: f64, k: f64, t: f64) -> f64 {
        let v = (f - k).abs() / (2. * price + (f - k));
        let eta = v / v.atanh();

        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        // for k in 0..A.len() {
        for (k, v) in A.iter().enumerate() {
            sum1 += v * eta.powi(k as i32);
        }

        // for k in 1..=B.len() {
        for (k, v) in B.iter().enumerate().skip(1) {
            sum2 += v * eta.powi(k as i32);
        }

        let hn = eta.sqrt() * sum1 / (1. + sum2);

        (FRAC_PI_2 / t).sqrt() * (2. * price + (f - k)) * hn
    }

    #[inline]
    pub(crate) fn call_delta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        N.cdf(d)
    }

    #[inline]
    pub(crate) fn put_delta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        N.cdf(d) - 1.0
    }

    #[inline]
    pub(crate) fn call_gamma(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        N.pdf(d) / (v * t.sqrt())
    }

    #[inline]
    pub(crate) fn put_gamma(f: f64, k: f64, t: f64, v: f64) -> f64 {
        call_gamma(f, k, t, v)
    }

    #[inline]
    pub(crate) fn call_vega(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);
        t.sqrt() * N.pdf(d)
    }

    #[inline]
    pub(crate) fn put_vega(f: f64, k: f64, t: f64, v: f64) -> f64 {
        call_vega(f, k, t, v)
    }

    #[inline]
    pub(crate) fn call_theta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);

        -v * N.pdf(d) / (2. * t.sqrt())
    }

    #[inline]
    pub(crate) fn put_theta(f: f64, k: f64, t: f64, v: f64) -> f64 {
        let d = d(f, k, t, v);

        -v * N.pdf(d) / (2. * t.sqrt())
    }

    /// Price a European option using the Bachelier model.
    pub fn price(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => call_price(self.f, k, t, self.r, self.v),
            TypeFlag::Put => put_price(self.f, k, t, self.r, self.v),
        }
    }

    /// ATM price of a European option using the Bachelier model.
    pub fn atm_price(&self, t: f64) -> f64 {
        bachelier::atm_price(t, self.v)
    }

    /// ATM volatility of a European option using the Bachelier model.
    pub fn atm_vol(&self, price: f64, t: f64) -> f64 {
        bachelier::atm_vol(price, t)
    }

    /// Implied volatility of a European option using the Bachelier model.
    pub fn iv(&self, price: f64, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => call_iv(price, self.f, k, t),
            TypeFlag::Put => put_iv(price, self.f, k, t),
        }
    }

    /// Delta of a European option using the Bachelier model.
    pub fn delta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => call_delta(self.f, k, t, self.v),
            TypeFlag::Put => put_delta(self.f, k, t, self.v),
        }
    }

    /// Gamma of a European option using the Bachelier model.
    pub fn gamma(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => call_gamma(self.f, k, t, self.v),
            TypeFlag::Put => put_gamma(self.f, k, t, self.v),
        }
    }

    /// Vega of a European option using the Bachelier model.
    pub fn vega(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => call_vega(self.f, k, t, self.v),
            TypeFlag::Put => put_vega(self.f, k, t, self.v),
        }
    }

    /// Theta of a European option using the Bachelier model.
    pub fn theta(&self, k: f64, t: f64, option_type: TypeFlag) -> f64 {
        match option_type {
            TypeFlag::Call => call_theta(self.f, k, t, self.v),
            TypeFlag::Put => put_theta(self.f, k, t, self.v),
        }
    }
}
