// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Actual/Actual day count factor calculation.
pub mod actual_actual;

/// Actual/xxx day count factor calculation.
pub mod actual_constant;

/// No-Leap day count factor calculation.
pub mod no_leap;

/// One/One day count factor calculation.
pub mod one_one;

/// Simple day count factor calculation.
pub mod simple;

/// Thirty/xxx day count factor calculation.
pub mod thirthy_360;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::time::Calendar;
use std::fmt;
use time::{Date, Duration};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Day count conventions.
///
/// From Wikipedia (<https://en.wikipedia.org/wiki/Day_count_convention>):
/// """
/// In finance, a day count convention determines how interest accrues
/// over time for a variety of investments, including bonds, notes,
/// loans, mortgages, medium-term notes, swaps, and forward rate agreements (FRAs).
/// This determines the number of days between two coupon payments,
/// thus calculating the amount transferred on payment dates and also the
/// accrued interest for dates between payments. The day count is also
/// used to quantify periods of time when discounting a cash-flow to its
/// present value. When a security such as a bond is sold between interest
/// payment dates, the seller is eligible to some fraction of the coupon amount.
/// """
#[allow(non_camel_case_types)]
pub enum DayCountConvention {
    /// The '1/1' day count, which always returns a day count of 1.
    One_One,

    /// The 'Act/360' day count, which divides the actual number of days by 360.
    Actual_360,

    /// The 'Act/364' day count, which divides the actual number of days by 364.
    Actual_364,

    /// The 'Act/364' day count, which divides the actual number of days by 366.
    Actual_366,

    /// The 'Act/365.25' day count, which divides the actual number of days by 365.25.
    Actual_365_25,

    /// The 'Act/365 Actual' day count, which divides the actual number of days
    /// by 366 if a leap day is contained, or by 365 if not.
    Actual_365_Actual,

    /// The 'Act/365F' day count, which divides the actual number of days by 365 (fixed).
    Actual_365_Fixed,

    /// The 'Act/365L' day count, which divides the actual number of days by 365 or 366.
    Actual_365_Leap,

    /// The 'Act/Act AFB' day count, which divides the actual number of days by 366
    /// if a leap day is contained, or by 365 if not, with additional rules for periods over one year.
    Actual_Actual_AFB,

    /// The 'Act/Act ICMA' day count, which divides the actual number of days by
    /// the actual number of days in the coupon period multiplied by the frequency.
    Actual_Actual_ICMA,

    /// The 'Act/Act ISDA' day count, which divides the actual number of days in a
    /// leap year by 366 and the actual number of days in a standard year by 365.
    Actual_Actual_ISDA,

    /// The 'NL/360' day count, which divides the actual number of days omitting leap days by 360.
    No_Leap_360,

    /// The 'NL/365' day count, which divides the actual number of days omitting leap days by 365.
    No_Leap_365,

    /// The '30/360 ISDA' day count, which treats input day-of-month 31 specially.
    Thirty_360_ISDA,

    /// The '30E/360' day count, which treats input day-of-month 31 specially.
    Thirty_E_360,

    /// The '30E/360 ISDA' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_E_360_ISDA,

    /// The '30E/365' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_E_365,

    /// The '30E+/360' day count, which treats input day-of-month 31 specially.
    Thirty_E_Plus_360,

    /// The '30U/360' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_U_360,
}

/// `DayCounter` trait.
///
/// This trait is used to compute:
///     - Day count fraction (fraction of year between two dates).
///     - Business day count (number of days between two dates, excluding weekends and holidays).
///     - Calendar day count (number of days between two dates).
pub trait DayCounter {
    /// Compute the number of calendar days between two dates.
    fn calendar_day_count(&self, date1: Date, date2: Date) -> i64;

    /// Compute the number of business days between two dates.
    fn business_day_count(&self, date1: Date, date2: Date) -> i64;

    /// Compute the day count factor between two dates.
    fn day_count_factor(&self, date1: Date, date2: Date, convention: &DayCountConvention) -> f64;

    /// Compute the number of calendar days between each date in a vector of dates.
    fn calendar_day_counts(&self, dates: &[Date]) -> Vec<i64>;

    /// Compute the number of business days between two dates.
    fn business_day_counts(&self, dates: &[Date]) -> Vec<i64>;

    /// Compute the day count factor between each date in a vector of dates.
    fn day_count_factors(&self, dates: &[Date], convention: &DayCountConvention) -> Vec<f64>;
}

