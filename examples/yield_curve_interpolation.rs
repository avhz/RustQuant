use time::{Date, Duration, OffsetDateTime};
use RustQuant::{
    curves::{Curve, YieldCurve},
    plot_vector,
    time::today,
};

fn main() {
    // Initial date of the curve (today).
    let t0 = today();

    // Create a treasury yield curve with 8 points (3m, 6m, 1y, 2y, 5y, 10y, 30y).
    // Values from Bloomberg: <https://www.bloomberg.com/markets/rates-bonds/government-bonds/us>
    let rate_vec = vec![0.0544, 0.0556, 0.0546, 0.0514, 0.0481, 0.0481, 0.0494];
    let date_vec = vec![
        t0 + Duration::days(90),
        t0 + Duration::days(180),
        t0 + Duration::days(365),
        t0 + Duration::days(2 * 365),
        t0 + Duration::days(5 * 365),
        t0 + Duration::days(10 * 365),
        t0 + Duration::days(30 * 365),
    ];

    let yield_curve = YieldCurve::from_dates_and_rates(&date_vec, &rate_vec);

    // Create a vector of dates to interpolate the yield curve at.
    let dates_to_plot = (91..(30 * 365))
        .step_by(10)
        .map(|i| t0 + Duration::days(i))
        .collect::<Vec<Date>>();

    // Compute the discount factors.
    let discount_factors = yield_curve.discount_factors(&dates_to_plot);

    // Plot the interpolated yield curve.
    plot_vector!(discount_factors, "./images/interpolated_yield_curve.png");
}
