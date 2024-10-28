// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::macros::date;
use RustQuant::time::{
    countries::oceania::australia::AustraliaCalendar, DateRollingConvention, DayCountConvention,
    Scheduler,
};

fn main() {
    let cal = AustraliaCalendar;

    let day_counting_cconvention = DayCountConvention::Actual_365_25;
    let date_rolling_convention = DateRollingConvention::ModifiedRolling;

    // Semi-annual coupon payment dates.
    let coupon_dates = &[
        date!(2024 - 07 - 01),
        date!(2025 - 01 - 01),
        date!(2025 - 07 - 01),
        date!(2026 - 01 - 01),
    ];

    // Generate the schedule. The schedule contains:
    // - The adjusted coupon dates,
    // - The day count factor for each period,
    // - The day counting and date rolling conventions.
    let schedule = cal.generate_schedule_from_dates(
        coupon_dates,
        date_rolling_convention,
        day_counting_cconvention,
    );

    // You will see that the dates that fall on New Year's Day are rolled to the next business day.
    println!("{}", schedule);
}