impl fmt::Display for DayCountConvention {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One_One               => write!(f, "1 / 1"),
            Self::Actual_360            => write!(f, "Actual / 360"),
            Self::Actual_364            => write!(f, "Actual / 364"),
            Self::Actual_366            => write!(f, "Actual / 366"),
            Self::Actual_365_25         => write!(f, "Actual / 365.25"),
            Self::Actual_365_Actual     => write!(f, "Actual / 365 Actual"),
            Self::Actual_365_Fixed      => write!(f, "Actual / 365F"),
            Self::Actual_365_Leap       => write!(f, "Actual / 365L"),
            Self::Actual_Actual_AFB     => write!(f, "Actual / Actual AFB"),
            Self::Actual_Actual_ICMA    => write!(f, "Actual / Actual ICMA"),
            Self::Actual_Actual_ISDA    => write!(f, "Actual / Actual ISDA"),
            Self::No_Leap_360           => write!(f, "No Leap / 360"),
            Self::No_Leap_365           => write!(f, "No Leap / 365"),
            Self::Thirty_360_ISDA       => write!(f, "30 / 360 ISDA"),
            Self::Thirty_E_360          => write!(f, "30 E / 360"),
            Self::Thirty_E_360_ISDA     => write!(f, "30 E / 360 ISDA"),
            Self::Thirty_E_365          => write!(f, "30 E / 365"),
            Self::Thirty_E_Plus_360     => write!(f, "30 E+ / 360"),
            Self::Thirty_U_360          => write!(f, "30 U / 360"),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS/METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl<C> DayCounter for C
where
    C: Calendar,
{
    /// Compute the number of calendar days between two dates.
    ///
    /// # Arguments
    ///
    /// * `date1` - The first date.
    /// * `date2` - The second date.
    ///
    /// # Example
    ///
    /// ```
    /// use time::macros::date;
    /// use calenda_rs::day_counting::DayCounter;
    /// use calenda_rs::countries::oceania::australia::AustraliaCalendar;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2025-01-01);
    ///
    /// let calendar = AustraliaCalendar;
    ///
    /// assert_eq!(calendar.calendar_day_count(date1, date2), 731);
    /// ```
    fn calendar_day_count(&self, date1: Date, date2: Date) -> i64 {
        (date2 - date1).whole_days()
    }

    /// Compute the number of business days between two dates.
    ///
    /// # Arguments
    ///
    /// * `date1` - The first date.
    /// * `date2` - The second date.
    ///
    /// # Example
    ///
    /// ```
    /// use time::macros::date;
    /// use calenda_rs::day_counting::DayCounter;
    /// use calenda_rs::countries::oceania::australia::AustraliaCalendar;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2023-02-01);
    ///
    /// let calendar = AustraliaCalendar;
    ///
    /// assert_eq!(calendar.business_day_count(date1, date2), 21);
    /// ```
    fn business_day_count(&self, date1: Date, date2: Date) -> i64 {
        let mut count = 0;
        let mut temp_date = date1;

        while temp_date <= date2 {
            if self.is_business_day(temp_date) {
                count += 1;
            }
            temp_date += Duration::days(1);
        }

        count
    }

    /// Computes the day count factor between two dates.
    ///
    /// # Arguments
    ///
    /// * `date1` - The first date.
    /// * `date2` - The second date.
    /// * `convention` - The day count convention.
    ///
    /// # Example
    ///
    /// ```
    /// use time::macros::date;
    /// use calenda_rs::day_counting::DayCounter;
    /// use calenda_rs::countries::oceania::australia::AustraliaCalendar;
    /// use calenda_rs::day_counting::DayCountConvention;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2024-01-01);
    ///
    /// let calendar = AustraliaCalendar;
    /// let convention = DayCountConvention::Actual_365_Actual;
    ///
    /// assert_eq!(calendar.day_count_factor(date1, date2, &convention), 0.997_267_759_562_841_5);
    /// ```
    fn day_count_factor(&self, date1: Date, date2: Date, convention: &DayCountConvention) -> f64 {
        convention.day_count_factor(date1, date2)
    }

    /// Compute the number of calendar days between each date in a vector of dates.
    ///
    /// # Arguments
    ///
    /// * `dates` - A vector of dates.
    ///
    /// # Example
    ///
    /// ```
    /// use time::macros::date;
    /// use calenda_rs::day_counting::DayCounter;
    /// use calenda_rs::countries::oceania::australia::AustraliaCalendar;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2024-01-01);
    /// let date3 = date!(2025-01-01);
    ///
    /// let dates = &[date1, date2, date3];
    /// let expected = vec![365, 366];
    ///
    /// let calendar = AustraliaCalendar;
    ///
    /// assert_eq!(calendar.calendar_day_counts(dates), expected);
    /// ```
    fn calendar_day_counts(&self, dates: &[Date]) -> Vec<i64> {
        dates
            .windows(2)
            .map(|window| self.calendar_day_count(window[0], window[1]))
            .collect()
    }

    /// Compute the number of calendar days between each date in a vector of dates.
    ///
    /// # Arguments
    ///
    /// * `dates` - A vector of dates.
    ///
    /// # Example
    ///
    /// ```
    /// use time::macros::date;
    /// use calenda_rs::day_counting::DayCounter;
    /// use calenda_rs::countries::oceania::australia::AustraliaCalendar;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2023-02-01);
    /// let date3 = date!(2023-03-01);
    ///
    /// let dates = &[date1, date2, date3];
    /// let expected = vec![21, 21];
    ///
    /// let calendar = AustraliaCalendar;
    ///
    /// assert_eq!(calendar.business_day_counts(dates), expected);
    /// ```
    fn business_day_counts(&self, dates: &[Date]) -> Vec<i64> {
        dates
            .windows(2)
            .map(|window| self.business_day_count(window[0], window[1]))
            .collect()
    }

    /// Compute the day count factors between each date in a vector of dates.
    ///
    /// # Arguments
    ///
    /// * `dates` - A vector of dates.
    ///
    /// # Example
    ///
    /// ```
    /// use time::macros::date;
    /// use calenda_rs::day_counting::DayCounter;
    /// use calenda_rs::countries::oceania::australia::AustraliaCalendar;
    /// use calenda_rs::day_counting::DayCountConvention;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2024-01-01);
    /// let date3 = date!(2025-01-01);
    ///
    /// let dates = &[date1, date2, date3];
    /// let expected = vec![0.997_267_759_562_841_5, 1.0];
    ///
    /// let calendar = AustraliaCalendar;
    /// let convention = DayCountConvention::Actual_365_Actual;
    ///
    /// assert_eq!(calendar.day_count_factors(dates, &convention), expected);
    /// ```
    fn day_count_factors(&self, dates: &[Date], convention: &DayCountConvention) -> Vec<f64> {
        dates
            .windows(2)
            .map(|window| self.day_count_factor(window[0], window[1], convention))
            .collect()
    }
}

impl Default for DayCountConvention {
    /// Default day count convention. Currently set to `Actual/Actual ISDA`.
    fn default() -> Self {
        Self::Actual_Actual_ISDA
    }
}

impl DayCountConvention {
    /// Entry point for day count factor calculation.
    #[rustfmt::skip]
    pub fn day_count_factor(&self, start_date: Date, end_date: Date) -> f64 {
        match self {
            Self::One_One               => Self::day_count_factor_one_one(start_date, end_date),
            Self::Actual_360            => Self::day_count_factor_actual_360(start_date, end_date),
            Self::Actual_364            => Self::day_count_factor_actual_364(start_date, end_date),
            Self::Actual_366            => Self::day_count_factor_actual_366(start_date, end_date),
            Self::Actual_365_25         => Self::day_count_factor_actual_365_25(start_date, end_date),
            Self::Actual_365_Actual     => Self::day_count_factor_actual_365_actual(start_date, end_date),
            Self::Actual_365_Fixed      => Self::day_count_factor_actual_365_fixed(start_date, end_date),
            Self::Actual_365_Leap       => Self::day_count_factor_actual_365_leap(start_date, end_date),
            Self::Actual_Actual_AFB     => Self::day_count_factor_actual_actual_afb(start_date, end_date),
            Self::Actual_Actual_ICMA    => Self::day_count_factor_actual_actual_icma(start_date, end_date),
            Self::Actual_Actual_ISDA    => Self::day_count_factor_actual_actual_isda(start_date, end_date),
            Self::No_Leap_360           => Self::day_count_factor_nl_360(start_date, end_date),
            Self::No_Leap_365           => Self::day_count_factor_nl_365(start_date, end_date),
            Self::Thirty_360_ISDA       => Self::day_count_factor_thirty_360_isda(start_date, end_date),
            Self::Thirty_E_360          => Self::day_count_factor_thirty_e_360(start_date, end_date),
            Self::Thirty_E_360_ISDA     => Self::day_count_factor_thirty_e_360_isda(start_date, end_date),
            Self::Thirty_E_365          => Self::day_count_factor_thirty_e_365(start_date, end_date),
            Self::Thirty_E_Plus_360     => Self::day_count_factor_thirty_e_plus_360(start_date, end_date),
            Self::Thirty_U_360          => Self::day_count_factor_thirty_u_360(start_date, end_date),
        }
    }
}
