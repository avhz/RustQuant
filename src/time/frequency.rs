// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use time::{Date, Duration};

use crate::time::constants::{
    ANNUALLY, BI_WEEKLY, DAILY, MONTHLY, QUARTERLY, SEMI_ANNUALLY, SEMI_MONTHLY, SEMI_QUARTERLY,
    TRI_ANNUALLY, WEEKLY,
};

/// Interest/coupon frequency per year.
/// This is important in finance, as it determines the number of times
/// a cash flow is paid in a year, and thus affects the present value
/// of the cash flows.
#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    /// Daily (252 per year).
    Daily = DAILY,

    /// Weekly (52 per year).
    Weekly = WEEKLY,

    /// Bi-weekly (26 per year).
    BiWeekly = BI_WEEKLY,

    /// Semi-monthly (24 per year).
    SemiMonthly = SEMI_MONTHLY,

    /// Monthly (12 per year).
    Monthly = MONTHLY,

    /// Semi-quarterly (8 per year).
    SemiQuarterly = SEMI_QUARTERLY,

    /// Quarterly.
    Quarterly = QUARTERLY,

    /// Tri-annually.
    TriAnnually = TRI_ANNUALLY,

    /// Semi-annually.
    SemiAnnually = SEMI_ANNUALLY,

    /// Annually.
    Annually = ANNUALLY,
}

impl Frequency {
    /// Function to infer the frequency between two `Date`s.
    ///
    /// This is a very simple (fallible) way to infer the frequency between two dates.
    ///
    /// # Panics
    ///
    /// Panics if the difference between the two dates is not a recognized frequency.
    pub fn infer_frequency(start: Date, end: Date) -> Frequency {
        let diff = end - start;

        if diff == Duration::days(1) {
            Frequency::Daily
        } else if diff == Duration::weeks(1) {
            Frequency::Weekly
        } else if diff == Duration::weeks(2) {
            Frequency::BiWeekly
        } else if diff > Duration::days(14) && diff < Duration::days(16) {
            Frequency::SemiMonthly
        } else if diff >= Duration::days(28) && diff <= Duration::days(31) {
            Frequency::Monthly
        } else if diff >= Duration::days(45) && diff <= Duration::days(46) {
            Frequency::SemiQuarterly
        } else if diff >= Duration::days(91) && diff <= Duration::days(92) {
            Frequency::Quarterly
        } else if diff >= Duration::days(121) && diff <= Duration::days(122) {
            Frequency::TriAnnually
        } else if diff >= Duration::days(182) && diff <= Duration::days(183) {
            Frequency::SemiAnnually
        } else if diff >= Duration::days(365) && diff <= Duration::days(366) {
            Frequency::Annually
        } else {
            panic!("Unable to infer frequency between the two dates.")
        }
    }

    /// Get the number of times the frequency occurs in a year.
    pub fn times_in_year(&self) -> isize {
        match self {
            Frequency::Daily => DAILY,
            Frequency::Weekly => WEEKLY,
            Frequency::BiWeekly => BI_WEEKLY,
            Frequency::SemiMonthly => SEMI_MONTHLY,
            Frequency::Monthly => MONTHLY,
            Frequency::SemiQuarterly => SEMI_QUARTERLY,
            Frequency::Quarterly => QUARTERLY,
            Frequency::TriAnnually => TRI_ANNUALLY,
            Frequency::SemiAnnually => SEMI_ANNUALLY,
            Frequency::Annually => ANNUALLY,
        }
    }
}
