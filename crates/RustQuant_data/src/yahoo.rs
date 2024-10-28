// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Module to fetch data from Yahoo! Finance,
//! and store it in a Polars DataFrame object.

use polars::prelude::*;
use time::OffsetDateTime;
use yahoo::YahooError;
use yahoo_finance_api as yahoo;
use RustQuant_error::RustQuantError;

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
    /// Returns
    pub returns: Option<DataFrame>,
    /// Options chain, if available.
    pub options_chain: Option<DataFrame>,
    /// Latest available quote.
    pub latest_quote: Option<DataFrame>,
}

/// Return type for the Yahoo! Finance data struct.
pub enum ReturnsType {
    /// Arithmetic/simple returns.
    Arithmetic,
    /// Logarithmic returns.
    Logarithmic,
    /// Absolute returns.
    Absolute,
}

/// Yahoo! Finance data reader trait.
pub trait YahooFinanceReader {
    /// Retrieves the price history from Yahoo! Finance.
    fn get_price_history(&mut self) -> Result<(), RustQuantError>;

    // /// Retrieves the options chain from Yahoo! Finance.
    // fn get_options_chain(&mut self) -> Result<(), RustQuantError>;

    /// Retrieves the latest quote from Yahoo! Finance.
    fn get_latest_quote(&mut self) -> Result<(), RustQuantError>;
}

