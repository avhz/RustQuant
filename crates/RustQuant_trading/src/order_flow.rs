// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Order flow: trade classification and order flow toxicity (VPIN).
//!
//! A trade is buyer-initiated ([`OrderSide::BID`]) or seller-initiated
//! ([`OrderSide::ASK`]). Three classifiers sign trades from the data available:
//!
//! - [`TickRule`]: prices only. An uptick is a buy, a downtick a sell, and an
//!   unchanged price repeats the last non-zero tick.
//! - [`LeeReady`]: prices and the prevailing quote (Lee and Ready, 1991).
//!   Trades above the midpoint are buys, below are sells, and trades at the
//!   midpoint fall back to the tick rule.
//! - [`BulkVolumeClassifier`]: bars instead of trades (Easley, Lopez de Prado
//!   and O'Hara, 2012). A bar's volume is split into a buy share
//!   $\Phi(\Delta P / \sigma)$ and a sell share $1 - \Phi(\Delta P / \sigma)$.
//!
//! [`Vpin`] turns classified volume into the volume-synchronized probability
//! of informed trading of Easley, Lopez de Prado and O'Hara (2012).

use crate::order_side::OrderSide;
use std::collections::VecDeque;
use RustQuant_error::RustQuantError;
use RustQuant_math::distributions::{Distribution, Gaussian};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TICK RULE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Streaming tick-rule trade classifier.
///
/// ```
/// use RustQuant::trading::{order_flow::TickRule, order_side::OrderSide};
///
/// let mut tick_rule = TickRule::new();
///
/// assert_eq!(tick_rule.classify(10.00), None); // No previous trade.
/// assert_eq!(tick_rule.classify(10.01), Some(OrderSide::BID)); // Uptick.
/// assert_eq!(tick_rule.classify(10.01), Some(OrderSide::BID)); // Zero tick repeats.
/// assert_eq!(tick_rule.classify(10.00), Some(OrderSide::ASK)); // Downtick.
/// ```
#[derive(Debug, Clone, Default)]
pub struct TickRule {
    last_price: Option<f64>,
    last_side: Option<OrderSide>,
}

impl TickRule {
    /// Create a classifier with no history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Classify a trade at `price`.
    ///
    /// Returns `None` for the first trade and for trades at an unchanged
    /// price before any non-zero tick. Non-finite prices are ignored.
    pub fn classify(&mut self, price: f64) -> Option<OrderSide> {
        if !price.is_finite() {
            return None;
        }

        let side = match self.last_price {
            Some(previous) if price > previous => Some(OrderSide::BID),
            Some(previous) if price < previous => Some(OrderSide::ASK),
            Some(_) => self.last_side,
            None => None,
        };

        self.last_price = Some(price);
        if side.is_some() {
            self.last_side = side;
        }

        side
    }

    /// Forget all history.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEE-READY
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Streaming Lee-Ready (1991) trade classifier: the quote rule first, the
/// tick rule for trades at the midpoint.
///
/// Pass the quote that prevailed when the trade printed. If the quote is
/// unusable (non-finite, non-positive or crossed), the tick rule decides.
///
/// ```
/// use RustQuant::trading::{order_flow::LeeReady, order_side::OrderSide};
///
/// let mut lee_ready = LeeReady::new();
///
/// // At the midpoint with no tick history.
/// assert_eq!(lee_ready.classify(100.0, 99.5, 100.5), None);
/// // Above the midpoint.
/// assert_eq!(lee_ready.classify(100.25, 99.5, 100.5), Some(OrderSide::BID));
/// // At the midpoint, a downtick.
/// assert_eq!(lee_ready.classify(100.0, 99.5, 100.5), Some(OrderSide::ASK));
/// ```
#[derive(Debug, Clone, Default)]
pub struct LeeReady {
    tick_rule: TickRule,
}

impl LeeReady {
    /// Create a classifier with no history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Classify a trade at `price` against the prevailing `bid` and `ask`.
    pub fn classify(&mut self, price: f64, bid: f64, ask: f64) -> Option<OrderSide> {
        // The tick rule sees every trade so its history stays complete.
        let by_tick = self.tick_rule.classify(price);

        let quote_is_valid = bid.is_finite() && ask.is_finite() && bid > 0.0 && ask >= bid;

        if price.is_finite() && quote_is_valid {
            let mid = 0.5 * (bid + ask);

            if price > mid {
                return Some(OrderSide::BID);
            }
            if price < mid {
                return Some(OrderSide::ASK);
            }
        }

        by_tick
    }

