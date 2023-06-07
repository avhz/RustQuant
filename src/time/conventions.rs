// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Date rolling business day conventions.
///
/// From Wikipedia (https://en.wikipedia.org/wiki/Date_rolling):
/// """
/// In finance, date rolling occurs when a payment day or date used to
/// calculate accrued interest falls on a holiday, according to a given
/// business calendar. In this case the date is moved forward or backward in
/// time such that it falls in a business day, according with the
/// same business calendar.
/// """
pub enum BusinessDayConvention {
    /// Actual: paid on the actual day, even if it is a non-business day.
    Actual,
    /// Following business day: the payment date is rolled to the next business day.
    Following,
    /// Modified following business day: the payment date is rolled to the
    /// next business day, unless doing so
    /// would cause the payment to be in the next calendar month,
    /// in which case the payment date is rolled to the previous business day.
    /// Many institutions have month-end accounting procedures that necessitate this.
    ModifiedFollowing,
    /// Previous business day: the payment date is rolled to the previous business day.
    Preceding,
    /// Modified previous business day: the payment date is rolled to the previous
    /// business day, unless doing so would cause the payment to be in the previous
    /// calendar month, in which case the payment date is rolled to the next
    /// business day. Many institutions have month-end accounting procedures
    /// that necessitate this.
    ModifiedPreceding,
    /// Modified Rolling business day: the payment date is rolled to the next
    /// business day. The adjusted week date is used for the next coupon date.
    /// So adjustments are cumulative (excluding month change).
    ModifiedRolling,
}

/// Day count conventions.
///
/// From Wikipedia (https://en.wikipedia.org/wiki/Day_count_convention):
/// """
/// In finance, a day count convention determines how interest accrues
/// over time for a variety of investments, including bonds, notes,
/// loans, mortgages, medium-term notes, swaps, and forward rate agreements (FRAs).
/// This determines the number of days between two coupon payments,
/// thus calculating the amount transferred on payment dates and also the
/// accrued interest for dates between payments.[1] The day count is also
/// used to quantify periods of time when discounting a cash-flow to its
/// present value. When a security such as a bond is sold between interest
/// payment dates, the seller is eligible to some fraction of the coupon amount.
/// """
pub enum DayCountConvention {
    /// Actual/365 day count convention.
    Actual365,
    /// Actual/360 day count convention.
    Actual360,
    /// Actual/364 day count convention.
    Actual364,
    /// Thirty/360 day count convention.
    Thirty360,
    // TODO: Implement the following day count conventions.
    // There are fiddly techicalities to consider, such as leap years.
    // Also need some sort of calendar to determine which days are holidays, etc.
    // Thirty360_BondBasis,
    // Thirty360_US,
    // ThirtyE360,
    // ThirtyE360_ISDA,
    // ActualActual_ICMA,
    // ActualActual_ISDA,
    // Actual365L,
    // ActualActual_AFB,
    // OneOne,
}
