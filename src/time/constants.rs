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
pub enum Frequency {
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

// /// Weekend mask enumeration.
// /// This indicates which days of the week are considered weekends.
// pub enum WeekendMask {
//     Saturday_Sunday = (0, 0, 0, 0, 0, 1, 1),
//     Friday_Saturday = (0, 0, 0, 0, 1, 1, 0),
//     Sunday_Only = (0, 0, 0, 0, 0, 0, 1),
//     None = (0, 0, 0, 0, 0, 0, 0),
// }
