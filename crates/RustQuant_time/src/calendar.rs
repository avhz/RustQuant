// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2022-2024 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module defines a `Calendar` type and its methods.

use crate::countries::*;
use crate::date_rolling::*;
use crate::day_counting::*;
use crate::schedule::Schedule;

use pyo3::{pyclass, pymethods};
use std::collections::BTreeSet;
use time::{Date, Duration, Weekday};

/// Market calendars as defined by MIC (ISO 10383) codes.
#[derive(Debug, Clone, Copy)]
#[pyclass]
pub enum Market {
    /// Null calendar.
    ///
    /// Usually used when you want to create a completely
    /// custom calendar via `add_holidays`.
    None,
    /// Weekend-only calendar.
    Weekends,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // COUNTRIES
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Argentina national calendar.
    Argentina,
    /// Australia national calendar.
    Australia,
    /// Austria national calendar.
    Austria,
    /// Botswana national calendar.
    Botswana,
    /// Brazil national calendar.
    Brazil,
    /// Canada national calendar.
    Canada,
    /// Chile national calendar.
    Chile,
    /// China national calendar.
    China,
    /// Czech Republic national calendar.
    CzechRepublic,
    /// Denmark national calendar.
    Denmark,
    /// Finland national calendar.
    Finland,
    /// France national calendar.
    France,
    /// Germany national calendar.
    Germany,
    /// Hong Kong national calendar.
    HongKong,
    /// Hungary national calendar.
    Hungary,
    /// Iceland national calendar.
    Iceland,
    /// India national calendar.
    India,
    /// Indonesia national calendar.
    Indonesia,
    /// Israel national calendar.
    Israel,
    /// Italy national calendar.
    Italy,
    /// Mexico national calendar.
    Mexico,
    /// Netherlands national calendar.
    Netherlands,
    /// New Zealand national calendar.
    NewZealand,
    /// Singapore national calendar.
    Singapore,
    /// United Kingdom national calendar.
    UnitedKingdom,
    /// United States national calendar.
    UnitedStates,
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // MARKETS / EXCHANGES
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // /// New York Stock Exchange
    // XNYS,
    // /// NASDAQ Stock Market
    // XNAS,
    // /// Australian Securities Exchange
    // XASX,
    // /// Chicago Board Options Exchange
    // XCBO,
    // // extend as necessary ...
}

/// Calendar struct.
#[derive(Debug, Clone)]
#[pyclass]
pub struct Calendar {
    /// Market the calendar is associated with.
    pub market: Market,

    // Additional holidays for users to add if required.
    extra: BTreeSet<Date>,
}

#[pymethods]
impl Calendar {
    /// Create a new calendar instance.
    #[new]
    pub const fn new(market: Market) -> Self {
        Self {
            market,
            extra: BTreeSet::new(),
        }
    }

    /// Get the market associated with the calendar.
    pub fn market(&self) -> Market {
        self.market
    }

    /// Add a holiday to the calendar.
    pub fn add_holiday(&mut self, date: Date) {
        self.extra.insert(date);
    }

    /// Add multiple holidays to the calendar.
    pub fn add_holidays(&mut self, dates: Vec<Date>) {
        for d in dates {
            self.add_holiday(d);
        }
    }

    /// Get the extra holidays.
    pub fn extra_holidays(&self) -> &BTreeSet<Date> {
        &self.extra
    }

