// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Enum to indicate the lifespan of an order.
pub enum TimeInForce {
    /// `GoodTillCancelled`: Order is valid until it is filled or cancelled.
    GoodTillCancelled,
    /// `ImmediateOrCancel`:
    /// An Immediate-Or-Cancel (IOC) order is an order to
    /// buy or sell a stock that must be executed immediately.
    /// Any portion of an IOC order that cannot be filled immediately
    /// will be cancelled.
    ImmediateOrCancel,
    /// `FillOrKill`:
    /// A Fill-Or-Kill (FOK) order is an order to buy or sell a stock that
    /// must be executed immediately in its entirety;
    /// otherwise, the entire order will be cancelled
    /// (i.e., no partial execution of the order is allowed).
    FillOrKill,
    /// `AllOrNone`: Order must be filled **in its entirety** and stays on the book until it is filled or cancelled.
    AllOrNone,
}
