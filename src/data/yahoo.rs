// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module to fetch data from Yahoo! Finance,
//! and store it in a Polars DataFrame object.

// use std::io::Cursor;
// use time::{macros::datetime, Date};

use polars::prelude::*;
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, TRAITS, AND ENUMS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Yahoo! Finance data struct.
pub struct YahooFinanceData {
    /// Ticker symbol, e.g. String::from("AAPL").
    pub ticker: Option<String>,
    /// Start date for the price history.
    pub start: Option<OffsetDateTime>,
    /// End date for the price history.
    pub end: Option<OffsetDateTime>,
    /// Price history time series.
    pub price_history: Option<DataFrame>,
    /// Options chain, if available.
    pub options_chain: Option<DataFrame>,
    /// Latest available quote.
    pub latest_quote: Option<DataFrame>,
}

/// Yahoo! Finance data reader trait.
pub trait YahooFinanceReader {
    /// Retrieves the price history from Yahoo! Finance.
    fn get_price_history(&mut self);
    /// Retrieves the options chain from Yahoo! Finance.
    fn get_options_chain(&mut self);
    /// Retrieves the latest quote from Yahoo! Finance.
    fn get_latest_quote(&mut self);
}

impl Default for YahooFinanceData {
    fn default() -> Self {
        Self {
            ticker: None,
            start: Some(OffsetDateTime::UNIX_EPOCH),
            end: Some(OffsetDateTime::now_utc()),
            price_history: None,
            options_chain: None,
            latest_quote: None,
        }
    }
}

impl YahooFinanceData {
    /// Creates a new Yahoo! Finance data struct.
    pub fn new(ticker: String) -> Self {
        Self {
            ticker: Some(ticker),
            ..Default::default()
        }
    }

    /// Sets the start date for the price history.
    pub fn set_start_date(&mut self, start: OffsetDateTime) {
        self.start = Some(start);
    }

    /// Sets the end date for the price history.
    pub fn set_end_date(&mut self, end: OffsetDateTime) {
        self.end = Some(end);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl YahooFinanceReader for YahooFinanceData {
    fn get_price_history(&mut self) {
        let provider = yahoo::YahooConnector::new();

        let response = tokio_test::block_on(provider.get_quote_history(
            self.ticker.as_ref().unwrap(),
            self.start.unwrap_or(OffsetDateTime::UNIX_EPOCH),
            self.end.unwrap_or(OffsetDateTime::now_utc()),
        ))
        .unwrap();

        let quotes = response.quotes().unwrap();

        // The timestamp from Yahoo! Finance is in seconds since UNIX Epoch (1970-01-01).
        // So we need to divide by the number of seconds in a day (86,400s) to get the date,
        // otherwise the date basically overflows out of the range.
        let date = quotes
            .iter()
            .map(|q| (q.timestamp / (24 * 60 * 60)) as i32)
            .collect::<Vec<_>>();
        let open = quotes.iter().map(|q| q.open).collect::<Vec<_>>();
        let high = quotes.iter().map(|q| q.high).collect::<Vec<_>>();
        let low = quotes.iter().map(|q| q.low).collect::<Vec<_>>();
        let close = quotes.iter().map(|q| q.close).collect::<Vec<_>>();
        let volume = quotes.iter().map(|q| q.volume as f64).collect::<Vec<_>>();
        let adjclose = quotes.iter().map(|q| q.adjclose).collect::<Vec<_>>();

        let df = df!(
            "date" => Series::new("date", date).cast(&DataType::Date).unwrap(),
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => volume,
            "adjusted" => adjclose
        );

        self.price_history = Some(df.unwrap());
    }

    fn get_options_chain(&mut self) {
        todo!()
    }

    fn get_latest_quote(&mut self) {
        todo!()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_yahoo {

    use super::*;

    #[test]
    fn test_get_price_history() {
        let mut yfd = YahooFinanceData::new("AAPL".to_string());

        yfd.set_start_date(time::macros::datetime!(2019 - 01 - 01 0:00 UTC));
        yfd.set_end_date(time::macros::datetime!(2020 - 01 - 01 0:00 UTC));

        yfd.get_price_history();

        println!("Apple's quotes: {:?}", yfd.price_history)
    }
}
