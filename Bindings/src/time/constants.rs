// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Day enumeration.
pub enum Day {
    /// Monday.
    Monday = 1,
    /// Tuesday.
    Tuesday = 2,
    /// Wednesday.
    Wednesday = 3,
    /// Thursday.
    Thursday = 4,
    /// Friday.
    Friday = 5,
    /// Saturday.
    Saturday = 6,
    /// Sunday.
    Sunday = 7,
}

/// Month enumeration.
pub enum Month {
    /// January.
    January = 1,
    /// February.
    February = 2,
    /// March.
    March = 3,
    /// April.
    April = 4,
    /// May.
    May = 5,
    /// June.
    June = 6,
    /// July.
    July = 7,
    /// August.
    August = 8,
    /// September.
    September = 9,
    /// October.
    October = 10,
    /// November.
    November = 11,
    /// December.
    December = 12,
}

/// Interest payment frequency/year enumeration.
pub enum PaymentFrequency {
    /// Daily.
    Daily = 252,
    /// Weekly.
    Weekly = 52,
    /// Bi-weekly.
    BiWeekly = 26,
    /// Semi-monthly.
    SemiMonthly = 24,
    /// Monthly.
    Monthly = 12,
    /// Semi-quarterly.
    SemiQuarterly = 6,
    /// Quarterly.
    Quarterly = 4,
    /// Tri-annually.
    TriAnnually = 3,
    /// Semi-annually.
    SemiAnnually = 2,
    /// Annually.
    Annually = 1,
}

#[allow(dead_code)]
pub(crate) const DAYS_IN_YEAR: usize = 365; // Or should it be 365.25?
#[allow(dead_code)]
pub(crate) const DAYS_IN_WEEK: usize = 7;
#[allow(dead_code)]
pub(crate) const HOURS_IN_DAY: usize = 24;
#[allow(dead_code)]
pub(crate) const MINS_IN_HOUR: usize = 60;
#[allow(dead_code)]
pub(crate) const SECS_IN_MIN: usize = 60;
#[allow(dead_code)]
pub(crate) const SECS_IN_HOUR: usize = SECS_IN_MIN * MINS_IN_HOUR;
#[allow(dead_code)]
pub(crate) const MINS_IN_DAY: usize = MINS_IN_HOUR * HOURS_IN_DAY;
#[allow(dead_code)]
pub(crate) const SECS_IN_DAY: usize = SECS_IN_MIN * MINS_IN_DAY;
#[allow(dead_code)]
pub(crate) const HOURS_IN_WEEK: usize = HOURS_IN_DAY * DAYS_IN_WEEK;
#[allow(dead_code)]
pub(crate) const MINS_IN_WEEK: usize = HOURS_IN_WEEK * MINS_IN_HOUR;
#[allow(dead_code)]
pub(crate) const SECS_IN_WEEK: usize = MINS_IN_WEEK * SECS_IN_HOUR;
#[allow(dead_code)]
pub(crate) const SECS_IN_YEAR: usize = SECS_IN_MIN * MINS_IN_HOUR * HOURS_IN_DAY * DAYS_IN_YEAR;

// /// Weekend mask enumeration.
// /// This indicates which days of the week are considered weekends.
// pub enum WeekendMask {
//     Saturday_Sunday,
//     Friday_Saturday,
//     Sunday_Only,
//     None,
// }
