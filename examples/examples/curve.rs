use std::time::Instant;
use time::macros::date;
use time::Duration;
use RustQuant::data::*;

fn main() {
    // Date	        1 Mo	2 Mo	3 Mo	4 Mo	6 Mo	1 Yr	2 Yr	3 Yr	5 Yr	7 Yr	10 Yr	20 Yr	30 Yr
    // 08/01/2024	5.55	5.46	5.37	5.28	5.08	4.62	4.16	3.96	3.84	3.89	3.99	4.35	4.27

    let start = Instant::now();

    let today = date!(2024 - 08 - 01);

    let dates = vec![
        today + Duration::days(30),
        today + Duration::days(60),
        today + Duration::days(90),
        today + Duration::days(120),
        today + Duration::days(180),
        today + Duration::days(365),
        today + Duration::days(365 * 2),
        today + Duration::days(365 * 3),
        today + Duration::days(365 * 5),
        today + Duration::days(365 * 7),
        today + Duration::days(365 * 10),
        today + Duration::days(365 * 20),
        today + Duration::days(365 * 30),
    ];

    let rates = vec![
        5.55, 5.46, 5.37, 5.28, 5.08, 4.62, 4.16, 3.96, 3.84, 3.89, 3.99, 4.35, 4.27,
    ];

    let mut curve = Curve::new(dates, rates, CurveType::Spot, InterpolationMethod::Linear).unwrap();

    // curve.plot();

    let new_dates = (1..365 * 30)
        .map(|i| today + Duration::days(i))
        .collect::<Vec<_>>();

    curve.get_rates_and_insert(new_dates);
    // curve.plot();

    println!("Elapsed: {:.2?}", start.elapsed());
}
