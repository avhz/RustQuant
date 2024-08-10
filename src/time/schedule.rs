// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::date_rolling::{DateRoller, DateRollingConvention};
use crate::time::day_counting::{DayCountConvention, DayCounter};
use crate::time::Calendar;
use std::fmt;
use time::Date;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Schedule struct.
///
/// Many financial instruments have a schedule of dates associated with them.
/// For example, interest rate caps/floors have a schedule of payment dates,
/// which are the expiration dates of the caplets/floorlets, or a coupon bond
/// has a schedule of coupon payment dates.
///
/// The Schedule struct is used to represent these schedules,
/// and pricing methods should be implemented using date/time functionality.
#[derive(Clone, Debug)]
pub struct Schedule {
    /// The dates of the schedule.
    pub dates: Vec<Date>,

    /// The day count factors of the schedule.
    pub day_count_factors: Vec<f64>,

    /// The convention of the schedule.
    pub day_counting_convention: DayCountConvention,

    /// The business day convention of the schedule.
    pub date_rolling_convention: DateRollingConvention,
}

/// The `Scheduler` trait.
/// This trait is used to generate schedules for a `Calendar`.
pub trait Scheduler {
    /// Generate a schedule from a slice of `Date`s.
    /// For example, a list of coupon payment dates.
    ///
    /// Note: The effective date is not included in the dates input, and assumed to be today.
    ///
    /// # Arguments
    ///
    /// * `dates` - A slice of `Date`s (such as coupon payment dates).
    /// * `date_rolling_convention` - The date rolling convention.
    /// * `day_counting_convention` - The day counting convention.
    fn generate_schedule_from_dates(
        &self,
        dates: &[Date],
        date_rolling_convention: DateRollingConvention,
        day_counting_convention: DayCountConvention,
    ) -> Schedule;
}