    /// Check if a date is a business day.
    pub fn is_business_day(&self, date: Date) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }

    /// Check if a date is a weekend.
    pub fn is_weekend(&self, date: Date) -> bool {
        let weekend = match self.market {
            Market::Israel => [Weekday::Friday, Weekday::Saturday],
            _ => [Weekday::Saturday, Weekday::Sunday],
        };

        weekend.contains(&date.weekday())
    }

    /// Roll a date given a DateRollingConvention type.
    pub fn roll_date(&self, date: Date, convention: DateRollingConvention) -> Date {
        match convention {
            DateRollingConvention::Actual => DateRollingConvention::roll_date_actual(date, self),
            DateRollingConvention::Following => {
                DateRollingConvention::roll_date_following(date, self)
            }
            DateRollingConvention::ModifiedFollowing => {
                DateRollingConvention::roll_date_modified_following(date, self)
            }
            DateRollingConvention::Preceding => {
                DateRollingConvention::roll_date_preceding(date, self)
            }
            DateRollingConvention::ModifiedPreceding => {
                DateRollingConvention::roll_date_modified_preceding(date, self)
            }
            DateRollingConvention::ModifiedRolling => {
                DateRollingConvention::roll_date_modified_rolling(date, self)
            }
        }
    }

    /// Roll multiple dates given a DateRollingConvention type.
    pub fn roll_dates(&self, dates: Vec<Date>, convention: DateRollingConvention) -> Vec<Date> {
        dates
            .into_iter()
            .map(|date| self.roll_date(date, convention))
            .collect()
    }

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
    /// use RustQuant::time::countries::australia::AustraliaCalendar;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2025-01-01);
    ///
    /// let calendar = AustraliaCalendar;
    ///
    /// assert_eq!(calendar.calendar_day_count(date1, date2), 731);
    /// ```
    pub fn calendar_day_count(&self, date1: Date, date2: Date) -> i64 {
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
    /// use RustQuant::time::countries::australia::AustraliaCalendar;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2023-02-01);
    ///
    /// let calendar = AustraliaCalendar;
    ///
    /// assert_eq!(calendar.business_day_count(date1, date2), 21);
    /// ```
    pub fn business_day_count(&self, date1: Date, date2: Date) -> i64 {
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
    /// use RustQuant::time::day_counting::DayCounter;
    /// use RustQuant::time::countries::oceania::australia::AustraliaCalendar;
    /// use RustQuant::time::day_counting::DayCountConvention;
    ///
    /// let date1 = date!(2023-01-01);
    /// let date2 = date!(2024-01-01);
    ///
    /// let calendar = AustraliaCalendar;
    /// let convention = DayCountConvention::Actual_365_Actual;
    ///
    /// assert_eq!(calendar.day_count_factor(date1, date2, &convention), 0.997_267_759_562_841_5);
    /// ```
    pub fn day_count_factor(
        &self,
        date1: Date,
        date2: Date,
        convention: DayCountConvention,
    ) -> f64 {
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
    /// use RustQuant::time::day_counting::DayCounter;
    /// use RustQuant::time::countries::oceania::australia::AustraliaCalendar;
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
    pub fn calendar_day_counts(&self, dates: Vec<Date>) -> Vec<i64> {
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
    /// use RustQuant::time::day_counting::DayCounter;
    /// use RustQuant::time::countries::oceania::australia::AustraliaCalendar;
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
    pub fn business_day_counts(&self, dates: Vec<Date>) -> Vec<i64> {
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
    /// use RustQuant::time::day_counting::DayCounter;
    /// use RustQuant::time::countries::oceania::australia::AustraliaCalendar;
    /// use RustQuant::time::day_counting::DayCountConvention;
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
    pub fn day_count_factors(&self, dates: Vec<Date>, convention: DayCountConvention) -> Vec<f64> {
        dates
            .windows(2)
            .map(|window| self.day_count_factor(window[0], window[1], convention))
            .collect()
    }

    /// Generate a schedule from a set of dates.
    pub fn generate_schedule_from_dates(
        &self,
        dates: Vec<Date>,
        date_rolling_convention: DateRollingConvention,
        day_counting_convention: DayCountConvention,
    ) -> Schedule {
        let today = crate::today();

        // First we need to roll the dates according to a given convention.
        let rolled_dates = self.roll_dates(dates, date_rolling_convention);

        // Then we need to compute the day count factors.
        let mut day_count_factors =
            self.day_count_factors(rolled_dates.clone(), day_counting_convention);
        day_count_factors.insert(
            0,
            self.day_count_factor(today, rolled_dates[0], day_counting_convention),
        );

        Schedule {
            dates: rolled_dates,
            day_count_factors,
            day_counting_convention,
            date_rolling_convention,
        }
    }

    /// Check if a date is a holiday.
    pub fn is_holiday(&self, date: Date) -> bool {
        self.extra.contains(&date)
            || match self.market {
                Market::Argentina => is_holiday_impl_argentina(date),
                Market::Australia => is_holiday_impl_australia(date),
                Market::Austria => is_holiday_impl_austria(date),
                Market::Botswana => is_holiday_impl_botswana(date),
                Market::Brazil => is_holiday_impl_brazil(date),
                Market::Canada => is_holiday_impl_canada(date),
                Market::Chile => is_holiday_impl_chile(date),
                Market::China => is_holiday_impl_china(date),
                Market::CzechRepublic => is_holiday_impl_czech_republic(date),
                Market::Denmark => is_holiday_impl_denmark(date),
                Market::Finland => is_holiday_impl_finland(date),
                Market::France => is_holiday_impl_france(date),
                Market::Germany => is_holiday_impl_germany(date),
                Market::HongKong => is_holiday_impl_hong_kong(date),
                Market::Hungary => is_holiday_impl_hungary(date),
                Market::Iceland => is_holiday_impl_iceland(date),
                Market::India => is_holiday_impl_india(date),
                Market::Indonesia => is_holiday_impl_indonesia(date),
                Market::Israel => is_holiday_impl_israel(date),
                Market::Italy => is_holiday_impl_italy(date),
                Market::Mexico => is_holiday_impl_mexico(date),
                Market::Netherlands => is_holiday_impl_netherlands(date),
                Market::NewZealand => is_holiday_impl_new_zealand(date),
                Market::Singapore => is_holiday_impl_singapore(date),
                Market::UnitedKingdom => is_holiday_impl_united_kingdom(date),
                Market::UnitedStates => is_holiday_impl_united_states(date),
                // Special case markets:
                Market::None => false,
                Market::Weekends => false,
            }
    }
}