    /// Forget all history.
    pub fn reset(&mut self) {
        self.tick_rule.reset();
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BULK VOLUME CLASSIFICATION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Streaming bulk volume classification (Easley, Lopez de Prado and O'Hara, 2012).
///
/// Feed one bar at a time (its closing price). The buy share of the bar's
/// volume is $\Phi(\Delta P / \sigma)$, where $\Delta P$ is the change from
/// the previous close and $\sigma$ the sample standard deviation of the last
/// `window` close changes, the current one included. If every change in the
/// window is zero, the bar is split evenly.
#[derive(Debug, Clone)]
pub struct BulkVolumeClassifier {
    window: usize,
    changes: VecDeque<f64>,
    last_price: Option<f64>,
}

impl BulkVolumeClassifier {
    /// Create a classifier whose sigma uses the last `window` close changes.
    ///
    /// # Errors
    ///
    /// Returns an error if `window` is less than 2.
    pub fn new(window: usize) -> Result<Self, RustQuantError> {
        if window < 2 {
            return Err(RustQuantError::InvalidArgument(format!(
                "BulkVolumeClassifier window must be at least 2, got {window}"
            )));
        }

        Ok(Self {
            window,
            changes: VecDeque::with_capacity(window + 1),
            last_price: None,
        })
    }

    /// Push a bar close and return the buy share of its volume, in `[0, 1]`.
    ///
    /// Returns `None` until `window` close changes have been seen.
    pub fn buy_fraction(&mut self, price: f64) -> Option<f64> {
        if !price.is_finite() {
            return None;
        }

        let previous = self.last_price.replace(price)?;
        let change = price - previous;

        self.changes.push_back(change);
        if self.changes.len() > self.window {
            self.changes.pop_front();
        }
        if self.changes.len() < self.window {
            return None;
        }

        let n = self.window as f64;
        let mean = self.changes.iter().sum::<f64>() / n;
        let variance = self
            .changes
            .iter()
            .map(|c| (c - mean) * (c - mean))
            .sum::<f64>()
            / (n - 1.0);
        let sigma = variance.sqrt();

        if sigma > 0.0 {
            Some(Gaussian::default().cdf(change / sigma))
        } else {
            Some(0.5)
        }
    }

    /// Push a bar and return its `(buy_volume, sell_volume)` split.
    pub fn split(&mut self, price: f64, volume: f64) -> Option<(f64, f64)> {
        let fraction = self.buy_fraction(price)?;

        Some((volume * fraction, volume * (1.0 - fraction)))
    }

    /// Forget all history (the window length is kept).
    pub fn reset(&mut self) {
        self.changes.clear();
        self.last_price = None;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// VPIN
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Streaming volume-synchronized probability of informed trading (VPIN).
///
/// Easley, Lopez de Prado and O'Hara (2012), "Flow Toxicity and Liquidity in a
/// High-frequency World", Review of Financial Studies 25(5).
///
/// Classified volume fills consecutive buckets of exactly $V$ units. A trade
/// that overflows a bucket is split, keeping its buy share, and the remainder
/// carries into the next bucket. Over the last $n$ full buckets:
///
/// $$
/// \text{VPIN} = \frac{1}{n V} \sum_{\tau} \left| V_\tau^B - V_\tau^S \right|
/// $$
///
/// ```
/// use RustQuant::trading::{order_flow::Vpin, order_side::OrderSide};
///
/// // Buckets of 10 units, VPIN over the last 2 buckets.
/// let mut vpin = Vpin::new(10.0, 2).unwrap();
///
/// vpin.update_side(OrderSide::BID, 6.0);
/// vpin.update_side(OrderSide::ASK, 4.0); // Bucket 1: |6 - 4| / 10 = 0.2.
/// let value = vpin.update_side(OrderSide::ASK, 10.0).unwrap(); // Bucket 2: 1.0.
///
/// assert!((value - 0.6).abs() < 1e-12);
/// ```
#[derive(Debug, Clone)]
pub struct Vpin {
    bucket_volume: f64,
    n_buckets: usize,
    bucket_buy: f64,
    bucket_sell: f64,
    bucket_fill: f64,
    imbalances: VecDeque<f64>,
    value: Option<f64>,
}

impl Vpin {
    /// Create a VPIN estimator over `n_buckets` buckets of `bucket_volume`
    /// units. ELO (2012) use 50 buckets per day, so `bucket_volume` is one
    /// fiftieth of average daily volume.
    ///
    /// # Errors
    ///
    /// Returns an error if `bucket_volume` is not finite and positive or
    /// `n_buckets` is zero.
    pub fn new(bucket_volume: f64, n_buckets: usize) -> Result<Self, RustQuantError> {
        if !(bucket_volume.is_finite() && bucket_volume > 0.0) {
            return Err(RustQuantError::InvalidArgument(format!(
                "Vpin bucket_volume must be finite and positive, got {bucket_volume}"
            )));
        }
        if n_buckets == 0 {
            return Err(RustQuantError::InvalidArgument(
                "Vpin n_buckets must be at least 1".to_string(),
            ));
        }

        Ok(Self {
            bucket_volume,
            n_buckets,
            bucket_buy: 0.0,
            bucket_sell: 0.0,
            bucket_fill: 0.0,
            imbalances: VecDeque::with_capacity(n_buckets + 1),
            value: None,
        })
    }

    /// Push a trade whose aggressor side is known (exchange flag, [`TickRule`]
    /// or [`LeeReady`]) and return the current VPIN, or `None` until
    /// `n_buckets` buckets are full.
    pub fn update_side(&mut self, side: OrderSide, volume: f64) -> Option<f64> {
        match side {
            OrderSide::BID => self.update(volume, 0.0),
            OrderSide::ASK => self.update(0.0, volume),
        }
    }

    /// Push classified buy and sell volume and return the current VPIN, or
    /// `None` until `n_buckets` buckets are full.
    ///
    /// Both may be non-zero (a bulk-classified bar). If the input spans
    /// several buckets, each portion keeps the input's buy share. Negative or
    /// non-finite volumes are ignored.
    pub fn update(&mut self, buy_volume: f64, sell_volume: f64) -> Option<f64> {
        let is_valid = |v: f64| v.is_finite() && v >= 0.0;

        if !(is_valid(buy_volume) && is_valid(sell_volume)) {
            return self.value;
        }

        let total = buy_volume + sell_volume;
        if total <= 0.0 {
            return self.value;
        }

        let buy_share = buy_volume / total;
        // Absorbs rounding, so a trade that exactly fills a bucket leaves no sliver.
        let epsilon = self.bucket_volume * 1e-12;
        let mut remaining = total;

        while remaining > epsilon {
            let space = self.bucket_volume - self.bucket_fill;

            if remaining < space - epsilon {
                self.bucket_buy += remaining * buy_share;
                self.bucket_sell += remaining * (1.0 - buy_share);
                self.bucket_fill += remaining;
                break;
            }

            self.bucket_buy += space * buy_share;
            self.bucket_sell += space * (1.0 - buy_share);
            remaining -= space;
            self.close_bucket();
        }

        self.value
    }

    fn close_bucket(&mut self) {
        let imbalance = ((self.bucket_buy - self.bucket_sell).abs() / self.bucket_volume).min(1.0);

        self.imbalances.push_back(imbalance);
        if self.imbalances.len() > self.n_buckets {
            self.imbalances.pop_front();
        }

        self.bucket_buy = 0.0;
        self.bucket_sell = 0.0;
        self.bucket_fill = 0.0;

        if self.imbalances.len() == self.n_buckets {
            let sum: f64 = self.imbalances.iter().sum();
            self.value = Some(sum / self.n_buckets as f64);
        }
    }

    /// The current VPIN, `None` until `n_buckets` buckets are full.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Order imbalance $|V^B - V^S| / V$ of each bucket in the window, oldest first.
    pub fn bucket_imbalances(&self) -> impl Iterator<Item = f64> + '_ {
        self.imbalances.iter().copied()
    }

    /// Forget all history (the parameters are kept).
    pub fn reset(&mut self) {
        self.bucket_buy = 0.0;
        self.bucket_sell = 0.0;
        self.bucket_fill = 0.0;
        self.imbalances.clear();
        self.value = None;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_order_flow {
    use super::*;
    use RustQuant_utils::assert_approx_equal;

    const EPS: f64 = 1e-12;

    fn alternating_unit_trades(vpin: &mut Vpin, n_trades: usize) {
        for i in 0..n_trades {
            let side = if i % 2 == 0 {
                OrderSide::BID
            } else {
                OrderSide::ASK
            };
            vpin.update_side(side, 1.0);
        }
    }

    #[test]
    fn test_tick_rule_ignores_non_finite_prices() {
        let mut tick_rule = TickRule::new();

        assert_eq!(tick_rule.classify(f64::NAN), None);
        assert_eq!(tick_rule.classify(10.0), None);
        assert_eq!(tick_rule.classify(f64::INFINITY), None);
        assert_eq!(tick_rule.classify(9.0), Some(OrderSide::ASK));

        tick_rule.reset();
        assert_eq!(tick_rule.classify(9.0), None);
    }

    #[test]
    fn test_lee_ready_uses_tick_rule_for_invalid_quotes() {
        let mut lee_ready = LeeReady::new();

        assert_eq!(lee_ready.classify(100.0, 99.0, 101.0), None);
        // Crossed quote: the uptick decides, not the quote.
        assert_eq!(
            lee_ready.classify(101.0, 102.0, 101.5),
            Some(OrderSide::BID)
        );
        // Below the midpoint of a valid quote, despite the uptick.
        assert_eq!(
            lee_ready.classify(102.0, 102.0, 103.0),
            Some(OrderSide::ASK)
        );
    }

    #[test]
    fn test_bulk_volume_classification_reference() {
        // Closes 100, 101, 100 give changes +1 and -1, so the sample sigma is
        // sqrt(2). The third bar's buy share is Phi(-1 / sqrt(2)) = erfc(1/2) / 2
        // = 0.239750061093467...
        let mut bvc = BulkVolumeClassifier::new(2).unwrap();

        assert_eq!(bvc.buy_fraction(100.0), None);
        assert_eq!(bvc.buy_fraction(101.0), None);
        let fraction = bvc.buy_fraction(100.0).unwrap();

        assert_approx_equal!(fraction, 0.239_750_061_093_467_4, EPS);
    }

    #[test]
    fn test_bulk_volume_classification_flat_window_splits_evenly() {
        let mut bvc = BulkVolumeClassifier::new(2).unwrap();

        bvc.split(100.0, 10.0);
        bvc.split(100.0, 10.0);

        assert_eq!(bvc.split(100.0, 10.0), Some((5.0, 5.0)));
        assert!(BulkVolumeClassifier::new(1).is_err());
    }

    #[test]
    fn test_vpin_odd_buckets() {
        // Ten buys and ten sells of one unit, alternating, into buckets of five:
        // every bucket holds three of one side and two of the other, so each
        // imbalance is |3 - 2| / 5 = 0.2 and VPIN is 0.2.
        let mut vpin = Vpin::new(5.0, 4).unwrap();
        alternating_unit_trades(&mut vpin, 20);

        assert_approx_equal!(vpin.value().unwrap(), 0.2, EPS);
    }

    #[test]
    fn test_vpin_even_buckets() {
        // Buckets of four hold two buys and two sells each: VPIN is exactly 0.
        let mut vpin = Vpin::new(4.0, 5).unwrap();
        alternating_unit_trades(&mut vpin, 20);

        assert_eq!(vpin.value(), Some(0.0));
    }

    #[test]
    fn test_vpin_large_trade_spans_buckets() {
        // Buy 25 fills two all-buy buckets (1.0, 1.0) and leaves 5 in a third.
        // Sell 15 completes the third at 5 vs 5 (0.0) and fills a fourth (1.0).
        // VPIN = (1 + 1 + 0 + 1) / 4 = 0.75.
        let mut vpin = Vpin::new(10.0, 4).unwrap();

        assert_eq!(vpin.update_side(OrderSide::BID, 25.0), None);
        let value = vpin.update_side(OrderSide::ASK, 15.0).unwrap();

        assert_approx_equal!(value, 0.75, EPS);
        assert_eq!(
            vpin.bucket_imbalances().collect::<Vec<_>>(),
            vec![1.0, 1.0, 0.0, 1.0]
        );
    }

    #[test]
    fn test_vpin_split_keeps_buy_share() {
        // 15 buy + 5 sell is a 0.75 buy share: each bucket gets 7.5 vs 2.5 -> 0.5.
        let mut vpin = Vpin::new(10.0, 2).unwrap();

        let value = vpin.update(15.0, 5.0).unwrap();

        assert_approx_equal!(value, 0.5, EPS);
    }

    #[test]
    fn test_vpin_from_bulk_classified_bars() {
        // The bulk classification reference bar with volume 10 fills one bucket:
        // |2 Phi(-1 / sqrt(2)) - 1| = erf(1/2) = 0.520499877813046...
        let mut bvc = BulkVolumeClassifier::new(2).unwrap();
        let mut vpin = Vpin::new(10.0, 1).unwrap();

        for close in [100.0, 101.0, 100.0] {
            if let Some((buy, sell)) = bvc.split(close, 10.0) {
                vpin.update(buy, sell);
            }
        }

        assert_approx_equal!(vpin.value().unwrap(), 0.520_499_877_813_046_5, EPS);
    }

    #[test]
    fn test_vpin_rejects_invalid_input() {
        assert!(Vpin::new(0.0, 2).is_err());
        assert!(Vpin::new(f64::NAN, 2).is_err());
        assert!(Vpin::new(10.0, 0).is_err());

        let mut vpin = Vpin::new(10.0, 1).unwrap();
        vpin.update(f64::NAN, 1.0);
        vpin.update(-1.0, 20.0);
        vpin.update(0.0, 0.0);
        assert_eq!(vpin.value(), None);

        vpin.update_side(OrderSide::BID, 10.0);
        assert_eq!(vpin.value(), Some(1.0));

        vpin.reset();
        assert_eq!(vpin.value(), None);
        assert_eq!(vpin.bucket_imbalances().count(), 0);
    }
}
