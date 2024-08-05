use plotly::{Plot, Scatter};
// use polars::prelude::*;
use time::macros::date;
use time::{Date, Duration};
use RustQuant::data::CurveModel;
use RustQuant::data::Curves;
use RustQuant::data::{Curve, DiscountCurve, SpotCurve};
use RustQuant::models::NelsonSiegelSvensson;
use RustQuant::time::oceania::australia::AustraliaCalendar;
// use RustQuant::time::Calendar;

fn main() {
    // let cal = AustraliaCalendar;
    // let curve = Curve::<Date>::new_from_slice(&DATES, &RATES);

    let mut discount_curve = SpotCurve::<Date, AustraliaCalendar>::new(&DATES, &RATES);

    let new_dates = [
        date!(2025 - 01 - 01),
        date!(2026 - 01 - 01),
        date!(2027 - 01 - 01),
        date!(2028 - 01 - 01),
        date!(2029 - 01 - 01),
        date!(2030 - 01 - 01),
        date!(2033 - 01 - 01),
        date!(2036 - 01 - 01),
        date!(2040 - 01 - 01),
        date!(2044 - 01 - 01),
        date!(2046 - 01 - 01),
        date!(2048 - 01 - 01),
        date!(2050 - 01 - 01),
        date!(2053 - 01 - 01),
    ];

    discount_curve.get_rates(&new_dates);

    discount_curve.plot();

    // let nss = NelsonSiegelSvensson::new(0.0806, -0.0031, -0.0625, -0.0198, 1.58, 0.15);

    // let date = date!(2027 - 01 - 01);
    // println!("Forward rate: {:?}", nss.forward_rate(date));
    // println!("Spot rate: {:?}", nss.spot_rate(date));
}

const DATES: [Date; 33] = [
    date!(2024 - 11 - 03),
    date!(2025 - 02 - 02),
    date!(2025 - 05 - 04),
    date!(2025 - 08 - 04),
    date!(2026 - 08 - 04),
    date!(2027 - 08 - 04),
    date!(2028 - 08 - 03),
    date!(2029 - 08 - 03),
    date!(2030 - 08 - 03),
    date!(2031 - 08 - 03),
    date!(2032 - 08 - 02),
    date!(2033 - 08 - 02),
    date!(2034 - 08 - 02),
    date!(2035 - 08 - 02),
    date!(2036 - 08 - 01),
    date!(2037 - 08 - 01),
    date!(2038 - 08 - 01),
    date!(2039 - 08 - 01),
    date!(2040 - 07 - 31),
    date!(2041 - 07 - 31),
    date!(2042 - 07 - 31),
    date!(2043 - 07 - 31),
    date!(2044 - 07 - 30),
    date!(2045 - 07 - 30),
    date!(2046 - 07 - 30),
    date!(2047 - 07 - 30),
    date!(2048 - 07 - 29),
    date!(2049 - 07 - 29),
    date!(2050 - 07 - 29),
    date!(2051 - 07 - 29),
    date!(2052 - 07 - 28),
    date!(2053 - 07 - 28),
    date!(2054 - 07 - 28),
];

const RATES: [f64; 33] = [
    0.03400521, 0.03259227, 0.0313705, 0.03031886, 0.02746567, 0.02614014, 0.02574612, 0.02590431,
    0.02637474, 0.02700684, 0.02770726, 0.02841916, 0.02910886, 0.02975736, 0.03035484, 0.03089715,
    0.0313836, 0.03181554, 0.03219547, 0.03252652, 0.03281203, 0.03305541, 0.03326001, 0.033429,
    0.03356541, 0.03367205, 0.03375153, 0.03380629, 0.03383858, 0.03385046, 0.03384384, 0.03382048,
    0.033782,
];