impl<C> Scheduler for C
where
    C: Calendar,
{
    fn generate_schedule_from_dates(
        &self,
        dates: &[Date],
        date_rolling_convention: DateRollingConvention,
        day_counting_convention: DayCountConvention,
    ) -> Schedule {
        let today = crate::time::today();

        // First we need to roll the dates according to a given convention.
        let rolled_dates = self.roll_dates(dates, &date_rolling_convention);

        // Then we need to compute the day count factors.
        let mut day_count_factors = self.day_count_factors(&rolled_dates, &day_counting_convention);
        day_count_factors.insert(
            0,
            self.day_count_factor(today, rolled_dates[0], &day_counting_convention),
        );

        Schedule {
            dates: rolled_dates,
            day_count_factors,
            day_counting_convention,
            date_rolling_convention,
        }
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Dates:                     {:?}\n\
            Day Count Factors:          {:?}\n\
            Day Counting Convention:    {}\n\
            Date Rolling Convention:    {}",
            self.dates,
            self.day_count_factors,
            self.day_counting_convention,
            self.date_rolling_convention
        )
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// impl Schedule {
//     /// Create a new schedule from a vector of dates.
//     ///
//     /// The list can be in any order, and the schedule will be
//     /// sorted in chronological order.
//     pub fn new(mut dates: &[Date]) -> Schedule {
//         dates.sort();

//         Schedule {
//             dates: dates.to_vec(),
//             start: Some(dates[0]),
//             end: Some(dates[dates.len() - 1]),
//             frequency: None,
//             day_count_convention: DayCountConvention::default(),
//             date_rolling_convention: DateRollingConvention::default(),
//         }
//     }

//     /// Creates a new schedule from a given start date, period length, and number
//     /// of periods in the schedule.
//     pub fn new_from_start(start: Date, period: Duration, num_periods: i64) -> Schedule {
//         let mut payments = Vec::with_capacity(num_periods as usize + 1);
//         let mut current_time = start;

//         for _ in 0..=num_periods {
//             payments.push(current_time);
//             current_time += period;
//         }

//         Schedule {
//             dates: payments,
//             start: Some(start),
//             end: Some(current_time),
//             frequency: None,
//             day_count_convention: DayCountConvention::Actual365,
//             date_rolling_convention: DateRollingConvention::Following,
//         }
//     }

//     /// Creates a new schedule from a given end date, period length, and number
//     /// of periods in the schedule.
//     pub fn new_from_end(end: Date, period: Duration, num_periods: i64) -> Schedule {
//         let mut payments = Vec::with_capacity(num_periods as usize + 1);

//         let mut current_time = end;

//         for _ in 0..=num_periods {
//             payments.push(current_time);
//             current_time -= period;
//         }

//         payments.reverse();

//         Schedule {
//             dates: payments,
//             start: Some(current_time),
//             end: Some(end),
//             frequency: None,
//             day_count_convention: DayCountConvention::Actual365,
//             date_rolling_convention: DateRollingConvention::Following,
//         }
//     }

//     /// Creates a new schedule from a vector of dates.
//     /// Dates must be in ascending order.
//     pub fn new_from_dates(dates: &&[Date]) -> Schedule {
//         assert!(&dates.windows(2).all(|window| window[0] < window[1]));

//         Schedule {
//             dates: dates.clone(),
//             start: Some(dates[0]),
//             end: Some(dates[dates.len() - 1]),
//             frequency: None,
//             day_count_convention: DayCountConvention::Actual365,
//             date_rolling_convention: DateRollingConvention::Following,
//         }
//     }

//     /// Drops a given date from the schedule.
//     pub fn drop(&mut self, date: Date) {
//         // let date = date.midnight_at(UtcOffset::UTC); // Convert to Date for comparison
//         self.dates.retain(|&payment| payment.date() != date.date());
//     }
// }

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // UNIT TESTS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// #[cfg(test)]
// mod test_schedule {
//     use super::*;
//     use time::macros::datetime;

//     #[test]
//     fn test_new_from_start() {
//         let start = datetime!(2023-06-01 0:0:0 UTC);
//         let period = Duration::days(30);
//         let num_periods = 3;
//         let schedule = Schedule::new_from_start(start, period, num_periods);
//         assert_eq!(schedule.dates.len(), num_periods as usize + 1);
//         assert_eq!(
//             schedule.dates,
//             vec![
//                 datetime!(2023-06-01 0:0:0 UTC),
//                 datetime!(2023-07-01 0:0:0 UTC),
//                 datetime!(2023-07-31 0:0:0 UTC),
//                 datetime!(2023-08-30 0:0:0 UTC),
//             ],
//         );
//         for i in 0..num_periods {
//             assert_eq!(schedule.dates[i as usize], start + period * i as i32);
//         }
//     }

//     #[test]
//     fn test_new_from_end() {
//         let end = datetime!(2023-08-01 0:0:0 UTC);
//         let period = Duration::days(30);
//         let num_periods = 3;
//         let schedule = Schedule::new_from_end(end, period, num_periods);
//         // Length is num_periods + 1 because the end date is included.
//         // Fencepost problem.
//         assert_eq!(schedule.dates.len(), num_periods as usize + 1);
//         assert_eq!(
//             schedule.dates,
//             vec![
//                 datetime!(2023-05-03 0:0:0 UTC),
//                 datetime!(2023-06-02 0:0:0 UTC),
//                 datetime!(2023-07-02 0:0:0 UTC),
//                 datetime!(2023-08-01 0:0:0 UTC),
//             ],
//         );
//         // for i in 0..num_periods {
//         //     assert_eq!(
//         //         schedule.dates[(num_periods - 1 - i) as usize],
//         //         end - period * i as i32
//         //     );
//         // }
//     }

//     #[test]
//     fn test_new_from_dates() {
//         let dates = vec![
//             datetime!(2023-06-01 0:0:0 UTC),
//             datetime!(2023-07-01 0:0:0 UTC),
//             datetime!(2023-08-01 0:0:0 UTC),
//         ];
//         let schedule = Schedule::new_from_dates(&dates);
//         assert_eq!(schedule.dates, dates);
//     }

//     #[test]
//     #[should_panic(expected = "assertion failed")]
//     fn test_new_from_dates_out_of_order() {
//         let dates = vec![
//             datetime!(2023-07-01 0:0:0 UTC),
//             datetime!(2023-06-01 0:0:0 UTC),
//         ];
//         let _ = Schedule::new_from_dates(&dates);
//     }

//     #[test]
//     fn test_drop() {
//         let mut schedule =
//             Schedule::new_from_start(datetime!(2023-06-01 0:0:0 UTC), Duration::days(30), 3);
//         schedule.drop(datetime!(2023-07-01 0:0:0 UTC));
//         assert_eq!(schedule.dates.len(), 3);
//         assert_eq!(
//             schedule.dates,
//             vec![
//                 datetime!(2023-06-01 0:0:0 UTC),
//                 // datetime!(2023-07-01 0:0:0 UTC), // Dropped
//                 datetime!(2023-07-31 0:0:0 UTC),
//                 datetime!(2023-08-30 0:0:0 UTC),
//             ]
//         );
//     }
// }
