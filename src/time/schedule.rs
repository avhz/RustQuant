// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use time::{Duration, OffsetDateTime};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, AND TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Schedule struct.
///
/// Many financial instruments have a schedule of dates associated with them.
/// For example, interest rate caps/floors have a schedule of payment dates, which are
/// the expiration dates of the caplets/floorlets.
///
/// The Schedule struct is used to represent these schedules,
/// and pricing methods should be implemented using date/time functionality.
pub struct Schedule {
    /// The dates of the schedule.
    pub dates: Vec<OffsetDateTime>,
    /// The start date of the schedule.
    /// This is optional, and is used if you wish to generate a schedule
    /// between two dates, with a given frequency.
    pub start: Option<OffsetDateTime>,
    /// The end date of the schedule.
    /// This is optional, and is used if you wish to generate a schedule
    /// between two dates, with a given frequency.
    pub end: Option<OffsetDateTime>,
    /// The frequency of the schedule.
    /// This is optional, and is used if you wish to generate a schedule
    /// between two dates, with a given frequency.
    pub frequency: Option<PaymentFrequency>,
    /// The convention of the schedule.
    pub day_count_convention: DayCountConvention,
    /// The business day convention of the schedule.
    pub business_day_convention: BusinessDayConvention,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Schedule {
    /// Creates a new schedule from a given start date, period length, and number
    /// of periods in the schedule.
    pub fn new_from_start(start: OffsetDateTime, period: Duration, num_periods: i64) -> Schedule {
        let mut payments = Vec::with_capacity(num_periods as usize + 1);
        let mut current_time = start;

        for _ in 0..=num_periods {
            payments.push(current_time);
            current_time += period;
        }

        Schedule {
            dates: payments,
            start: Some(start),
            end: Some(current_time),
            frequency: None,
            day_count_convention: DayCountConvention::Actual365,
            business_day_convention: BusinessDayConvention::Following,
        }
    }

    /// Creates a new schedule from a given end date, period length, and number
    /// of periods in the schedule.
    pub fn new_from_end(end: OffsetDateTime, period: Duration, num_periods: i64) -> Schedule {
        let mut payments = Vec::with_capacity(num_periods as usize + 1);

        let mut current_time = end;

        for _ in 0..=num_periods {
            payments.push(current_time);
            current_time -= period;
        }

        payments.reverse();

        Schedule {
            dates: payments,
            start: Some(current_time),
            end: Some(end),
            frequency: None,
            day_count_convention: DayCountConvention::Actual365,
            business_day_convention: BusinessDayConvention::Following,
        }
    }

    /// Creates a new schedule from a vector of dates.
    /// Dates must be in ascending order.
    pub fn new_from_dates(dates: Vec<OffsetDateTime>) -> Schedule {
        assert!(&dates.windows(2).all(|window| window[0] < window[1]));

        Schedule {
            dates: dates.clone(),
            start: Some(dates[0]),
            end: Some(dates[dates.len() - 1]),
            frequency: None,
            day_count_convention: DayCountConvention::Actual365,
            business_day_convention: BusinessDayConvention::Following,
        }
    }

    /// Drops a given date from the schedule.
    pub fn drop(&mut self, date: OffsetDateTime) {
        // let date = date.midnight_at(UtcOffset::UTC); // Convert to OffsetDateTime for comparison
        self.dates.retain(|&payment| payment.date() != date.date());
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_schedule {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_new_from_start() {
        let start = datetime!(2023-06-01 0:0:0 UTC);
        let period = Duration::days(30);
        let num_periods = 3;
        let schedule = Schedule::new_from_start(start, period, num_periods);
        assert_eq!(schedule.dates.len(), num_periods as usize + 1);
        assert_eq!(
            schedule.dates,
            vec![
                datetime!(2023-06-01 0:0:0 UTC),
                datetime!(2023-07-01 0:0:0 UTC),
                datetime!(2023-07-31 0:0:0 UTC),
                datetime!(2023-08-30 0:0:0 UTC),
            ],
        );
        for i in 0..num_periods {
            assert_eq!(schedule.dates[i as usize], start + period * i as i32);
        }
    }

    #[test]
    fn test_new_from_end() {
        let end = datetime!(2023-08-01 0:0:0 UTC);
        let period = Duration::days(30);
        let num_periods = 3;
        let schedule = Schedule::new_from_end(end, period, num_periods);
        // Length is num_periods + 1 because the end date is included.
        // Fencepost problem.
        assert_eq!(schedule.dates.len(), num_periods as usize + 1);
        assert_eq!(
            schedule.dates,
            vec![
                datetime!(2023-05-03 0:0:0 UTC),
                datetime!(2023-06-02 0:0:0 UTC),
                datetime!(2023-07-02 0:0:0 UTC),
                datetime!(2023-08-01 0:0:0 UTC),
            ],
        );
        // for i in 0..num_periods {
        //     assert_eq!(
        //         schedule.dates[(num_periods - 1 - i) as usize],
        //         end - period * i as i32
        //     );
        // }
    }

    #[test]
    fn test_new_from_dates() {
        let dates = vec![
            datetime!(2023-06-01 0:0:0 UTC),
            datetime!(2023-07-01 0:0:0 UTC),
            datetime!(2023-08-01 0:0:0 UTC),
        ];
        let schedule = Schedule::new_from_dates(dates.clone());
        assert_eq!(schedule.dates, dates);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_new_from_dates_out_of_order() {
        let dates = vec![
            datetime!(2023-07-01 0:0:0 UTC),
            datetime!(2023-06-01 0:0:0 UTC),
        ];
        Schedule::new_from_dates(dates);
    }

    #[test]
    fn test_drop() {
        let mut schedule =
            Schedule::new_from_start(datetime!(2023-06-01 0:0:0 UTC), Duration::days(30), 3);
        schedule.drop(datetime!(2023-07-01 0:0:0 UTC));
        assert_eq!(schedule.dates.len(), 3);
        assert_eq!(
            schedule.dates,
            vec![
                datetime!(2023-06-01 0:0:0 UTC),
                // datetime!(2023-07-01 0:0:0 UTC), // Dropped
                datetime!(2023-07-31 0:0:0 UTC),
                datetime!(2023-08-30 0:0:0 UTC),
            ]
        );
    }
}
