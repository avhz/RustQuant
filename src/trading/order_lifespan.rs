// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt;

/// Enum to indicate the lifespan of an order.
///
/// See here: https://www.interactivebrokers.com/en/trading/ordertypes.php
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderTimeInForce {
    /// """
    /// An order that uses the Good-Til-Canceled (GTC) time in force will
    /// continue to work until the order fills or is canceled 1. The ability
    /// to enter a bid well below the prevailing trading price for most asset
    /// classes, or an offer higher than its current level, allows an investor
    /// to place a resting order for days, weeks or months in advance without
    /// having to repeat the process each day. The GTC order type allows
    /// traders to pinpoint in advance levels at which they would like to
    /// enter or exit the market.
    /// """
    GoodTillCancelled,
    /// """
    /// The Immediate-or Cancel (IOC) time in force applied to an order
    /// dictates that any portion of the order that does not fill
    /// immediately will be canceled.   
    /// """
    ImmediateOrCancel,
    /// """
    /// Setting FOK as the time in force dictates that the entire order must
    /// execute immediately or be canceled. A trader might see a short-lived
    /// opportunity to buy or sell an option that would suit a strategy or
    /// fit within a portfolio. However, the time opportunity might be subject
    /// to buying or selling a minimum number contracts. The fill-or-kill order
    /// type is designed to ensure that the investor does not receive a partial
    /// fill that would not suit his current appetite. Failure to fill the
    /// entire order upon immediate submission to the market causes the system
    /// to cancel the order in its entirety.
    FillOrKill,
    /// `AllOrNone`: Order must be filled **in its entirety** and stays on
    /// the book until it is filled or cancelled.
    AllOrNone,
}

impl fmt::Display for OrderTimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderTimeInForce::GoodTillCancelled => write!(f, "GTC"),
            OrderTimeInForce::ImmediateOrCancel => write!(f, "IOC"),
            OrderTimeInForce::FillOrKill => write!(f, "FOK"),
            OrderTimeInForce::AllOrNone => write!(f, "AON"),
        }
    }
}