impl Default for YahooFinanceData {
    fn default() -> Self {
        Self {
            ticker: None,
            start: Some(OffsetDateTime::UNIX_EPOCH),
            end: Some(OffsetDateTime::now_utc()),
            price_history: None,
            returns: None,
            options_chain: None,
            latest_quote: None,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

    /// Sets both the start and end dates for the price history.
    pub fn set_date_range(&mut self, start: OffsetDateTime, end: OffsetDateTime) {
        self.start = Some(start);
        self.end = Some(end);
    }

    /// Computes the returns from the price history.
    pub fn compute_returns(&mut self, returns_type: ReturnsType) -> Result<(), RustQuantError> {
        if self.price_history.is_none() {
            self.get_price_history()?
        }

        // Closure to select all columns except for the date and volume columns.
        // These are: open, high, low, close, adjusted.
        let price_columns = || col("*").exclude(["date", "volume"]);

        // Compute the returns.
        match returns_type {
            ReturnsType::Arithmetic => {
                self.returns = Some(
                    self.price_history
                        .clone()
                        .ok_or(YahooError::EmptyDataSet)?
                        .lazy()
                        .select(vec![
                            col("date"),
                            col("volume"),
                            (price_columns() / price_columns().shift(lit(1)) - lit(1.))
                                .name()
                                .suffix("_arithmetic"),
                        ])
                        .collect()?,
                );
            }
            ReturnsType::Absolute => {
                self.returns = Some(
                    self.price_history
                        .clone()
                        .ok_or(YahooError::EmptyDataSet)?
                        .lazy()
                        .select(vec![
                            col("date"),
                            col("volume"),
                            (price_columns() - price_columns().shift(lit(1)))
                                .name()
                                .suffix("_absolute"),
                        ])
                        .collect()?,
                );
            }
            ReturnsType::Logarithmic => {
                fn logarithm(col: &Column) -> Series {
                    col.f64().unwrap().apply(|x| Some(x?.ln())).into_series()
                }

                // THIS IS EXTREMELY HACKY AND SHOULD BE FIXED
                // IF YOU SEE THIS, FEEL FREE TO SUBMIT A PULL REQUEST
                // If you venture past here, you'll see more .unwrap()
                // calls than you've ever seen before in your life.
                let mut prices = self.price_history.clone().ok_or(YahooError::EmptyDataSet)?;
                prices.apply("open", logarithm)?;
                prices.apply("high", logarithm)?;
                prices.apply("low", logarithm)?;
                prices.apply("close", logarithm)?;
                prices.apply("adjusted", logarithm)?;

                self.returns = Some(
                    prices
                        .lazy()
                        .select(vec![
                            col("date"),
                            col("volume"),
                            (price_columns() - price_columns().shift(lit(1)))
                                .name()
                                .suffix("_logarithmic"),
                        ])
                        .collect()?,
                );
            }
        }
        Ok(())
    }
}

impl YahooFinanceReader for YahooFinanceData {
    fn get_price_history(&mut self) -> Result<(), RustQuantError> {
        let provider = yahoo::YahooConnector::new()?;

        let response = tokio_test::block_on(provider.get_quote_history(
            self.ticker.as_ref().ok_or(RustQuantError::MissingInput(
                "No ticker provided.".to_string(),
            ))?,
            self.start.unwrap_or(OffsetDateTime::UNIX_EPOCH),
            self.end.unwrap_or(OffsetDateTime::now_utc()),
        ))?;

        let quotes = response.quotes()?;

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
            "date" => Series::new("date".into(), date).cast(&DataType::Date)?,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => volume,
            "adjusted" => adjclose
        );

        self.price_history = Some(df?);

        Ok(())
    }

    // fn get_options_chain(&mut self) -> Result<(), RustQuantError> {
    //     let provider = yahoo::YahooConnector::new()?;
    //     let response = tokio_test::block_on(provider.search_options(self.ticker.as_ref().ok_or(
    //         RustQuantError::MissingInput("No ticker provided.".to_string()),
    //     )?))?;

    //     let options = response.option_chain.result;

    //     // YOptionResult { name: "AAPL230526C00250000", strike: 250.0, last_trade_date: "2023-05-25 3:12PM EDT",
    //     //                 last_price: 250.0, bid: 250.0, ask: 250.0, change: 250.0, change_pct: 250.0,
    //     //                 volume: 0, open_interest: 0, impl_volatility: 250.0 }

    //     // CANNOT IMPLEMENT THIS YET, BECAUSE THE YAHOO FINANCE API
    //     // DOES NOT RETURN THE CORRECT OPTIONS CHAIN.
    //     // Issue opened: https://github.com/xemwebe/yahoo_finance_api/issues/28
    //     // Pull request opened: https://github.com/xemwebe/yahoo_finance_api/pull/29
    //     // Pull request was merged.
    //     // Now we need to wait for the next release of the yahoo_finance_api crate.

    //     let contract = options
    //         .iter()
    //         .map(|o| o.underlying_symbol)
    //         .collect::<Vec<_>>();
    //     let strike = options.iter().map(|o| o.strikes).collect::<Vec<_>>();
    //     let last_trade_date = options
    //         .iter()
    //         .map(|o| o.options.last_trade_date.clone()) //(o.last_trade_date / (24 * 60 * 60)) as i32)
    //         .collect::<Vec<_>>();
    //     let last_price = options
    //         .iter()
    //         .map(|o| o.options..last_price)
    //         .collect::<Vec<_>>();
    //     let bid = options.iter().map(|o| o.bid).collect::<Vec<_>>();
    //     let ask = options.iter().map(|o| o.ask).collect::<Vec<_>>();
    //     let change = options.iter().map(|o| o.change).collect::<Vec<_>>();
    //     let change_pct = options.iter().map(|o| o.change_pct).collect::<Vec<_>>();
    //     let volume = options.iter().map(|o| o.volume as u64).collect::<Vec<_>>();
    //     let open_interest = options
    //         .iter()
    //         .map(|o| o.open_interest as u64)
    //         .collect::<Vec<_>>();
    //     let impl_volatility = options
    //         .iter()
    //         .map(|o| o.impl_volatility)
    //         .collect::<Vec<_>>();

    //     let df = df!(
    //         "contract" => contract,
    //         "strike" => strike,
    //         "last_trade_date" => Series::new("last_trade_date".into(), last_trade_date)
    //             .cast(&DataType::Date)
    //             ?,
    //         "last_price" => last_price,
    //         "bid" => bid,
    //         "ask" => ask,
    //         "change" => change,
    //         "change_pct" => change_pct,
    //         "volume" => volume,
    //         "open_interest" => open_interest,
    //         "impl_volatility" => impl_volatility
    //         // "contract" => Series::new("date", date).cast(&DataType::Date)?,
    //     );

    //     self.options_chain = Some(df?);

    //     println!("{:?}", self.options_chain);

    //     Ok(())
    // }

    fn get_latest_quote(&mut self) -> Result<(), RustQuantError> {
        let provider = yahoo::YahooConnector::new()?;
        let response = tokio_test::block_on(
            provider.get_latest_quotes(
                self.ticker
                    .as_ref()
                    .ok_or(RustQuantError::MissingInput(
                        "No ticker provided.".to_string(),
                    ))?
                    .as_str(),
                "1d",
            ),
        )
        .unwrap();
        let quote = response.last_quote().unwrap();
        // Check if the timestamp is within the acceptable range

        let timestamp = vec![quote.timestamp];
        let open = vec![quote.open];
        let high = vec![quote.high];
        let low = vec![quote.low];
        let close = vec![quote.close];
        let volume = vec![quote.volume as f64];
        let adjclose = vec![quote.adjclose];

        let df = df!(
            "timestamp" => timestamp,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => volume,
            "adjusted" => adjclose
        );

        self.latest_quote = Some(df?);

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_yahoo {
    // cargo t test_yahoo --all-features  -- --nocapture

    use super::*;

    #[test]
    fn test_get_price_history() {
        let mut yfd = YahooFinanceData::new("AAPL".to_string());

        yfd.set_start_date(time::macros::datetime!(2019 - 01 - 01 0:00 UTC));
        yfd.set_end_date(time::macros::datetime!(2020 - 01 - 01 0:00 UTC));

        let _ = yfd.get_price_history();

        println!("Apple's quotes: {:?}", yfd.price_history)
    }

    #[test]
    fn test_compute_returns() {
        let mut yfd = YahooFinanceData::new("AAPL".to_string());

        yfd.set_start_date(time::macros::datetime!(2019 - 01 - 01 0:00 UTC));
        yfd.set_end_date(time::macros::datetime!(2020 - 01 - 01 0:00 UTC));

        let _ = yfd.compute_returns(ReturnsType::Logarithmic);

        println!("Apple's returns: {:?}", yfd.returns)
    }
    #[test]
    fn test_get_latest_quote() {
        let mut yfd = YahooFinanceData::new("AAPL".to_string());

        let _ = yfd.get_latest_quote();

        println!("Apple's latest quote: {:?}", yfd.latest_quote)
    }

    // #[test]
    // fn test_get_options_chain() {
    //     let mut yfd = YahooFinanceData::new("AAPL".to_string());

    //     let _ = yfd.get_options_chain();

    //     // println!("Apple's options chain: {:?}", yfd.options_chain)
    // }
}
