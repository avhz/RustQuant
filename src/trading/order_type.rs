// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::fmt;

/// Order type enum.
/// Definitions from:   
///     - <https://www.interactivebrokers.com/en/trading/ordertypes.php>
///     - <https://www.nasdaqtrader.com/content/productsservices/trading/ordertypesg.pdf>
#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    /// """
    /// A Market order is an order to buy or sell at the market bid or
    /// offer price. A market order may increase the likelihood of a fill
    /// and the speed of execution, but unlike the Limit order a Market order
    /// provides no price protection and may fill at a price far lower/higher
    /// than the current displayed bid/ask.
    /// """
    Market,

    /// """
    /// A Limit order is an order to buy or sell at a specified price or better.
    /// The Limit order ensures that if the order fills, it will not fill at a
    /// price less favorable than your limit price, but it does not guarantee a fill.
    /// """
    Limit,

    /// """
    /// A Stop order is an instruction to submit a buy or sell market order
    /// if and when the user-specified stop trigger price is attained or
    /// penetrated. A Stop order is not guaranteed a specific execution price
    /// and may execute significantly away from its stop price. A Sell Stop
    /// order is always placed below the current market price and is typically
    /// used to limit a loss or protect a profit on a long stock position.
    /// A Buy Stop order is always placed above the current market price.
    /// It is typically used to limit a loss or help protect a profit
    /// on a short sale.
    /// """
    Stop,

    /// """
    /// A Stop-Limit order is an instruction to submit a buy or sell limit
    /// order when the user-specified stop trigger price is attained or
    /// penetrated. The order has two basic components: the stop price
    /// and the limit price. When a trade has occurred at or through the
    /// stop price, the order becomes executable and enters the market as
    /// a limit order, which is an order to buy or sell at a
    /// specified price or better.
    /// """
    StopLimit,

    /// """
    /// A sell trailing stop order sets the stop price at a fixed amount
    /// below the market price with an attached "trailing" amount. As the
    /// market price rises, the stop price rises by the trail amount, but
    /// if the stock price falls, the stop loss price doesn't change, and
    /// a market order is submitted when the stop price is hit. This technique
    /// is designed to allow an investor to specify a limit on the maximum
    /// possible loss, without setting a limit on the maximum possible gain.
    /// "Buy" trailing stop orders are the mirror image of sell trailing
    /// stop orders, and are most appropriate for use in falling markets.
    /// """
    TrailingStop,

    /// """
    /// A trailing stop limit order is designed to allow an investor to
    /// specify a limit on the maximum possible loss, without setting a
    /// limit on the maximum possible gain. A SELL trailing stop limit
    /// moves with the market price, and continually recalculates the stop
    /// trigger price at a fixed amount below the market price, based on
    /// the user-defined "trailing" amount. The limit order price is also
    /// continually recalculated based on the limit offset. As the market
    /// price rises, both the stop price and the limit price rise by the
    /// trail amount and limit offset respectively, but if the stock price
    /// falls, the stop price remains unchanged, and when the stop price
    /// is hit a limit order is submitted at the last calculated limit
    /// price. A "Buy" trailing stop limit order is the mirror image of a
    /// sell trailing stop limit, and is generally used in falling markets.
    /// """
    TrailingStopLimit,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "MARKET"),
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::Stop => write!(f, "STOP"),
            OrderType::StopLimit => write!(f, "STOP_LIMIT"),
            OrderType::TrailingStop => write!(f, "TRAILING_STOP"),
            OrderType::TrailingStopLimit => write!(f, "TRAILING_STOP_LIMIT"),
        }
    }
}
